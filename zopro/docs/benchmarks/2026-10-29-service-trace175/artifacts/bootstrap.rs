use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/service-trace175";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g2-packed-index175";
const HIST: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-arithmetic-leaves/artifacts";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let trace = fs::read_to_string(
        Path::new(HIST).join("candidate/crates/zopro-core/src/arithmetic_trace.rs"),
    )
    .unwrap()
    .replace("LIMIT:usize=8192", "LIMIT:usize=1024");
    fs::write(r.join("service_trace.rs"), trace).unwrap();
    let driver = fs::read_to_string(Path::new(HIST).join("driver.rs"))
        .unwrap()
        .replace("arithmetic_trace as trace", "service_trace as trace");
    fs::write(r.join("driver.rs"), driver).unwrap();
    fs::copy(
        "/home/ubuntu/src/zk-tools/zopro/crates/zopro-bench/examples/support/perf.rs",
        r.join("support/perf.rs"),
    )
    .unwrap();
    let old = fs::read_to_string(Path::new(OLD).join("native.rs")).unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(label:").unwrap();
    let source = format!(
        "{}{}\n{}",
        &old[..a],
        fs::read_to_string(r.join("setup.rs")).unwrap(),
        &old[b..]
    )
    .replace(OLD, R);
    fs::write(r.join("native.rs"), source).unwrap();
}
