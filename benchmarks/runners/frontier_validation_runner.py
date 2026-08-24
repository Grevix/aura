#!/usr/bin/env python3
"""AURA Frontier Validation runner.

Only records measurements returned by the executing environment. Any unavailable
measurement is null, and any failed model run is NOT_EXECUTED/FAILED rather than
being converted to an estimate.
"""
from __future__ import annotations
import argparse, csv, datetime as dt, hashlib, json, os, platform, shutil, subprocess, sys, time, traceback
from pathlib import Path

PROMPTS = [
    "Explain why layer-wise weight streaming reduces peak VRAM but increases latency.",
    "Write a Rust function that computes a rolling average without allocating per item.",
    "Compare dense transformers with sparse mixture-of-experts models.",
    "What is the difference between TTFT and decode throughput?",
    "Design a reproducible benchmark for CPU-to-GPU tensor transfers.",
    "Give three failure modes of asynchronous prefetching and mitigations.",
    "Summarize the role of the KV cache during autoregressive decoding.",
    "Explain how safetensors can support selective tensor reads.",
    "What metadata is required to plan out-of-core inference?",
    "Describe double buffering for streamed neural-network layers.",
    "Write a JSON schema for a benchmark result with provenance.",
    "Why can a model be memory-feasible but operationally impractical?",
    "Compare NVMe bandwidth, PCIe bandwidth, and GPU memory bandwidth.",
    "How should an inference runtime handle an out-of-memory error?",
    "Explain expert routing and top-k selection in an MoE layer.",
    "Design a cache eviction policy for frequently selected experts.",
    "What measurements are needed to validate a prefetcher?",
    "Explain why simulated throughput must not be reported as measured throughput.",
    "Write pseudocode for a bounded producer-consumer tensor pipeline.",
    "How does quantization change the memory and bandwidth equations?",
    "Describe a fair comparison between two inference runtimes.",
    "What is the purpose of constructing a model on the meta device?",
    "Explain CPU offload versus disk offload.",
    "How would long context affect KV-cache memory?",
    "What evidence is required before claiming model support?",
    "Design telemetry for layer load, transfer, compute, and eviction events.",
    "Explain backpressure in an out-of-core execution scheduler.",
    "What makes a benchmark reproducible across cloud GPU instances?",
    "Compare page cache reads with explicit asynchronous file I/O.",
    "How should a runtime report a model that cannot be downloaded?",
    "Describe the difference between active and total parameters in an MoE model.",
    "Write a short checklist for auditing a model-loading implementation.",
    "Why does the largest layer matter for minimum VRAM?",
    "Explain how pinned host memory affects transfer performance.",
    "What should be kept resident during streamed generation?",
    "Describe a safe experiment for measuring peak RSS.",
    "How can a notebook avoid confusing cloud execution with local execution?",
    "What is a model revision and why must it be recorded?",
    "Explain why generation correctness and performance are separate tests.",
    "Design a smoke test that generates exactly one token.",
    "What causes prefetch misses?",
    "How can a runtime measure disk bytes read without guessing?",
    "Describe the trade-off between expert granularity and I/O overhead.",
    "What does an honest NOT_EXECUTED result contain?",
    "Explain why downloading a checkpoint is not proof that it can run.",
    "How would you compare a 4-bit checkpoint with an unquantized checkpoint?",
    "What is the role of synchronization after an asynchronous GPU copy?",
    "Describe a hardware profile needed for frontier-model experiments.",
    "How should benchmark artifacts be committed to a source repository?",
    "State the difference between PlannerEstimated, Simulated, and REAL results.",
]

SCHEMA_VERSION = "aura.frontier.validation.v1"

