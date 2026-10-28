use std::fs;
fn main() {
    let s =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/concurrent-callers/close.rs").unwrap();
    let head = &s[..s.find("fn archive()").unwrap()];
    let mut head = head
        .replace(
            "/zopro-perf-work/concurrent-callers",
            "/zopro-perf-work/gcd-active-words",
        )
        .replace(
            "2026-10-28-concurrent-callers",
            "2026-10-28-gcd-active-words",
        );
    let old = "if [\"source\", \"target\", \"profiles\", \"frozen\"].contains(&n.as_str()) {";
    assert_eq!(head.matches(old).count(), 1);
    head=head.replace(old,"if n.starts_with(\"target-\") || [\"source\", \"target\", \"profiles\", \"frozen\"].contains(&n.as_str()) {");
    let body = fs::read_to_string("close-body.rs").unwrap();
    fs::write("close.rs", format!("{head}{body}")).unwrap();
}
