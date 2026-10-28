use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/direct-ref-pack";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-direct-ref-pack";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn copy_tree(root: &Path, dir: &Path, out: &Path) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let n = e.file_name().to_string_lossy().to_string();
        let t = e.file_type().unwrap();
        if t.is_dir() {
            if n == "source"
                || n == "profiles"
                || n.starts_with("target-")
                || n.starts_with("frozen")
            {
                continue;
            }
            copy_tree(root, &p, out);
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
    let root = Path::new(R);
    let a = read(root.join("analysis.json"));
    let tables = read(root.join("tables.json"));
    assert_eq!(a["latency_processes"], 558);
    assert_eq!(a["independent_proof_count"], 12983);
    assert!(!Path::new(D).exists());
    let final_pins = read(root.join("capture-pins-final.json"));
    for (p, h) in final_pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let audit = Command::new(root.join("native-helper"))
        .arg("audit")
        .output()
        .unwrap();
    assert!(audit.status.success());
    fs::write(root.join("post-capture-source-audit.log"), audit.stdout).unwrap();
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let gate = a["gate_passed"].as_bool().unwrap();
    let status = if gate {
        "passes the addition-caller screen; full SDK qualification is required"
    } else {
        "fails the addition-caller screen and is not adopted"
    };
    let mut report=format!("# Direct coordinate-reference packing in G1\n\nThe candidate **{status}**. This is an actual streaming-kernel component result, not a complete MSM or proof result. Production and the accepted SDK remain unchanged.\n\n");
    for label in ["frozen", "fresh"] {
        let i = &a["aggregates"][label];
        writeln!(report,"The four-G1-context paired aggregate selected/{label} median is **{:.9}**, interval [{:.9}, {:.9}], {}/31 wins ({:+.3}% latency change; positive is slower).",i["median"].as_f64().unwrap(),i["low"].as_f64().unwrap(),i["high"].as_f64().unwrap(),i["wins"],100.*(i["median"].as_f64().unwrap()-1.)).unwrap();
    }
    report.push_str("\n**The G2 control calibration fails by a large margin.** Fresh OFF takes about 3.11×/3.18× frozen at 200/352 pairs, despite unchanged G2 source, matching outputs and capacities, and equivalent command environments. Selected/fresh G2 ratios near 0.32 therefore receive no optimization credit. An offline [control audit](artifacts/control-layout-audit-v2.json) confirms byte-identical `.text`, `.rodata`, `.data` and `.rela.dyn` sections in the OFF executables; `.data.rel.ro` differs only in 20 source-location line numbers. The timing cause is unresolved; runtime or address-layout effects were not isolated. G1 calibration passes, but its separate aggregate gain requirements fail against both controls. No G2 timing was repeated or substituted.\n");
    report.push_str("\n## Change and scope\n\nThe new Rust AVX2/AVX-512 constructor reads eight existing 32-byte field references into vector registers, joins register halves, and performs the same transpose/radix-52 split as the original mixed-R256 constructor. It bypasses the contiguous eight-field input-array API. Only full-vector G1 forward input packs use it; field products, prefix order, scratch records, scalar tails, reverse packing, point order, G2, trees, four-lane and portable paths retain their original code. No heap buffer, key lookup table, cached plan, scheduler change or assembly rewrite is introduced.\n\nThis differs from the rejected [reference-staging experiment](../2026-10-27-g1-reference-staging/README.md), which changed temporary point arrays but still passed copied field arrays into the original constructor. It also differs from late/input-anchor records and G2 index-tree gathers. The official generated IFMA files and generator remain byte-identical and its consistency check passes. The new field impl is in a separate module.\n\nUnsafe intrinsics are confined to field loads and vector operations. Valid references cover the complete initialized 32-byte loads, allow read aliasing, and require only field alignment; the module guard requires all CPU features used. The new field test checks both primes, lazy boundaries, noncontiguous and 32-byte-unaligned addresses, lane permutations and repeated references against an independent integer bit split and the original constructor/multiplication. It covers 24,864 lane inputs, including duplicates. Both 34-test native field suites pass.\n\nAll 172 production files remain exact. Frozen private source adds only two diagnostic core suffixes, retaining 170 exact originals. Fresh/selected private source retains 168 exact originals; the other four recover exactly through recorded inverse edits. No inverse-arithmetic change from earlier rejected candidates is inherited. Existing diagnostic adapters call the actual stream kernels; they do not reproduce the new arithmetic. G1 adapter successes from the initial baseline attempt and the corrected G2 case are retained, with 122 full core tests passing for fresh and selected.\n\n## Fixed caller measurements\n\nSix contexts use G1 owned unsigned, G1 borrowed signed and G2 prefix/norm additions at 200/352 pairs. These are synthetic valid generator multiples at primary batch sizes, not a captured witness distribution or complete bucket scheduler. Each process uses CPU 2, one active caller, three eight-round primes/checks, an outside-clock reset, and 1,024 complete stream calls. Fixture construction and independent weighted-generator checks are outside timing; the call loop, loading/packing, arithmetic, record traffic and destination writes are inside. Every destination is checked.\n\nValues are medians of process means, in microseconds per complete batch. Paired ratios use corresponding blocks.\n\n| Caller | Pairs | Frozen | Fresh OFF | Selected ON |\n|---|---:|---:|---:|---:|\n");
    report.push_str(tables["latency"].as_str().unwrap());
    report.push_str("\n| Caller | Pairs | Paired ratio | Median | Marginal interval | Wins | Check |\n|---|---:|---|---:|---:|---:|---|\n");
    report.push_str(tables["pairs"].as_str().unwrap());
    writeln!(report,"\nFailed checks: `{}`. The fixed gate requires every fresh/frozen calibration interval within [1/1.03, 1.03], selected nonregression upper bounds ≤1.02 for all six contexts against both controls, and four-G1-context geometric aggregate upper <1 with at least 21/31 wins against both. The two G2 cases are controls, not included in the gain aggregate. Bounds use ordered ranks 10 through 22 of 31 paired process means, nominal 97.055% marginal coverage with no familywise guarantee. No gain floor, exclusions, extensions, successful-capture repeats or parameter sweeps were used.\n",a["failed_checks"]).unwrap();
    report.push_str("All 558 latency captures match independent weighted-generator outputs, pairwise digests and scratch capacities; all report zero timed allocations. G1 record payload is 40,000/70,400 bytes at 200/352; G2 is 16,000/28,160. The borrowed diagnostic input vector is 3,200/5,632 bytes outside timing, not a proposed new prover allocation. Unchanged capacities do not establish compiler stack behavior or a whole-process RAM saving. CPU/process-RSS observations are retained in analysis; they are not minimum-RAM measurements.\n\n## Code generation and profiles\n\nThe [code-generation audit](artifacts/codegen-audit.json) uses the actual linked caller binaries and retains every selected G1 stream disassembly. Counts below are static whole-function sites; they are not retired instructions, dynamic copies, spills or removable latency. Loads may fold into memory-operand inserts.\n\n| Variant | Stream | Text bytes | Instructions | IFMA sites | Stack-address sites | Insert sites / memory inserts |\n|---|---|---:|---:|---:|---:|---:|\n");
    let cg = read(root.join("codegen-audit.json"));
    for label in ["frozen", "fresh", "selected"] {
        for row in cg["variants"][label]["G1_stream_symbols"]
            .as_array()
            .unwrap()
        {
            let name = row["symbol"].as_str().unwrap().replace('|', "/");
            writeln!(
                report,
                "| {label} | `{name}` | {} | {} | {} | {} | {} / {} |",
                row["text_bytes"],
                row["instructions"],
                row["IFMA_sites"],
                row["stack_address_operand_sites"],
                row["vinserti64x4_sites"],
                row["vinserti64x4_memory_operand_sites"]
            )
            .unwrap();
        }
    }
    report.push_str("\nFour FIFO-gated warm 199 Hz cycles:u profiles follow all latency: G1 owned/200 and borrowed-signed/352, fresh/selected, 262,144 calls each after three eight-call primes. All profiles precede decoding, hashes and archive work. No owned helper, build, hash, archive or other benchmark overlaps capture. Unrelated host work is recorded, with no exclusive-host claim.\n\n| Caller | Variant | Samples | Lost |\n|---|---|---:|---:|\n");
    report.push_str(tables["profiles"].as_str().unwrap());
    let total: u64 = a["profiles"]
        .as_object()
        .unwrap()
        .values()
        .map(|v| v["samples"].as_u64().unwrap())
        .sum();
    writeln!(report,"\nTotal samples: {total}. Every profile has one sampled PID and zero lost records. Symbol weights are component observations, not full-proof wall-time fractions. Raw perf.data files stay local with hashes; decoded observations are included.\n").unwrap();
    report.push_str("The control audit freezes every transitive rlib and shared object immediately after each isolated variant build, checks actual field/core rustflag fingerprints, and links only the frozen dependency directory. OFF/ON field and probe hashes differ. Compiler, source, protocol, code-generation observations and binaries were pinned before timing; all final pins pass after capture.\n\nTwo preparation failures are preserved. The original native test environment omitted the prefix-record flag required by the G2 control; both successful G1 adapter tests were retained and only the failed G2 test was retried with the accepted runtime settings. The next freezer assumed a hashed core rlib filename, while Cargo emits an un-hashed core rlib for its cdylib/rlib target. The freezer now audits matching core fingerprints and freezes that artifact immediately; the successful G2 test and library build were not repeated. These repairs occurred before all timing and changed no candidate kernel source.\n\n## Accepted comparison (inherited)\n\nThe original all-eight 10× CPU objective remains active and unmet, 0/8. These are accepted 172-source full-proof observations, not this component's all-shape measurements. The M5 CPU and Metal times are supplied reference data.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | M5 CPU / accepted | M5 Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
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
    report.push_str("\nAccepted primary historical peak RSS: 134.50 MiB; merge54: 161.341835 ms / 1910.1875 MiB peak, with no supplied M5 reference. Primary CPU target: 3.7 ms. The accepted FFT adoption retains its explicit engineering exception and false full-performance/qualified-scope gates.\n\nNo new SDK/Gnark proof was generated; the independently verified ledger remains 12,983. No new complete-MSM, all-eight/54, portable/mobile/TEE, pressure/concurrency, giant-circuit or AWS/Aeglos result is claimed. No cloud resource was created.\n\nResearch checked [PSSM](https://www.mdpi.com/2079-9292/15/16/3660), which schedules independent scalar-multiplication sequences, and [Versal AIE point additions](https://arxiv.org/abs/2502.11660), which target different hardware and field size. Neither supplies a matched performance prediction for this already vectorized prover. The implemented load change follows the current source dataflow; no paper or external code speedup is transferred. Earlier SRE, memory-adaptive buckets, GLV/ring and shifted-base investigations remain as recorded.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [capture seal](artifacts/capture-seal.json), [identity audit](artifacts/control-identity-audit.json) and all raw logs are archived.\n\n");
    let next = if gate {
        "Qualify this exact direct-input kernel in complete SDK proofs, then all-eight/54, portable and resource/concurrency checks before adoption."
    } else {
        "Close this exact direct-input candidate without SDK integration. Preserve all positive estimates and control failures; do not repeat unchanged input layout or intrinsic variants. Before further G2 kernel work, investigate the control discrepancy with a separately declared baseline diagnostic that records runtime data layout and profiles unchanged arithmetic; this does not rescue the closed G1 candidate."
    };
    report.push_str(next);
    report.push('\n');
    report.push_str("\nAn additional post-capture diagnostic mistake is recorded in [control restoration](artifacts/control-restoration.json). GNU `objcopy --dump-section INPUT` with no separate output rewrote the two original OFF ELF files after all captures, workflow pin checks and initial analysis had finished. Those rewritten files were preserved locally. Repeating only the recorded probe-link commands, with the same source, frozen libraries, output paths and working directory, reconstructed both exact pre-capture SHA-256 values. All final capture pins pass after restoration. No Cargo build, test, timing or profile was repeated. The selected executable was untouched. The first offline source-location reader also missed PIE relocation addends; the corrected reader uses direct ELF reads of the restored originals and preserves the original failure log.\n");
    assert_eq!(
        read(root.join("control-layout-audit-v2.json"))
            ["data_rel_ro_equal_after_masking_only_20_source_line_fields"],
        true
    );
    save(
        root.join("closeout.json"),
        &json!({"gate_passed":gate,"adopted":false,"production172_unchanged":true,"accepted_SDK_unchanged":true,"final_pins_verified":final_pins.as_object().unwrap().len(),"profile_samples":total,"captures":562,"tests":"34fieldOFF/34fieldON,122corefresh/122coreON,initial2frozenG1plus1repairedG2","kernel_unchanged_after_first_write":true,"G2_control_gap_unresolved":true,"postcapture_OFF_probe_reconstruction":"control-restoration.json; exact pre-capture hashes recovered, only probe links repeated","new_Gnark_exports":0,"proof_ledger":12983,"next":next}),
    );
    fs::create_dir_all(Path::new(D).join("artifacts")).unwrap();
    copy_tree(root, root, &Path::new(D).join("artifacts"));
    for f in [
        "crates/zopro-bn254/src/lib.rs",
        "crates/zopro-bn254/src/direct_input.rs",
        "crates/zopro-core/src/lib.rs",
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/simd.rs",
        "crates/zopro-core/src/msm/direct_input.rs",
        "crates/zopro-core/src/msm/caller_probe.rs",
    ] {
        let dest = Path::new(D).join("artifacts/private-source").join(f);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(root.join("source").join(f), dest).unwrap();
    }
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
        &json!({"hashes":files.len(),"git_files":files.len()+1,"gate_passed":gate,"GNU_strict_passed":true}),
    );
    println!(
        "Archived{}hashes/{}files;gate{gate},ledger12983unchanged",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let a = read(format!("{R}/analysis.json"));
    let summary = read(format!("{R}/archive-summary.json"));
    let close = read(format!("{R}/closeout.json"));
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    for(k,v)in[("status",json!(format!("direct referenced-coordinate G1 packing component gate={};committed/pushed/verified;not adopted",a["gate_passed"]))),("root",json!(R)),("report",json!(format!("{D}/README.md"))),("commit",json!(commit)),("pushed",json!(true)),("remote_verified",json!(true)),("production_changed",json!(false)),("adopted",json!(false)),("last_goal_turn_classification",json!("PROGRESS")),("SDK_requests",json!(0)),("new_independent_exports",json!(0)),("independent_proof_count",json!(12983)),("artifact_hashes",summary["hashes"].clone()),("git_files_new_report",summary["git_files"].clone()),("component",a.clone()),("previous_completed_experiment",previous),("next_action",close["next"].clone()),("owned_live_jobs",json!([])),("owned_cloud_resources",json!([]))]{s[k]=v;}
    s["control_runtime_audit"] = read(format!("{R}/control-layout-audit-v2.json"));
    s["postcapture_control_restoration"] = read(format!("{R}/control-restoration.json"));
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — DIRECT G1 REFERENCE PACKING COMPONENT\n\nHEAD/remote verified {commit}. Root {R}; report {D}/README.md. Gate {}; not adopted. {} verified hashes / {}Gitfiles. Accepted172/SDK14da unchanged, primary8.324853ms/historical134.50MiB, goalACTIVEunmet0/8, ledger12983 unchanged. LastgoalturnPROGRESS:new explicit referenced-coordinate load API/actualG1 integration/independentbitchecks/callerstudy/profiling published. No ownedjobs/cloud.\n\nRead analysis.json/cells/pairs/aggregates for exact sixcase ns and31block bounds. Gate needs every calibrationfactor1.03/everycandidateupper<=1.02 +4G1aggregateupper<1,>=21wins versusboth;G2excludedgainaggregate. 558latency1024rounds+3x8primes,THEN4FIFO199Hzprofiles262144roundsG1owned200/borrowedsigned352fresh/ON,THEN12decode/owner/loss. All562generatororacles/zerotimedalloc/digests/capacities. Profiles onePIDzeroLost;finalcapturepins {}exact. No fullMSM/SDK/newGnark/all8/54/portable/resource/AWSqualification. Core122fresh/122ON;field34OFF/34ONincluding24864rawlanes/bitchecks;officialgeneratorcheckPASSunchanged. Frozen initial2G1successes+1correctedG2success. Source168fresh/170frozenoriginalsexact;all172productionexact;inverseeditsrecoverothers. No newfield arithmetic/inverse/IFMAgenerated change.\n\nNewfieldmodule crates/zopro-bn254/src/direct_input.rs implementsIfma8<C,true>::from_refs([&Fp;8])with8unaligned256bitloads,4registerhalfjoins,existingtranspose/split. Corebridgein simd.rs;new msm/direct_input.rs has sameforwardarithmetic/records with direct refs,onlynativeG1fullvectors selectcfgzopro_direct_ref_pack (+AVX2/IFMA/VL/DQ). Originalbranchkeptunderinversecfg. This differsfailedG1referencestaging(originalcontiguousconstructorretained),no unchangedrepeat. Extra safetycommentload32bytesvalid/aliasreadallowed. Actualcodegen-audit.json+disassembly has wholeG1streamsitecounts,notdynamicspills/copysavings. ScratchsameG1 40000/70400B,G2 16000/28160B;fixtureborrowedrefs3200/5632Boutsideclocknotproductionbuffer.\n\nPreparation failuresbeforetiming preserved: native-first frozen G2requiresmissing runtimePREFIXflag,initial2G1successes retained;repair-preparation.rs/v2restoresacceptedtestenv,retriesONLYG2pass1 thenfrozenbuildPASS;v2freeze failedunhashedcorefilename. repair-freezer.rs/native-v3 handlescorecdylibrlibnameandactualfingerprints;native-rest workflow skips2successfulstepsandfinishesremaining,freezeimmediateafterisolatedbuild(nointerveningbuild),.so included. No successfultest/build/capture repeated. Handles99206/60301failedterminal;80566native-restterminal. Helpersv1/v2/v3preserved;kernel source unchanged. experiment-v3 link/prepare/finish usesnewreceipts/testpaths;capture-plan-final.json+capture-pins-final fromseal.rs,originalrunplan/pinsretained.\n\nNEXT: {}. Continueautonomous,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite;commitANDpush eachcompletedexperiment includingnegative. PreserveacceptedFFTengineeringexception/falsefullperformancegates. LatestAWSstale17137261af77. Previousheadershistorical.\n\n",a["gate_passed"],summary["hashes"],summary["git_files"],close["final_pins_verified"],close["next"].as_str().unwrap());
    let critical="IMPORTANT CONTROL/RECOVERY EVIDENCE: G1 aggregate ON/fresh1.001151757 CI[1.000432622,1.001453872],4/31wins;ON/frozen1.000899387 CI[1.000423392,1.001465770],8/31. All4G1calibration/nonregpass, aggregategainFAILboth. Borrowed352positive.033%vsfresh but uncertain. G2fresh control3.11x/3.18x slowerthan frozen at200/352; calibrationFAIL,notcandidategain. ExactOFF .text/.rodata/.data/.rela.dyn BYTEIDENTICAL; .data.rel.ro only20source-locationlinewordsdiffer. CauseUNRESOLVED,noCPU/memorycausalclaim. control-layout-audit-v2.json reads exactoriginalELF/relocationsandcommands;all62G2env/argpairsmatchexceptbinarypath.\n\nPOSTCAPTURE MISTAKE: objcopy --dump-section INPUT withoutOUTPUT rewrotebothOFFprobeELFs AFTER allcaptures+workflowpinchecks+initialanalysis. Rewrittenfilespreservedas*-probe.objcopy-mutated. restore-controls.rs repeatedONLYrecordedprobeLINKcommands(notCargo/tests/timing/profile),samepaths/cwd/frozeninputs,andrestoredEXACToriginalSHAfrozenf2b69dae.../fresh173a496d.... control-restoration.jsonrecordsfullhashes. Selectedprobeuntouched;ALLfinalcapturepinsverifiedagainatarchive. InitialofflinecontrolauditreaderfailedbecausePIEfilepointersneedRELATIVEaddends;v2directELFparserfixedthat,notmeasurements. AlwaysuseREADONLYELFreader or distinctOUTPUTforobjcopy. Neverinplaceobjcopyevidence. Terminalcapture1159;no livejobs.\n\n";
    fs::write(hp, format!("{header}{critical}{old}")).unwrap();
    println!("State/handoffupdated;goalactive0/8;ledger12983");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
