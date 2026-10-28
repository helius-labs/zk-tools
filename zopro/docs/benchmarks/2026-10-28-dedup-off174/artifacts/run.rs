use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/dedup-off174";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-dedup-off174";
const PARENT: &str = "/home/ubuntu/src/zopro-perf-work/current174-g2-callers";
const SDK: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/native/candidate-sdk";
const HASH: &str = "81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption/artifacts/production-source-manifest.json";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
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
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn output(a: &[&str]) -> String {
    let o = Command::new(a[0]).args(&a[1..]).output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn prepare() {
    assert!(!r("plan.json").exists());
    assert_eq!(hash(SDK), HASH);
    let mut pins = BTreeMap::new();
    for (f, h) in read(MANIFEST)["source_files_sha256"].as_object().unwrap() {
        let p = Path::new("/home/ubuntu/src/zk-tools/zopro").join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{PARENT}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 15053);
    for (f, h) in read(format!("{PARENT}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 15059);
    save(r("prior-proof-sha256.json"), &json!(prior));
    let cases = read("/home/ubuntu/src/zopro-perf-work/current174-stages/cases.json");
    assert_eq!(cases.as_array().unwrap().len(), 4);
    save(r("cases.json"), &cases);
    let env = read("/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/run-plan.json")
        ["steps"][0]["environment"]
        .clone();
    let mut jobs = vec![];
    let mut steps = vec![];
    for b in 0..7 {
        let mut order = vec![0, 1, 2, 3];
        order.rotate_left(b % 4);
        if b % 2 != 0 {
            order.reverse();
        }
        for i in order {
            let c = &cases[i];
            let labels = if (b + i) % 2 == 0 {
                ["dedup", "plain"]
            } else {
                ["plain", "dedup"]
            };
            for label in labels {
                let name = format!("b{b}-{}-{label}", c["name"].as_str().unwrap());
                let rounds = if c["shape"] == "1x2" { 151 } else { 31 };
                let mut e = env.clone();
                e["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
                e["ZOPRO_CPU_DEDUP"] = json!(if label == "dedup" { "1" } else { "0" });
                let dir = r("proofs").join(&name);
                let argv = json!([
                    "taskset",
                    "-c",
                    c["cpu_affinity"],
                    SDK,
                    c["image"],
                    c["request"],
                    rounds.to_string(),
                    dir
                ]);
                steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":e,"timeout_seconds":900}));
                jobs.push(json!({"name":name,"case":c,"variant":label,"block":b,"rounds":rounds,"directory":dir}));
            }
        }
    }
    assert_eq!(jobs.len(), 56);
    for j in &jobs {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        steps.push(json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 112);
    save(r("jobs.json"), &json!(jobs));
    for c in cases.as_array().unwrap() {
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
        pin(&mut pins, c["image"].as_str().unwrap());
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    for ent in
        fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap()
    {
        pin(&mut pins, ent.unwrap().path());
    }
    for f in [SDK, MANIFEST, VERIFY, X] {
        pin(&mut pins, f);
    }
    for f in [
        "design.md",
        "run.rs",
        "run-helper",
        "cases.json",
        "jobs.json",
        "prior-proof-sha256.json",
    ] {
        pin(&mut pins, r(f));
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    fs::write(
        r("host-before.txt"),
        output(&[
            "ps",
            "-eo",
            "pid,ppid,comm,pcpu,pmem,etimes",
            "--sort=-pcpu",
        ]),
    )
    .unwrap();
    println!("Prepared112steps,56captures/5264SDK/56Gnark;sameinode/explicitone-byteDEDUP1vs0");
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (f, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 15059);
    let mut inventory = BTreeMap::new();
    let mut cells = BTreeMap::<String, Vec<Value>>::new();
    let mut requests = 0;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let times = text
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| {
                serde_json::from_str::<Value>(l).unwrap()["ms"]
                    .as_f64()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(times.len() as u64, j["rounds"].as_u64().unwrap());
        assert!(times.iter().all(|x| x.is_finite() && *x > 0.));
        let s: Value = serde_json::from_str(
            text.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(s["workers"], j["case"]["workers"]);
        assert_eq!(s["rounds"], j["rounds"]);
        assert_eq!(s["priming"], 3);
        requests += times.len() + 3;
        let verified = fs::read_to_string(r(format!("runs/verify-{name}/stdout.log"))).unwrap();
        assert!(verified.contains("VERIFIED proof_sha256="));
        assert!(verified.contains(&format!(
            "image_sha256={}",
            j["case"]["image_sha256"].as_str().unwrap()
        )));
        for ext in ["proof", "public"] {
            let p = Path::new(j["directory"].as_str().unwrap()).join(format!("0.{ext}"));
            let h = hash(&p);
            if ext == "proof" {
                assert_eq!(fs::metadata(&p).unwrap().len(), 164);
                assert!(prior.insert(h.clone()));
            } else {
                assert_eq!(h, j["case"]["public_sha256"].as_str().unwrap());
            }
            assert!(verified.contains(&format!("{ext}_sha256={h}")));
            inventory.insert(p.display().to_string(), h);
        }
        let cpu = (s["user_cpu_ms"].as_f64().unwrap() + s["system_cpu_ms"].as_f64().unwrap())
            / times.len() as f64;
        let rss = s["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.;
        cells.entry(format!("{}/{}",j["case"]["name"].as_str().unwrap(),j["variant"].as_str().unwrap())).or_default().push(json!({"name":name,"block":j["block"],"median_ms":median(&times),"CPU_ms":cpu,"RSS_MiB":rss,"summary":s}));
    }
    assert_eq!(requests, 5264);
    assert_eq!(prior.len(), 15115);
    assert_eq!(inventory.len(), 112);
    save(r("proof-inventory.json"), &json!(inventory));
    let mut summaries = BTreeMap::new();
    for (k, rows) in &cells {
        assert_eq!(rows.len(), 7);
        let collect = |f: &str| {
            rows.iter()
                .map(|r| r[f].as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        summaries.insert(k.clone(),json!({"median_ms":median(&collect("median_ms")),"CPU_ms":median(&collect("CPU_ms")),"median_RSS_MiB":median(&collect("RSS_MiB")),"max_RSS_MiB":collect("RSS_MiB").into_iter().fold(0.,f64::max),"blocks":rows}));
    }
    let mut pairs = BTreeMap::new();
    let mut refs = BTreeMap::new();
    for c in read(r("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap();
        let a = &cells[&format!("{case}/plain")];
        let b = &cells[&format!("{case}/dedup")];
        let mut ratios = vec![];
        let mut savings = vec![];
        for i in 0..7 {
            assert_eq!(a[i]["block"], i);
            assert_eq!(b[i]["block"], i);
            let x = a[i]["median_ms"].as_f64().unwrap();
            let y = b[i]["median_ms"].as_f64().unwrap();
            ratios.push(x / y);
            savings.push(y - x);
        }
        let mut ordered = ratios.clone();
        ordered.sort_by(f64::total_cmp);
        let wins = ratios.iter().filter(|&&x| x < 1.).count();
        pairs.insert(case.to_string(),json!({"median":ordered[3],"low":ordered[0],"high":ordered[6],"wins":wins,"ratios":ratios,"paired_saving_ms":median(&savings),"nominal_coverage":0.984375}));
        if c["workers"] == 32 && !c["reference"].is_null() {
            let mut vs = BTreeMap::new();
            for v in ["dedup", "plain"] {
                let ms = summaries[&format!("{case}/{v}")]["median_ms"]
                    .as_f64()
                    .unwrap();
                let cpu = c["reference"]["cpu_ms"].as_f64().unwrap();
                let metal = c["reference"]["metal_ms"].as_f64().unwrap();
                vs.insert(v,json!({"ms":ms,"CPU_speedup":cpu/ms,"Metal_speedup":metal/ms,"CPU10x":ms<=cpu/10.}));
            }
            refs.insert(
                case.to_string(),
                json!({"reference":c["reference"],"variants":vs}),
            );
        }
    }
    let primary = &pairs["1x2-w32"];
    let advances = primary["high"].as_f64().unwrap() < 1. && primary["wins"].as_u64().unwrap() >= 5;
    let analysis = json!({"status":"complete existing-switch comparison; productionunchanged","primary_advancement_condition":advances,"pairs":pairs,"cells":summaries,"new_reference_comparisons":refs,"inherited_accepted_M5":read(MANIFEST)["m5"],"SDK_requests":requests,"new_independent_exports":56,"independent_proof_count":15115,"acceptedSDK":SDK,"acceptedSDK_sha256":HASH,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"accepted_original8_CPU10x_rows":0,"scope":"configuration comparison on4cells only; not globaldefault or all8 candidate qualification"});
    save(r("analysis.json"), &analysis);
    fs::write(
        r("host-after.txt"),
        output(&[
            "ps",
            "-eo",
            "pid,ppid,comm,pcpu,pmem,etimes",
            "--sort=-pcpu",
        ]),
    )
    .unwrap();
    println!("Primaryadvance{advances};5264SDK/56freshGnark/ledger15115");
    for (k, v) in &summaries {
        println!(
            "{k} {:.6}ms CPU{:.3} RSS{:.2}/{:.2}",
            v["median_ms"].as_f64().unwrap(),
            v["CPU_ms"].as_f64().unwrap(),
            v["median_RSS_MiB"].as_f64().unwrap(),
            v["max_RSS_MiB"].as_f64().unwrap()
        );
    }
    for (k, v) in pairs {
        println!(
            "{k} plain/dedup{} [{},{}],wins{},saving{}ms",
            v["median"], v["low"], v["high"], v["wins"], v["paired_saving_ms"]
        );
    }
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn seal() {
    let d = Path::new(D);
    let mut paths = vec![];
    walk(d, &mut paths);
    paths.sort();
    let mut pins = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(d).unwrap().display().to_string();
        if ["SHA256SUMS", "artifact-index.json"].contains(&rel.as_str()) {
            continue;
        }
        pins.insert(rel, hash(p));
    }
    save(d.join("artifact-index.json"), &json!(pins));
    pins.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        pins.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Sealed{}hashes", pins.len());
}
fn archive() {
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    let a = read(r.join("analysis.json"));
    let mut text=String::from("# Existing scalar-grouping switch on the adopted SDK\n\nOne immutable adopted174-source SDK is compared with explicit ZOPRO_CPU_DEDUP=1(dedup) and0(plain). No source, native library or executable changes. This bypasses optional grouping across queries and changes its workspace forecast, so it does not isolate only G2 or justify a global default change. Seven fixed interleaved blocks, primary16/32 plus16x2/32 andmerge54/32,151primary or31other requests after3primes; no profiling or owned overlapping CPU work. Witness solving, randomized proving, SDK verification and serialization are timed; loading is excluded. Labels and within-pair argument/environment lengths match.\n\n| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |\n|---|---|---:|---:|---:|---:|\n");
    for (k, v) in a["cells"].as_object().unwrap() {
        let (case, variant) = k.split_once('/').unwrap();
        writeln!(
            text,
            "| {case} | {variant} | {:.6} | {:.3} | {:.2} | {:.2} |",
            v["median_ms"].as_f64().unwrap(),
            v["CPU_ms"].as_f64().unwrap(),
            v["median_RSS_MiB"].as_f64().unwrap(),
            v["max_RSS_MiB"].as_f64().unwrap()
        )
        .unwrap();
    }
    text.push_str("\n| Cell | Plain/dedup paired median | Lower | Upper | Wins | Paired saving ms |\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["pairs"].as_object().unwrap() {
        writeln!(
            text,
            "| {k} | {:.9} | {:.9} | {:.9} | {}/7 | {:.6} |",
            v["median"].as_f64().unwrap(),
            v["low"].as_f64().unwrap(),
            v["high"].as_f64().unwrap(),
            v["wins"],
            v["paired_saving_ms"].as_f64().unwrap()
        )
        .unwrap();
    }
    writeln!(text,"\nPrimary advancement condition passed: **{}**. Bounds use ordered extremes of7, nominal98.4375% under independent exchangeable blocks, without familywise guarantees. The prospective primary condition requires upper<1 and at least5/7wins. Other cells are retained to expose broader tradeoffs. There is no minimum gain floor, exclusion, extension or successful-capture repeat. No source or default configuration is adopted.\n",a["primary_advancement_condition"]).unwrap();
    text.push_str("The [caller diagnostic](../2026-10-28-current174-g2-callers/README.md) motivated this actual bypass comparison. Its interval alone did not establish a full-map rejection or removable CPU time. All5264SDKcalls verify internally;56fresh164-byte exports independently pass Gnark/public bindings/freshness checks, ledger15059→15115. Native arithmetic and prior fallback tests are inherited; this study does not rerun their suites. Source174, SDK81033 and all input pins match after capture. RSS is whole-process high water including load/primes, not minimum RAM or requested-allocation traffic.\n\nNew comparisons against supplied M5 references cover only1x2 and16x2.\n\n| Cell | Mode | ms | M5 CPU speedup | M5 Metal speedup |\n|---|---|---:|---:|---:|\n");
    for (k, v) in a["new_reference_comparisons"].as_object().unwrap() {
        for (mode, x) in v["variants"].as_object().unwrap() {
            writeln!(
                text,
                "| {k} | {mode} | {:.6} | {:.3}× | {:.3}× |",
                x["ms"].as_f64().unwrap(),
                x["CPU_speedup"].as_f64().unwrap(),
                x["Metal_speedup"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    text.push_str("\nThe complete accepted-source all-eight comparison below is inherited, not a new candidate all-shape result. Original10×CPU goal remains active/unmet; accepted0/8. No supplied54reference. No new AWS/Aeglos, mobile/ARM/TEE, giant, swap, minimum-RAM or pressure qualification.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["inherited_accepted_M5"].as_object().unwrap() {
        writeln!(
            text,
            "| {k} | {:.6} | {} | {:.3}× | {} | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"],
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_reference_ms"],
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    text.push_str("\n[Protocol](artifacts/design.md), [all observations](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [source/input pins](artifacts/pins.json).\n");
    let mut paths = vec![];
    walk(r, &mut paths);
    for from in paths {
        let ext = from.extension().and_then(|x| x.to_str()).unwrap_or("");
        if !["rs", "md", "json", "txt", "log", "proof", "public"].contains(&ext) {
            continue;
        }
        let to = d.join("artifacts").join(from.strip_prefix(r).unwrap());
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        assert_eq!(hash(&from), hash(&to));
    }
    fs::write(d.join("README.md"), text).unwrap();
    seal();
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("prepare|finish|archive|seal"),
    }
}
