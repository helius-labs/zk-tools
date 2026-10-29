fn n(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn write_readme(a: &Value, m: &Value, profiles: &[Value], d: &Path) {
    let mut cpu = BTreeMap::new();
    for (x, y) in [("OFF", "accepted"), ("ON", "OFF"), ("ON", "accepted")] {
        let xx = a["cells"][format!("1x2-w32/{x}")]["blocks"]
            .as_array()
            .unwrap();
        let yy = a["cells"][format!("1x2-w32/{y}")]["blocks"]
            .as_array()
            .unwrap();
        assert_eq!(xx.len(), 15);
        assert_eq!(yy.len(), 15);
        let mut ratios = vec![];
        let mut saves = vec![];
        for i in 0..15 {
            assert_eq!(xx[i]["block"], i);
            assert_eq!(yy[i]["block"], i);
            ratios.push(n(&xx[i], "CPU_ms") / n(&yy[i], "CPU_ms"));
            saves.push(n(&yy[i], "CPU_ms") - n(&xx[i], "CPU_ms"));
        }
        let wins = ratios.iter().filter(|&&r| r < 1.).count();
        let mut sorted = ratios.clone();
        sorted.sort_by(f64::total_cmp);
        saves.sort_by(f64::total_cmp);
        cpu.insert(format!("{x}/{y}"),json!({"median":sorted[7],"low":sorted[3],"high":sorted[11],"wins":wins,"ratios":ratios,"paired_saving_CPU_ms":saves[7],"postcapture_descriptive_not_selection_gate":true,"nominal_individual_coverage_if_independent_exchangeable":0.96484375}));
    }
    save(
        d.join("descriptive-CPU-pairs.json"),
        &json!({"analysis_sha256":hash(r("analysis.json")),"scope":"postcapture descriptive paired CPU ratios; does not change predeclared latency gate", "pairs":cpu}),
    );
    let mut b=String::from("# Balanced G2 phi windows: SDK pilot\n\n**The latency gain gate fails; no adoption.** Primary ON latency is 8.002155 ms versus 8.032553 ms for fresh OFF and 7.998378 ms for accepted. Paired savings are +13.336 µs against OFF and +7.153 µs against accepted, with 9/15 wins each; both intervals cross parity. OFF/accepted calibration passes. CPU service falls by about 6% despite nearly unchanged single-proof latency. This positive work reduction is retained, without replacing the declared latency gate or claiming a throughput improvement.\n\n## Change\n\nThe current eligible G2 query splits each scalar into two signed components with magnitudes below 2^126. The candidate replaces ten full 12-bit windows and one narrow indexed top with eleven balanced signed spans covering 127 bits: five spans of width 11 and six of width 12. The Booth reader uses the preceding bit as carry and keeps component signs separate. Variable-width Horner recombination reconstructs the same scalar. Every span uses the existing borrowed pending/collision/exception state, high-component bucket endomorphism, two point partitions, bucket merge and weighted reduction.\n\nThe earlier uniform 11-bit/four-partition G2 trial remains rejected; it changed partition count and window count. The failed ordinary G1 balanced-window pilot remains closed. This candidate changes G2 digit boundaries and top reduction, retaining the current decomposition and field kernels. No result from accepted balanced K/H is transferred. [History audit](artifacts/history-audit.md).\n\nThe new per-partition sum of bucket capacities is 17,408, versus 20,480 ordinary slots plus the old narrow top. Capacity is not a count of executed additions: collisions, inversions and replacing the indexed top can change costs. The existing forecast already charges eleven full 2,048-bucket parents per two partitions, the maximum 200-entry batch and extra top scratch. New heap geometry fits that envelope; eleven spans add 176 bytes of query metadata on this 64-bit host, with compiler temporaries additional. There is no key table, point copy, new scalar/digit buffer, pool, unsafe code, dependency or retained plan. This is not minimum-RAM or memory-pressure qualification.\n\nTwo cfg-only parent edits and one child make a 176-file private snapshot: 173 originals are byte-exact, and reversing the edits recovers all 175 accepted files. K/H, A, FFT, solver, scheduler, workspace forecast and all 17 field files remain exact. OFF is byte-identical to accepted SDK `20fde19f…`; ON is `d343f8e1…`. The linked audit checks actual cfg/features, immutable libraries, compiled test dispatch and ON-only balanced text contexts. Defining and generic symbol contexts remain distinguished from exclusive profile attribution. [Source audit](artifacts/source-audit.json), [linked audit](artifacts/code-audit.json).\n\n## Complete verified requests\n\nRyzen 9950X, 16 physical / 32 logical CPUs, 32 workers pinned to CPUs 0–31. All nine shapes × three variants pass native preflight. Fifteen fixed rotated/reversed primary blocks each measure 151 complete SDK requests after three primes. Loading is excluded; solving, fresh randomized proving, SDK verification and serialization are included. Displayed medians need not reproduce paired estimates.\n\n|Variant|Median ms|CPU ms/request|Median/max RSS MiB|M5 CPU speedup|M5 Metal speedup|\n|---|---:|---:|---:|---:|---:|\n");
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
    b+="\nRSS is process high-water including loading/primes, not required RAM. The modest observed RSS differences do not establish low-memory behavior.\n\n|Latency comparison|Paired ratio|Interval|Wins / 15|Paired saving µs|\n|---|---:|---:|---:|---:|\n";
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
    b+="\nThe predeclared gate requires ON/control upper ratios below one with at least 10/15 wins against both controls, and OFF/accepted intervals within [1/1.03,1.03]. Sorted ranks 4–12 of 15 have nominal 96.484375% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. Both gain checks fail. There is no minimum positive-gain floor, exclusion, extension, successful rerun, width/partition sweep or subset promotion. [Fixed design](artifacts/design.md), [all observations](artifacts/analysis.json).\n\n## CPU work and diagnostics\n\nThe following paired CPU comparisons were computed after capture as descriptive evidence. They are not additional selection gates and do not rescue the failed latency result. CPU time sums process user and system service per measured request.\n\n|CPU comparison|Paired ratio|Rank interval|Wins / 15|Paired CPU saving ms|\n|---|---:|---:|---:|---:|\n";
    for (name, p) in &cpu {
        writeln!(
            b,
            "|{name}|{:.6}|{:.6}–{:.6}|{}|{:+.3}|",
            n(p, "median"),
            n(p, "low"),
            n(p, "high"),
            p["wins"],
            n(p, "paired_saving_CPU_ms")
        )
        .unwrap();
    }
    b+="\nThe lower CPU service is consistent with less aggregate work. It does not isolate a field, bucket, scheduling or cache cause. The prior accepted-source service trace placed both G2 and K/H on the finishing path, but this candidate has no fresh per-query critical-path trace; unchanged wall time alone cannot prove which query limits it. No throughput, energy or AWS scaling benefit is measured. [Descriptive CPU observations](artifacts/descriptive-CPU-pairs.json).\n\nThree FIFO-gated four-event stat runs measure 31 requests each, and three separate 199 Hz user-cycle profiles measure 251 requests each, after three excluded primes. All 45 latency captures precede their Gnark verifications. All six diagnostics precede their verification/decoding; loading/primes are outside their gates. Every stat event runs 100%; profiles have one process each and zero lost records. No owned build/helper/hash/archive/other workload overlaps capture; the shared host is not claimed exclusive.\n\n|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|\n|---|---:|---:|---:|---:|\n";
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
    b += "\n|Profile|Samples|Defining borrowed G2 apply sampled period|\n|---|---:|---:|\n";
    for p in profiles {
        writeln!(
            b,
            "|{}|{}|{:.2}%|",
            p["name"].as_str().unwrap(),
            p["samples"],
            100. * n(p, "defining_g2_apply_period_fraction")
        )
        .unwrap();
    }
    b+="\nRaw/demangled symbols and original decoded streams are retained. Defining-path shares exclude generic-context name matches and do not include every inlined/callee operation. Single stat/profile runs are descriptive, not exclusive costs, causal stalls or removable wall time. Raw perf files remain locally with hashes.\n\n## Correctness and archive\n\nBoth fresh native builds pass all 127 core and 8 prover tests on their first executions. Three new core tests check 196,988 independent digit checks and 17,908 signed-component reconstructions; 48 known-generator query cases across 1/4 workers, 1/2 partitions and lengths 0/1/17/1,024/4,097/30,089; and compiled OFF=0/ON=1 dispatch. Fixtures include full-width and adversarial scalars, identity, repeated and opposite points. Unchanged field files inherit 33 field-test results; no field suite is rerun.\n\nAll 7,902 complete SDK requests verify internally. All 78 fresh 164-byte exports independently pass pinned Gnark, exact image/public binding and uniqueness against 18,592 prior proof identities. Ledger 18,670; 156 proof/public members archived. Ten native and 165 capture steps complete; the separate linked audit is a directly observed exit-zero invocation with `audit.log`, not a fabricated xtask receipt. There are no failed builds/tests/captures or successful repeats in this experiment. The archive checks 175 receipts / 700 original streams, source recovery, proof/public hashes, compression round trips and closing pins. [Closeout](artifacts/closeout.json).\n\n## Inherited all-eight CPU and Metal comparison\n\nThese accepted-source measurements use supplied historical M5 references. Only the primary row above is newly timed in this pilot; native all-nine preflights do not qualify other-shape latency.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n";
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
    b+="\nAccepted primary remains 8.022165 ms / 134 MiB maximum RSS; 54→1 remains 162.250464 ms without a supplied M5 reference. The accepted full performance gate remains false with its recorded physical16 calibration engineering exception; final artifact gate remains true. No candidate physical16, portable/mobile/TEE, pressure/swap/minimum-RAM, concurrent-proof, 250/4999, AWS or Aeglos qualification is claimed. Production source175/SDK20fde is unchanged.\n\nThe exact balanced-phi layout is closed by this failed latency pilot. Its positive CPU-work and small uncertain latency estimates remain evidence. Any subsequent complementary algorithm needs its own declared controls and complete-proof test; no adoption or unchanged rerun follows here. The all-eight CPU 10× goal remains active and unmet (0/8).\n";
    fs::write(Path::new(D).join("README.md"), b).unwrap();
}
