use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-reduced-gauss";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-reduced-gauss";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let q = e.path();
        if e.file_type().unwrap().is_dir() {
            let n = e.file_name();
            let n = n.to_string_lossy();
            if n != "profiles" && n != "frozen" && !n.starts_with("target-") {
                walk(&q, out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(q)
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["captures"], 845);
    assert_eq!(a["timed_kernel_calls"], 179101696);
    let gate = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let pins = read(format!("{R}/pins-v2.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let dst = Path::new(D);
    fs::create_dir_all(dst.join("artifacts")).unwrap();
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let mut local = BTreeMap::new();
    let mut compressed = vec![];
    for p in files {
        let rel = p.strip_prefix(R).unwrap();
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") {
            local.insert(p.display().to_string(), hash(p));
            continue;
        }
        let q = dst.join("artifacts").join(rel);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        if rel.to_string_lossy().contains("runs/loss-") && rel.ends_with("stdout.log") {
            let out = Command::new("gzip")
                .args(["-n", "-c"])
                .arg(&p)
                .output()
                .unwrap();
            assert!(out.status.success());
            let z = q.with_extension("log.gz");
            fs::write(&z, out.stdout).unwrap();
            let back = Command::new("gzip").arg("-dc").arg(&z).output().unwrap();
            assert!(back.status.success());
            assert_eq!(back.stdout, bytes);
            compressed.push(json!({"path":z.strip_prefix(dst).unwrap(),"uncompressed_sha256":hash(p),"gzip_sha256":hash(z)}));
        } else {
            fs::write(q, bytes).unwrap();
        }
    }
    assert_eq!(compressed.len(), 4);
    save(
        dst.join("artifacts/local-executable-pins.json"),
        &json!(local),
    );
    save(
        dst.join("artifacts/loss-compression.json"),
        &json!(compressed),
    );
    let f = &a["logical32_aggregate"]["on-over-frozen"];
    let o = &a["logical32_aggregate"]["on-over-off"];
    let verdict = if gate {
        "passes the declared actual-caller screen; SDK qualification is still required"
    } else {
        "fails the declared actual-caller screen and is not adopted"
    };
    let mut report=format!("# Reduced Gauss products in native G2 callers\n\nThe candidate **{verdict}**. G2 prefix batches of 200 at 32 logical CPUs have ON/OFF paired ratio **{:.9}**, bounds [{:.9},{:.9}], {}/31 wins; ON/frozen is **{:.9}**, bounds [{:.9},{:.9}], {}/31 wins. Production and the accepted SDK remain unchanged.\n\n## Formula and scope\n\nFor a complex product over Fq[u]/(u²+1), compute P=(a0+a1)b0, Q=a1(b0+b1), R=a0(b1-b0), then return (P-Q,P+R). For a times conjugate(b), compute P=(a0+a1)b0, Q=a1(b0-b1), R=a0(b0+b1), then return (P-Q,P-R). Original shared real-factor scaling follows unchanged. All operations use the existing reduced field API and its lazy bounds.\n\nThis removes one raw bilinear product and adds one modular reduction: three ordinary products use 300 IFMA sites, the same as two fused expressions. Five reduced linear operations and different dependencies remain costs. The earlier shared/streamed/nonnegative raw Gauss methods used three raw products and two reductions, with wider signed or biased accumulators. Those rejected kernels are absent here. This tests a different arithmetic trade; fewer raw products do not establish fewer instructions or faster proving.\n\nOnly two native G2 formula bodies in msm.rs change. Their old bodies remain verbatim fallback under a private cfg. Removing the two adapter suffixes and reversing those body edits recovers all 172 originals; 170 original files remain byte-exact. Field, G1, FFT, solver, curve formats, keys and public APIs retain accepted source. No new unsafe arithmetic, heap buffer, key table, cache, execution plan or public runtime option is added.\n\n## Complete caller measurements\n\nRyzen 9950X, 31 fixed rotated/reversed blocks, three callers at solo CPU2, physical16 CPUs0–15 and logical32 CPUs0–31, with independent frozen accepted-library, fresh OFF and selected ON executables. Three checked/reset 1024-call primes precede 8192 timed calls per worker. Ready/start/done/release barriers place every worker's checking and allocation outside every worker's clock. Main wave time includes the slowest worker and barrier tail; per-worker elapsed, CPU and throughput remain in raw results. These synthetic ordinary points at real stream sizes are not proof or complete-MSM timings.\n\n| Caller/pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF | Marginal interval | Wins |\n|---|---|---:|---:|---:|---:|---|---:|\n",o["median"].as_f64().unwrap(),o["low"].as_f64().unwrap(),o["high"].as_f64().unwrap(),o["wins"],f["median"].as_f64().unwrap(),f["low"].as_f64().unwrap(),f["high"].as_f64().unwrap(),f["wins"]);
    for (kind, n) in [
        ("g2-prefix", 32),
        ("g2-prefix", 200),
        ("g1-borrowed-signed", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let val = |mode| {
                a["cells"][format!("{kind}/{n}/{t}/m{mode}")]["wall_ns_per_wave"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1000.
            };
            let v = &a["pairs"][format!("{kind}/{n}/{t}/2-over-1")];
            writeln!(
                report,
                "| {kind}/{n} | {t} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9},{:.9}] | {}/31 |",
                val(0),
                val(1),
                val(2),
                v["median"].as_f64().unwrap(),
                v["low"].as_f64().unwrap(),
                v["high"].as_f64().unwrap(),
                v["wins"]
            )
            .unwrap();
        }
    }
    report.push_str("\n| Caller/pairs/layout | OFF/frozen | Marginal interval | ON/frozen | Marginal interval |\n|---|---:|---|---:|---|\n");
    for (kind, n) in [
        ("g2-prefix", 32),
        ("g2-prefix", 200),
        ("g1-borrowed-signed", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let x = &a["pairs"][format!("{kind}/{n}/{t}/1-over-0")];
            let y = &a["pairs"][format!("{kind}/{n}/{t}/2-over-0")];
            writeln!(
                report,
                "| {kind}/{n}/{t} | {:.9} | [{:.9},{:.9}] | {:.9} | [{:.9},{:.9}] |",
                x["median"].as_f64().unwrap(),
                x["low"].as_f64().unwrap(),
                x["high"].as_f64().unwrap(),
                y["median"].as_f64().unwrap(),
                y["low"].as_f64().unwrap(),
                y["high"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    writeln!(
        report,
        "\nCalibration: **{}**; nonregression: **{}**; target G2/200 logical32 gain: **{}**.\n",
        a["gates"]["calibration"], a["gates"]["nonregression"], a["gates"]["logical32_gain"]
    )
    .unwrap();
    report.push_str("The candidate is slower than fresh OFF in all six G2 size/topology comparisons, with paired medians from 11.18% to 15.04% slower and every interval above parity. The primary G2/200 logical32 comparison loses all31 blocks. The frozen-control comparison independently shows the primary regression. The only calibration failure is the supplemental G2/32 logical32 OFF/frozen interval; it remains failed and does not rescue the negative G2 comparisons.\n\nThe frozen and fresh OFF executables are byte-identical after independent linking, and the mode argument only labels results. Thus their timing difference is not a different compiled arithmetic implementation. Process placement/address layout, barrier tails and host variation remain unresolved. The ON executable differs. This equality and all hashes are recorded in interpretation.json.\n\n");
    report.push_str("The primary gain check applies only to G2 prefix/200 at logical32, requiring upper ratio below parity and at least 21/31 wins against both controls. The legacy analysis field named logical32_aggregate now contains exactly this single caller, not a cross-caller average. All nine ON comparisons require upper<=1.02; all OFF/frozen calibration bounds must fit [1/1.03,1.03]. Bounds use ordered ranks10–22 of31, nominal97.055% under independent exchangeable blocks, without familywise guarantees. Every positive estimate, uncertainty and failed check is retained; there is no gain floor, exclusion, extension or unchanged reroll.\n\n## Correctness and provenance\n\nOFF and ON each pass the full 123-test native core suite: 119 accepted tests, three original generator-oracle point adapters, and one new scalar-Fq2 expression oracle. The new case checks 512 groups of eight lanes through eight changing steps, both ordinary complex multiplication and conjugate multiplication with shared real scaling: 65,536 Fq2 outputs per configuration. Boundary inputs and existing lazy add/sub paths are included. Repeating the same cases in both builds is not additional distinct input coverage. Existing G2 caller tests independently check generator scalar sums, tails and scratch reuse. Some optional external-fixture cases return when fixtures are absent; those scenarios are not claimed.\n\nEach independent frozen/OFF/ON harness passes three tests, including concurrency, barriers, affinity, complete point outputs and inherited perf-helper checks. No fresh field suite is claimed; field source and its accepted qualification are unchanged. No SDK/Gnark proof is generated.\n\nThe first workflow stops in its first process at the reused harness's argument check, which permits only G2 length200 and rejects the prospectively planned length32. No fixture construction, priming or timed loop ran. The preserved repair extends only this main argument check to lengths32/200, then links three benchmark executables against the same frozen libraries. Test bodies and algorithm bytes are unchanged; successful native or harness tests and successful captures are not repeated. The original zero-completion workflow is retained under runs-initial-argument-failure with its original protocol and receipts. New captures use run-plan-v2/pins-v2. An offline analyzer changes only its pin-file path. Read-only G2 disassembly is regenerated for the actual relinked executables. All failures, initial sources/binaries and pin revisions remain available.\n\nEvery transitive rlib/proc-macro object is frozen immediately in separate target roots. Actual core/field flags/features are checked. Adapters and harness code are identical across variants. Static IFMA or instruction counts are not executed cost, dynamic spills or a causal timing decomposition. No successful test/library build/capture is repeated for bookkeeping.\n\nAll 845 kernel processes verify full generator results, capacity consistency and zero global timed allocations. There are **179,101,696 timed kernel calls**, not proofs. The new algebra adds no retained scratch; per-worker capacities remain in the analysis. Fixed temporaries and process RSS are not stack bounds or minimum RAM claims.\n\n## Counters and profiles\n\nAll 837 latency processes finish before four FIFO-gated grouped perf-stat processes, then four separate FIFO-gated 199Hz cycles:u profiles. Each uses logical32 and 262144 calls per worker after the same primes. All eight PMU captures finish before owner/decode/loss, hash or archive steps. No owned helper/build/hash/archive/second benchmark overlaps capture; host snapshots do not establish exclusive-host use.\n\n");
    report.push_str("| Counter case | Cycles/call | Instructions/call | IPC | Branch misses/call | Generic cache misses/call |\n|---|---:|---:|---:|---:|---:|\n");
    for (name, v) in a["stat"].as_object().unwrap() {
        let c = v["events"].as_object().unwrap();
        writeln!(
            report,
            "| {name} | {} | {} | {} | {} | {} |",
            c["cycles:u"]["per_kernel_call"],
            c["instructions:u"]["per_kernel_call"],
            v.get("IPC").unwrap_or(&Value::Null),
            c["branch-misses:u"]["per_kernel_call"],
            c["cache-misses:u"]["per_kernel_call"]
        )
        .unwrap();
    }
    let off = &a["stat"]["stat-g2-prefix-200-m1"]["events"];
    let on = &a["stat"]["stat-g2-prefix-200-m2"]["events"];
    let instructions = (on["instructions:u"]["per_kernel_call"].as_f64().unwrap()
        / off["instructions:u"]["per_kernel_call"].as_f64().unwrap()
        - 1.)
        * 100.;
    let cycles = (on["cycles:u"]["per_kernel_call"].as_f64().unwrap()
        / off["cycles:u"]["per_kernel_call"].as_f64().unwrap()
        - 1.)
        * 100.;
    writeln!(report,"\nThe single grouped G2 observation records {instructions:.2}% more instructions and {cycles:.2}% more cycles per call; all four groups report100% running. The actual G2 prefix symbols have1450 IFMA sites in both algorithms. These observations retain the added work despite one fewer raw product; they do not isolate an exclusive microarchitectural cause.\n").unwrap();
    report.push_str("\nCounter observations are descriptive, not confidence bounds. IPC, generic cache misses and sampled period do not identify a memory level, DRAM bandwidth, an execution-port cause or removable proof latency. SMT cycles sum logical-thread events, not disjoint physical-core cycles.\n\n| Profile | Samples | Sampled worker TIDs | Lost |\n|---|---:|---:|---:|\n");
    let mut samples = 0;
    for (name, v) in a["profiles"].as_object().unwrap() {
        let n = v["samples"].as_u64().unwrap();
        samples += n;
        let tids = v
            .get("sampled_worker_tids")
            .and_then(Value::as_array)
            .map(|v| v.len());
        writeln!(
            report,
            "| {name} | {n} | {} | 0 |",
            tids.map(|n| n.to_string())
                .unwrap_or_else(|| "see artifact".into())
        )
        .unwrap();
    }
    writeln!(report,"\nTotal {samples} samples; one sampled process and zero lost records per profile. Worker coverage and full symbol weights remain in analysis.json. Raw perf.data and executables remain local with hashes. Four loss dumps are archived using deterministic gzip and verified decompression hashes.\n").unwrap();
    report.push_str(if gate{"The caller screen permits a pure native SDK comparison of these exact formula changes before any adoption. It does not establish proof speed, memory pressure or the original tenfold goal.\n\n"}else{"This exact reduced-Gauss integration is closed without an SDK trial or adoption. Positive estimates remain reported within their measured scope. No unchanged formula, annotation, layout or scheduling reroll is selected.\n\n"});
    report.push_str("## Inherited CPU and Metal comparison\n\nNo new full-proof timing or peak RSS is established. The accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight CPU10x objective remains active and unmet, 0/8. These accepted local timings and supplied M5 references are inherited.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |\n|---|---:|---:|---:|---:|---:|\n");
    let m=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json");
    for case in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let v = &m["m5"][case];
        writeln!(
            report,
            "| {case} | {:.6} | {} | {} | {:.3}× | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"],
            v["metal_reference_ms"],
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    report.push_str("\nHistorical primary peak is134.50MiB; merge54 remains161.341835ms /1910.1875MiB, with no supplied M5 reference. No new portable/mobile/TEE, concurrent proof, pressure, giant-circuit or AWS/Aeglos result is claimed. Independent proof ledger remains13,181.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [new oracle](artifacts/oracle.rs), [actual code](artifacts/codegen.json), [native suites](artifacts/native/result.json), [harness checks](artifacts/linked/result.json), [all captures](artifacts/runs/result.json).\n");
    fs::write(dst.join("README.md"), report).unwrap();
    let mut files = vec![];
    walk(dst, &mut files);
    files.sort();
    let mut sums = String::new();
    for p in &files {
        writeln!(
            sums,
            "{}  {}",
            hash(p),
            p.strip_prefix(dst).unwrap().display()
        )
        .unwrap();
    }
    fs::write(dst.join("SHA256SUMS"), sums).unwrap();
    let out = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(dst)
        .output()
        .unwrap();
    fs::write(format!("{R}/archive-verification.log"), out.stdout).unwrap();
    assert!(out.status.success());
    save(
        format!("{R}/archive-summary.json"),
        &json!({"hashes":files.len(),"git_files":files.len()+1,"GNU_strict_passed":true,"loss_gzip_roundtrips":4,"profile_samples":samples}),
    );
    println!("Archived{}hashes;gate={gate}", files.len());
}
fn status(commit: &str) {
    let a = read(format!("{R}/analysis.json"));
    let gate = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let next = if gate {
        "Pure accepted/OFF/ON SDK before broader all8/54/resource/portable qualification"
    } else {
        "ReducedGaussnativecallerclosed; keepallpositiveestimates andfailures; choose distinctworkremoval withoutunchangedreroll"
    };
    for (k, v) in [
        ("root", json!(R)),
        ("report", json!(format!("{D}/README.md"))),
        ("commit", json!(commit)),
        ("pushed", json!(true)),
        ("remote_verified", json!(true)),
        (
            "status",
            json!(format!(
                "ReducedGaussactualcaller gate={gate};pushedverified;notadopted"
            )),
        ),
        ("component", a.clone()),
        ("previous_completed_experiment", previous),
        ("last_goal_turn_classification", json!("PROGRESS")),
        ("SDK_requests", json!(0)),
        ("new_independent_exports", json!(0)),
        ("independent_proof_count", json!(13181)),
        ("production_changed", json!(false)),
        ("adopted", json!(false)),
        ("owned_live_jobs", json!([])),
        ("owned_cloud_resources", json!([])),
        ("next_action", json!(next)),
    ] {
        s[k] = v;
    }
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — REDUCED GAUSS G2 ACTUAL CALLERS\n\nHEAD/remoteverified{commit};root{R};report{D}/README.md;readanalysis.json/archive-summary.jsonforALLmedians/gates/counts. Caller gate={gate},NOTadopted/proofgain. Accepted172/SDK14da unchanged,historical1x2 8.324853ms/RSS134.50MiB;goalACTIVEunmet0/8;proofledger13181unchanged,noownedjobs/cloud. PreviousgoalturnPROGRESS:GLVcomponentd01e3939bpassed38.47%assemblybutSDKc6f3098b0failedacceptedcalibrationdespitefreshON/OFF32gain1.156%CIbelowparity,16gain3.293%CIbelowparity;exactGLVSDKclosed,preservepositives. ThisgoalturnPROGRESS:distinctboundedreduced-Gaussformula+nativeindependentoracle+actualconcurrentcallers/profiles.\n\nNewalgorithmuses3EXISTINGordinaryreducedfieldproductspercomplexoutputpairvs2fusedexpressions. Normal P=(a0+a1)b0,Q=a1(b0+b1),R=a0(b1-b0),outputsP-Q/P+R;conj P=(a0+a1)b0,Q=a1(b0-b1),R=a0(b0+b1),outputsP-Q/P-R;oldreal-factor2Munchanged. FewerrawbilinearproductsBUTextraREDC;IFMAs300=300,5linearops,newdependencygraph. Earlierraw3product/2REDCsigned/streamed/nonnegativeGaussnotreused. No newwidecarrykernel/unsafe/heap/keytable/publicoption/layoutsweep. Only2nativeG2bodiesmsm.rsprivatecfgzopro_g2_reduced_gauss;oldfallbackverbatim. Diagnosticcfgzopro_reduced_gauss_caller_probe originaldirect-ref-pack adapters+oneoracletest.170originalsexact+msm/libeditsreverseall172. Field/solver/FFT/G1/proofformatunchanged.\n\nNative6stepscoreOFF/ON123each(119accepted+3originaladapters+1newscalarFq2oracle),librariesandfreeze. Oracle512groupsx8lanesx8changingstepsx2outputs65536Fq2resultsperconfiguration;repeatedbuildsnotnewdistinctinputs. Frozen/OFF/ON3harnesstestseachPASS. Allrlib/sofreezeimmediateindependenttargets,fingerprintsflagschecked. Nativefieldtests0inheritedqualification,NOTfresh. ActualG2prefixexecutablesread-onlyobjdumpIFMA/instructioncountsretainednotdynamic/causal.\n\n31blocks9contexts3variants837latency: G2prefix32/200andG1borrow200unchangedcontrol xsolo/physical16/logical32. Each3x1024primechecksresetthen8192timedcalls/worker,allworkerpostchecks/allocsoutsideallclocksbarriers. Then4FIFOgroupstatG2prefix200/G1borrow200OFF/ON32threads262144calls/worker;then4FIFO199Hzcycles same;then12owner/decode/loss.845kernelprocesses857steps179101696timedkernelcallsNOTproofs. Allgenerator/digests/capacity/zeroGLOBALtimedalloc;allpinschecked,onePID/profilezeroLost/all32workertidscoverage. PrimaryG2prefix200logical32only,legacyJSONlogical32_aggregateonecaseNOTcrosscaseaverage. All9calib/nonregandprimarygainfrozengatesretained;allpositives/uncertaintiespreserved. Nohelper/build/hash/archive/secondbenchduringcapture. Readanalysis.jsonforallactualratios/CPU/counters/profiles noIPC/cachecausalclaim. Rawperf/exeslocalhashes,4lossgzroundtripverified.\n\nNEXT:{next}. No newSDK/Gnark/all8/54/portable/stack/RSSmin/phone/TEE/pressure/giant/AWSclaim. LatestfullSDKremainsGLVnegativec6f3098b0;newproofseedrerandomized-glv-sdk/prior-proof-sha256.json13082+rootproof-inventory.json99proofsonly=>13181. LatestAWSstale17137261af77. PreserveacceptedFFTengineeringexceptionfalsegates. CommitANDpushclosedexperimentsinclnegative,noquestions/noagents,Rustxtask,nohugetables/Casmrewrite. Olderheadershistorical.\n\n");
    let audit="ADDITIONAL AUDIT: frozen/OFF executables byte-identical281d5ad675a7f5b34dd79c5e4d08cd9e775248509310c349d4fe69d4c1aeedf9, modeonlylabelsJSON; ON8ddb664c9e87201b361280643171c25b3a0dc5e647f9a7e5c9e878598e8072f4. All6G2ON/OFFmedians11.18..15.04%SLOWER,allCIaboveparity,target200logical32ratio1.111847210CI[1.104041023,1.120968058],0/31wins;ON/frozen1.112867630CI[1.108363946,1.121356422],0/31. OnlycalibfailureG2/32logical32OFF/frozenupper1.058361;primarycalibpasses. 4countergroups100%running;G2inst109428.032->123128.807/call,cycles58883.899->66745.051;G1controlinstunchanged~52350.298. 4profiles73295samples,all32workerTIDsonePIDzeroLost. ActualG2prefixsymbols1450IFMAOFF/ON,notexecutedinstructioncount. InitialcapturefirstprocessfailedMAINargassert allowingonlyG2/200beforefixture/prime/time. RepairedONLYallow32/200,3benchmark-onlylinks samefrozenlibs;no core/harnesstest/successfulcapture repeated. Initialsources/binaries/pins/codegen preservedinitial-argument-failure;workflowrelocatedruns-initial-argument-failure byteexact. Newrun-plan-v2/pins-v2;offline.rs changesONLYpinfilename fromdriver.rs;collectstrongall32profileworkerassert compiledbeforecapture. Referargument-repair.json/relinked/result.json. Noinverse/vectorGCD/GLVexperimentsinruntime.\n\n";
    fs::write(hp, format!("{header}{audit}{old}")).unwrap();
    println!("Updatedstate/handoff;goalactive0/8;gate={gate}");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
