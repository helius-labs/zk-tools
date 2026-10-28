use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g1-weighted-anchor";
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
fn copy(root: &Path, dir: &Path, out: &Path) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let n = e.file_name().to_string_lossy().to_string();
        let t = e.file_type().unwrap();
        if t.is_dir() {
            if ["profiles", "source", "target", "frozen"].contains(&n.as_str()) {
                continue;
            }
            copy(root, &p, out);
        } else if t.is_file()
            && ["rs", "md", "json", "jsonl", "log", "txt"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            let dest = out.join(p.strip_prefix(root).unwrap());
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(p, dest).unwrap();
        }
    }
}
fn inventory(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        if e.file_type().unwrap().is_dir() {
            inventory(root, &p, out)
        } else {
            let rel = p.strip_prefix(root).unwrap().display().to_string();
            if rel != "SHA256SUMS" {
                out.insert(rel, hash(p));
            }
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let root = Path::new(R);
    let a = read(root.join("analysis.json"));
    assert_eq!(a["captures"], 566);
    assert_eq!(a["gates"]["component_gate_passed"], true);
    assert_eq!(
        read(root.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        578
    );
    let pins = read(root.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let audit = read(root.join("source-audit.json"));
    let mut recovered =
        fs::read_to_string(root.join("source/crates/zopro-core/src/msm.rs")).unwrap();
    let suffix = audit["msm_suffix"].as_str().unwrap();
    assert!(recovered.ends_with(suffix));
    recovered.truncate(recovered.len() - suffix.len());
    for e in audit["msm_edits"].as_array().unwrap().iter().rev() {
        assert_eq!(recovered.matches(e["new"].as_str().unwrap()).count(), 1);
        recovered = recovered.replace(e["new"].as_str().unwrap(), e["old"].as_str().unwrap());
    }
    assert_eq!(
        recovered,
        fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap()
    );
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
        if f != "crates/zopro-core/src/msm.rs" && f != "crates/zopro-core/src/lib.rs" {
            assert_eq!(hash(root.join("source").join(f)), h.as_str().unwrap());
        }
    }
    let core = fs::read_to_string(root.join("source/crates/zopro-core/src/lib.rs")).unwrap();
    assert_eq!(
        core.strip_suffix(audit["core_suffix"].as_str().unwrap())
            .unwrap(),
        fs::read_to_string(format!("{Z}/crates/zopro-core/src/lib.rs")).unwrap()
    );
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let mut report=String::from("# G1 weighted Y anchor\n\n**The complete point-addition caller improves about 1.24% in the four-context aggregate.** This passes the frozen component screen and permits a separate full-SDK trial. It is not a proof-latency result or production adoption.\n\nStore `T = 2Ax + Bx` in the existing Ax record slot. After computing `U = lambda²`, compute `X = U − (Ax + Bx)` and `Y = lambda(T − U) − Ay`. Y no longer waits for the final X subtraction. This costs one additional forward vector addition per eight pairs. The existing general add restores the lazy bound below 2p; a canonical-only add would not cover the T input range. Field kernels, prefixes, inversions, signed inputs, scalar tails, point order and the five-vector record layout remain unchanged. No key table, extra heap buffer, assembly or new unsafe operation is added.\n\n## Caller measurements\n\nOne executable contains both const-generic kernels; two labels select the ordinary kernel and the third selects weighted. A private scratch flag is set before priming. Production default selection is controlled by a private native compile cfg, with no new public runtime option. G2 uses the unchanged kernel in all three modes. This common-image experiment has no independent frozen-executable calibration and cannot establish proof speed.\n\n31 rotated/reversed blocks cover six contexts and three modes, giving 558 fresh latency processes. CPU 2, one caller; three checked/reset 1,024-call primes precede 4,096 timed complete calls. Fixtures use independent generator-derived points and are synthetic ordinary batches, not witness/MSM/proof requests.\n\n| Caller | Pairs | Ordinary 0 µs | Ordinary 1 µs | Weighted µs | Weighted/ordinary 0 paired median | Marginal bounds |\n|---|---:|---:|---:|---:|---:|---|\n");
    for kind in ["g1-owned", "g1-borrowed-signed", "g2-prefix"] {
        for n in [200, 352] {
            let v = &a["pairs"][format!("{kind}/{n}/2-over-0")];
            writeln!(
                report,
                "| {kind} | {n} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9}, {:.9}] |",
                a["cells"][format!("{kind}/{n}/m0")]["median_ns_per_batch"]
                    .as_f64()
                    .unwrap()
                    / 1000.,
                a["cells"][format!("{kind}/{n}/m1")]["median_ns_per_batch"]
                    .as_f64()
                    .unwrap()
                    / 1000.,
                a["cells"][format!("{kind}/{n}/m2")]["median_ns_per_batch"]
                    .as_f64()
                    .unwrap()
                    / 1000.,
                v["median"].as_f64().unwrap(),
                v["low"].as_f64().unwrap(),
                v["high"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    report.push_str("\nTables show medians of process means; paired ratios are computed within blocks. Rank 10–22 bounds on 31 ratios give nominal 97.055% marginal coverage under independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusion, sample extension or successful-capture retry was used. All data, including an unusually slow ordinary control in block 5, remain included.\n\n| Four-G1-context geometric aggregate | Paired median | Marginal bounds | Wins |\n|---|---:|---|---:|\n");
    for (k, v) in a["G1_aggregate"].as_object().unwrap() {
        writeln!(
            report,
            "| {k} | {:.9} | [{:.9}, {:.9}] | {}/31 |",
            v["median"].as_f64().unwrap(),
            v["low"].as_f64().unwrap(),
            v["high"].as_f64().unwrap(),
            v["wins"]
        )
        .unwrap();
    }
    report.push_str("\nBoth aggregate upper bounds are below parity and both comparisons win all 31 blocks. All six ordinary-control calibrations fit the frozen factor-1.03 bounds. Every weighted/ordinary upper bound is at most 1.02, including unchanged G2 controls; G2 is excluded from the gain aggregate. Borrowed signed G1 improves about 2.0–2.3%; owned unsigned G1 improves roughly 0.24–0.39%. Small owned gains are retained. Signedness and borrowing differ together in these fixtures, so the measurements do not isolate which makes the benefit larger.\n\n## Correctness and storage\n\nAll 124 native core tests pass with weighted selected by default: 119 accepted tests, three existing caller-adapter checks, and two new tests. New coverage checks 1,024 eight-lane groups against independent scalar field formulas, including p−1 and lazy-bound cases, and alternates formulas on reused owned/borrowed scratch across scalar tails, boundaries and canonical/vector-inverse/write modes. Complete MSM/curve/exception coverage comes from the core suite. No field arithmetic source changed; its previous tests are inherited.\n\nAll 566 latency/stat/profile processes pass independent full-output generator checks, matching output digests per shape/round scope, and zero timed allocations. G1 scratch remains 40,000/70,400 bytes for 200/352 pairs; G2 remains 16,000/28,160 bytes. The existing borrowed-fixture reference vector is 3,200/5,632 bytes and is created outside clocks. These are scratch capacities, not proof peak RSS or a device RAM minimum.\n\nThe first artifact-freezer step failed after the successful tests and library build because it expected two fingerprints and found three: two field-library Cargo profiles plus one core library. The preserved versioned repair accepts and checks both field profiles' identical features/flags and distinct profile IDs. It freezes the exact bench build artifacts. No test, Cargo build or timing was repeated. All transitive rlibs/proc-macro objects were copied to immutable storage before linking. The source audit reverses the private changes to all 172 accepted originals; 170 remain exact, and only msm.rs plus diagnostic reexports differ.\n\n## Counters and profiles\n\nAll 558 latency captures finish first, followed by four FIFO-gated grouped perf-stat processes and four separate 199 Hz cycles:u profiles, each 262,144 calls after three 1,024-call primes. Every capture finishes before decode/hash/archive work. All four counter groups report 100% running. No owned build/helper/hash/archive/other benchmark overlaps capture; unrelated ceremony host work is recorded and there is no exclusive-host claim.\n\n| Stat context | Cycles/batch | Instructions/batch | IPC | Generic cache misses/batch |\n|---|---:|---:|---:|---:|\n");
    for (k, v) in a["stat"].as_object().unwrap() {
        for e in v["events"].as_object().unwrap().values() {
            assert_eq!(e["running_percent"], 100.);
        }
        writeln!(
            report,
            "| {k} | {:.3} | {:.3} | {:.6} | {:.6} |",
            v["events"]["cycles:u"]["per_batch"].as_f64().unwrap(),
            v["events"]["instructions:u"]["per_batch"].as_f64().unwrap(),
            v["IPC"].as_f64().unwrap(),
            v["events"]["cache-misses:u"]["per_batch"].as_f64().unwrap()
        )
        .unwrap();
    }
    let total: u64 = a["profiles"]
        .as_object()
        .unwrap()
        .values()
        .map(|v| v["samples"].as_u64().unwrap())
        .sum();
    writeln!(report,"\nThe added arithmetic raises instruction counts while measured cycles fall and IPC rises, consistent with the intended shorter dependency chain. These single stat observations do not isolate an exclusive instruction cost. Four cycle captures total {total} samples, one sampled PID each, zero lost records. Profiles identify the corresponding ordinary/weighted kernels and scalar inversion; sampled periods are not removable proof wall fractions. Generic cache misses do not identify a cache level or DRAM bottleneck. Raw perf.data remains local with recorded hashes; decodes and read-only disassemblies are archived.\n").unwrap();
    report.push_str("## Accepted full-proof comparison (inherited)\n\nNo proof result changed in this caller study. The all-eight 10× CPU objective remains active and unmet, 0/8. Supplied M5 values are references.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let v = &m["m5"][shape];
        writeln!(
            report,
            "| {shape} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    report.push_str("\nHistorical accepted 1x2 peak RSS is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB. Primary CPU target is 3.7 ms. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.\n\nNo SDK/Gnark proof was generated; ledger remains 12,983. There is no new all-eight/54, pressure/concurrency, giant-circuit, portable/ARM/mobile/TEE or AWS/Aeglos result. No cloud resource was created.\n\nNext: build a pure SDK copy without diagnostic adapters, compare accepted/fresh ordinary/weighted full-proof latency, then qualify all eight plus 54 and resource/portable behavior if that screen passes. Component success alone is insufficient for adoption. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [code generation](artifacts/codegen.json) and raw logs are included.\n");
    save(
        root.join("closeout.json"),
        &json!({"component_gate_passed":true,"full_proof_gain_proven":false,"adopted":false,"production172_unchanged":true,"captures":566,"latency_processes":558,"native_core_tests":124,"stat_processes":4,"cycle_processes":4,"cycle_samples":total,"timed_allocations":0,"final_pins_verified":pins.as_object().unwrap().len(),"new_Gnark_exports":0,"proof_ledger":12983,"next":"PureSDK accepted/freshOFF/weightedON primary screen, then all8/54/portable/resources if passed"}),
    );
    fs::create_dir_all(Path::new(D).join("artifacts")).unwrap();
    copy(root, root, &Path::new(D).join("artifacts"));
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    let mut files = BTreeMap::new();
    inventory(Path::new(D), Path::new(D), &mut files);
    fs::write(
        Path::new(D).join("SHA256SUMS"),
        files
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    let o = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    fs::write(root.join("archive-verification.log"), o.stdout).unwrap();
    assert!(o.status.success());
    save(
        root.join("archive-summary.json"),
        &json!({"hashes":files.len(),"git_files":files.len()+1,"GNU_strict_passed":true}),
    );
    println!(
        "Archived{}hashes/{}files;componentPASSnotadopted",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let summary = read(format!("{R}/archive-summary.json"));
    for(k,v)in[("status",json!("G1 weighted-anchor caller screen passed; committed/pushed; full SDK pending; not adopted")),("root",json!(R)),("report",json!(format!("{D}/README.md"))),("commit",json!(commit)),("pushed",json!(true)),("remote_verified",json!(true)),("production_changed",json!(false)),("adopted",json!(false)),("last_goal_turn_classification",json!("PROGRESS")),("previous_status_turn_classification",json!("NO_PROGRESS")),("SDK_requests",json!(0)),("independent_proof_count",json!(12983)),("artifact_hashes",summary["hashes"].clone()),("git_files_new_report",summary["git_files"].clone()),("component",read(format!("{R}/analysis.json"))),("previous_completed_experiment",previous),("next_action",json!("PureSDK accepted/freshOFF/weightedON primary screen; broad qualification only if passed")),("owned_live_jobs",json!([])),("owned_cloud_resources",json!([]))]{s[k]=v;}
    s["weighted_anchor_component"] = read(format!("{R}/closeout.json"));
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let head=format!("CURRENT COMPLETE/PUSHED — G1 WEIGHTED Y ANCHOR CALLER SCREEN\n\nHEAD/remote verified {commit}; root {R}; report {D}/README.md. {}strictverifiedhashes/{}Gitfiles. ComponentPASS,NOadoption/fullproofgainclaim. Accepted172/SDK14da unchanged,local1x2historical8.324853ms/RSS134.50MiB,goalACTIVEunmet0/8,ledger12983unchanged,noownedjobs/cloud. Laststatus-onlyturnNO_PROGRESS;currentturnPROGRESS completes native correctness+actualcaller test.\n\nGraphS=Ax+Bx,T=Ax+S,U=slopeSquared;cacheT inexistingAxslot;X=U-S,Y=slope*(T-U)-Ay. OneextraGENERALvectoraddper8,shorterYdependency,noheap/table/newunsafe/fieldchange. Newprivateweighted_anchorbooldefaultcompilecfgzopro_g1_weighted_anchor undernativeIFMA8guards. Diagnosticcfgzopro_anchor_caller_probe enablesoldCalleradapters+newset_weighted/tests;singleimageordinary0/1vsweighted2. Source170originalsexact+msm/libdiagchangesreverseexactall172. Native124corePASSONCEdefaultON;buildPASSONCE. Freezerexpected2fingerprintsfound3(core1,bn2542profiles),native/failurepreserved;repair-freezer.rs->native-v2.rs+native-helper-v2 checksidenticalfieldflags/featuresdistinctprofiles;native-repairworkflowFREEZEONLY. NoCargo/tests/capturesrepeated. NoSDKadapterinproductionyet.\n\nCapture2209TERMINALexit0:31blocks6contexts3modes558latency4096callsafter3x1024primes/check/reset;then4groupstat262144calls100%running;then4cycles199Hz262144calls;then12decode/owner/loss=578steps. All566fullgenerator/digest/capacitychecks+zerotimedalloc. Oneexecutable,11monomorphsreadonlydisasm,453finalpinsverified. Noindependentfrozenexecutablecalibration;commonimagecomponentonly. G1aggregateON/ordinary0 .987602733CI[.987134894,.988071718],31/31;vsordinary1 .987367665CI[.986677287,.988236230],31/31. Sixcontrolcalibs/allnonregPASS. G1owned2004032.869->4022.464ns,3526608.669->6582.629;borrowed2004230.740->4148.649,3526963.631->6805.006. G2nearparity. Allrawincludingb5slowcontrolretained. ScratchG140k/70.4k,G216k/28.16k unchanged. Statsowned200cycles21964.52->21845.98,inst51479.19->52483.18,IPC2.3437->2.4024;borrow352cycles37849.95->36959.15,inst85033.98->87367.99,IPC2.2466->2.3639.1109profilesamplesonePIDzeroLost;notproofwallcost.\n\nNEXT: NEWpurefullSDKrootg1-weighted-anchor-sdk fromaccepted172 +exactRUNTIMEedits,NOdiagnosticadapter/reexport. BOTHordinaryOFFandweightedON mustshareidenticalpureproductioncandidatebytes,onlyprivatecfgdiffers. Needfullcore119/raw2/cubic6OFF/ON,freezealllibs/so,SDKaccepted/freshOFF/ON15block32/16protocolandGnark. See scalar-square-sdk workinghelpersbutNEVERinheritrejectedfieldsource/flags/12884ledger. Prior12983=scalar-square-sdk/prior-proof-sha256.json12884+99proofmembers fromscalar-square-sdk/proof-inventory.json. Noexpectedfullproofgain;positiveonlypermitsall8/54portable/resources. SourceadoptionremainsunchangedFFTengineeringexceptionfalsegates. LatestAWSstale17137261af77;noAWSuntilqualifiednewcandidate. CommitANDpush eachcompletedexperiment,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite. Olderheadershistorical.\n\n",summary["hashes"],summary["git_files"]);
    fs::write(hp, format!("{head}{old}")).unwrap();
    println!("Updatedstate/handoff;goalactive0/8;ledger12983");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
