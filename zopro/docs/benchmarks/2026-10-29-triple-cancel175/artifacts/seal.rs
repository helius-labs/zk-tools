use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-cancel175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-triple-cancel175";
fn hash(p: &str) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let p = format!("{D}/artifact-index.json");
    let mut index: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    for (f, h) in &index {
        assert_eq!(hash(&format!("{D}/{f}")), *h);
    }
    let receipts: Value =
        serde_json::from_slice(&fs::read(format!("{D}/artifacts/run-receipts.json")).unwrap())
            .unwrap();
    let mut streams = 0;
    for (name, files) in receipts.as_object().unwrap() {
        for (file, v) in files.as_object().unwrap() {
            let text = v["text"].as_str().unwrap();
            assert_eq!(
                format!("{:x}", Sha256::digest(text.as_bytes())),
                v["sha256"].as_str().unwrap()
            );
            assert_eq!(
                text.as_bytes(),
                fs::read(format!("{R}/runs/{name}/{file}")).unwrap()
            );
            streams += 1;
        }
    }
    assert_eq!(streams, 4572);
    let f = format!("{D}/README.md");
    let mut text = fs::read_to_string(&f).unwrap();
    for (a, b) in [
        ("below2p", "below 2p"),
        ("is1.", "is 1."),
        (",0/31wins", ", 0/31 wins"),
        ("least21/31wins", "least 21/31 wins"),
        ("factor1.03", "factor 1.03"),
        ("ranks10–22of31", "ranks 10–22 of 31"),
        ("nominal97.055%", "nominal 97.055%"),
        ("All372latency", "All 372 latency"),
        ("inclusive2p", "inclusive 2p"),
        ("The33unchanged", "The 33 unchanged"),
        (
            "checks32canonicalFq2outputs",
            "checks 32 canonical Fq2 outputs",
        ),
        ("scalarFq2", "scalar Fq2"),
        ("duplicate8finalstates", "duplicate 8 final states"),
        (
            "so32outputs do not mean32independentstates",
            "so 32 outputs do not mean 32 independent states",
        ),
        (
            ",1,000,000iterations at199Hz",
            ", 1,000,000 iterations at 199 Hz",
        ),
        ("All392workflow", "All 392 workflow"),
        (
            ";288source/tool/library/binary",
            "; 288 source/tool/library/binary",
        ),
        ("The175production", "The 175 production"),
        ("all17originalfieldfiles", "all 17 original field files"),
        ("other16stay", "the other 16 stay"),
        ("ZeroSDKrequests", "Zero SDK requests"),
        ("remains**17,811**", "remains **17,811**"),
        ("remains134MiB", "remains 134 MiB"),
        ("and16-worker", "and 16-worker"),
        ("CPU10×", "CPU 10×"),
        (
            "accepted54→1 remains162.250464ms",
            "accepted 54→1 remains 162.250464 ms",
        ),
    ] {
        text = text.replace(a, b);
    }
    for (a, b) in [
        (
            "at199Hz for30,000,000iterations",
            "at 199 Hz for 30,000,000 iterations",
        ),
        ("All619samples", "All 619 samples"),
        ("All1,144workflow", "All 1,144 workflow"),
        ("and289pins", "and 289 pins"),
        ("All1,116latency", "All 1,116 latency"),
        ("processes,12preflights", "processes, 12 preflights"),
        ("check32returned", "check 32 returned"),
        ("not32independentchains", "not 32 independent chains"),
        ("The33unchanged", "The 33 unchanged"),
        ("bothcontrols", "both controls"),
        ("all12cases'", "all 12 cases'"),
        ("below1", "below 1"),
        ("least21/31wins", "least 21/31 wins"),
        ("nominal97.055%", "nominal 97.055%"),
        ("ranks10–22of31", "ranks 10–22 of 31"),
        ("output below2p", "output below 2p"),
        (
            "Fr/R260 chains improve0.406%",
            "Fr/R260 chains improve 0.406%",
        ),
        ("and0.326%", "and 0.326%"),
        ("and0.535%", "and 0.535%"),
        ("29/31wins", "29/31 wins"),
        ("25/31wins", "25/31 wins"),
        ("all17originalfieldfiles", "all 17 original field files"),
        ("other16are", "other 16 are"),
        ("All175production", "All 175 production"),
        ("ZeroSDKrequests", "Zero SDK requests"),
        ("remains**17,811**", "remains **17,811**"),
        (
            "Accepted54→1 remains162.250464ms",
            "Accepted 54→1 remains 162.250464 ms",
        ),
    ] {
        text = text.replace(a, b);
    }
    let needle = "No core/prover rebuild or new verifier run occurs.";
    assert_eq!(text.matches(needle).count(), 1);
    text=text.replace(needle,&format!("{needle} Archive review reconstructs all 4,572 encoded streams byte-for-byte. Three whitespace findings are trailing blank lines in two original disassembly excerpts and one native test log; these bytes remain unchanged. Authored Rust source has no whitespace findings."));
    fs::write(&f, text).unwrap();
    let v = format!("{D}/artifacts/postcapture-review.json");
    let mut review: Value = serde_json::from_slice(&fs::read(&v).unwrap()).unwrap();
    review["reconstructed_streams_exact"] = json!(4572);
    review["whitespace_findings"] = json!(3);
    review["authored_source_whitespace_findings"] = json!(0);
    fs::write(&v, serde_json::to_vec_pretty(&review).unwrap()).unwrap();
    index.insert("artifacts/postcapture-review.json".into(), hash(&v));
    fs::copy(format!("{R}/seal.rs"), format!("{D}/artifacts/seal.rs")).unwrap();
    index.insert(
        "artifacts/seal.rs".into(),
        hash(&format!("{D}/artifacts/seal.rs")),
    );
    fs::write(&p, serde_json::to_vec_pretty(&index).unwrap()).unwrap();
    index.insert("README.md".into(), hash(&f));
    index.insert("artifact-index.json".into(), hash(&p));
    let sums = index
        .iter()
        .map(|(f, h)| format!("{h}  {f}\n"))
        .collect::<String>();
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    let pins: Value = serde_json::from_slice(&fs::read(format!("{R}/pins.json")).unwrap()).unwrap();
    for (f, h) in pins.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    println!(
        "Sealed{}hashes/{}files;4572originalstreamsreconstructed;{}pinsclosed",
        index.len(),
        index.len() + 1,
        pins.as_object().unwrap().len()
    );
}
