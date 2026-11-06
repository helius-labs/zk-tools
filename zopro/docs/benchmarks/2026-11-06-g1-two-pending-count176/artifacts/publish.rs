use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-two-pending-count176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-two-pending-count176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash_file(p: &Path) -> String {
    let mut file = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buffer[..n]);
    }
    format!("{:x}", h.finalize())
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else {
            v.push(p)
        }
    }
    v.sort();
    v
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text = std::str::from_utf8(&b).expect("only explicit UTF8 helper/delta/receipt members");
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text})
}
fn archive(path: &Path, members: &BTreeMap<String, Value>) {
    let bytes = serde_json::to_vec(members).unwrap();
    let mut p = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(path).unwrap())
        .spawn()
        .unwrap();
    p.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(p.wait().unwrap().success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(path)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&out.stdout).unwrap();
    for v in decoded.as_object().unwrap().values() {
        let b = v["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(b), v["sha256"]);
    }
}
fn main() {
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["complete"], true);
    assert_eq!(read(format!("{R}/runs/result.json"))["status"], "complete");
    let base = Path::new(R).join("runs/two-arrival-count-model");
    let exit = read(base.join("exit.json"));
    assert_eq!(exit["returncode"], 0);
    assert_eq!(exit["success"], true);
    for key in ["stdout", "stderr"] {
        assert_eq!(
            hash_file(&base.join(format!("{key}.log"))),
            exit[format!("{key}_sha256")]
        );
    }
    let plan = read(format!("{R}/plan.json"));
    let cmd = read(base.join("command.json"));
    assert_eq!(
        &cmd["argv"].as_array().unwrap()[4..],
        plan["steps"][0]["argv"].as_array().unwrap().as_slice()
    );
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash_file(Path::new(p)), h.as_str().unwrap());
    }
    let old = read("/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176/analysis.json");
    assert_eq!(a["baseline"]["arrivals"], old["original"]["arrivals"]);
    assert_eq!(a["baseline"]["first_writes"], old["original"]["occupied"]);
    assert_eq!(
        a["baseline"]["logical_affine_updates"].as_u64().unwrap()
            + a["baseline"]["projective_arrivals"].as_u64().unwrap(),
        old["original"]["generic_fill_joins"].as_u64().unwrap()
    );
    let mut reviewed = 0;
    for (key, totalkey) in [
        ("baseline_windows", "baseline"),
        ("two_arrival_windows", "two_arrival"),
    ] {
        let rows = a[key].as_array().unwrap();
        assert_eq!(rows.len(), 20);
        let mut total = BTreeMap::<String, u64>::new();
        for (w, row) in rows.iter().enumerate() {
            assert_eq!(row["window"], w);
            let c = &row["counts"];
            let n = |k: &str| c[k].as_u64().unwrap();
            assert_eq!(
                n("arrivals"),
                n("first_writes")
                    + n("ordinary_affine_updates")
                    + 2 * n("triple_updates")
                    + n("projective_arrivals")
            );
            assert_eq!(
                n("queue_arrivals"),
                n("queue_drained") + n("projective_arrivals")
            );
            assert_eq!(
                n("projective_arrivals"),
                n("projective_initializations") + n("projective_accumulator_adds")
            );
            assert_eq!(
                n("generic_base_field_M_site_proxy"),
                6 * n("ordinary_affine_updates") + 13 * n("triple_updates")
            );
            assert_eq!(
                n("inverse_stages_model"),
                n("ordinary_batches") + n("triple_batches")
            );
            assert_eq!(c["integer_group_output_conservation_passed"], true);
            for (hist, count, batches, tail) in [
                (
                    "ordinary_batch_histogram",
                    "ordinary_affine_updates",
                    "ordinary_batches",
                    "ordinary_scalar_tail_updates",
                ),
                (
                    "triple_batch_histogram",
                    "triple_updates",
                    "triple_batches",
                    "triple_scalar_tail_updates",
                ),
            ] {
                let mut hc = 0;
                let mut hn = 0;
                let mut ht = 0;
                for (k, v) in c[hist].as_object().unwrap() {
                    let k = k.parse::<u64>().unwrap();
                    let v = v.as_u64().unwrap();
                    hc += v;
                    hn += k * v;
                    ht += (k % 8) * v;
                }
                assert_eq!(hn, n(count));
                assert_eq!(hc, n(batches));
                assert_eq!(ht, n(tail));
            }
            for (k, v) in a[totalkey].as_object().unwrap() {
                assert!(v.is_u64());
                *total.entry(k.clone()).or_default() += n(k);
            }
            reviewed += 1;
        }
        assert_eq!(json!(total), a[totalkey]);
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Two-arrival G1 streaming count screen\n\nThis fixed state-machine model replays retained primary K/H inputs with one pending point per bucket, making a triple ready only after the next arrival. **No native candidate is selected.** The model reduces projective routing but increases generic multiplication work, inverse stages and pending storage. It is a count screen, not a prover timing result.\n\n");
    report.push_str(&fs::read_to_string(format!("{R}/decision.md")).unwrap());
    report.push_str("\n\n## Policy and complete counts\n\nThe original balanced 255-bit cover has 20 windows, five of width 12 and fifteen of width 13. Original batches remain 200/352; new ready limits are 96/176 triples. The same signed digit reader, input identities and original bounded newest-first queue geometry are used. After ready triples and queue draining finish, unmatched pending points run through ordinary-size batches. The source/model/design were frozen before one Rust/xtask replay; no parameter search or model extension followed.\n\nThis differs from the [earlier mixed SDK](../2026-10-28-mixed-sdk/README.md), which consumed selected queue collisions from already-full binary batches and remains rejected. The [H-only native algebra](../2026-11-06-triple-h-native176/README.md) supplies the 13M generic formula count, but is not integrated here.\n\n|Modeled work|Original|Two-arrival|\n|---|---:|---:|\n");
    for k in [
        "arrivals",
        "first_writes",
        "ordinary_affine_updates",
        "triple_updates",
        "logical_affine_updates",
        "queue_arrivals",
        "queue_drained",
        "projective_arrivals",
        "projective_initializations",
        "projective_accumulator_adds",
        "queue_flushes",
        "ordinary_batches",
        "triple_batches",
        "inverse_stages_model",
        "ordinary_scalar_tail_updates",
        "triple_scalar_tail_updates",
        "generic_base_field_M_site_proxy",
        "additional_pending_payload_forecast_bytes",
    ] {
        report.push_str(&format!(
            "|{}|{}|{}|\n",
            k, a["baseline"][k], a["two_arrival"][k]
        ));
    }
    report.push_str("\nAll 40 window rows, histograms, pending/queue/ready maxima and conservation values are retained in the analysis. The baseline arrival/occupied/generic-join totals exactly match the independently reviewed earlier [K/H work model](../2026-11-06-g1-tagged-phi-review176/README.md). This does not make the new baseline replay a native branch census; no input coordinates or exceptional-case evidence were added.\n\n## Validation and limitations\n\nThe replay checks all 1,229,020 digits against a separate bit reader, reconstructs all 61,451 canonical scalar words and checks 60 crafted state machines using independent signed integer-group sums. Each actual-window sum also matches its independent per-destination input sum. The publication reader independently reconstructs aggregate totals, 40 window conservation identities, batch/tail histograms, input/hash/command receipts and the old generic-join counts. It does not rerun the successful replay or certify native curve behavior.\n\nThe first helper compile failed on ambiguous integer sum types in assert_eq; explicit sum::<u64> annotations fixed it before freezing/execution. Original source and the preparation note are retained. No successful solver extraction, native test, profile, SDK proof or count replay was repeated. The private cache stays local and is represented only by size/hash; no scalar, key, image, witness, proof or executable bytes are included.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let sec = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&sec[..sec.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n## Continuation and artifacts\n\nThe [next research step](artifacts/next.md) seeks a different reduction in dominant streamed field work or inverse service, without sweeping this fixed coalescer or reopening closed captures. Production, accepted latency/memory and the 30,846-proof ledger remain unchanged. All eight CPU10× targets remain active and unmet. No current AWS/Aeglos, phone/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or proof-throughput result is added.\n\n[Analysis](artifacts/analysis.json), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [model](artifacts/model.rs), [preparation](artifacts/preparation.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS).\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "design.md",
        "decision.md",
        "next.md",
        "model.rs",
        "preparation.md",
        "plan.json",
        "pins.json",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut members = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            members.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    for p in walk(&Path::new(R).join("runs")) {
        members.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
    }
    archive(&art.join("helpers-and-receipts.json.gz"), &members);
    fs::write(art.join("publication.json"),serde_json::to_vec_pretty(&json!({"passed":true,"model_commands":1,"model_window_rows_reviewed":reviewed,"old_arrival_occupied_generic_join_totals_reproduced":true,"scalar_rows":61451,"digits_checked":1229020,"small_integer_group_cases":60,"helper_receipt_UTF8_members":members.len(),"archive_hashes_and_roundtrip_verified":true,"private_input_bytes_included":false,"native_curve_correctness_claimed":false,"new_proofs":0,"new_timings":0,"adopted":false,"production_changed":false,"goal_status":"active/unmet0/8"})).unwrap()).unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash_file(&p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published {} files/{} exactUTF8 members/40 window count checks; no native candidate or timing",walk(Path::new(D)).len(),members.len());
}
