# Requires: Ollama running (ollama serve), Python 3.8+, AURA built
$ScriptDir = Split-Path $MyInvocation.MyCommand.Path
python "$ScriptDir\run_community_benchmark.py" @args
