use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-r260-screen";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-scaled-r260-screen";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
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
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let q = e.path();
        if e.file_type().unwrap().is_dir() {
            walk(&q, out)
        } else if e.file_type().unwrap().is_file() {
            out.push(q)
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["status"], "exact model passed");
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    assert_eq!(
        read(format!("{R}/native/result.json"))["status"],
        "complete"
    );
    assert_eq!(read(format!("{R}/runs/result.json"))["status"], "complete");
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
    }
    fs::create_dir_all(format!("{D}/artifacts")).unwrap();
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let mut local = BTreeMap::new();
    for p in files {
        let b = fs::read(&p).unwrap();
        if b.starts_with(b"\x7fELF") {
            local.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let q = Path::new(D)
            .join("artifacts")
            .join(p.strip_prefix(R).unwrap());
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::write(q, b).unwrap();
    }
    save(
        format!("{D}/artifacts/local-executable-pins.json"),
        &json!(local),
    );
    let mut report=String::from("# Coordinate scaling for wide R260 arithmetic\n\nThe exact arithmetic screen passes. Native performance remains unmeasured; production and the accepted SDK are unchanged. This permits a native prototype after accounting for conversion, packing and inversion costs.\n\n## Mechanism\n\nUse the curve isomorphism (x,y) → (x/4,y/8) with Montgomery radix R=2^260=16H, where H=2^256 is the host radix. A canonical host raw coordinate a=xH mod p imports numerically as 4a for X and 2a for Y. Export uses modular division by 4 or 2 followed by canonicalization. The isomorphic curve has coefficient b/64; ordinary affine addition commutes with the scaling. Original compact host key and bucket coordinates remain the proposed storage format, without a persistent recoded point copy or lookup table.\n\nKeep the original two-fused-product complex formulas. Let wider linear intermediates survive several operations: imported X<4p and Y<2p; dx<8p, dy<4p, norm<3p, slope<2p, square real<2p and imaginary<4p, X output<12p, finishing difference<16p, Y output<4p. Since 64p<R, product bounds permit the remaining reductions. Divide the wide X before packing into four words. The export result is below 4p and needs conditional subtraction of 2p then p. Wider values must use explicit bounded operations; ordinary compact/get/difference2 contracts are not widened.\n\nThe source work ledger retains 1450 IFMA sites per eight-point forward/reverse group. Potential savings concern eleven general linear add/sub operations and five mixed fused corrections. Shift/packing imports, modular half/quarter exports, extra final corrections, root-inversion codecs and register pressure remain costs. Counts are source-graph estimates, not emitted instructions or predicted speed. See [work ledger](artifacts/work-ledger.md).\n\nThe rejected lazy-linear experiment normalized every linear result below 2p using a quotient/table method. Earlier raw-R260 bucket experiments persistently recoded points. Their negative results remain closed; this screen uses coordinate scaling and wider intermediate bounds. No novelty claim is made.\n\n## Exact checks\n\nThree new focused model tests pass, followed by the fixed exact study once. A BigUint full-radix REDC oracle is compared with separate scalar five-by-52-bit CIOS across wider domains. These are scalar integer models, not actual SIMD instruction execution.\n\n| Domain | Operand bounds in p | Output bound | Reductions |\n|---|---|---:|---:|\n");
    let mut reductions = 0;
    for c in a["CIOS_domains"].as_array().unwrap() {
        reductions += c["reductions"].as_u64().unwrap();
        writeln!(
            report,
            "| {} | {} | {}p | {} |",
            c["case"].as_str().unwrap(),
            c["limits_p"],
            c["output_bound_p"],
            c["reductions"]
        )
        .unwrap();
    }
    assert_eq!(reductions, 42885);
    report.push_str("\nThere are 42,885 reduction comparisons, 20,810 codec/alias checks, and 2,560 ordinary BN254 G2 outputs at batch sizes 8/16/32/64/200 over eight changing steps, checked against independent Arkworks generator scalar sums. Eight boundary ordinary pairs are each repeated across eight lanes; 17 boundary attempts take whole-call guards.\n\nThe toy F49 curve has 38 finite points. All 1,368 ordinary pairs agree, each repeated across eight lanes; 76 equal-x pairs take guards. Three tail lengths take guards, and the empty case accepts without arithmetic. Identity/equal-x/non-multiple-of-eight cases use typed whole-call fallback before arithmetic. This does not implement or qualify a new native exceptional path.\n\nThe first compilation failed at an ambiguous integer shift in a boundary fixture. The repair adds only an explicit BigUint type and usize shift. Original source, pins, plan and failed build receipts are preserved; the repaired three tests and fixed study then pass. No successful test or study was repeated for bookkeeping.\n\nAll 172 production files and 396 run pins are verified after execution. Source, plans, compiler, dependency hashes, failures and receipts are archived; executables remain local with hashes. The model's arbitrary-precision allocations are diagnostic storage. There is no native speed, CPU profile, proof timing, memory peak, minimum RAM, mobile/TEE/pressure/giant-circuit or new AWS/Aeglos result. Proof ledger remains 13,181.\n\n## Inherited CPU and Metal comparison\n\nThe all-eight CPU10x objective remains active and unmet, 0/8. The accepted FFT adoption retains its engineering exception and false full-performance and qualified-scope gates. These accepted local timings and supplied M5 references are inherited, not measurements of this model.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted | CPU10x target ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    let shape = [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ];
    let accepted = [
        8.324853, 11.460675, 15.854365, 24.084626, 33.693397, 39.981636, 66.010670, 65.086624,
    ];
    let cpu = [37., 59., 109., 189., 284., 335., 528., 507.];
    let metal = [24., 30., 48., 74., 101., 124., 176., 175.];
    for i in 0..8 {
        writeln!(
            report,
            "| {} | {:.6} | {:.1} | {:.1} | {:.3}× | {:.3}× | {:.1} |",
            shape[i],
            accepted[i],
            cpu[i],
            metal[i],
            cpu[i] / accepted[i],
            metal[i] / accepted[i],
            cpu[i] / 10.
        )
        .unwrap();
    }
    report.push_str("\nHistorical primary peak is 134.50 MiB. Merge54 remains 161.341835 ms / 1910.1875 MiB, without a supplied M5 reference. No production change or full-proof improvement is established.\n\n[Design](artifacts/design.md), [model](artifacts/model.rs), [analysis](artifacts/analysis.json), [tests](artifacts/native/tests/stdout.log), [fixed study](artifacts/runs/result.json), [pin closeout](artifacts/closeout.json), [repair](artifacts/repair.json).\n");
    fs::write(format!("{D}/README.md"), report).unwrap();
    let mut all = vec![];
    walk(Path::new(D), &mut all);
    all.sort();
    let mut sums = String::new();
    for p in &all {
        writeln!(
            sums,
            "{}  {}",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        )
        .unwrap();
    }
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    let check = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    fs::write(format!("{R}/archive-verification.log"), &check.stdout).unwrap();
    assert!(check.status.success());
    save(
        format!("{R}/archive-summary.json"),
        &json!({"report":D,"artifact_hashes":all.len(),"Git_files":all.len()+1,"run_pins_verified":pins.as_object().unwrap().len(),"production_sources_verified":172,"exact_model_passed":true,"native_speedup_proven":false,"proof_ledger":13181,"goal_status":"active","CPU10x_rows":0}),
    );
    println!(
        "Archived {} strict verified hashes; native prototype remains required",
        all.len()
    );
}
fn state() {
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(head.status.success());
    let head = String::from_utf8(head.stdout).unwrap().trim().to_string();
    let remote = Command::new("git")
        .args([
            "-c",
            "url.https://github.com/.insteadOf=git@github.com:",
            "-c",
            "credential.helper=!gh auth git-credential",
            "ls-remote",
            "origin",
            "refs/heads/sergei/zopro-cpu-perf",
        ])
        .output()
        .unwrap();
    assert!(remote.status.success());
    assert_eq!(
        String::from_utf8(remote.stdout)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap(),
        head
    );
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut v = read(p);
    let previous = v["commit"].clone();
    v["previous_completed_experiment"] = previous;
    v["commit"] = json!(head);
    v["status"]=json!("Coordinate-scaled R260 exact model passed; pushed verified; native speed unproven; not adopted");
    v["root"] = json!(R);
    v["report"] = json!(format!("{D}/README.md"));
    v["component"] = a_null();
    v["scaled_r260_screen"] = read(format!("{R}/analysis.json"));
    v["last_goal_turn_classification"] = json!("progress");
    v["previous_status_turn_classification"] = json!("no progress; status-only turn revalidated");
    v["next_action"]=json!("Native point caller after cost review; separately test canonical X sum via existing helper. No unchanged rejected reroll.");
    v["pushed"] = json!(true);
    v["remote_verified"] = json!(true);
    v["goal_status"] = json!("active");
    v["goal_achieved"] = json!(false);
    v["production_changed"] = json!(false);
    v["adopted"] = json!(false);
    v["owned_live_jobs"] = json!([]);
    v["owned_cloud_resources"] = json!([]);
    v["artifact_hashes"] = read(format!("{R}/archive-summary.json"))["artifact_hashes"].clone();
    v["git_files_new_report"] = read(format!("{R}/archive-summary.json"))["Git_files"].clone();
    save(p, &v);
    let handoff = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(handoff).unwrap();
    let prefix=format!("CURRENT COMPLETE/PUSHED — COORDINATE-SCALED R260 EXACT SCREEN\n\nHEAD/remote verified {head}; root {R}; report {D}/README.md. Exact model PASS, native speed unproven/not adopted. All172 production unchanged; accepted SDK14da/historical1x2 8.324853ms/RSS134.50MiB; goalACTIVE0/8, proofledger13181. 3focused tests plus fixed model once:42885 scalarCIOS/BigUintREDC reductions,20810 codecs,2560 BN254G2ordinary outputs, exhaustive1368 toy F49ordinary pairs/76equalxguards. InitialE0282fixture repaired only explicit BigUint/usize type; preserved original.396pins verified. No nativeSIMD/timing/profiles/proofs/RSS/cloud/jobs. Previous status-onlygoalturn NO PROGRESS; current PROGRESS closes/publishes screen.\n\nMechanism: coordinate(x/4,y/8),R260=16H; importhostrawX=4a,Y=2a,exportmodquarter/half;no persistent recodedkey/buckets. Wide dx8p,dy4p,norm3p,slope2p,X12p,D16p,Y4p;64p<R. Original2fusedcomplex formulas kept. Workledger retains1450IFMA/group;potential11linearops+5mixedcorrections saved, must pay12importvectors/4exports,8vs4finalcorrections,rootinversecodecs/registerspills/exceptiontails. No speedclaim. Nativeboundedtypes required, don't widen get/compact/difference2 contracts. Distinct fromclosed normalizedlazylinear/persistentR260 buckets.\n\nNEXT: small separate canonical Xsum source opportunity prefix reverse usesgenericFq2lanes_add canonicalAx/Bx; oldstream alreadycanonical_addFAST; history README search no priorhit. Test existinghelper first as lowcost subset; no claimedgain. BroaderwideR260native prototype permitted afterledger; fullconcurrentcaller required before SDK. LatestreducedGauss callerclosednegative1b14736ad; exactGLVSDKclosednegativec6f3098b0. Keep accepted engineeringexceptionfalsegates. CommitANDpush everycompletedexperiment. Noquestions/subagents/largecaches/Casmrewrite. Olderheadershistorical.\n\n");
    fs::write(handoff, format!("{prefix}{old}")).unwrap();
    println!("State/handoff updated after verified push {head}");
}
fn publication() {
    let note = json!({"immutable_execution_logs_preserved":true,"git_diff_check_exceptions":["artifacts/native/build-model-tests/stderr.log:10: new blank line at EOF","artifacts/native/tests/stdout.log:8: new blank line at EOF"],"reason":"Preserve captured bytes and pinned test log hashes; check authored sources separately."});
    save(format!("{D}/artifacts/publication-check.json"), &note);
    let p = format!("{D}/README.md");
    let mut s = fs::read_to_string(&p).unwrap();
    s.push_str("\nPublication check preserves two captured logs with trailing blank lines byte-for-byte; authored sources are checked separately. See [receipt](artifacts/publication-check.json).\n");
    fs::write(p, s).unwrap();
    fs::copy(format!("{R}/close.rs"), format!("{D}/artifacts/close.rs")).unwrap();
    let mut files = vec![];
    walk(Path::new(D), &mut files);
    files.retain(|p| p.file_name().unwrap() != "SHA256SUMS");
    files.sort();
    let mut sums = String::new();
    for p in &files {
        writeln!(
            sums,
            "{}  {}",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        )
        .unwrap();
    }
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    let check = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    assert!(check.status.success());
    fs::write(format!("{R}/archive-verification.log"), check.stdout).unwrap();
    let p = format!("{R}/archive-summary.json");
    let mut a = read(&p);
    a["artifact_hashes"] = json!(files.len());
    a["Git_files"] = json!(files.len() + 1);
    save(p, &a);
    println!("Preserved logs, verified {} hashes", files.len());
}
fn a_null() -> Value {
    Value::Null
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "archive" => archive(),
        "publication" => publication(),
        "state" => state(),
        _ => panic!("mode"),
    }
}
