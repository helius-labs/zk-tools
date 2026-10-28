use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/rerandomized-glv-callers";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-rerandomized-glv-callers";
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
            if q.file_name().unwrap() != "profiles" {
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
    assert_eq!(a["gate_passed"], true);
    let pins: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{R}/pins.json"))).unwrap();
    for (p, h) in pins {
        assert_eq!(hash(p), h);
    }
    let dst = Path::new(D);
    fs::create_dir_all(dst.join("artifacts")).unwrap();
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let mut local = BTreeMap::new();
    let mut compressed = vec![];
    for p in files {
        let relative = p.strip_prefix(R).unwrap();
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") {
            local.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let q = dst.join("artifacts").join(relative);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        if relative.to_string_lossy().contains("runs/loss-") && relative.ends_with("stdout.log") {
            let output = Command::new("gzip")
                .args(["-n", "-c"])
                .arg(&p)
                .output()
                .unwrap();
            assert!(output.status.success());
            let z = q.with_extension("log.gz");
            fs::write(&z, output.stdout).unwrap();
            let back = Command::new("gzip").arg("-dc").arg(&z).output().unwrap();
            assert!(back.status.success());
            assert_eq!(back.stdout, bytes);
            compressed.push(json!({"path":z.strip_prefix(dst).unwrap(),"uncompressed_sha256":hash(p),"gzip_sha256":hash(z)}));
        } else {
            fs::write(q, bytes).unwrap();
        }
    }
    for (name, source) in [
        ("groth16.rs", "crates/zopro-core/src/groth16.rs"),
        (
            "groth16_rerandomize.rs",
            "crates/zopro-core/src/groth16_rerandomize.rs",
        ),
        ("g2_phi.rs", "crates/zopro-core/src/msm_affine/g2_phi.rs"),
    ] {
        fs::create_dir_all(dst.join("artifacts/originals")).unwrap();
        fs::copy(
            format!("/home/ubuntu/src/zk-tools/zopro/{source}"),
            dst.join("artifacts/originals").join(name),
        )
        .unwrap();
    }
    save(
        dst.join("artifacts/local-executable-pins.json"),
        &json!(local),
    );
    save(
        dst.join("artifacts/loss-compression.json"),
        &json!(compressed),
    );
    let mut report=String::from("# GLV in rerandomized proof assembly callers\n\nThe bounded GLV prototype passes the declared caller screen. The combined 32-worker assembly caller is **38.47% faster**, winning all 31 paired blocks. This is an assembly-only result, not a full-proof speedup; production and the accepted SDK remain unchanged.\n\nThe current assembly computes two scalar multiples of one G1 base and a two-term G2 sum. The candidate uses the accepted two-component integer decomposition and width-five signed digits. Both groups share the coordinate cube root, but their eigenvalues differ. G2 uses the existing components (a,b). G1 uses (a-b,-b), since lambda_G1=-1-lambda_G2. Its components have magnitude below2^127. This shortens the doubling chains while preserving the scalars. Small endomorphism images of the existing eight odd multiples are computed per call; no new persistent key table or execution plan is introduced.\n\n## Complete caller comparison\n\nRyzen9950X, 31 fixed rotated/reversed process blocks, four cases, two variants. Each process primes three complete16-fixture passes, then times256 changing calls. All timed outputs are retained in a preallocated vector and verified afterward against independent Arkworks generator-scalar results. Odd-table creation, recoding, allocation/drop and endomorphism images are inside clocks; the existing fixed delta table is prepared beforehand. Both algorithms use the exact accepted field rlib. Controls are verbatim extracted original functions in the same executable, not a frozen native prover binary.\n\nThe combined caller preserves the windowed_outputs join and final C addition. Fixture scalars obey inverse=u^-1 and uv=u*v; fixtures include nonunit-Z projective coordinates. Pools and caller are restricted to CPU2 for serial cases or CPUs0..15/0..31 for16/32 workers. Actual worker masks are checked before and after timing. These are synthetic assembly inputs, not witness-derived proof requests.\n\n| Caller | Original µs | GLV µs | Paired GLV/original | Marginal interval | Wins |\n|---|---:|---:|---:|---|---:|\n");
    for case in ["g1", "g2", "w16", "w32"] {
        let v = &a["pairs"][case];
        writeln!(
            report,
            "| {case} | {:.3} | {:.3} | {:.9} | [{:.9},{:.9}] | {}/31 |",
            a["cells"][format!("{case}/m0")]["median_ns"]
                .as_f64()
                .unwrap()
                / 1000.,
            a["cells"][format!("{case}/m1")]["median_ns"]
                .as_f64()
                .unwrap()
                / 1000.,
            v["median"].as_f64().unwrap(),
            v["low"].as_f64().unwrap(),
            v["high"].as_f64().unwrap(),
            v["wins"]
        )
        .unwrap();
    }
    report.push_str("\nRatios pair process medians by block; ratios of displayed medians may differ. Bounds use ordered ranks10..22 of31, nominal97.055% under independent exchangeable blocks, without a familywise guarantee. Both serial callers and combined32 require an upper bound below parity and at least21 wins; combined16 requires upper<=1.02. Every check passes. There is no gain floor, exclusion, extension or unchanged reroll. The serial G1 candidate's block4 outlier remains included.\n\n## Validation, allocation and profiles\n\nThe checked-overflow/debug and optimized test executables each pass five cases: three new decomposition/digit, G1 pair and G2 joint oracle cases, plus two inherited FIFO helper tests. The decomposition case checks33 boundary scalars plus4096 full-width samples for both eigenvalues, including signed-digit integer reconstruction. Group tests use those33 boundary scalars plus32 full-width samples, identity/nonunit-Z bases, prepared and unprepared delta paths. They compare the original and candidate to independent Arkworks results. These are component tests; no fresh full field/core/SDK suite is claimed.\n\nThe initial checked build failed on an untyped closure index/zero. Its source, command and failure are preserved. Explicit usize/u64 types fix that compile error before tests/timing. After the tests, review corrected the benchmark-only scalar correlation to use inverse=u^-1 outside timing. Only the harness fixture and call arguments changed; original/candidate/test function bytes remained exact. A link-only step rebuilt the benchmark, retaining the successful test-source/executable identities. No successful tests or library builds were repeated. All corrected fixtures pass the normal three prime passes before any measured call.\n\nThe 248 latency processes and four profiles verify **129,024 timed assembly calls**, not SDK/Gnark proofs. All478 capture pins match after capture, including172 unchanged production source files and available linked/unused Rust dependencies. No owned build/helper/hash/archive or second benchmark overlaps capture. The process snapshots do not establish exclusive host use.\n\n| Caller | Original / GLV allocation calls per round | Original / GLV requested bytes per round |\n|---|---:|---:|\n");
    for case in ["g1", "g2", "w16", "w32"] {
        writeln!(
            report,
            "| {case} | {:.3} / {:.3} | {:.3} / {:.3} |",
            a["cells"][format!("{case}/m0")]["allocation_calls_per_round_median"]
                .as_f64()
                .unwrap(),
            a["cells"][format!("{case}/m1")]["allocation_calls_per_round_median"]
                .as_f64()
                .unwrap(),
            a["cells"][format!("{case}/m0")]["requested_bytes_per_round_median"]
                .as_f64()
                .unwrap(),
            a["cells"][format!("{case}/m1")]["requested_bytes_per_round_median"]
                .as_f64()
                .unwrap()
        )
        .unwrap();
    }
    report.push_str("\nRequested bytes count allocation/reallocation traffic, not peak live memory or RSS. The fixed GLV-derived G1 image array is512bytes, and the three G2 image/odd-delta arrays total3072bytes. Four128-byte digit arrays replace two256-byte arrays. Compiler frames and other temporaries are additional; these payloads are not stack bounds or whole-proof memory claims. Existing odd-table allocations remain included. No mobile/TEE, concurrency, pressure or giant-circuit qualification follows.\n\nAll latency precedes four separate FIFO-gated199Hz cycles:u profiles of16384 calls each. Capture enables after priming and disables before output checks. All four profiles finish before decode and loss audits.\n\n| Profile | Samples | Sampled TIDs | Lost |\n|---|---:|---:|---:|\n");
    let mut total = 0;
    for (name, p) in a["profiles"].as_object().unwrap() {
        total += p["samples"].as_u64().unwrap();
        writeln!(
            report,
            "| {name} | {} | {} | 0 |",
            p["samples"],
            p["tids"].as_array().unwrap().len()
        )
        .unwrap();
    }
    writeln!(report,"\nTotal{total}samples; one sampled PID per profile, no lost records. Full symbol weights and raw hashes are retained. Sample shares are not removable proof time. Raw perf.data and executables stay local with hashes; four loss dumps are archived with verified deterministic-gzip round trips.\n").unwrap();
    report.push_str("## Full-proof scope and next step\n\nNo SDK request, independent proof export or AWS/Aeglos timing occurs; the proof ledger remains13082. This passes only the standalone assembly gate. Next port the exact runtime mechanism into a private accepted-source snapshot, run native core/prover checks and a controlled accepted/fresh-OFF/ON SDK comparison before broader qualification or adoption. This assembly-only reduction cannot by itself close the2.25x primary gap.\n\nThe following accepted local timings and supplied M5 CPU/Metal references are inherited. The accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight10xCPU objective remains active and unmet0/8.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
    report.push_str("\nHistorical primary peak134.50MiB; merge54 remains161.341835ms /1910.1875MiB without a supplied M5 reference. No new full-proof latency, RSS or minimum RAM is established.\n\nReferences: [Arkworks G1 GLV constants](https://raw.githubusercontent.com/arkworks-rs/curves/master/bn254/src/curves/g1.rs), [G2 constants](https://raw.githubusercontent.com/arkworks-rs/curves/master/bn254/src/curves/g2.rs). The implementation reuses the accepted bounded integer decomposition rather than importing Arkworks BigInt code. This is a new assembly use of the known GLV method, not a novelty claim.\n\n[Design](artifacts/design.md), [candidate](artifacts/candidate.rs), [extracted controls](artifacts/original.rs), [extraction audit](artifacts/extraction-audit.json), [results](artifacts/analysis.json), [all rows](artifacts/rows.json), [checked tests](artifacts/native-v2/test-checked/stdout.log), [optimized tests](artifacts/native-v2/test-optimized/stdout.log), [fixture correction](artifacts/fixture-repair.json).\n");
    fs::write(dst.join("README.md"), report).unwrap();
    let mut paths = vec![];
    walk(dst, &mut paths);
    paths.sort();
    let mut sums = String::new();
    for p in &paths {
        writeln!(
            sums,
            "{}  {}",
            hash(p),
            p.strip_prefix(dst).unwrap().display()
        )
        .unwrap();
    }
    fs::write(dst.join("SHA256SUMS"), sums).unwrap();
    let o = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(dst)
        .output()
        .unwrap();
    fs::write(format!("{R}/archive-verification.log"), &o.stdout).unwrap();
    assert!(o.status.success());
    save(
        format!("{R}/archive-summary.json"),
        &json!({"hashes":paths.len(),"git_files":paths.len()+1,"GNU_strict_passed":true,"gzip_roundtrips":4,"profile_samples":total}),
    );
    println!("Archived{}verifiedhashes", paths.len());
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    for(k,v)in [("root",json!(R)),("report",json!(format!("{D}/README.md"))),("commit",json!(commit)),("pushed",json!(true)),("remote_verified",json!(true)),("status",json!("GLV rerandomized assembly caller gate passed; no SDK gain/adoption yet")),("component",read(format!("{R}/analysis.json"))),("previous_completed_experiment",previous),("last_goal_turn_classification",json!("PROGRESS")),("production_changed",json!(false)),("adopted",json!(false)),("owned_live_jobs",json!([])),("owned_cloud_resources",json!([])),("next_action",json!("Native exact GLV final-assembly integration, then accepted/OFF/ON primarySDK; unchangedproduction172/14da/goalactive0/8/ledger13082"))]{s[k]=v;}
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — GLV RERANDOMIZED ASSEMBLY CALLERS\n\nHEAD/remoteverified{commit};root{R};report{D}/README.md. Caller gatePASS: G1pair39.10%, G2joint40.04%, combined16 38.99%, combined32 38.47% faster;ALL31/31wins. No fullSDKgain/adoption yet. Accepted172/SDK14da unchanged, historical1x2 8.324853ms/RSS134.50MiB;all-eight10xgoalACTIVEunmet0/8,ledger13082unchanged,noownedjobs/cloud. PreviousgoalturnPROGRESS:negativeSIMDnativecaller858e1127d closedpushed. ThisturnPROGRESS:newGLVassemblyalgorithm+independentoracles+fixedcallerperf/profiles.\n\nExactoriginalpair_g1/joint_g2/odd_multiples/add_digit/blinding_digits/FixedBase::new,mul extractedfromacceptedfiles;visibilitywrapperONLY. Acceptedg2_phi boundedphi_split reused. G2eiglambda440792... G1=-1-lambdaG2; G1components(a-b,-b),magnitudes<2^127. wNAF5shortchains;fixedderivedG1image512B,G2phi/dt/dp3072B(notstack/RSSbounds),noheap/table/keyplanaddedbeyondoriginalcallallocations. Runtimecandidate.rs beforecfgtestmoduleexactprototypefornativeport. Candidateusesoriginal::odd_multiples/add_digit needadaptto parenthelpersONLY;splitmodule inline/copyacceptedhelpers;removecomponent-onlytestsrequiringprobeoracles. No C/asmrewrite/multigiBcache/publicoption.\n\nInitialcheckedbuild faileduntypedclosureindex/zero;preservedpreparation-v1andnative/failure.json;explicitusize/u64repairbeforetest/capture. native-v2checked5/optimized5PASS(3new+2perf). 33boundary+4096fullwidthscalar decompositionsbothgroups/digitreconstruction;65G1pairand65G2jointcasesprepared/unpreparedagainstArk,identity/nonunitZ. Beforecapturecorrectedbenchfixtureleft=inverse(u),jointu=u,uv=u*v,outsideclock;fixture-v2containsactualsuccessfultestsource/exes;functions/testsunchanged. linkedonlybenchnewprobe;nosuccessfultest/libraryrerun.\n\nCapture36661terminalexit0;248latency31blocks4cases2modes256callsafter3x16primes;then4FIFO199HzcyclesG2solo/combined32both16384calls;then12decode/loss264steps252captures129024timedassemblycallsNOTproofs. AlloutputscheckedagainstArk,capture478pinsverified. Nohelper/build/hash/archive/secondbenchduringcapture. SameimageverbatimoldfunctionscontrolNOTfrozennativeSDK. G1ratio.608995507CI[.608394076,.609893091];G2.599642114[.598865668,.601370039];16.610079685[.605489338,.613733708];32.615310472[.609945035,.627869441];all31wins. Readanalysis.jsonforallns/allocation/profiles,block4G1outlierretained. AllprofileonePIDzeroLost. Allocationsstilltimed,notzero claim. Profile rawlocal+hash,lostdumpsgzroundtripverified.\n\nNEXT rootrerandomized-glv-sdk: accepted172purecopy;newgroth16/glv_blinding.rs+inlineexactsplit helper,dispatchonlypair_g1/joint_g2 privatecfgzopro_glv_blinding. Preserveoriginalfallback/pointertablePreparedFixedPoints/randomness/distribution/verification/MSM/FFT/solver/field. Adaptcandidateparenthelperreferencesonly. Need119nativecoreoriginal+appropriateassemblyoracletests,raw2cubic6OFF/ON,frozenlibs/SDKreceipts,thenfixedaccepted/OFF/ON32/16fullproof andGnarkbeforeall8/54portable/resources. Parentcomponent38.5%notfullproofgain,cannotaloneclose2.25xgap. ActualSDKlastweightednegativea0d72e62b;AWSstale17137261af77. CommitANDpush eachclosedexperiment,noquestions/noagents. Olderheadershistorical.\n\n");
    fs::write(hp, format!("{header}{old}")).unwrap();
    println!("Updatedstate/handoffgoalactive0/8");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
