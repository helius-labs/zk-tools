use std::fs;
fn main() {
    let s =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/concurrent-callers/close.rs").unwrap();
    let mut head = s[..s.find("fn archive()").unwrap()]
        .replace(
            "/zopro-perf-work/concurrent-callers",
            "/zopro-perf-work/gcd-vector-callers",
        )
        .replace(
            "2026-10-28-concurrent-callers",
            "2026-10-28-gcd-vector-callers",
        );
    let old = "if [\"source\", \"target\", \"profiles\", \"frozen\"].contains(&n.as_str()) {";
    assert_eq!(head.matches(old).count(), 1);
    head=head.replace(old,"if n.starts_with(\"target-\") || [\"source\", \"target\", \"profiles\", \"frozen\"].contains(&n.as_str()) {");
    fs::write(
        "close.rs",
        format!("{head}{}", fs::read_to_string("close-body.rs").unwrap()),
    )
    .unwrap();
}
