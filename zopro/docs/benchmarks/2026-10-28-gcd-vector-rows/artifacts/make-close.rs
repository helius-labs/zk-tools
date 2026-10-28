use std::fs;
fn main() {
    let old =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/concurrent-callers/close.rs").unwrap();
    let head = old[..old.find("fn archive()").unwrap()]
        .replace(
            "/zopro-perf-work/concurrent-callers",
            "/zopro-perf-work/gcd-vector-rows",
        )
        .replace(
            "2026-10-28-concurrent-callers",
            "2026-10-28-gcd-vector-rows",
        );
    fs::write(
        "close.rs",
        format!("{head}{}", fs::read_to_string("close-body.rs").unwrap()),
    )
    .unwrap();
}
