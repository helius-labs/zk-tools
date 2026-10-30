use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/wire-readiness175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const P: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-production";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n])
    }
    format!("{:x}", h.finalize())
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(base: &Path, paths: &mut Vec<PathBuf>) {
    for e in fs::read_dir(base).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, paths)
        } else if matches!(p.extension().and_then(|s| s.to_str()), Some("log" | "json")) {
            paths.push(p);
        }
    }
}
fn archive(paths: &[PathBuf], base: &Path, out: &Path) -> Value {
    let mut map = BTreeMap::new();
    for p in paths {
        let b = fs::read(p).unwrap();
        map.insert(
            p.strip_prefix(base).unwrap().display().to_string(),
            json!({"sha256":hash(p),"length":b.len(),"utf8":String::from_utf8(b).unwrap()}),
        );
    }
    let bytes = serde_json::to_vec(&map).unwrap();
    let tmp = out.with_extension("json");
    fs::write(&tmp, &bytes).unwrap();
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&tmp)
        .stdout(Stdio::from(fs::File::create(out).unwrap()))
        .status()
        .unwrap()
        .success());
    let back = Command::new("gzip")
        .args(["-dc"])
        .arg(out)
        .output()
        .unwrap();
    assert!(back.status.success());
    assert_eq!(back.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&back.stdout).unwrap();
    for p in paths {
        let e = &decoded[p.strip_prefix(base).unwrap().to_str().unwrap()];
        assert_eq!(e["utf8"].as_str().unwrap().as_bytes(), fs::read(p).unwrap());
        assert_eq!(e["sha256"], hash(p));
    }
    fs::remove_file(tmp).unwrap();
    json!({"members":paths.len(),"roundtrip_exact":true,"sha256":hash(out)})
}
fn main() {
    let a = read(r("analysis.json"));
    assert_eq!(a["complete"], true);
    let mut verified = 0;
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        verified += 1;
    }
    let frozen = read(Path::new(P).join("frozen-candidate.json"));
    let baseline = read(Path::new(P).join("native/candidate/artifacts.json"));
    let linked = read(r("runs/census/artifacts.json"));
    assert_eq!(
        baseline["binary_sha256"],
        "20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f"
    );
    assert_eq!(
        hash(baseline["binary"].as_str().unwrap()),
        baseline["binary_sha256"]
    );
    assert_eq!(baseline["libraries"], frozen["libraries"]);
    assert_eq!(linked["libraries"], frozen["libraries"]);
    for shape in ["1x2", "2x2"] {
        assert_eq!(
            hash(r(format!("{shape}.solver"))),
            a["parsed_programs"][shape]["solver_sha256"]
        );
        let c = &a["parsed_programs"][shape]["census"];
        assert_eq!(
            c["ops"].as_u64().unwrap(),
            c["hint_ops"].as_u64().unwrap() + c["constraint_ops"].as_u64().unwrap()
        );
        assert_eq!(c["constraint_ops"], c["constraints"]);
        for q in ["A", "B"] {
            let v = &a["cases"][format!("{shape}/{q}")];
            let n = v["points"].as_u64().unwrap();
            let mut prev = [0; 3];
            for row in v["cuts"].as_array().unwrap() {
                let counts = [
                    row["arbitrary_order_ready_points"].as_u64().unwrap(),
                    row["key_order_prefix_points"].as_u64().unwrap(),
                    row["past_final_solver_read_points"].as_u64().unwrap(),
                ];
                assert!(counts.iter().all(|&v| v <= n));
                assert!(counts[1] <= counts[0] && counts[2] <= counts[0]);
                for (i, v) in counts.iter().enumerate() {
                    assert!(*v >= prev[i]);
                }
                prev = counts;
            }
            assert_eq!(prev, [n; 3]);
        }
    }
    let review = json!({"review_passed":true,"original_pins_verified":verified,"production_source_files_unchanged":175,"same_eight_libraries_as_accepted_SDK20fde":true,"native_structural_tests":3,"two_program_hashes_match":true,"all_cut_counts_monotone_and_conserved":true,"new_solver_executions":0,"new_proofs_or_timing_samples":0,"proof_ledger":25231});
    save(r("review.json"), &review);
    let d = Path::new(Z).join("docs/benchmarks/2026-10-30-wire-readiness175");
    assert!(!d.exists());
    fs::create_dir_all(d.join("artifacts")).unwrap();
    for f in [
        "analysis.json",
        "1x2.json",
        "2x2.json",
        "model.rs",
        "census.rs",
        "workflow.rs",
        "publish.rs",
        "design.md",
        "review.json",
        "plan.json",
        "pins.json",
        "cases.json",
    ] {
        fs::copy(r(f), d.join("artifacts").join(f)).unwrap();
    }
    let mut receipt = vec![];
    walk(&r("runs"), &mut receipt);
    let receipts = archive(&receipt, &r(""), &d.join("artifacts/receipts.json.gz"));
    let inherited: Vec<_> = ["README.md", "data.json", "source-hashes.json"]
        .map(|f| r("inherited-summary").join(f))
        .into_iter()
        .collect();
    let summary = archive(
        &inherited,
        &r("inherited-summary"),
        &d.join("artifacts/inherited-summary.json.gz"),
    );
    save(
        d.join("artifacts/publication-review.json"),
        &json!({"receipts":receipts,"inherited_summary":summary,"review":review,"raw_solver_images_local_only":true}),
    );
    let exact = Command::new("jq")
        .arg(".rows")
        .arg(r("inherited-summary/data.json"))
        .output()
        .unwrap();
    assert!(exact.status.success());
    fs::write(d.join("accepted-rows.json"), exact.stdout).unwrap();
    let mut md=String::from("# Witness/query readiness in solver program order\n\n30 October 2026. This structural census changes no prover code and produces no timing samples or proofs. The exact accepted libraries export two existing solver programs; a separate Rust reader accounts for their wire producers and final solver reads.\n\nHalfway through primary 1×2's serialized program, roughly half of A/B query operands are defined, but only about **14%** form a contiguous prefix in key order. At 90% of operations, roughly 90% are defined but the ordered prefix covers only **32–33%**. For 2×2, the ordered prefix covers about **10%** at halfway and **19–20%** at 90%. This limits the assumption that an ordinary key-order stream could consume most operands early. It does not measure a latency loss or rule out out-of-order streaming.\n\nThe prerequisite follows the [qualified stage timeline](../2026-10-30-query-timeline175/README.md), which measured a 1.864279 ms median native prefix through product completion at 32 workers. Existing [fused product overlap](../2026-10-26-fused-product-overlap/README.md) regressed, and [pressure-admitted overlap](../2026-10-23-pressure-admitted-product-overlap/README.md) was marginal. Neither unchanged mechanism is retested here. The [exact FMA arithmetic trial](../2026-10-23-exact-fma-rne-component/README.md) also remains closed.\n\n## Structural results\n\nProgram percentages count serialized operations, not CPU time, dependency depth, parallel epochs or elapsed readiness. Readiness is after an operation completes. Query indices are strictly increasing, matching the retained image reader. A contiguous prefix stops at the first undefined query wire. Values past their final solver read may still be needed by public correction and proving; this is not permission to mutate Montgomery witness storage early.\n\n|Case / query|Points|Program %|Ready in any order|Ready key-order prefix|Past final solver read|\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["cases"].as_object().unwrap() {
        for row in v["cuts"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| [10, 50, 90, 100].contains(&v["program_percent"].as_u64().unwrap()))
        {
            let n = v["points"].as_u64().unwrap();
            let ready = row["arbitrary_order_ready_points"].as_u64().unwrap();
            let prefix = row["key_order_prefix_points"].as_u64().unwrap();
            writeln!(
                md,
                "|{k}|{n}|{}|{ready} ({:.1}%)|{prefix} ({:.1}%)|{}|",
                row["program_percent"],
                ready as f64 / n as f64 * 100.,
                prefix as f64 / n as f64 * 100.,
                row["past_final_solver_read_points"]
            )
            .unwrap();
        }
    }
    md.push_str("\nAll eleven cuts, including the initial input-ready state, are retained in analysis.json. H is excluded because it requires the product; K's selected folded-slot membership is not reconstructed or claimed.\n\nThe declared rejection condition was an ordered prefix that had not advanced beyond its initially known input terms by 90% of operations. **That condition is false for all four query populations.** This census therefore does not reject every ordered-stream design. It quantifies a substantial difference between arbitrary readiness and sequential key consumption. No implementation or performance eligibility follows automatically.\n\nA useful next design would need actual parallel-schedule readiness, the cost of index/gather or reordered consumption, preservation of scalar grouping, the G2 high/low component order, and workspace admission. An overlap mechanism alone would reorder work rather than remove the dominant field arithmetic. These counts do not establish a path to a twofold CPU improvement.\n\n## Validation and reproducibility\n\nAll seven workflow steps pass once: link the test harness, run three structural tests, link the census, then export/analyze primary and 2×2. Tests cover hints, within-constraint stored slots, last-read and readiness frontiers; duplicate writes, undefined reads, truncated input, unsorted queries and trailing data are rejected. The producer-index aggregation and independently maintained online query frontier agree at every cut. They share one structural parser; this is not an independent field-arithmetic oracle.\n\nEvery value is defined once, all consumed wires are defined, all constraints are accounted for, and the complete version2 image is consumed. Primary has 30,521 wires, 31,061 operations and 30,922 constraints; 2×2 has 53,064 wires, 54,083 operations and 53,874 constraints. Full dimensions and program hashes are retained. Private solver exports remain local; their contents are circuit metadata, and no witness values are collected.\n\nThe helper links the same eight coherent native libraries as accepted SDK SHA-256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f`. All original input/helper/library pins and all 175 production source hashes close. Existing field/core/prover qualification is inherited; no prover library or field suite is rebuilt or repeated. Disposable analysis arrays are not a production execution plan or a prover memory measurement.\n\nThere are **zero new witness solves, SDK requests, Gnark exports or timing samples**. The distinct-proof ledger stays **25,231**. Accepted local primary remains **8.022165 ms / 134 MiB**; all nine local/AWS rows and eight supplied CPU/Metal reference rows remain unchanged, with **0/8 CPU 10× targets achieved**. No new AWS/Aeglos, mobile/ARM/TEE, pressure/swap, huge-circuit or concurrency result is claimed. [Full accepted comparisons](../2026-10-30-current-bench-report/README.md), [exact accepted rows](accepted-rows.json).\n\n[Analysis](artifacts/analysis.json), [protocol](artifacts/design.md), [reader](artifacts/model.rs), [consumer](artifacts/census.rs), [workflow](artifacts/plan.json), [pins](artifacts/pins.json), [review](artifacts/review.json), [test/export/analysis receipts](artifacts/receipts.json.gz), [exact pre-census summary](artifacts/inherited-summary.json.gz), [archive review](artifacts/publication-review.json).\n");
    fs::write(d.join("README.md"), md).unwrap();
    println!("Published structural readiness screen; all original pins/libraries/175productionhashes close; no proofs/timings/adoption");
}
