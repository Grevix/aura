use aura_audit::evaluate_release_audit;

#[test]
fn test_release_audit_evaluation() {
    let report = evaluate_release_audit();
    assert_eq!(report.total_gates, 10);
    assert_eq!(report.passed_gates, 10);
    assert_eq!(report.failed_gates, 0);
}
