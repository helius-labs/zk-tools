use std::fs;
fn main() {
    let mut s = fs::read_to_string("close-body.rs").unwrap();
    let old = "All native/link/capture workflows then pass on their first attempts.";
    assert_eq!(s.matches(old).count(), 1);
    s=s.replace(old,"A later link-plan preparer stopped before probe linking on inherited make-harness filenames; its preserved fix pins the actual make-native helpers. No native build/test or capture was repeated. The native/link/capture workflows themselves pass on their first attempts. An inherited console label says field34x3, while the enforced assertions and all three unfiltered field-suite logs correctly record 35; the original output is retained.");
    fs::write("close-body.rs", s).unwrap();
    fs::write("console-label-audit.json",r#"{"inherited_console_label":"field34x3","actual_required_and_unfiltered_field_count":35,"all_three_native_field_logs_passed":true,"validation_gate_changed":false,"original_console_output_preserved":true}"#).unwrap();
}
