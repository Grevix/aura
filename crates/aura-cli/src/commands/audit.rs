use aura_audit::evaluate_release_audit;
use std::fs::File;
use std::io::Write;

pub fn execute_audit(out_path: &str) {
    println!("🛡️ Executing AURA Release Audit System Gate Checks...\n");

    let report = evaluate_release_audit();

    println!("=== RELEASE AUDIT SUMMARY ===");
    println!("Audit Version  : {}", report.audit_version);
    println!("Target Release : v{}", report.version);
    println!("Total Gates    : {}", report.total_gates);
    println!("Passed Gates   : {}", report.passed_gates);
    println!("Failed Gates   : {}", report.failed_gates);

    println!("\n=== AUDIT GATE DETAILED RESULTS ===");
    for gate in &report.gates {
        println!("[{}] {:<30} : {} ({})", gate.gate_id, gate.name, gate.status, gate.details);
    }

    if report.failed_gates == 0 {
        println!("\n✅ AUDIT PASSED: All release gates satisfied. Build approved for release distribution.");
    } else {
        println!("\n❌ AUDIT FAILED: Critical release gate failure detected.");
    }

    let json_str = serde_json::to_string_pretty(&report).unwrap();
    if let Ok(mut f) = File::create(out_path) {
        let _ = f.write_all(json_str.as_bytes());
        println!("📝 Machine-Readable Audit Report Saved to: {}", out_path);
    }
}
