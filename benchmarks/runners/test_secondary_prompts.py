import urllib.request
import json
import sys

# Ensure UTF-8 output encoding for multilingual characters across all platforms
if sys.stdout.encoding != 'utf-8':
    sys.stdout.reconfigure(encoding='utf-8')

def test_prompts_matrix():
    prompts = [
        ("P1", "What is quantum computing? Explain it in 3 simple sentences."),
        ("P2", "Write a Python function to calculate the Fibonacci sequence."),
        ("P3", "Explain the difference between RAM and VRAM."),
        ("P4", "What is 17 * 38?"),
        ("P5", "Translate 'Good morning, how are you?' into Hindi."),
        ("P6", "Return exactly this JSON: {\"status\":\"ok\",\"value\":42}"),
        ("P7", "Find the bug in: for i in range(10) print(i)"),
        ("P8", "Explain what a transformer neural network is in simple terms."),
        ("P9", "Write a SQL query that returns the top 5 customers by total spending."),
        ("P10", "Give three practical uses of large language models.")
    ]
    
    print("==================================================================")
    print("AURA SECONDARY 10-PROMPT MATRIX EXECUTION (nous-hermes2:latest)")
    print("==================================================================")
    
    for pid, prompt_text in prompts:
        payload = {
            "model": "nous-hermes2:latest",
            "prompt": prompt_text,
            "stream": False,
            "options": {"num_gpu": 99, "num_thread": 8}
        }
        
        req = urllib.request.Request(
            "http://localhost:11434/api/generate",
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json"}
        )
        
        try:
            with urllib.request.urlopen(req, timeout=90) as resp:
                data = json.loads(resp.read().decode("utf-8"))
                resp_text = data.get("response", "").strip().replace("\n", " ")[:60]
                eval_cnt = data.get("eval_count", 0)
                eval_ns = data.get("eval_duration", 1)
                tok_s = eval_cnt / (eval_ns / 1e9) if eval_ns > 0 else 0.0
                print(f"[{pid}] Tok/s: {tok_s:5.2f} | Answer: {resp_text}...")
        except Exception as e:
            print(f"[{pid}] FAILED: {e}")

if __name__ == "__main__":
    test_prompts_matrix()
