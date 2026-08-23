# AURA V5 Failure Injection & Chaos Audit Report

**Date:** 23 August 2026  

---

## Adversarial Fault Injection Scenarios

| Fault Scenario | Expected System Behavior | Actual System Behavior | Audit Verdict |
|---|---|---|---|
| **Corrupted GGUF Header** | Fail preflight validation with clean error | Returns `AuraError::ModelLoadError` | **PASSED** |
| **Missing Model File** | Fail gracefully before allocation | Returns `AuraError::ModelNotFound` | **PASSED** |
| **Insufficient Memory Budget (<1 GB)** | Reject plan as INFEASIBLE | Rejects plan cleanly (`is_feasible = false`)| **PASSED** |
| **Process Interruption (Ctrl+C / SIGTERM)** | Clean process termination and Job Object cleanup | Process terminates cleanly | **PASSED** |
| **Invalid Memory Argument String** | Fail argument parsing gracefully | Returns usage error cleanly | **PASSED** |
