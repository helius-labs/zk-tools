use std::{fs,path::Path};
fn main(){
let root=Path::new("/home/ubuntu/src/zopro-perf-work/representative-54-diagnostic175");
let original=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-current-cpu-summary/generate.rs").unwrap();
let mut s=original.replace("a1f789c02350b4a80846330a0de244e362f820ca","ab9afc137b813b5e33048c9b4d3514255a9b8fb0");
let needle="        \"2026-10-29-representative-sdk175\",";
assert_eq!(s.matches(needle).count(),1);
s=s.replace(needle,&format!("{needle}\n        \"2026-10-29-representative-54-diagnostic175\","));
let needle="    let root = Path::new(Z);";
assert_eq!(s.matches(needle).count(),1);
s=s.replace(needle,&format!("{needle}\n    let diagnostic = read(root.join(\"docs/benchmarks/2026-10-29-representative-54-diagnostic175/artifacts/analysis.json\"));\n    assert_eq!(diagnostic[\"complete\"], true);\n    assert_eq!(diagnostic[\"independent_proof_count\"],21703);"));
let needle="    fs::write(dest.join(\"README.md\"), md).unwrap();";
assert_eq!(s.matches(needle).count(),1);
s=s.replace(needle,r#"
    md += "\n## Frozen 54 to 1 CPU diagnostic\n\nThe exact accepted, OFF and ON SDK executables are profiled on 54→1 at 32 workers, with no source rebuild or new latency qualification. Each engine has one gated four-event PMU capture and one separate 199 Hz cycles profile, each covering 31 fresh requests after three primes. All six captures precede offline analysis, and all six exported proofs pass independent Gnark verification. The original all-nine candidate gate remains failed and production is unchanged. [Full diagnostic and counters](../2026-10-29-representative-54-diagnostic175/README.md).\n\n|Executable|Retired instructions per proof|Cycles per proof|IPC|Profile samples|Lost records|\n|---|---:|---:|---:|---:|---:|\n";
    for v in ["accepted","OFF","ON"] {
        let c=&diagnostic["counters"][v]; let p=&diagnostic["profiles"][v];
        if c["qualified"] == true {
            writeln!(md,"|{v}|{:.0}|{:.0}|{:.3}|{}|0|",c["events"]["instructions:u"]["per_request"].as_f64().unwrap(),c["events"]["cycles:u"]["per_request"].as_f64().unwrap(),c["instructions_per_cycle"].as_f64().unwrap(),p["samples"]).unwrap();
        }else{writeln!(md,"|{v}|unqualified|unqualified|unqualified|{}|0|",p["samples"]).unwrap();}
    }
    md += "\nThese single captures locate sampled work; they do not supply paired speed bounds or assign exclusive wall time to flat symbols. Raw counters retain event running percentages and unavailable or multiplexed events. This diagnostic adds 204 SDK requests and six fresh independent exports, taking the distinct-proof ledger to **21,703**. The all-eight CPU 10× objective remains unmet; the accepted CPU and Metal comparisons above are unchanged.\n";
    fs::write(dest.join("README.md"), md).unwrap();
"#);
let needle="    fs::write(\n        dest.join(\"data.json\"),";
assert_eq!(s.matches(needle).count(),1);
s=s.replace(needle,&format!(r#"    data["latest_54_diagnostic"] = diagnostic;
    data["historical_distinct_Gnark_ledger"] = json!(21703);
{needle}"#));
s=s.replace("The historical distinct-proof ledger is **21,697**.","The historical distinct-proof ledger is **21,703**.");
fs::write(root.join("summary-generator.rs"),s).unwrap();
let old=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/representative-sdk175/publish.rs").unwrap();
let s=old.replace("representative-sdk175","representative-54-diagnostic175");
fs::write(root.join("publish.rs"),s).unwrap();
println!("Prepared revised summary generator; all original rows and failed gates retained");
}
