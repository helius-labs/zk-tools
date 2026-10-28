use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    io::Read,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const P: &str = "/home/ubuntu/src/zopro-perf-work/sdk-control-copies";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-qualification";
const A: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk";
const V: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-production";
const VS: [&str; 3] = ["accepted", "control", "candidate"];
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
fn snap(f: &str) {
    let o = Command::new("ps")
        .args(["-eo", "pid,ppid,comm,pcpu,pmem,etimes", "--sort=-pcpu"])
        .output()
        .unwrap();
    assert!(o.status.success());
    fs::write(r(f), o.stdout).unwrap();
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn num(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (f, h) in &pins {
        assert_eq!(hash(f), *h, "{f}");
    }
    let mut ids = BTreeMap::new();
    let mut inos = BTreeSet::new();
    for v in VS {
        let from = if v == "accepted" {
            PathBuf::from(A)
        } else {
            r(format!("native/{v}-sdk"))
        };
        let to = r(match v {
            "accepted" => "accept-sdk",
            "control" => "contro-sdk",
            _ => "borrow-sdk",
        });
        fs::hard_link(&from, &to).unwrap();
        let m = fs::metadata(&to).unwrap();
        assert!(inos.insert(m.ino()));
        assert_eq!(hash(&from), hash(&to));
        pin(&mut pins, &from);
        pin(&mut pins, &to);
        ids.insert(v,json!({"path":to,"source":from,"inode":m.ino(),"device":m.dev(),"sha256":hash(&to),"bytes":m.len()}));
    }
    save(r("identities.json"), &json!(ids));
    for l in ["control", "candidate"] {
        let p = r(format!("native/{l}/artifacts.json"));
        let a = read(&p);
        for (f, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            pin(&mut pins, f);
        }
        pin(&mut pins, p);
    }
    let mut cases = read(format!("{P}/cases.json")).as_array().unwrap().clone();
    assert_eq!(cases.len(), 9);
    for c in &cases {
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
        pin(&mut pins, c["image"].as_str().unwrap());
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    let mut extra = cases[0].clone();
    extra["name"] = json!("1x2-w16");
    extra["workers"] = json!(16);
    extra["cpu_affinity"] = json!("0-15");
    cases.push(extra);
    save(r("cases.json"), &json!(cases));
    for e in fs::read_dir(format!("{P}/lagrange-cache")).unwrap() {
        pin(&mut pins, e.unwrap().path());
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{Q}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 14462);
    for (f, h) in read(format!("{Q}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 14551);
    save(r("prior-proof-sha256.json"), &json!(prior));
    let mut env =
        read("/home/ubuntu/src/zopro-perf-work/center-census/run-plan.json")["environment"].clone();
    env["ZOPRO_BASIS_KEY_DIR"] = json!(format!("{P}/images"));
    env["ZOPRO_CPU_LAGRANGE_DIR"] = json!(format!("{P}/lagrange-cache"));
    let mut jobs = vec![];
    let mut steps = vec![];
    let mut push = |c: &Value, v: &str, b: usize, phase: &str| {
        let name = format!("{phase}-{b:02}-{}-{v}", c["name"].as_str().unwrap());
        let rounds = if phase == "preflight" {
            1
        } else if c["shape"] == "1x2" {
            151
        } else {
            31
        };
        let mut e = env.clone();
        e["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
        let dir = r("proofs").join(&name);
        steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c",c["cpu_affinity"],ids[v]["path"],c["image"],c["request"],rounds.to_string(),dir],"environment":e,"timeout_seconds":900}));
        jobs.push(json!({"name":name,"variant":v,"case":c,"block":b,"phase":phase,"rounds":rounds,"directory":dir}));
    };
    for c in cases.iter().take(9) {
        for v in VS {
            push(c, v, 0, "preflight");
        }
    }
    for b in 0..15 {
        let mut order: Vec<usize> = (0..10).collect();
        order.rotate_left(b % 10);
        if b % 2 != 0 {
            order.reverse();
        }
        for i in order {
            let mut vs = VS;
            vs.rotate_left((b + i) % 3);
            if (b + i) % 2 != 0 {
                vs.reverse();
            }
            for v in vs {
                push(&cases[i], v, b, "latency");
            }
        }
    }
    assert_eq!(jobs.len(), 477);
    for j in &jobs {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        steps.push(json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[V,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":180}));
    }
    save(r("jobs.json"), &json!(jobs));
    snap("host-before.txt");
    for f in [
        "final-protocol.md",
        "final.rs",
        "final-helper",
        "identities.json",
        "cases.json",
        "prior-proof-sha256.json",
        "jobs.json",
        "code-audit.json",
        "code-audit.rs",
        "source-audit.json",
        "native/result.json",
        "host-before.txt",
    ] {
        pin(&mut pins, r(f));
    }
    for f in [V, X] {
        pin(&mut pins, f);
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared fixed477SDK+477independent-verifier steps, no production mutation");
    state("final production artifact capture prepared", json!([]));
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (f, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap(), "{f}");
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 14551);
    let mut inventory = BTreeMap::new();
    let mut cells: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut requests = 0u64;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let t = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let times: Vec<f64> = t
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| num(&serde_json::from_str::<Value>(l).unwrap(), "ms"))
            .collect();
        let s: Value =
            serde_json::from_str(t.lines().find_map(|l| l.strip_prefix("SUMMARY ")).unwrap())
                .unwrap();
        let n = j["rounds"].as_u64().unwrap();
        assert_eq!(times.len() as u64, n);
        assert!(times.iter().all(|x| x.is_finite() && *x > 0.));
        assert_eq!(s["rounds"], n);
        assert_eq!(s["workers"], j["case"]["workers"]);
        assert_eq!(s["priming"], 3);
        requests += n + 3;
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
        if j["phase"] == "latency" {
            cells.entry(format!("{}/{}",j["case"]["name"].as_str().unwrap(),j["variant"].as_str().unwrap())).or_default().push(json!({"name":name,"block":j["block"],"median_ms":median(&times),"cpu_ms_per_request":(num(&s,"user_cpu_ms")+num(&s,"system_cpu_ms"))/n as f64,"RSS_MiB":num(&s,"whole_process_maxrss_kib")/1024.,"summary":s}));
        }
    }
    assert_eq!(prior.len(), 15028);
    assert_eq!(inventory.len(), 954);
    save(r("proof-inventory.json"), &json!(inventory));
    let mut summary = BTreeMap::new();
    for (k, rows) in &cells {
        assert_eq!(rows.len(), 15);
        let collect = |f: &str| rows.iter().map(|x| num(x, f)).collect::<Vec<_>>();
        let rss = collect("RSS_MiB");
        summary.insert(k.clone(),json!({"median_ms":median(&collect("median_ms")),"cpu_ms":median(&collect("cpu_ms_per_request")),"median_RSS_MiB":median(&rss),"max_RSS_MiB":rss.iter().copied().fold(0.,f64::max),"blocks":rows}));
    }
    let mut pairs = BTreeMap::new();
    let mut checks = BTreeMap::new();
    let mut failed = vec![];
    let mut refs = BTreeMap::new();
    let mut achieved = 0;
    for c in read(r("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap();
        for (a, b) in [
            ("control", "accepted"),
            ("candidate", "accepted"),
            ("candidate", "control"),
        ] {
            let x = &cells[&format!("{case}/{a}")];
            let y = &cells[&format!("{case}/{b}")];
            let mut ratios = vec![];
            for i in 0..15 {
                assert_eq!(x[i]["block"], i);
                assert_eq!(y[i]["block"], i);
                ratios.push(num(&x[i], "median_ms") / num(&y[i], "median_ms"));
            }
            let mut s = ratios.clone();
            s.sort_by(f64::total_cmp);
            let wins = s.iter().filter(|&&r| r < 1.).count();
            let k = format!("{case}/{a}/{b}");
            pairs.insert(k.clone(),json!({"median":s[7],"low":s[3],"high":s[11],"wins":wins,"all_ratios":ratios,"nominal_coverage":0.96484375}));
            if c["workers"] == 32 {
                let pass = if a == "control" {
                    s[3] >= 1. / 1.03 && s[11] <= 1.03
                } else if case == "1x2-w32" {
                    s[11] < 1. && wins >= 11
                } else {
                    s[11] <= 1.02
                };
                checks.insert(k.clone(), pass);
                if !pass {
                    failed.push(k);
                }
            }
        }
        if c["workers"] == 32 && !c["reference"].is_null() {
            let mut vs = BTreeMap::new();
            for v in VS {
                let ms = num(&summary[&format!("{case}/{v}")], "median_ms");
                let cpu = num(&c["reference"], "cpu_ms");
                let metal = num(&c["reference"], "metal_ms");
                if v == "candidate" && ms <= cpu / 10. {
                    achieved += 1;
                }
                vs.insert(v,json!({"ms":ms,"cpu_speedup":cpu/ms,"metal_speedup":metal/ms,"CPU10x":ms<=cpu/10.}));
            }
            refs.insert(
                c["shape"].as_str().unwrap().to_string(),
                json!({"reference":c["reference"],"variants":vs}),
            );
        }
    }
    let a = json!({"status":"completed final artifact screen; adoption review required","final_artifact_gate_passed":failed.is_empty(),"failed_checks":failed,"checks":checks,"pairs":pairs,"cells":summary,"m5":refs,"SDK_requests":requests,"new_independent_exports":477,"independent_proof_count":15028,"original8_CPU10x_rows":achieved,"goal_achieved":false,"goal_status":"active","production_changed":false,"adopted":false,"old_full_performance_gate_passed":false,"old_failed_checks":["1x2-w16/off/frozen","1x2-w16/on/frozen"],"forced_pressure":"accepted baseline timed out; candidate unexecuted; no new resource capture"});
    save(r("analysis.json"), &a);
    snap("host-after.txt");
    println!(
        "477Gnark/ledger15028;SDKrequests{requests};final gate{};failed{};CPU10x{achieved}/8",
        a["final_artifact_gate_passed"], a["failed_checks"]
    );
    for (k, v) in &summary {
        println!(
            "{k} {:.6}ms RSS{:.2}/{:.2}MiB",
            num(v, "median_ms"),
            num(v, "median_RSS_MiB"),
            num(v, "max_RSS_MiB")
        );
    }
    for (k, v) in &pairs {
        if k.starts_with("1x2-") || k.starts_with("merge-54") {
            println!(
                "{k} {} [{},{}] wins{}",
                v["median"], v["low"], v["high"], v["wins"]
            );
        }
    }
    state(
        "clean production artifact screen completed; adoption pending",
        json!([]),
    );
}
fn report() -> String {
    let a = read(r("analysis.json"));
    let mut s = String::from("# Clean borrowed G2 production artifact\n\n");
    writeln!(s,"Final artifact screen passed: **{}**. Production adoption: **pending**. All 477 exported proofs verified independently with Gnark; {} complete SDK requests also verified internally. The original tenfold CPU goal remains active: **{}/8 shapes**.\n",a["final_artifact_gate_passed"],a["SDK_requests"],a["original8_CPU10x_rows"]).unwrap();
    s.push_str("The clean candidate replaces copied G2 point queues with proof-local borrowed references and omits two unused scalar buffers. It removes experimental flags and public probe exports, using the existing supported-native guard. No persistent key table or expanded plan is introduced. The source audit retains 169 of 172 original files exactly; reversing three edits and removing two modules recovers the original source. Native tests pass: 122 core and 8 raw/cubic prover tests.\n\nThe rebuilt control has identical allocated ELF sections to the accepted SDK. Clean candidate code differs from the earlier experimental build, so these are fresh fixed-artifact measurements, not inherited timings. See `artifacts/final-protocol.md`, executable identities, all raw logs and independent verifier records. Fifteen interleaved rotated/reversed blocks, 32 workers on CPUs 0–31, plus a descriptive physical16 primary cell. Each process primes three times, then measures151 primary or31 other requests. Loading is excluded; solving, randomized proving, SDK verification and serialization are included. No exclusive host or parallel-proof throughput claim. RSS includes load/primes.\n\n| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |\n|---|---|---:|---:|---:|---:|\n");
    for c in read(r("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap();
        for v in VS {
            let x = &a["cells"][format!("{case}/{v}")];
            writeln!(
                s,
                "| {case} | {v} | {:.6} | {:.3} | {:.2} | {:.2} |",
                num(x, "median_ms"),
                num(x, "cpu_ms"),
                num(x, "median_RSS_MiB"),
                num(x, "max_RSS_MiB")
            )
            .unwrap();
        }
    }
    s.push_str("\n| Shape | M5 CPU ms | M5 Metal ms | Candidate ms | CPU speedup | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["m5"].as_object().unwrap() {
        let c = &v["variants"]["candidate"];
        writeln!(
            s,
            "| {k} | {} | {} | {:.6} | {:.3}× | {:.3}× |",
            v["reference"]["cpu_ms"],
            v["reference"]["metal_ms"],
            num(c, "ms"),
            num(c, "cpu_speedup"),
            num(c, "metal_speedup")
        )
        .unwrap();
    }
    s.push_str("\nPaired median ratio bounds use ranks4 and12 of15, nominal96.484375% without familywise guarantees. Default32 control/accepted calibration must fit[1/1.03,1.03]; candidate primary upper<1 and at least11wins against both controls; other candidate shapes upper<=1.02. Physical16 is descriptive here. All blocks are retained; no minimum gain floor or successful-capture reroll.\n\n| Pair | Median ratio | Lower | Upper | Wins | Check |\n|---|---:|---:|---:|---:|---|\n");
    for (k, v) in a["pairs"].as_object().unwrap() {
        let check = a["checks"]
            .get(k)
            .map(|x| if x == true { "pass" } else { "fail" })
            .unwrap_or("descriptive");
        writeln!(
            s,
            "| {k} | {:.9} | {:.9} | {:.9} | {}/15 | {check} |",
            num(v, "median"),
            num(v, "low"),
            num(v, "high"),
            v["wins"]
        )
        .unwrap();
    }
    writeln!(
        s,
        "\nFailed final-artifact checks: `{}`.\n",
        a["failed_checks"]
    )
    .unwrap();
    s.push_str("The earlier full experimental screen remains failed at two physical16 uncertainty bounds; its positive32 estimate is retained. Compatibility/resource evidence is in [the qualification report](../2026-10-28-g2-borrowed-qualification/README.md). Forced-high stress [timed out on the accepted baseline](../2026-10-28-g2-borrowed-forced-pressure/README.md); candidate was not run under that condition. No new pressure/minimum-RAM/mobile/swap/giant/AWS claim follows from this study. Earlier allocation-traffic savings are not a resident-memory guarantee. Any adoption must explicitly retain these limitations. Binary/rlib identities are hashed locally; large executable artifacts are not committed.\n");
    s
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let q = e.path();
        let name = e.file_name();
        let n = name.to_string_lossy();
        if e.file_type().unwrap().is_dir() {
            if !["target-control", "target-candidate", "frozen"].contains(&n.as_ref()) {
                walk(&q, out)
            }
        } else if e.file_type().unwrap().is_file() {
            let ext = q.extension().and_then(|x| x.to_str()).unwrap_or("");
            if ["rs", "md", "json", "jsonl", "log", "txt", "proof", "public"].contains(&ext) {
                out.push(q);
            }
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    files.sort();
    let mut index = BTreeMap::new();
    for f in files {
        let rel = f.strip_prefix(R).unwrap();
        let to = Path::new(D).join("artifacts").join(rel);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&f, &to).unwrap();
        let h = hash(&f);
        assert_eq!(hash(&to), h);
        index.insert(format!("artifacts/{}", rel.display()), h);
    }
    fs::write(Path::new(D).join("README.md"), report()).unwrap();
    index.insert("README.md".into(), hash(Path::new(D).join("README.md")));
    save(Path::new(D).join("artifact-index.json"), &json!(index));
    index.insert(
        "artifact-index.json".into(),
        hash(Path::new(D).join("artifact-index.json")),
    );
    let text = index
        .iter()
        .map(|(p, h)| format!("{h}  {p}\n"))
        .collect::<String>();
    fs::write(Path::new(D).join("SHA256SUMS"), text).unwrap();
    println!("Archived {} independently hash-checked files", index.len());
}
fn state(status: &str, jobs: Value) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    s["previous_status_turn_classification"] =
        json!("no progress: status only; this turn advances final artifact qualification");
    s["status"] = json!(status);
    s["root"] = json!(R);
    s["report"] = json!(D);
    s["owned_live_jobs"] = jobs;
    s["goal_status"] = json!("active");
    s["goal_achieved"] = json!(false);
    s["next_action"]=json!("Complete fixed clean-artifact timing/independent verification, review actual results and explicit engineering exceptions; preserve full10x goal");
    if r("analysis.json").exists() {
        s["g2_borrowed_production"] = read(r("analysis.json"));
        s["independent_proof_count"] = json!(15028);
    }
    save(p, &s);
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("archive") => archive(),
        _ => panic!("prepare|finish|archive"),
    }
}
