use std::{fs, path::Path};
const R: &str = "/tmp/zopro-g1-raw-scratch";
fn main() {
    let r = Path::new(R);
    let old = "/tmp/zopro-clean-fft-leaves";
    let mut s = fs::read_to_string(format!("{old}/prepare-sdk.rs"))
        .unwrap()
        .replace(old, R)
        .replace(
            "/tmp/zopro-composed-adoption/production-source-manifest.json",
            "/tmp/zopro-clean-fft-adoption/production-source-manifest.json",
        );
    let begin = s.find("fn prior() -> BTreeSet<String> {").unwrap();
    let end = s.find("fn prepare() {").unwrap();
    s.replace_range(begin..end,r#"fn prior() -> BTreeSet<String> {
 let input=read("/tmp/zopro-clean-fft-qualification/inputs.json");
 let mut seen:BTreeSet<String>=serde_json::from_value(input["prior_proof_sha256"].clone()).unwrap();assert_eq!(seen.len(),11575);
 let inventory=read("/tmp/zopro-clean-fft-qualification/proof-inventory.json");assert_eq!(inventory.as_object().unwrap().len(),170);
 for(f,h)in inventory.as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());if f.ends_with(".proof"){assert_eq!(fs::metadata(f).unwrap().len(),164);assert!(seen.insert(h.as_str().unwrap().to_owned()));}}
 assert_eq!(seen.len(),11660);seen
}
"#);
    s = s.replace(
        "            119\n        } else {\n            116",
        "            121\n        } else {\n            119",
    );
    s = s.replace(
        "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
        "/tmp/zopro-clean-fft-leaves/native/candidate/artifacts.json",
    );
    s = s.replace("\"prepare-close.rs\",", "\"storage.rs\",");
    s = s
        .replace("11119", "11669")
        .replace("prior11110", "prior11660");
    fs::write(r.join("prepare-sdk.rs"), s).unwrap();
    let s = fs::read_to_string(format!("{old}/analyze.rs"))
        .unwrap()
        .replace(old, R)
        .replace("11119", "11669")
        .replace("11575", "12125")
        .replace(
            "\"fresh_core_tests\":116,\"candidate_core_tests\":119",
            "\"fresh_core_tests\":119,\"candidate_core_tests\":121",
        )
        .replace(
            "clean_bounded_serial_fft_leaves",
            "lossless_g1_batch_storage",
        )
        .replace("CleanFFTLeaves", "G1RawScratch");
    fs::write(r.join("analyze.rs"), s).unwrap();
    fs::copy(format!("{old}/stats.rs"), r.join("stats.rs")).unwrap();
    fs::copy(r.join("design.md"), r.join("sdk-design.md")).unwrap();
    fs::write(r.join("integration-review.md"),"# Storage and admission review\n\nOnly native eight-lane G1 batch records use existing lossless numerical packing, five320byte values become five256byte values. Same fields and exact R256 radix, no arithmetic reconstruction. Existing compact/expand API requires normalized values below2^256; Fq constructors and all outgoing ordinary/fused/signed batch operations return normalized values below2p<2^255. These are the existing arithmetic contracts, not a new lazy bound. Packed values are never interpreted as host Fq or changed toR260. Reverse expansion restores allfive52bitdigits. No fieldsourcechange.\n\nFallback stores unchanged FqBatch values. G2/tree/FFT/preparation unchanged. Existing raw spare-capacity initialization retains allfiveCopyfields before lenpublication; no newunsafe. Vecreserve/growth/tails/outputorder remain. Planner replaces typeFqAddGroup by [FqBatch;5], equalto original1600bytes/alignment64, so existing requested-capacity/header/growthbound stays conservative and numerically unchanged. No increased admission or concurrency is credited for narrower storage. ActualRSS can differ duecodegen/allocatorlifetimes;20%recordshrink is not totalRAMorlatency saving. CompleteSDK/profiledecides. Resource/concurrent/largecircuitqualificationrequiredseparatelybeforeadoption.\n").unwrap();
    println!(
        "Prepared fixed15block support; prior11660/preflight11669/final12125;43410SDK465exports"
    );
}
