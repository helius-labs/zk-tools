fn n(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn archive_training(d: &Path) {
    let mut files = BTreeMap::new();
    let mut paths: Vec<_> = fs::read_dir(r("raw"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(paths.len(), 9);
    paths.extend([
        r("merged.profdata"),
        r("profile-functions.json"),
        r("profile-counts.log"),
        r("profile-merge.stdout.log"),
        r("profile-merge.stderr.log"),
        r("profile-show.stderr.log"),
        r("generate-symbols.txt"),
    ]);
    for p in paths {
        files.insert(
            p.strip_prefix(R).unwrap().display().to_string(),
            encoded(&fs::read(&p).unwrap()),
        );
    }
    compressed(d, "PGO-profile-data", &json!(files));
    let t = read(r("training-audit.json"));
    assert_eq!(t["SDK_prove_count"], 99);
    assert_eq!(t["SDK_load_count"], 0);
    assert_eq!(t["SDK_constructor_count"], 0);
    assert_eq!(t["proof_ledger"], 18769);
}
fn write_readme(a: &Value, m: &Value, profiles: &[Value], d: &Path) {
    let ids = read(r("identities.json"));
    let mut candidate = m.clone();
    candidate["adopted"] = json!(false);
    candidate["production_changed"] = json!(false);
    candidate["candidate_only"] = json!(true);
    candidate["native_binary"] = ids["ON"]["binary"].clone();
    candidate["native_binary_sha256"] = ids["ON"]["sha256"].clone();
    candidate["rustflags"] = json!(fs::read_to_string(r("candidate-flags.txt")).unwrap());
    candidate["native_build_source_pins"] = json!(r("use-pins.json"));
    candidate["native_workflow_result"] = json!(r("native-use/result.json"));
    candidate["primary_latency_pilot_gate_passed"] = json!(true);
    candidate["full_performance_gate_passed"] = json!(false);
    candidate["full_candidate_qualification_pending"] = json!(true);
    candidate["final_artifact_gate_passed"] = json!(false);
    candidate["proof_ledger"] = json!(18847);
    candidate["profile"] = json!({"path":r("merged.profdata"),"sha256":hash(r("merged.profdata")),"source_files_exact":175,"SDK_prove_count":99,"SDK_load_count":0,"SDK_constructor_count":0,"equal_weight_raw_profiles":9});
    candidate["ms"] = json!({"1x2":a["cells"]["1x2-w32/ON"]["median_ms"]});
    candidate["m5"] = json!({"1x2":a["new_reference_comparisons"]["1x2-w32"]["variants"]["ON"]});
    candidate["inherited_production_full_performance_failures"] =
        m["performance_failures_retained"].clone();
    candidate["candidate_performance_failures"] = a["failed_checks"].clone();
    candidate["measured_primary_peak_RSS_MiB"] = a["cells"]["1x2-w32/ON"]["max_RSS_MiB"].clone();
    candidate["qualification_note"]=json!("primary pilot only; all9nativepreflight correctness; no all8latency/resource/compatibility/adoption qualification");
    let allowed = [
        "adopted",
        "production_changed",
        "candidate_only",
        "native_binary",
        "native_binary_sha256",
        "native_binary_interface",
        "rustflags",
        "environment",
        "source_files_sha256",
        "native_build_source_pins",
        "native_workflow_result",
        "primary_latency_pilot_gate_passed",
        "full_candidate_qualification_pending",
        "proof_ledger",
        "profile",
        "ms",
        "m5",
        "inherited_production_full_performance_failures",
        "candidate_performance_failures",
        "measured_primary_peak_RSS_MiB",
        "qualification_note",
    ];
    candidate
        .as_object_mut()
        .unwrap()
        .retain(|key, _| allowed.contains(&key.as_str()));
    candidate["full_performance_gate_passed"] = Value::Null;
    candidate["final_artifact_gate_passed"] = Value::Null;
    candidate["qualification_status"] = json!({"primary_latency_pilot":"passed","all_shape_latency":"pending","physical16":"pending","compatibility":"pending","resources":"pending","final_artifact":"pending","adoption":"pending"});
    candidate["native_tests_fresh_each"] = json!({"field":33,"core":124,"prover":8});
    candidate["frozen_USE_libraries"] =
        json!({"path":r("frozen-candidate.json"),"sha256":hash(r("frozen-candidate.json"))});
    save(d.join("candidate-manifest.json"), &candidate);
    let mut b=String::from("# Current-source proof-only PGO: SDK pilot\n\n**The primary latency pilot passes; broader qualification is required before adoption.** Candidate latency is 7.924730 ms versus 7.988771 ms accepted and 8.032753 ms fresh OFF. Paired improvements are 0.828% against accepted and 1.320% against OFF, with savings of 66.155 µs and 104.678 µs. The candidate wins 13/15 and 15/15 blocks; both gain bounds and control calibration pass. This is a small verified primary gain, not a 2× breakthrough or all-eight qualification. Production remains unchanged.\n\n## Compiler change and controls\n\nAll 175 prover source files are byte-exact to accepted production. This candidate uses one fresh proof-only profile to guide the current compiler; there are no new arithmetic algorithms, key tables, point copies, plans, memory caps or runtime tuning. The previous 22 October PGO study used older source with a 13.6 ms baseline, two preparation profiles and primary weighting; its positive estimates and failed old gate remain recorded. No old profile or rejected source candidate is reused.\n\nRust 1.98.1 / LLVM 22.1.8 generate and consume the profile. Explicit Cargo target keeps host build scripts outside target instrumentation. GEN uses atomic counter updates; its instrumentation can perturb scheduling and is not claimed negligible. USE has no profiling runtime symbols/sections or instrumentation buffers. Separate target directories and immutable hash-named libraries prevent accidental mixing. Accepted SDK is `20fde19f…`, fresh plain `2e3634f0…`, USE `144621f1…`; plain differs from accepted and must pass calibration. The new link passes the baseline cfg flags explicitly; this difference is recorded without assigning it an exclusive timing cause.\n\nThe workflow follows the [rustc PGO guide](https://doc.rust-lang.org/rustc/profile-guided-optimization.html) and [LLVM profile tools](https://llvm.org/docs/CommandGuide/llvm-profdata.html). The [LLVM 22 dump implementation](https://github.com/llvm/llvm-project/blob/release/22.x/compiler-rt/lib/profile/InstrProfilingFile.c) explains the one-dump behavior used to stop collection before exports. Upstream is a reference, not asserted byte-identical to the installed runtime. Actual tool hashes, profiles and counter audits are retained.\n\n## Fixed training and provenance\n\nOne process per supplied shape, including 54→1, uses 32 workers on CPUs 0–31. Mmap loading, SDK construction and three complete priming requests precede counter reset. Eleven fresh SDK requests follow reset, then one profile dump occurs before proof/public exports and log formatting. All nine captures finish before 99 independent Gnark verifications and merging. Nine disjoint raw profiles are combined with equal weights, without a primary weight, cap variant, profile choice or training extension. Training times are not speed evidence.\n\nThe 126 SDK requests verify internally, including 27 excluded primes. All 99 recorded 164-byte proof exports independently pass Gnark and exact image/public binding and freshness. Ledger 18,670→18,769. The merged profile contains 6,229 function records and audits exactly 99 SDK proving calls, zero key loads and zero SDK constructions. The count checks use the selected records' maximum block counts, cross-checked with the fixed request/export counts; generic loop maxima are not treated as function-call counts. Nine raw profiles and the 2.5 MiB merged profile are retained byte-exact.\n\nTwo parser tests pass: one reconstructs the prior real IR-counter format and one checks zero/full-u64 counts. The old counter log is only a parser fixture, not a training input. Current-source native plain and USE each freshly pass all 33 field, 124 core and 8 prover tests. Existing arithmetic inputs are reused across these distinct compiler builds; no new mathematical-case count is claimed.\n\nThe linked audit retains nonzero training records for the hot G1 stream, borrowed G2 apply, balanced K/H feed and SDK proving function. No matching hot missing-profile or mismatch warning appears in the normal consumer library/link logs. All missing-function warnings, including unseen test and driver functions, remain archived. This warning/symbol check is not full optimization-remark coverage. PGO can inline away a standalone definition; symbol absence alone is not proof that work disappeared. [Training audit](artifacts/training-audit.json), [linked audit and warnings](artifacts/code-audit.json), [source audit](artifacts/source-audit.json).\n\n## Complete verified latency pilot\n\nRyzen 9950X, 16 physical / 32 logical CPUs; 32 workers pinned to CPUs 0–31. All nine shapes × three variants pass native preflight. Fifteen fixed rotated/reversed primary blocks each measure 151 complete SDK requests after three primes. Loading is excluded; witness solving, fresh randomized proving, SDK verification and serialization are included. Medians below need not reproduce within-block paired estimates.\n\n|Variant|Median ms|CPU ms/request|Median/max RSS MiB|M5 CPU speedup|M5 Metal speedup|\n|---|---:|---:|---:|---:|---:|\n");
    for v in ["accepted", "OFF", "ON"] {
        let c = &a["cells"][format!("1x2-w32/{v}")];
        let t = n(c, "median_ms");
        writeln!(
            b,
            "|{v}|{t:.6}|{:.3}|{:.2}/{:.2}|{:.3}×|{:.3}×|",
            n(c, "CPU_ms"),
            n(c, "median_RSS_MiB"),
            n(c, "max_RSS_MiB"),
            37. / t,
            24. / t
        )
        .unwrap();
    }
    b+="\nRSS is process high-water including loading/primes, not minimum required RAM. Smaller observed peaks do not establish a memory-pressure or mobile improvement.\n\n|Paired latency comparison|Ratio|Rank interval|Wins / 15|Saving µs|\n|---|---:|---:|---:|---:|\n";
    for (name, p) in a["pairs"].as_object().unwrap() {
        writeln!(
            b,
            "|{name}|{:.9}|{:.9}–{:.9}|{}|{:+.3}|",
            n(p, "median"),
            n(p, "low"),
            n(p, "high"),
            p["wins"],
            1000. * n(p, "paired_saving_ms")
        )
        .unwrap();
    }
    b+="\nThe predeclared pilot requires both ON/control upper ratios below one with at least 10/15 wins, and OFF/accepted intervals inside [1/1.03,1.03]. Every condition passes. Ranks 4–12 of 15 have nominal 96.484375% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. There is no minimum positive-gain floor, exclusion, extension, successful rerun, profile choice or subset promotion. Passing permits the next qualification stage; it does not establish all-eight performance, compatibility, memory-pressure behavior, adoption or the 10× objective. [Design](artifacts/design.md), [all observations](artifacts/analysis.json), [candidate inputs for qualification](artifacts/candidate-manifest.json).\n\n## CPU diagnostics\n\nAll 45 latency captures precede their Gnark verifications. Three FIFO-gated four-event stat runs measure 31 requests each, and three separate 199 Hz user-cycle profiles measure 251 each, after three excluded primes. All six diagnostics finish before verification/decoding. Events run at least 99.9%; each profile has one sampled process and zero lost records. No owned build/helper/hash/archive/other CPU work overlaps training or performance captures. The host is shared; exclusivity is not claimed.\n\n|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|\n|---|---:|---:|---:|---:|\n";
    for p in a["diagnostics"]["counters"].as_object().unwrap().values() {
        let e = &p["events"];
        writeln!(
            b,
            "|{}|{:.3}|{:.3}|{:.3}|{:.3}|",
            p["variant"].as_str().unwrap(),
            n(&e["cycles:u"], "per_request") / 1e6,
            n(&e["instructions:u"], "per_request") / 1e6,
            n(&e["cache-misses:u"], "per_request") / 1e6,
            n(&e["branch-misses:u"], "per_request") / 1e6
        )
        .unwrap();
    }
    b += "\n|Profile|Samples|Lost|\n|---|---:|---:|\n";
    for p in profiles {
        writeln!(b, "|{}|{}|0|", p["name"].as_str().unwrap(), p["samples"]).unwrap();
    }
    b+="\nOriginal decoded streams and raw/demangled symbol weights remain available; raw perf data stay local with hashes. Static code, single counter runs and sampled periods do not isolate a register, cache, scheduling or inlining cause, nor removable wall time. Inlined work can move to enclosing symbols.\n\n## Verification and preparation correction\n\nThe performance pilot completes 7,902 internally verified SDK requests and 78 fresh independent Gnark exports, including all-nine preflight and diagnostics. Proof/image/public bindings, 164-byte sizes and uniqueness against 18,769 prior identities pass; ledger is 18,847. Across training and performance: 8,028 SDK requests, 177 new independent proofs and 354 proof/public members.\n\nPhase A completes 9 native steps, training 109, USE qualification 6 and performance 165. The archive retains 289 receipts / 1,156 original streams, all 175 sources, all proof/public members, raw/merged PGO data and counters, with checked hashes and gzip round trips. These are separate workflows, not one native receipt. The linked audit ran directly with observed exit zero and retained `audit.log`. [Closeout](artifacts/closeout.json).\n\nThe first performance preparation stopped before creating identities or a run plan because it expected the old `native-flags.txt` filename. The controller was corrected to read `control-flags.txt`, then preparation passed before the first capture. The generator produces the preserved initial controller; the one-filename repair record reconstructs the final version. Initial sources, original helper hash and observed direct failure are retained. No source arithmetic, compiler flags, profile, SDK, measurement gate, successful build/test/proof or timing capture was changed or repeated. [Failure](artifacts/initial-capture-controller/failure.json), [repair](artifacts/initial-capture-controller/repair.json).\n\n## Inherited all-eight CPU and Metal comparison\n\nThese accepted-production times use the supplied historical M5 references. Only primary latency above is newly measured for the candidate; all-nine native preflights qualify correctness, not other-shape latency.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n";
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
        let row = &m["m5"][shape];
        let t = n(row, "accepted_ms");
        let c = n(row, "cpu_reference_ms");
        let g = n(row, "metal_reference_ms");
        writeln!(
            b,
            "|{shape}|{t:.6}|{c:.0}|{:.3}×|{g:.0}|{:.3}×|{:.1}|",
            c / t,
            g / t,
            c / 10.
        )
        .unwrap();
    }
    b+="\nAccepted primary remains 8.022165 ms / 134 MiB maximum RSS; 54→1 remains 162.250464 ms without a supplied M5 reference. Its full performance gate remains false with the recorded physical16 calibration engineering exception; final artifact gate remains true. No candidate physical16, portable/mobile/TEE, pressure/swap/minimum-RAM, concurrency, 250/4999, AWS or Aeglos qualification is claimed.\n\nNext is all-eight/54 latency and physical16 qualification, followed by required compatibility/resource/final-artifact checks if those pass. There is no production adoption yet. The all-eight CPU 10× goal remains active and unmet; this candidate primary is about 4.67× the supplied M5 CPU reference and still above its 3.7 ms target.\n";
    fs::write(Path::new(D).join("README.md"), b).unwrap();
}