def now(): return dt.datetime.now(dt.timezone.utc).isoformat()
def sha256_file(p):
    h=hashlib.sha256()
    with open(p,'rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''): h.update(b)
    return h.hexdigest()
def rss():
    try:
        import psutil; return psutil.Process().memory_info().rss
    except Exception: return None
def gpu_info():
    try:
        out=subprocess.check_output(['nvidia-smi','--query-gpu=index,name,memory.total,memory.used,utilization.gpu,power.draw,temperature.gpu','--format=csv,noheader,nounits'], text=True, stderr=subprocess.DEVNULL)
        rows=[]
        for line in out.strip().splitlines():
            x=[v.strip() for v in line.split(',')]
            if len(x)>=7: rows.append({'index':int(x[0]),'name':x[1],'memory_total_mb':float(x[2]),'memory_used_mb':float(x[3]),'utilization_gpu_pct':float(x[4]),'power_w':float(x[5]),'temperature_c':float(x[6])})
        return rows
    except Exception: return []
def gpu_peak():
    vals=gpu_info(); return max([x['memory_used_mb'] for x in vals], default=None)
def disk_bytes(path):
    try: return shutil.disk_usage(path).used
    except Exception: return None
def model_disk(path):
    try:
        return sum(p.stat().st_size for p in Path(path).rglob('*') if p.is_file())
    except Exception: return None

def hardware():
    try:
        import torch
        cuda_available=bool(torch.cuda.is_available()); torch_version=getattr(torch,'__version__',None)
    except Exception:
        cuda_available=False; torch_version=None
    try:
        import psutil
        ram_total=psutil.virtual_memory().total
    except Exception:
        ram_total=None
    return {'execution_location':'remote/cloud or local as reported by this process','hostname':platform.node(),'python':sys.version.split()[0],'platform':platform.platform(),'cpu':platform.processor(),'ram_total_bytes':ram_total,'cuda_available':cuda_available,'cuda_devices':gpu_info(),'torch':torch_version}

def inspect_model(repo, revision=None, token=None, cache_dir=None):
    t=time.perf_counter()
    try:
        from huggingface_hub import snapshot_download
    except Exception as e:
        return {'schema_version':SCHEMA_VERSION,'status':'NOT_EXECUTED','execution_class':'REAL','execution_scope':'Remote/Cloud execution' if os.getenv('COLAB_GPU') or os.getenv('KAGGLE_KERNEL_RUN_TYPE') else 'REAL execution','repo_id':repo,'revision':revision,'download_or_cache_time_ms':None,'disk_bytes':None,'config':{},'failure':{'type':type(e).__name__,'message':str(e)},'hardware':{'hostname':platform.node(),'python':sys.version.split()[0]}}
    try:
        p=snapshot_download(repo_id=repo, revision=revision, token=token, cache_dir=cache_dir)
    except Exception as e:
        return {'schema_version':SCHEMA_VERSION,'status':'NOT_EXECUTED','execution_class':'REAL','execution_scope':'Remote/Cloud execution' if os.getenv('COLAB_GPU') or os.getenv('KAGGLE_KERNEL_RUN_TYPE') else 'REAL execution','repo_id':repo,'revision':revision,'download_or_cache_time_ms':(time.perf_counter()-t)*1000,'disk_bytes':None,'config':{},'failure':{'type':type(e).__name__,'message':str(e)},'hardware':{'hostname':platform.node(),'python':sys.version.split()[0]}}
    elapsed=(time.perf_counter()-t)*1000
    cfg={}
    for name in ('config.json','generation_config.json'):
        q=Path(p)/name
        if q.exists():
            try: cfg.update(json.loads(q.read_text()))
            except Exception: pass
    return {'schema_version':SCHEMA_VERSION,'status':'INSPECTED','execution_class':'REAL','execution_scope':'Remote/Cloud execution' if os.getenv('COLAB_GPU') or os.getenv('KAGGLE_KERNEL_RUN_TYPE') else 'REAL execution','repo_id':repo,'revision':revision,'snapshot_path':p,'download_or_cache_time_ms':elapsed,'disk_bytes':model_disk(p),'config':cfg,'hardware':hardware()}

def run_model(repo, revision, backend, prompts, max_new_tokens, token, cache_dir, do_generate):
    import torch
    result={'schema_version':SCHEMA_VERSION,'run_id':hashlib.sha256(f'{repo}|{revision}|{backend}|{now()}'.encode()).hexdigest()[:16],'started_at':now(),'repo_id':repo,'revision_requested':revision,'backend':backend,'execution_class':'REAL','execution_scope':'Remote/Cloud execution' if os.getenv('COLAB_GPU') or os.getenv('KAGGLE_KERNEL_RUN_TYPE') else 'REAL execution','hardware_before':hardware(),'status':'NOT_EXECUTED','failure':None,'model':{},'measurements':[],'events':[]}
    t0=time.perf_counter(); snap=None; model=None; tok=None; counters={'layer_loads':0,'layer_moves':0,'expert_loads':0,'prefetch_hooks':0}
    try:
        from huggingface_hub import snapshot_download
        snap=snapshot_download(repo_id=repo, revision=revision, token=token, cache_dir=cache_dir)
        result['model']={'snapshot_path':snap,'disk_bytes_after_download':model_disk(snap),'snapshot_sha256_manifest':{p.name:sha256_file(p) for p in Path(snap).glob('*.json') if p.is_file()}}
        download_ms=(time.perf_counter()-t0)*1000
        load_start=time.perf_counter()
        if backend=='airllm':
            from airllm import AutoModel
            import airllm.airllm_base as ab
            orig_load=ab.AirLLMBaseModel.load_layer_to_cpu
            orig_move=ab.AirLLMBaseModel.move_layer_to_device
            orig_pre=ab.AirLLMBaseModel._pre_hook
            orig_expert=ab.AirLLMBaseModel._expert_pre_hook
            def load(self,*a,**k): counters['layer_loads']+=1; return orig_load(self,*a,**k)
            def move(self,*a,**k): counters['layer_moves']+=1; return orig_move(self,*a,**k)
            def pre(self,*a,**k): counters['prefetch_hooks']+=1; return orig_pre(self,*a,**k)
            def expert(self,*a,**k): counters['expert_loads']+=1; return orig_expert(self,*a,**k)
            ab.AirLLMBaseModel.load_layer_to_cpu=load; ab.AirLLMBaseModel.move_layer_to_device=move; ab.AirLLMBaseModel._pre_hook=pre; ab.AirLLMBaseModel._expert_pre_hook=expert
            model=AutoModel.from_pretrained(snap, prefetching=True, profiling_mode=True)
            tok=model.tokenizer
        elif backend=='transformers':
            from transformers import AutoTokenizer, AutoModelForCausalLM
            tok=AutoTokenizer.from_pretrained(snap, token=token)
            model=AutoModelForCausalLM.from_pretrained(snap, torch_dtype='auto', device_map='auto', low_cpu_mem_usage=True, token=token)
            model.eval()
        else: raise ValueError('backend must be airllm or transformers')
        load_ms=(time.perf_counter()-load_start)*1000
        result['status']='LOADED'; result['measurements'].append({'phase':'load','download_ms':download_ms,'model_load_ms':load_ms,'ram_bytes':rss(),'vram_used_mb':gpu_peak(),'disk_bytes':model_disk(snap),'timestamp':now()})
        if not do_generate: result['status']='EXECUTED_NO_GENERATION'; return result
        for i,prompt in enumerate(prompts):
            before_ram=rss(); before_gpu=gpu_peak(); start=time.perf_counter()
            try:
                inputs=tok(prompt, return_tensors='pt')
                if backend=='airllm':
                    inputs={k:v.cuda() for k,v in inputs.items()}
                else:
                    dev=next(model.parameters()).device; inputs={k:v.to(dev) for k,v in inputs.items()}
                input_tokens=int(inputs['input_ids'].shape[-1]); gen_start=time.perf_counter();
                with torch.inference_mode(): out=model.generate(**inputs,max_new_tokens=max_new_tokens,do_sample=False,use_cache=True)
                total_ms=(time.perf_counter()-start)*1000; gen_ms=(time.perf_counter()-gen_start)*1000
                out_tokens=int(out.shape[-1]-input_tokens); text=tok.decode(out[0],skip_special_tokens=True)
                result['measurements'].append({'phase':'generation','prompt_index':i,'prompt_sha256':hashlib.sha256(prompt.encode()).hexdigest(),'input_tokens':input_tokens,'output_tokens':out_tokens,'ttft_ms':None,'decode_tok_per_s':(out_tokens/(gen_ms/1000)) if gen_ms else None,'total_ms':total_ms,'ram_before_bytes':before_ram,'ram_after_bytes':rss(),'vram_before_mb':before_gpu,'vram_after_mb':gpu_peak(),'vram_peak_mb':gpu_peak(),'disk_bytes':model_disk(snap),'output_preview':text[-240:],'timestamp':now()})
            except torch.cuda.OutOfMemoryError as e:
                result['measurements'].append({'phase':'generation','prompt_index':i,'status':'OOM','error':str(e),'ram_bytes':rss(),'vram_peak_mb':gpu_peak(),'timestamp':now()}); result['status']='PARTIAL_OOM'; torch.cuda.empty_cache()
            except Exception as e:
                result['measurements'].append({'phase':'generation','prompt_index':i,'status':'FAILED','error':f'{type(e).__name__}: {e}','timestamp':now()}); result['status']='PARTIAL_FAILURE'
        result['events'].append({'type':'airllm_hook_counters','counters':counters,'prefetch_enabled':getattr(model,'prefetching',None)})
        result['peak_ram_bytes']=max([m.get('ram_after_bytes') or 0 for m in result['measurements']] + [rss() or 0]) or None
        result['peak_vram_used_mb']=max([m.get('vram_peak_mb') or 0 for m in result['measurements']] + [gpu_peak() or 0]) or None
        result['status']='EXECUTED' if any(m.get('phase')=='generation' and m.get('output_tokens',0)>0 for m in result['measurements']) else result['status']
        return result
    except Exception as e:
        result['status']='NOT_EXECUTED'; result['failure']={'type':type(e).__name__,'message':str(e),'traceback':traceback.format_exc(limit=12)}; result['hardware_after']=hardware(); return result
    finally:
        result['finished_at']=now(); result['elapsed_wall_ms']=(time.perf_counter()-t0)*1000

def write_artifacts(results, out):
    out=Path(out); out.mkdir(parents=True,exist_ok=True)
    (out/'results.json').write_text(json.dumps(results,indent=2,default=str))
    with (out/'results.jsonl').open('w') as f:
        for r in results: f.write(json.dumps(r,default=str)+'\n')
    rows=[]
    for r in results:
        ms=[m for m in r.get('measurements',[]) if m.get('phase')=='generation']
        rows.append({'run_id':r.get('run_id'),'repo_id':r.get('repo_id'),'revision_requested':r.get('revision_requested'),'backend':r.get('backend'),'status':r.get('status'),'execution_class':r.get('execution_class'),'execution_scope':r.get('execution_scope'),'model_load_ms':next((m.get('model_load_ms') for m in r.get('measurements',[]) if m.get('phase')=='load'),None),'peak_ram_bytes':r.get('peak_ram_bytes'),'peak_vram_used_mb':r.get('peak_vram_used_mb'),'prompts_attempted':len(ms),'prompts_succeeded':sum(1 for m in ms if m.get('output_tokens',0)>0),'decode_tok_per_s_mean':(sum(m['decode_tok_per_s'] for m in ms if m.get('decode_tok_per_s') is not None)/max(1,sum(1 for m in ms if m.get('decode_tok_per_s') is not None))),'failure':json.dumps(r.get('failure')) if r.get('failure') else ''})
    with (out/'results.csv').open('w',newline='') as f:
        w=csv.DictWriter(f,fieldnames=rows[0].keys() if rows else ['status']); w.writeheader(); w.writerows(rows)
    lines=['# AURA Frontier Validation Results','',f'Generated: {now()}','', 'Only values emitted by the execution process are included. Null means not measured; no estimate is substituted.','', '| Model | Backend | Status | Scope | Load ms | Peak VRAM MB | Peak RAM bytes | Prompts succeeded |','|---|---|---|---:|---:|---:|---:|---:|']
    for x in rows: lines.append(f"| {x['repo_id']} | {x['backend']} | {x['status']} | {x['execution_scope']} | {x['model_load_ms']} | {x['peak_vram_used_mb']} | {x['peak_ram_bytes']} | {x['prompts_succeeded']}/{x['prompts_attempted']} |")
    (out/'REPORT.md').write_text('\n'.join(lines)+'\n')

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--repo',required=True); ap.add_argument('--revision'); ap.add_argument('--backend',default='airllm',choices=['airllm','transformers']); ap.add_argument('--out',default='artifacts/frontier'); ap.add_argument('--cache-dir'); ap.add_argument('--max-new-tokens',type=int,default=32); ap.add_argument('--prompts',type=int,default=50); ap.add_argument('--token',default=os.getenv('HF_TOKEN')); ap.add_argument('--inspect-only',action='store_true'); ap.add_argument('--no-generate',action='store_true'); args=ap.parse_args()
    prompts=PROMPTS[:args.prompts]
    results=[inspect_model(args.repo,args.revision,args.token,args.cache_dir)] if args.inspect_only else [run_model(args.repo,args.revision,args.backend,prompts,args.max_new_tokens,args.token,args.cache_dir,not args.no_generate)]
    write_artifacts(results,args.out); print(json.dumps(results,indent=2,default=str))
if __name__=='__main__': main()
