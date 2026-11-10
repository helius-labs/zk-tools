use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let r = Path::new("/home/ubuntu/src/zopro-perf-work/task-job-families176");
    let d = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-10-task-job-families176",
    );
    let a = d.join("artifacts");
    assert!(!a.exists());
    fs::create_dir_all(&a).unwrap();
    let result = read(r.join("analysis.json"));
    assert_eq!(result["graph_rows"].as_array().unwrap().len(), 270);
    assert_eq!(result["translations"].as_array().unwrap().len(), 18);
    for t in result["translations"].as_array().unwrap() {
        assert_eq!(t["candidate_count"], 1);
        assert_eq!(t["distinct_function_addresses"], 37);
        assert_eq!(t["all_addresses_match_defined_text"], true);
    }
    let summary = result["summary"].as_array().unwrap();
    let mut totals = BTreeMap::new();
    for w in [32, 96] {
        let rows = summary
            .iter()
            .filter(|x| x["workers"] == w && x["phase"] == 5)
            .collect::<Vec<_>>();
        let all = rows
            .iter()
            .map(|x| x["total_publications_27_graphs"].as_u64().unwrap())
            .sum::<u64>();
        let phi = rows
            .iter()
            .find(|x| x["symbol_address"] == "0x49b000")
            .unwrap()["total_publications_27_graphs"]
            .as_u64()
            .unwrap();
        totals.insert(w, (all, phi));
    }
    let share = (totals[&96].1 - totals[&32].1) as f64 / (totals[&96].0 - totals[&32].0) as f64;
    assert!(share > 0.998 && share < 1.);
    let review = json!({"status":"PASS","source_data":"prior completed task-interval-c7a176 recording; no recapture","processes":18,"graphs":54,"phase_count_crosschecks":270,"classified_phase_relative_addresses":33,"distinct_published_function_addresses_per_process":37,"unique_translation_per_process":true,"phi_share_of_additional_MSM_publications":share,"new_SDK_requests":0,"new_proofs":0,"canonical_ledger":50208,"target_observer_qualified":false,"causal_or_uninstrumented_claim":false,"production_changed":false});
    fs::write(
        a.join("review.json"),
        serde_json::to_vec_pretty(&review).unwrap(),
    )
    .unwrap();
    let pins: BTreeMap<String, String> = serde_json::from_value(read(r.join("pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h);
    }
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    let exit = read(r.join("runs/decode-published-job-families/exit.json"));
    assert_eq!(exit["success"], true);
    for (f, k) in [
        ("stdout.log", "stdout_sha256"),
        ("stderr.log", "stderr_sha256"),
    ] {
        assert_eq!(
            hash(r.join("runs/decode-published-job-families").join(f)),
            exit[k]
        );
    }
    for f in [
        "analysis.json",
        "decode.rs",
        "pins.json",
        "plan.json",
        "program-headers.txt",
        "nm-version.txt",
        "publish.rs",
    ] {
        fs::copy(r.join(f), a.join(f)).unwrap();
    }
    let o = Command::new("gzip")
        .args(["-n", "-c"])
        .arg(r.join("symbols.txt"))
        .output()
        .unwrap();
    assert!(o.status.success());
    fs::write(a.join("symbols.txt.gz"), o.stdout).unwrap();
    let o = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(a.join("symbols.txt.gz"))
        .output()
        .unwrap();
    assert!(o.status.success());
    assert_eq!(o.stdout, fs::read(r.join("symbols.txt")).unwrap());
    assert!(Command::new("tar")
        .args([
            "--sort=name",
            "--mtime=2026-11-10 00:00:00Z",
            "--owner=0",
            "--group=0",
            "--numeric-owner",
            "-czf"
        ])
        .arg(a.join("workflow.tar.gz"))
        .arg("-C")
        .arg(r)
        .arg("runs")
        .status()
        .unwrap()
        .success());
    fs::create_dir(r.join("workflow-roundtrip")).unwrap();
    assert!(Command::new("tar")
        .arg("-xzf")
        .arg(a.join("workflow.tar.gz"))
        .arg("-C")
        .arg(r.join("workflow-roundtrip"))
        .status()
        .unwrap()
        .success());
    for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
        let rel = Path::new("runs/decode-published-job-families").join(f);
        assert_eq!(
            hash(r.join(&rel)),
            hash(r.join("workflow-roundtrip").join(&rel))
        );
    }
    let text=format!("# Job families in the C7a interval traces\n\nOne G2 scalar-decomposition iterator accounts for **{:.3}% of additional MSM job publications** between 32 and 96 workers in the retained instrumented graphs. Its measured job counts are not established for the uninstrumented prover, and the target observer overhead qualification still fails. No performance improvement or production change is claimed.\n\nThis is offline analysis of the [completed C7a diagnostic](../2026-11-10-task-interval-c7a176/README.md). It adds no SDK requests, proof exports, profiling or AWS resources. All 18 main ON processes and all 54 retained graphs are included. Each process has 37 distinct published-job function pointers. Exactly one common page-aligned translation maps all pointers to defined text symbols in the exact measured PIE executable, SHA-256 `7652efb7367834f4c81bf86c4b38e49e5a0080ba9518a3502be9c94636ae50eb`. All 33 relative addresses with publications in phases 2–6 have one symbol alias. Native symbols retain the concrete iterator and closure types. All 270 per-graph phase publication totals agree with the earlier independent phase reader.\n\n|MSM publications across 27 graphs per worker count|32 workers|96 workers|Increase|\n|:---|---:|---:|---:|\n|All job families|69,645|450,194|380,549|\n|G2 `hb_raw` scalar decomposition|39,172|419,144|379,972|\n|Other families combined|30,473|31,050|577|\n\nThe decomposition job is symbol `0x49b000`, whose concrete iterator is `IterMutProducer<[u64;4]>` with `g2_phi::hb_raw::{{closure#0}}`. Source contains the unrestricted [in-place phi_split iterator](../../../crates/zopro-core/src/msm_affine/g2_phi.rs#L194). Its per-graph publication median is 417 at32 and15,304 at96. Aggregate counts include all27graphs, including large counts in some32-worker graphs; medians and totals are different summaries. The other MSM families do not show a comparable increase in these recordings.\n\nScalar preparation has a separate increase: the two unrestricted A/B gather closures dominate its publications. Their per-family medians are3928/4487 at32 and13098/11163 at96. The already bounded canonical conversion remains126→127. These medians are not additive. This identifies concrete source sites rather than attributing an aggregate sampled scheduler percentage to removable latency.\n\nThe history review found the exact `phi_split.with_min_len(128)` idea in the [October27 local study](../2026-10-27-g2-scalar-grain/README.md). It showed no confirmed full-proof gain: primary32 paired candidate/accepted0.997331, interval[0.995288,1.001898], with failed overall qualification and no regression rows. It measured local16/32workers and created no AWS resources. The [October29 sequential decomposition study](../2026-10-29-serial-phi175/README.md) also failed its full-SDK performance gate. Those local outcomes remain in force; neither is a new optimization discovered here.\n\nThe next discriminating test is the existing minimum-grain idea on **C7a96physicalcores**, where the new trace evidence identifies its specific job family as a scaling lead. Repeating the old local timing campaign would not answer that question. A current-source native candidate, accepted control, fixed32/96target comparison, fresh proof verification and RAM/CPU checks are required. This is a new target condition, not a claim that a prior local failure has passed. Full all-shape qualification remains necessary before adoption. The A/B gather hypothesis is secondary to testing this identified MSM family. No large lookup table, expanded key plan, new worker pool or architectural RAM cap is proposed.\n\nThe observer's96-worker ON/OFF ratio is1.095031, above the declared1.05ceiling. Its adaptive splitting may respond to observation overhead, so these counts cannot demonstrate the cause of the uninstrumented latency loss or predict a speedup. Job counts are not durations; nested execution spans are not exclusive CPU. The original all-eight verified10×M5CPU objective remains active/unmet, with [CPU/Metal context](../2026-11-10-accepted-current-c7a176/README.md). The cumulative distinct verified-proof ledger remains50208.\n\nThe offline decoder workflow passes once with26inputpins and retained command/exit/output hashes. An initial direct preparation compile typo was corrected before that run; direct compilation is outside the workflow receipt count. The publisher rechecks pins, arithmetic totals and receipts without rerunning decoding or any proof. Symbol compression and archived receipt round trips match exact bytes.\n\n[Analysis](artifacts/analysis.json), [reader source](artifacts/decode.rs), [input pins](artifacts/pins.json), [symbols](artifacts/symbols.txt.gz), [ELF headers](artifacts/program-headers.txt), [workflow](artifacts/workflow.tar.gz), [review](artifacts/review.json), [checksums](SHA256SUMS). Raw graph inputs are in the preceding C7a report; executables remain local with pinned identities.\n",share*100.);
    fs::write(d.join("README.md"), text).unwrap();
    let mut sums = String::new();
    for e in fs::read_dir(&a).unwrap() {
        let p = e.unwrap().path();
        sums += &format!(
            "{}  artifacts/{}\n",
            hash(&p),
            p.file_name().unwrap().to_str().unwrap()
        );
    }
    sums += &format!("{}  README.md\n", hash(d.join("README.md")));
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    assert!(Command::new("sha256sum")
        .args(["-c", "SHA256SUMS"])
        .current_dir(d)
        .status()
        .unwrap()
        .success());
    println!("{review}");
}
