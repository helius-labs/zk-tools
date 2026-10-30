// Offline documentation only: consumes completed benchmark records.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};

const ROOT: &str = "/home/ubuntu/src/zk-tools/zopro";
const DEST: &str = "docs/benchmarks/2026-10-30-current-benchmark-report";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn n(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap() }
fn main() {
    let root = Path::new(ROOT);
    let dest = root.join(DEST);
    assert!(!dest.exists(), "snapshot already exists");
    let summary = root.join("docs/benchmarks/2026-10-29-current-cpu-summary");
    let data = read(summary.join("data.json"));
    let sdk_dir = root.join("docs/benchmarks/2026-10-30-loose-delta-sdk175");
    let sdk = read(sdk_dir.join("artifacts/analysis.json"));
    let prior = read(root.join("docs/benchmarks/2026-10-29-cpu-benchmark-report/data.json"));
    assert_eq!(data["rows"], prior["rows"]);
    assert_eq!(data["rows"].as_array().unwrap().len(), 9);
    assert_eq!(data["historical_distinct_Gnark_ledger"], 23971);
    assert_eq!(data["accepted_original8_CPU10x"], 0);
    assert_eq!(data["latest_loose_delta_SDK_experiment"]["cells"], sdk["cells"]);
    assert_eq!(sdk["complete"], true);
    assert_eq!(sdk["SDK_requests"], 55788);
    assert_eq!(sdk["new_independent_exports"], 972);
    assert_eq!(sdk["independent_proof_count"], 23962);
    assert_eq!(sdk["production_changed"], false);
    assert_eq!(sdk["adopted"], false);
    assert!(sdk["profiles"].as_object().unwrap().values().all(|v| v["attribution_qualified"] == true && v["zero_lost"] == true));
    assert!(sdk["counters"].as_object().unwrap().values().all(|v| v["qualified"] == true));
    let mut md = String::from("# CPU prover benchmark report\n\n30 October 2026. Snapshot of accepted local and AWS benchmarks, including the completed bounded G2 delta SDK experiment. This report consolidates existing captures and adds no benchmark samples.\n\nThe accepted Zopro build measures **8.022 ms for 1×2 locally**, with **134 MiB observed peak process RSS**. On AWS c7a.24xlarge, Zopro measures **11.916 ms versus Aeglos Sorted’s 7.959 ms**, using each prover’s fastest tested worker count. Zopro has lower best-count latency on six of nine AWS circuits and lower observed peak RSS on all nine. Aeglos leads the three smallest circuits. The requested 10× speedup against the supplied M5 CPU references remains **unmet on all eight shapes**.\n\n");
    let primary = &sdk["cells"]["1x2-w32/ON"];
    let pair = &sdk["pairs"]["1x2-w32/ON/OFF"]["statistics"];
    writeln!(md, "The latest full-proof candidate measures **{:.6} ms for 1×2**, with **{:.2} MiB peak RSS**. Its paired ON/OFF latency ratio is **{:.9}**, with bounds **[{:.9}, {:.9}]** and **{}/31 wins**. Its declared full-request performance gate is **{}**; production is unchanged. Kernel improvements and proof improvements are reported separately.\n", n(primary,"median_ms"), n(primary,"max_RSS_MiB"), n(pair,"median"), n(pair,"low"), n(pair,"high"), pair["wins"], sdk["gate_passed"]).unwrap();
    assert_eq!(sdk["gate_passed"],false);
    md.push_str("This is a negative full-proof result: **1×2 is 1.61% slower versus OFF and 1.52% slower versus accepted** in paired comparisons. The original-eight aggregate is 0.34% slower versus OFF and 0.45% slower versus accepted. The isolated caller gain did not transfer to this SDK integration.\n\n");
    md.push_str("## Measurement scope\n\nLocal: Ryzen 9 9950X, 16 physical cores, 32 logical CPUs, 128 GiB RAM, shared host. The accepted table uses 32 workers. AWS: one c7a.24xlarge, 96 physical cores without SMT, testing 32/64/96 workers in a network-disabled container with a 96-CPU quota, 30 GiB memory limit and no swap. That memory limit is a benchmark setting.\n\nAccepted results are medians of 15 process medians after three priming proofs. Local processes measure 151 requests for 1×2 and 31 for other circuits; AWS processes measure 101 and 17 respectively. Each timed Zopro request includes fresh witness solving, fresh randomized proving, SDK verification and serialization. Loading and preparation are outside latency and inside lifetime process RSS. Peak RSS is the largest process high-water mark, not minimum required RAM or incremental workspace.\n\nAWS Zopro uses the accepted 175-file source compiled for Zen 4; the local executable targets the Ryzen host. Aeglos uses the Sorted engine at commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, with prepared-key cache hits checked. These results describe the tested versions and configurations. M5 CPU and Metal numbers are supplied historical references, not new matched-hardware measurements.\n\n## Accepted local results\n\nSpeedup is reference latency divided by Zopro latency.\n\n|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for r in data["rows"].as_array().unwrap() {
        let shape = r["shape"].as_str().unwrap();
        let l = &r["local_accepted"];
        let refs = &r["M5_reference_and_local_ratios"];
        if refs.is_null() {
            writeln!(md,"|{shape}|{:.3}|{:.2}|—|—|—|—|—|",n(l,"ms"),n(l,"max_RSS_MiB")).unwrap();
        } else {
            writeln!(md,"|{shape}|{:.3}|{:.2}|{:.0}|{:.3}×|{:.0}|{:.3}×|{:.1}|",n(l,"ms"),n(l,"max_RSS_MiB"),n(refs,"cpu_reference_ms"),n(refs,"cpu_speedup"),n(refs,"metal_reference_ms"),n(refs,"metal_speedup"),n(refs,"required_ms_for_CPU10x")).unwrap();
        }
    }
    md.push_str("\nNo M5 reference was supplied for 54→1. The accepted local 1×2 result needs approximately 54% lower latency to reach 3.7 ms.\n\nThe accepted build’s original full performance gate remains **false** because the physical-16 fresh-control calibration interval exceeded its declared range. Adoption used an explicit engineering exception; candidate gain/nonregression checks and a separate final-artifact gate passed. Later experiments do not inherit this exception. [Adoption decision](../2026-10-29-balanced-kh-adoption/README.md), [final-artifact qualification](../2026-10-29-balanced-kh-production/README.md).\n\n## AWS comparison with Aeglos\n\nEach displayed latency is that prover’s fastest median among 32/64/96 workers; RSS comes from the selected count. This descriptive selection does not establish a universal worker policy. The linked source retains the complete matrix and paired comparisons at equal worker counts.\n\n|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro peak RSS MiB|Aeglos peak RSS MiB|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for r in data["rows"].as_array().unwrap() {
        let a=&r["aws_best_tested_descriptive"]; let z=&a["plain"]; let g=&a["aeglos"];
        writeln!(md,"|{}|{:.3}|{}|{:.3}|{}|{:.2}|{:.2}|",r["shape"].as_str().unwrap(),n(z,"ms"),z["workers"],n(g,"ms"),g["workers"],n(z,"max_RSS_MiB"),n(g,"max_RSS_MiB")).unwrap();
    }
    md.push_str("\nFor 54→1, Zopro measures 122.397 ms and 1.83 GiB RSS versus Aeglos’s 173.236 ms and 12.81 GiB. These are observations of the tested configurations, not an explanation of the latency difference. [Complete AWS matrix and artifacts](../2026-10-29-proof-pgo-c7a175/README.md).\n\nThe 1×2 scaling gap is visible at equal worker counts:\n\n|Workers|Zopro ms|Aeglos ms|Zopro summed CPU ms/request|Aeglos summed CPU ms/request|\n|---|---:|---:|---:|---:|\n|32|11.916|11.868|256.128|328.248|\n|64|11.995|8.774|420.399|430.446|\n|96|13.778|7.959|704.247|513.394|\n\nAeglos gains latency as workers increase; Zopro spends more CPU without reducing latency in this sweep. The evidence supports investigating useful parallel work, critical paths and coordination. It does not establish RAM capacity as the sole cause. Local 32-worker runs use SMT; AWS does not, so local topology effects cannot be transferred directly.\n\n## Latest full-proof experiment\n\nThe bounded G2 delta candidate replaces two general modular-subtraction preparations with fixed-offset carry preparation inside the final G2 Y product. It preserves four raw products, two Montgomery reductions, field output bounds, exceptional handling, queues and worker policy. It adds no heap buffer, point-array copy, persistent key table or duplicate plan. The earlier caller experiment passed its gate with a **0.405% aggregate gain versus OFF**; that is a caller result, not a proof result. [Caller model, tests and measurements](../2026-10-30-loose-delta175/README.md).\n\nThe SDK run uses 31 fixed blocks across all nine circuits at 32 workers and primary 1×2 at 16 workers. Case and variant order rotate/reverse. Accepted is the retained production executable; fresh OFF is byte-identical to it; ON enables the candidate. Primary processes measure 151 requests and others 31, after three primes, with the same full-request timing scope.\n\n|Case|Accepted ms|OFF ms|ON ms|Accepted peak RSS MiB|ON peak RSS MiB|\n|---|---:|---:|---:|---:|---:|\n");
    for shape in ["1x2-w32","1x2-w16","2x2-w32","4x2-w32","8x2-w32","12x2-w32","16x2-w32","24x2-w32","merge-24x1-w32","merge-54x1-w32"] {
        let c=&sdk["cells"]; let a=&c[format!("{shape}/accepted")]; let off=&c[format!("{shape}/OFF")]; let on=&c[format!("{shape}/ON")];
        writeln!(md,"|{shape}|{:.6}|{:.6}|{:.6}|{:.2}|{:.2}|",n(a,"median_ms"),n(off,"median_ms"),n(on,"median_ms"),n(a,"max_RSS_MiB"),n(on,"max_RSS_MiB")).unwrap();
    }
    md.push_str("\nRatios pair medians within blocks and need not equal ratios of displayed medians. Lower is faster.\n\n|Paired comparison|Median ratio|Bounds|Wins /31|Pass|\n|---|---:|---|---:|---|\n");
    for key in ["1x2-w32/ON/accepted","1x2-w32/ON/OFF","1x2-w16/ON/accepted","1x2-w16/ON/OFF","merge-54x1-w32/ON/accepted","merge-54x1-w32/ON/OFF"] {
        let p=&sdk["pairs"][key]; let s=&p["statistics"];
        writeln!(md,"|{key}|{:.9}|[{:.9}, {:.9}]|{}|{}|",n(s,"median"),n(s,"low"),n(s,"high"),s["wins"],p["passed"]).unwrap();
    }
    for control in ["accepted","OFF"] {
        let p=&sdk["original8_aggregates"][control]; let s=&p["statistics"];
        writeln!(md,"|Original eight ON/{control}|{:.9}|[{:.9}, {:.9}]|{}|{}|",n(s,"median"),n(s,"low"),n(s,"high"),s["wins"],p["passed"]).unwrap();
    }
    writeln!(md,"\nDeclared gate: **{}**. Failed checks: `{}`. No adoption or accepted performance change follows from this experiment.\n",sdk["gate_passed"],sdk["failed_checks"]).unwrap();
    md.push_str("The six failures comprise four gain checks (primary and original-eight aggregate, each against both controls), physical-16 OFF/accepted calibration, and physical-16 ON/accepted nonregression. The physical-16 ON/accepted interval spans parity; its 1.028 upper endpoint is not an established 2.8% slowdown. All other nonregression/calibration checks and all relative RSS guards pass. At 54→1, the ON/OFF interval spans parity, so the smaller displayed median does not establish a gain versus OFF.\n\n");
    md.push_str("Primary 32-worker and equal-log original-eight aggregate checks require upper ON/control ratios below 1 and at least 21/31 wins against both controls. Other cases require upper ratios ≤1.02, every OFF/accepted calibration interval must fit factor 1.03, and maximum ON RSS must be ≤1.05 times each control. The relative RSS guard is not an architecture cap. Sorted ranks 10–22 of 31 give nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No observations were excluded and no thresholds changed.\n\nAll **55,788 SDK requests** verify internally. All **972 fresh exported proofs** independently pass Gnark with public/image/key identity checks and uniqueness checks, bringing the retained distinct-proof ledger from 22,990 to 23,962. Native OFF/ON each pass 124 core and 8 prover tests; portable ON passes 101 core and 8 prover tests. The byte-exact field sources inherit 35 passed field tests and the caller’s integer model. All six counter panels meet running-coverage checks and all six profiles pass attribution/loss checks. [Full SDK report, all checks, profiles, source and proof receipts](../2026-10-30-loose-delta-sdk175/README.md).\n\n## Other findings and limits\n\nThe earlier combined cached-hash/queue-drain candidate measured 7.846 ms for local 1×2 and 87 MiB peak RSS, with a 2.47% paired gain versus accepted and 35.68% lower RSS. It remains unadopted: its 8×2 nonregression interval exceeded 1.02 and physical-16 OFF/accepted calibration failed. An interval spanning parity does not establish a slowdown equal to its upper endpoint. This candidate is separate from bounded G2 delta. [Complete combined experiment](../2026-10-29-cached-drain175/README.md).\n\nLocal hardware counters show much larger growth in summed CPU time than in retired instructions as worker count rises. That supports investigating stalls, contention and coordination, but does not isolate a cause. Absolute sample-period accounting failed 21/24 panels in a separate diagnostic. Profile shares remain qualitative; they do not measure exclusive stage costs, local CPI or removable wall time. [Worker counters](../2026-10-29-worker-work175/README.md), [sampling accounting](../2026-10-29-sample-accounting175/README.md).\n\nThere is no new AWS measurement of the latest candidate. These local runs do not qualify minimum RAM, swap, memory-pressure behavior, ARM/mobile/TEE, forester 250/4999, or concurrent proof throughput. Loading and preparation remain excluded from proof latency.\n\nThe next decision must follow complete-request evidence. A candidate that passes performance still needs resource/admission and final-artifact qualification before adoption. Further work should remove useful arithmetic or coordination cost in complete MSM callers while preserving bounded memory. This report does not forecast a 2× breakthrough.\n\n## Records\n\n[Machine-readable snapshot](data.json), [source hashes](source-hashes.json), [offline report generator](refresh.rs), [checksums](SHA256SUMS). The previous [29 October report](../2026-10-29-cpu-benchmark-report/README.md) remains a dated snapshot.\n");
    let endpoint_dir=root.join("docs/benchmarks/2026-10-30-g2-endpoints175");
    let endpoint=read(endpoint_dir.join("artifacts/analysis.json"));
    assert_eq!(endpoint["complete"],true);
    assert_eq!(endpoint["caller_gate"],false);
    assert_eq!(endpoint["proof_ledger"],23971);
    assert_eq!(endpoint["SDK_requests"],0);
    assert_eq!(endpoint["production_changed"],false);
    let mut update=String::from("## Latest completed caller experiment\n\nThe G2 prefix-endpoint candidate removes multiplication by one in the first forward group and an unused final inverse update. It changes no heap buffer or key table. On real captured operands, it is **slower and rejected before SDK timing**. All eleven paired case intervals lie above parity. These microseconds measure complete batch waves, not proofs. [Full caller experiment and profiles](../2026-10-30-g2-endpoints175/README.md).\n\n|Case|OFF µs/batch wave|ON µs/batch wave|ON/OFF ratio|Bounds|Wins /31|Check|\n|---|---:|---:|---:|---|---:|---|\n");
    for(key,p)in endpoint["pairs"].as_object().unwrap(){
        let s=&p["statistics"];
        let off=&endpoint["cells"][format!("{key}/OFF")];
        let on=&endpoint["cells"][format!("{key}/ON")];
        writeln!(update,"|{key}|{:.6}|{:.6}|{:.9}|[{:.9}, {:.9}]|{}|{}|",n(off,"median_ns_per_batch_wave")/1000.,n(on,"median_ns_per_batch_wave")/1000.,n(s,"median"),n(s,"low"),n(s,"high"),s["wins"],p["passed"]).unwrap();
    }
    let a=&endpoint["original8_aggregate"]["statistics"];
    writeln!(update,"\nPrimary32 is **1.311% slower**. The equal-log original-eight aggregate is **{:.9} [{:.9}, {:.9}], {}/31 wins**, or 1.454% slower. Failed checks: `{}`. Passing a nonregression tolerance does not establish a gain.\n",n(a,"median"),n(a,"low"),n(a,"high"),a["wins"],endpoint["failed_checks"]).unwrap();
    update.push_str("The 31 fixed blocks retain all nine dominant200 cases at32 workers, primary16 and primary owned512. All152 inherited real fixtures preflight in each arm; outputs and capacities match with zero timed allocations. The run completes742 workflow steps and278,889,664 kernel calls. Primary instructions fall about0.575% in the diagnostic panels while cycles rise2.11% at16 workers and2.79% at32. This does not isolate a microarchitectural cause. All six counter and six profile-attribution panels pass; the known absolute sample-accounting limitation remains. No SDK request or independent Gnark export is added by this caller run.\n\nThe preceding production-operand diagnostic contributed36 SDK requests and9 fresh independently verified proofs. It checked152 selected batches containing42,658 signed point additions with Arkworks. The current retained distinct-proof ledger is **23,971**. Captures are order-selected, not frequency-weighted; compact replay does not preserve original key/bucket locality or the proof scheduler. [Replay census and independent checks](../2026-10-30-real-g2-replay175/README.md).\n\n## Current experiment: native stage only\n\nThe cold-tail candidate extracts the exact scalar fallback statements from the two G2 prefix functions into private cold, never-inline helpers. It adds no heap buffer or per-key lookup table. Its12-step native workflow is complete: OFF and ON each pass127 unfiltered core tests and8 prover tests, including a512KiB stack test covering owned/borrowed calls, full batches, tails and both inversion modes. SDK consumers are linked, not benchmarked. Native receipts and the prospective design are saved below.\n\n**No latency, peak prover RSS, calibration or performance-gate result exists for this candidate at this snapshot.** The native stack test does not establish a production stack bound or minimum RAM. The planned31-block caller comparison covers frozen/OFF/ON builds,22 contexts including every captured tail fixture, and automatic counters/profiles. A caller pass would still require separate complete-proof, portable, resource and final-artifact qualification before adoption. The rejected endpoint arithmetic is absent.\n\n");
    md=md.replacen("including the completed bounded G2 delta SDK experiment", "including the completed G2 prefix-endpoint caller experiment and the cold-tail native stage",1);
    md=md.replacen("## Records\n",&format!("{update}## Records\n"),1);
    md=md.replace("The previous [29 October report](../2026-10-29-cpu-benchmark-report/README.md) remains a dated snapshot.","The earlier [30 October full-proof report](../2026-10-30-cpu-benchmark-report/README.md) remains a dated snapshot. [Cold-tail native receipts](artifacts/cold-tail-native.json), [prospective design](artifacts/cold-tail-design.md), [stack test source](artifacts/cold-tail-stack-test.rs).");
    md=md.replacen("The latest full-proof candidate measures", "The latest completed caller experiment is **1.31% slower for primary 1×2 replay** and was rejected before proof timing. The next cold-tail candidate has completed native tests and builds; **timing has not started**.\n\nThe latest full-proof candidate measures",1);
    for(a,b)in[("Primary32 is","Primary at 32 workers is"),("dominant200 cases at32 workers, primary16 and primary owned512","dominant 200-pair cases at 32 workers, primary at 16 workers and primary owned 512-pair batches"),("All152","All 152"),("completes742","completes 742"),("and278,889,664","and 278,889,664"),("about0.575%","about 0.575%"),("rise2.11% at16 workers and2.79% at32","rise 2.11% at 16 workers and 2.79% at 32"),("contributed36","contributed 36"),("and9 fresh","and 9 fresh"),("checked152","checked 152"),("containing42,658","containing 42,658"),("Its12-step","Its 12-step"),("pass127","pass 127"),("and8 prover","and 8 prover"),("a512KiB","a 512 KiB"),("planned31-block","planned 31-block"),("builds,22 contexts","builds, 22 contexts")]{md=md.replace(a,b);}
    let cold=Path::new("/home/ubuntu/src/zopro-perf-work/g2-cold-tail175");
    let native=read(cold.join("native/result.json"));
    assert_eq!(native["status"],"complete");
    assert_eq!(native["steps"].as_array().unwrap().len(),12);
    assert!(!cold.join("runs/result.json").exists());
    assert!(!cold.join("analysis.json").exists());
    let mut native_tests=BTreeMap::new();
    let mut native_pins=BTreeMap::new();
    for arm in ["control","candidate"]{for(krate,count)in[("core",127),("prover",8)]{
        let base=format!("native/{arm}-{krate}");
        let tests=read(cold.join(format!("{base}/tests.json")));
        assert_eq!(tests["unfiltered"],true);
        assert_eq!(tests["passed_tests"].as_array().unwrap().len(),count);
        native_tests.insert(format!("{arm}-{krate}"),tests);
        for name in ["tests.json","exit.json","command.json","stdout.log","stderr.log"]{let p=format!("{base}/{name}");native_pins.insert(p.clone(),hash(cold.join(p)));}
    }}
    for name in ["native/result.json","source-audit.json","design.md","stack-test.rs","build-plan.json","build-pins.json"]{native_pins.insert(name.to_string(),hash(cold.join(name)));}
    let cold_record=json!({"stage":"native complete; caller timings not started", "local_root":cold,"native_result":native,"tests":native_tests,"source_audit":read(cold.join("source-audit.json")),"local_source_hashes":native_pins,"SDK_bench_requests":0,"new_independent_exports":0,"production_changed":false,"performance_gate":null,"prover_peak_RSS_MiB":null});
    let mut pins=BTreeMap::new();
    for p in [endpoint_dir.join("README.md"),endpoint_dir.join("artifacts/analysis.json"),root.join("docs/benchmarks/2026-10-30-real-g2-replay175/README.md"),root.join("docs/benchmarks/2026-10-30-real-g2-replay175/artifacts/analysis.json")]{pins.insert(p.strip_prefix(root).unwrap().display().to_string(),hash(&p));}
    for path in [summary.join("README.md"),summary.join("data.json"),summary.join("source-hashes.json"),sdk_dir.join("README.md"),sdk_dir.join("artifacts/analysis.json"),sdk_dir.join("artifacts/publication-review.json"),root.join("docs/benchmarks/2026-10-29-cpu-benchmark-report/data.json"),root.join("docs/benchmarks/2026-10-29-proof-pgo-c7a175/artifacts/analysis.json")] {
        pins.insert(path.strip_prefix(root).unwrap().display().to_string(),hash(&path));
    }
    for(path,expected)in read(summary.join("source-hashes.json")).as_object().unwrap(){assert_eq!(hash(root.join(path)),expected.as_str().unwrap());}
    let manifest=read(root.join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    assert_eq!(manifest["source_files_sha256"].as_object().unwrap().len(),175);
    for(path,expected)in manifest["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(root.join(path)),expected.as_str().unwrap());}
    let data=json!({"as_of":"2026-10-30","source_commit":"84c807dcaa2cc359863b55b42224bdfa6a9559d7","snapshot_kind":"Accepted local/AWS results; completed G2 endpoint caller; cold-tail native stage only; no new captures for this document","rows":data["rows"],"accepted_original8_CPU10x":0,"historical_distinct_Gnark_ledger":23971,"latest_sdk_analysis":sdk,"latest_endpoint_caller":endpoint,"cold_tail_stage":cold_record,"new_timings_for_document":0,"new_tests_for_document":0,"new_proofs_for_document":0,"production_changed":false});
    fs::create_dir_all(&dest).unwrap();
    fs::create_dir_all(dest.join("artifacts")).unwrap();
    fs::write(dest.join("artifacts/cold-tail-native.json"),serde_json::to_vec_pretty(&cold_record).unwrap()).unwrap();
    fs::copy(cold.join("design.md"),dest.join("artifacts/cold-tail-design.md")).unwrap();
    fs::copy(cold.join("stack-test.rs"),dest.join("artifacts/cold-tail-stack-test.rs")).unwrap();
    fs::write(dest.join("README.md"),md).unwrap();
    // Retain the completed records' original decimal spellings in the snapshot.
    let mut exact_data=data.clone();
    exact_data["latest_sdk_analysis"]=json!("__RAW_COMPLETED_SDK_RECORD__");
    exact_data["latest_endpoint_caller"]=json!("__RAW_COMPLETED_CALLER_RECORD__");
    let exact_text=serde_json::to_string_pretty(&exact_data).unwrap()
        .replace("\"__RAW_COMPLETED_SDK_RECORD__\"",&fs::read_to_string(sdk_dir.join("artifacts/analysis.json")).unwrap())
        .replace("\"__RAW_COMPLETED_CALLER_RECORD__\"",&fs::read_to_string(endpoint_dir.join("artifacts/analysis.json")).unwrap());
    fs::write(dest.join("data.json"),exact_text).unwrap();
    fs::write(dest.join("source-hashes.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
    fs::write(dest.join("refresh.rs"),fs::read(file!()).unwrap()).unwrap();
    let sums=["README.md","data.json","refresh.rs","source-hashes.json","artifacts/cold-tail-native.json","artifacts/cold-tail-design.md","artifacts/cold-tail-stack-test.rs"].iter().map(|p|format!("{}  {p}\n",hash(dest.join(p)))).collect::<String>();
    fs::write(dest.join("SHA256SUMS"),sums).unwrap();
    let md=fs::read_to_string(dest.join("README.md")).unwrap();
    for tail in md.split("](").skip(1){let p=tail.split(')').next().unwrap();assert!(dest.join(p).is_file(),"missing link {p}");}
    println!("Wrote current report: 9 unchanged accepted local/AWS rows, 8 CPU/Metal references, completed SDK/caller results, cold-tail native stage; ledger 23971; no new captures");
}
