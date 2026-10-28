use std::fs;
fn main() {
    let mut s = fs::read_to_string("close.rs").unwrap();
    let old = "Unrelated ceremony host work is recorded; no exclusive-host claim is made.";
    assert_eq!(s.matches(old).count(), 1);
    s=s.replace(old,"A fresh host snapshot is recorded. The ceremony job present in the parent component study had ended before the SDK capture; no exclusive-host claim is made.");
    let needle = "    report.push_str(\"## Correctness and profiles";
    assert_eq!(s.matches(needle).count(), 1);
    let note = r#"    report.push_str("At 32 workers, accepted/fresh OFF/ON medians are 8.365439/8.361170/8.378203 ms. The paired ON/OFF estimate is 0.333% slower, with bounds crossing parity; this is no verified proof gain. At 16 workers the raw ON median is 24.977 µs below accepted, but its paired median is slightly slower and the calibration/nonregression bounds fail. That small positive raw-median observation is retained without treating it as a qualified gain. Median CPU work falls from 191.289517 to 191.179662 ms/request at 32 workers, and from 124.894060 to 124.317576 at 16, descriptive reductions of about 0.057% and 0.462%. Median peak RSS at 32 is 129.00 MiB OFF versus 131.50 ON; maxima are 135.50/135.00 MiB. No RAM reduction is established.\n\nThe parent component used one active caller on one logical CPU; full proofs use parallel work and a different batch mix. Shortening a dependency chain at the cost of more instructions may behave differently when execution resources are shared or saturated. This is a hypothesis, not a demonstrated SMT, port-pressure or scheduler cause. The full-proof data justify closing this exact candidate; the component improvement remains valid within its measured scope.\n\n");
"#;
    s = s.replace(needle, &format!("{note}{needle}"));
    fs::write("close-v2.rs", s).unwrap();
    fs::write("close-report-revision.json",r#"{"scope":"report-only wording revision after completed captures and analysis","reason":"fresh SDK host snapshot shows earlier parent ceremony job absent; distinguish actual full-proof estimates from caller gains and state concurrency hypothesis without causal attribution","pinned_original_close_source_preserved":true,"production_kernel_changed":false,"tests_builds_captures_profiles_or_verifiers_repeated":false}"#).unwrap();
}
