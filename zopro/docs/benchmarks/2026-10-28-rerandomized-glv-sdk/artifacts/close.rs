use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/rerandomized-glv-sdk";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-rerandomized-glv-sdk";
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
    assert_eq!(a["independent_proof_count"], 13181);
    let table = read(format!("{R}/tables.json"));
    let pins: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{R}/pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let dst = Path::new(D);
    fs::create_dir_all(dst.join("artifacts")).unwrap();
    let mut paths = vec![];
    walk(Path::new(R), &mut paths);
    let mut binaries = BTreeMap::new();
    let mut compression = vec![];
    for p in paths {
        let rel = p.strip_prefix(R).unwrap();
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") {
            binaries.insert(p.display().to_string(), hash(p));
            continue;
        }
        let q = dst.join("artifacts").join(rel);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        if rel.to_string_lossy().contains("runs/loss-") && rel.ends_with("stdout.log") {
            let o = Command::new("gzip")
                .args(["-n", "-c"])
                .arg(&p)
                .output()
                .unwrap();
            assert!(o.status.success());
            let z = q.with_extension("log.gz");
            fs::write(&z, o.stdout).unwrap();
            let back = Command::new("gzip").arg("-dc").arg(&z).output().unwrap();
            assert!(back.status.success());
            assert_eq!(back.stdout, bytes);
            compression.push(json!({"path":z.strip_prefix(dst).unwrap(),"uncompressed_sha256":hash(p),"compressed_sha256":hash(z)}));
        } else {
            fs::write(q, bytes).unwrap();
        }
    }
    assert_eq!(compression.len(), 6);
    save(
        dst.join("artifacts/local-executable-pins.json"),
        &json!(binaries),
    );
    save(
        dst.join("artifacts/loss-compression.json"),
        &json!(compression),
    );
    let gate = a["gate_passed"].as_bool().unwrap();
    let p = &a["pairs"]["w32/on/off"];
    let q = &a["pairs"]["w32/on/control"];
    let verdict = if gate {
        "passes the declared full-proof screen; broader qualification is still required"
    } else {
        "fails the declared full-proof screen and is not adopted"
    };
    let mut report=format!("# GLV final assembly in full SDK proofs\n\nThe candidate **{verdict}**. Primary32 ON/OFF paired median is **{:.9}**, bounds[{:.9},{:.9}], {}/15wins. ON/accepted is **{:.9}**, bounds[{:.9},{:.9}], {}/15wins. All positive estimates, uncertainty and failed checks are retained. Production and the accepted SDK remain unchanged.\n\nThe [standalone caller study](../2026-10-28-rerandomized-glv-callers/README.md) improved final assembly by about38.5%. This ports that exact runtime mechanism to the native prover. The existing two-component integer lattice decomposition shortens both G1 and G2 variable-base doubling chains. G1 uses components(a-b,-b) because its eigenvalue is-1-lambda_G2. Width-five digits and small endomorphism images are proof-local. Randomness, proof distribution, field arithmetic, MSM/FFT/solver algorithms, verification and image formats are unchanged. There is no new persistent point/key table or execution plan.\n\n## Verified request latency\n\nRyzen9950X,15 fixed rotated/reversed process blocks, accepted/freshOFF/ON at32logical and16physical workers. Every latency process has3 primes and151 timed requests. Loading is excluded; witness solving, fresh randomized proving, SDK verification and serialization are included. Times are medians of block medians; CPU includes coordination and RSS includes loading/primes.\n\n{}\n{}\n",p["median"].as_f64().unwrap(),p["low"].as_f64().unwrap(),p["high"].as_f64().unwrap(),p["wins"],q["median"].as_f64().unwrap(),q["low"].as_f64().unwrap(),q["high"].as_f64().unwrap(),q["wins"],format!("| Workers | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |\n|---|---|---:|---:|---:|---:|\n{}",table["latency"].as_str().unwrap()),format!("| Workers | Ratio | Paired median | Marginal interval | Wins | Check |\n|---|---|---:|---|---:|---|\n{}",table["pairs"].as_str().unwrap()));
    writeln!(report, "Failed checks: `{}`.\n", a["failed_checks"]).unwrap();
    report.push_str("Bounds use ranks4..12 of15, nominal96.484375% under independent exchangeable blocks, without familywise guarantees. OFF/accepted must fit[1/1.03,1.03] at both worker counts. PrimaryON requires upper<1 and at least11wins against both controls;16-workerON requires upper<=1.02. No minimum gain floor, exclusion, extension, successful-capture repeat or unchanged reroll is applied. Crossing a required bound does not by itself establish a causal slowdown. The component gain is not transferred to proof latency. In this run the 32-worker GLV/fresh estimate improves 1.156%, with bounds showing a 0.290% to 2.547% reduction, and 12/15 wins. The 16-worker GLV/fresh estimate improves 3.293%, bounds 1.043% to 5.082%, also 12/15 wins. Both positive observations are retained. Against accepted code the 32-worker point estimate is 0.936% slower with an interval spanning parity, while the 16-worker estimate is 0.200% faster with a broad interval. Fresh/accepted calibration fails at both worker counts. Thus the study supports the measured comparisons against fresh code but does not demonstrate an improvement over accepted production. It does not prove that all observed changes arise solely from GLV; host variation and compiled context remain unresolved.\n\n## Integration, validation and memory\n\nOnly groth16.rs and groth16_rerandomize.rs change reversibly, plus one private module.170 other original source files remain byte-exact; reversing the two edits recovers all172. Original pair/joint bodies remain verbatim fallback. Parent helper-path visibility and inline placement of the exact accepted split helper are the only runtime adaptations. There are no diagnostic observers in the SDK. The private cfg selects the new final assembly; no public runtime option is introduced. Field source is unchanged and its prior qualification is inherited.\n\nFreshOFF and ON each pass all119 native core tests, two raw-image tests and six cubic prover tests. The retained assembly exponent oracle covers prepared/unprepared paths, full-width/negative/zero scalars, identity rejection/fallback, poisoned omitted-B1 bytes and proof encoding. The parent's independent Arkworks/decomposition tests qualify the unchanged helper bodies; they are not counted as new native tests. Some optional external-fixture core tests return when their fixture is absent; those scenarios are not claimed. Complete real SDK proofs are checked separately below.\n\nAll transitive rlibs/proc-macro shared objects are frozen immediately in separate target roots. Actual core/field flags and native features are checked, and the standard SDK driver links exact eight-library receipts. Original and selected full SDK hashes differ. No successful build/test/capture/verifier is rerun for bookkeeping.\n\nGLV image arrays add fixed local payloads:512bytes in G1 and3072bytes in G2, with compiler frames/other temporaries additional. Existing table allocations remain within the timed request. These payloads are not measured stack bounds, minimum RAM or memory-pressure qualification. Process peaks above are observations, not a universal resource guarantee. No new persistent key cache or input-sized buffer is added. Broader portable, stack, resource, concurrent and250/4999 qualification remains necessary before adoption.\n\n## Proofs and profiles\n\nAll90 latency captures precede six separate FIFO-gated199Hz cycles:u profiles,251 requests after3 primes each. Three4-worker preflights precede latency. All99 SDK captures finish before independent verification, decode or archive. No owned build/helper/hash/archive or second benchmark overlaps capture. The post-capture host snapshot contains unrelated CPU workloads; there is no per-process capture-time trace establishing their contribution. It does not establish exclusive-host use or an exclusive cause of calibration drift.\n\nAll15,396 complete SDK requests verify internally. All99 fresh164-byte exports independently pass Gnark, expected public binding, member hashing and uniqueness against13,082 previous identities. The independent proof ledger is now **13,181**. Copies/subinventories are not counted again.\n\n");
    report.push_str("| Workers | Variant | Samples | Lost | Matched groth16 symbol period |\n|---|---|---:|---:|---:|\n");
    report.push_str(table["assembly_profiles"].as_str().unwrap());
    report.push_str("\nThis narrow symbol-name matcher misses inlined and generic curve arithmetic. A near-zero matched share does not mean assembly was eliminated.\n");
    let samples: u64 = a["profiles"]
        .as_object()
        .unwrap()
        .values()
        .map(|p| p["samples"].as_u64().unwrap())
        .sum();
    writeln!(report,"\nThe six profiles contain{samples}samples, one sampled SDK PID each, zero lost records. Assembly-symbol period includes the matched functions and is not removable proof wall time. Complete symbol maps and raw profile hashes are retained; raw perf files stay local. Six loss dumps have verified deterministic-gzip round trips.\n").unwrap();
    report.push_str(if gate{"The primary screen permits broader all-eight/54 and resource/portability qualification of these exact bytes. It does not authorize adoption or establish the original tenfold goal.\n\n"}else{"This exact native integration is closed without adoption. Its standalone caller gain and every small positive full-proof estimate remain within their measured scopes. No unchanged source/flag/layout/schedule reroll is selected.\n\n"});
    report.push_str("## Inherited CPU and Metal comparison\n\nThese accepted local observations and supplied historical M5 references are inherited, not all-eight measurements from this primary-only screen. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged. Original all-eight CPU10x objective is active and unmet0/8.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
    report.push_str("\nHistorical primary8.324853ms/RSS134.50MiB; merge54 161.341835ms/RSS1910.1875MiB without a supplied M5 reference. No new AWS/Aeglos, phone/TEE, giant-circuit or minimum-RAM result is claimed.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [new runtime module](artifacts/source/crates/zopro-core/src/groth16/glv_blinding.rs), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [all99 workflow receipts](artifacts/runs/result.json).\n");
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
    let out = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(dst)
        .output()
        .unwrap();
    fs::write(format!("{R}/archive-verification.log"), out.stdout).unwrap();
    assert!(out.status.success());
    save(
        format!("{R}/archive-summary.json"),
        &json!({"hashes":paths.len(),"git_files":paths.len()+1,"GNU_strict_passed":true,"loss_gzip_roundtrips":6,"profile_samples":samples}),
    );
    println!("Archived{}verifiedhashes;gate={gate}", paths.len());
}
fn status(commit: &str) {
    let a = read(format!("{R}/analysis.json"));
    let gate = a["gate_passed"].as_bool().unwrap();
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let next = if gate {
        "Qualify exact GLV SDK on all8/54 and portable/resource/stack/concurrent/250/4999 before any adoption; preserve original primary screen"
    } else {
        "ExactGLVassemblySDKintegrationclosed; preservecomponentgain/fullproofestimates; choose distinct actualworkremoval without unchangedreroll"
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
                "GLVfinalassembly SDK gate={gate};committed/pushed/verified;notadopted"
            )),
        ),
        ("latest_full_sdk_measurement", a.clone()),
        ("previous_completed_experiment", previous),
        ("last_goal_turn_classification", json!("PROGRESS")),
        ("production_changed", json!(false)),
        ("adopted", json!(false)),
        ("independent_proof_count", json!(13181)),
        ("new_independent_exports", json!(99)),
        ("SDK_requests", json!(15396)),
        ("owned_live_jobs", json!([])),
        ("owned_cloud_resources", json!([])),
        ("next_action", json!(next)),
    ] {
        s[k] = v;
    }
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — GLV FINAL ASSEMBLY SDK\n\nHEAD/remoteverified{commit};root{R};report{D}/README.md;readanalysis.json/archive-summary.jsonforgates/counts. FullprimarySDKgate={gate};NOadoption. Accepted172/SDK14da unchanged,historical1x2 8.324853ms/RSS134.50MiB;goalACTIVEunmet0/8. Ledger13082->13181after99freshindependentGnarkproofs198members. Noownedjobs/cloud. ThisgoalturnPROGRESS:distinctGLVassemblycallerpositivepublished d01e3939b, exactnativeintegrationandfullSDKcomparecompleted.\n\nPureprivatecopyaccepted172:170unchangedoriginals,groth16.rsnewcfgmoduleinclude,groth16_rerandomize.rs2guardedreturns,originalfallbackbodiesverbatim. Newgroth16/glv_blinding.rs EXACTpassedparentruntimeprefixexceptparenthelperpaths/inlineacceptedphi_splithelpers. CandidateG1(a-b,-b)components/eig=-1-lambdaG2;G2ordinary(a,b);wNAF5shortchains+smallproof-localimages. Randomness/distribution/fixedtableconstruction/MSM/FFT/solver/field/verification/publicAPIsunchanged. No observer/hugecache/Casm/publicruntimeoption. Addedfixedlocalpayload512BG1/3072BG2NOTstack/RSSbounds.\n\nNativeOFF/ON119core/raw2/cubic6PASS;existingindependentrerandomizedexponentoracleexercisesselectedprepared/unprepared/zero/identity/poisonedB1paths. No newnativeorfieldtests;parent3newArk/decompcasesinheriteddistinctscope. Alltransitiverlib/sofreezeimmediateinindependenttargets;core/fieldflagsfeatures/eightlibrarySDKreceiptschecked.\n\n15blocks32logical/16physicalaccepted/OFF/ON151+3,3preflight4workers1+3,6warmFIFO199Hzprofiles251+3. All99capturesbefore99Gnark/decodes;15396SDKcalls,allinternallyverified,exports164byte/publicbound/freshagainst13082,ledger13181. RawprofilesonePIDeachzeroLost;allpinschecked. Nohelper/build/hash/archive/secondbenchduringcapture. Readanalysis.jsonforALLpairedratios/CIs/medians/CPU/RSSandfailedchecks;parent38.47%assemblyNOTfullproofgain;smallpositiveestimatesnotzero.\n\nNEXT:{next}. Neednewproofseed thisrootprior-proof-sha256.json13082+rootproof-inventory.json99proofsonly=>13181;nevercountpublics/copies/subinventories. Parentcaller33boundary4096fullwidthpergroup+65groupcasesretained,separatefixture-onlyinversecorrectionandinitialtypecompilerfailurearchivedinparent. LatestAWSstale17137261af77. PreserveacceptedFFTengineeringexceptionfalsegates,all8goalunmet. CommitANDpushclosedexperimentsinclnegatives,noquestions/noagents,Rustxtask,nohugetables/Casmrewrite. Olderheadershistorical.\n\n");
    fs::write(hp, format!("{header}{old}")).unwrap();
    println!("State/handoffupdated;goalactive0/8;gate={gate}");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
