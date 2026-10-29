use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/row-quotient175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-row-quotient175";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() {
    let d = Path::new(D);
    let p = d.join("artifacts/publication-review.json");
    let mut v: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    let mut displays = vec![];
    for name in ["candidate-selected.s", "frozen-selected.s"] {
        let original = fs::read(Path::new(R).join(name)).unwrap();
        let s = std::str::from_utf8(&original).unwrap();
        let joined = s.lines().map(str::trim_end).collect::<Vec<_>>().join("\n");
        let trimmed = format!("{}\n", joined.trim_end());
        fs::write(d.join("artifacts").join(name), &trimmed).unwrap();
        displays.push(json!({"file":name,"original_local":Path::new(R).join(name),"original_sha256":hash(&original),"published_sha256":hash(trimmed.as_bytes()),"transformation":"trim trailing whitespace and terminal blank lines from derived display only; instruction text/addresses unchanged"}));
    }
    v["assembly_display_normalization"] = json!(displays);
    v["raw_test_log_whitespace_findings"] = json!(1);
    v["raw_test_log_EOF_blank_line_retained"] = json!(true);
    v["authored_Rust_whitespace_findings"] = json!(0);
    v["static_code_instructions_metadata_erratum"]=json!("Counts raw disassembly lines including wrapped byte-only lines; not actual instruction counts. No latency gate uses it.");
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
    let p = d.join("README.md");
    let mut md = fs::read_to_string(&p).unwrap();
    md=md.replace("Whole-function instruction/IFMA/RSP sites include setup and cold paths; stack operands are not measured spills or peak RSS.","Whole-function IFMA/RSP sites include setup and cold paths; stack operands are not measured spills or peak RSS. The analysis field static_code.instructions counts raw disassembly lines, including wrapped byte-only lines, and is not an instruction count.");
    let marker = "## Inherited CPU and Metal comparison";
    assert!(md.contains(marker));
    md=md.replace(marker,"Publication review trims trailing whitespace in derived disassembly displays; originals remain local with recorded hashes. One trailing blank line in the raw native test log is retained. Authored Rust has no whitespace findings.\n\n## Inherited CPU and Metal comparison");
    fs::write(p, md).unwrap();
    fs::copy(
        Path::new(R).join("publication-review.rs"),
        d.join("artifacts/publication-review.rs"),
    )
    .unwrap();
    println!("Publication display/metadata review complete; data and gates unchanged");
}
