fn publish() {
    let root = Path::new(R);
    let dest = Path::new(D);
    assert!(!dest.exists());
    let a = read(r("analysis.json"));
    assert_eq!(a["complete"], true);
    assert_eq!(a["independent_proof_count"], 24943);
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let art = dest.join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let names = [
        "capture.rs",
        "analysis.rs",
        "sampling.rs",
        "counter.rs",
        "publication.rs",
        "publish.rs",
        "summary-generator.rs",
        "prepare-capture.rs",
        "final-review.rs",
        "design.md",
        "history-audit.md",
        "analysis-diagnostics.md",
        "source-audit.json",
        "final-source-review.json",
        "pins.json",
        "plan.json",
        "build-pins.json",
        "build-plan.json",
        "identities.json",
        "no-sdk-probes.json",
        "jobs.json",
        "cases.json",
        "analysis.json",
        "rows.json",
        "proof-inventory.json",
        "proof-sha256-after.json",
        "frozen-control.json",
        "frozen-candidate.json",
        "pre-capture-review.json",
    ];
    for name in names {
        fs::copy(r(name), art.join(name)).unwrap();
    }
    let mut source = files(&r("source"));
    assert_eq!(source.len(), 176);
    source.extend(
        names
            .iter()
            .filter(|s| s.ends_with(".rs") || s.ends_with(".md"))
            .map(r),
    );
    let src = archive(&source, root, &art.join("source.json.gz"));
    let mut receipts = files(&r("native"));
    receipts.extend(files(&r("runs")));
    receipts.retain(|p| {
        matches!(
            p.extension().and_then(|s| s.to_str()),
            Some("json") | Some("log")
        )
    });
    let receipt = archive(&receipts, root, &art.join("receipts.json.gz"));
    let proof = archive(&files(&r("proofs")), root, &art.join("proofs.json.gz"));
    assert_eq!(proof["members"], 1944);
    let symbols = archive(
        &[r("profile-symbols.json")],
        root,
        &art.join("profile-symbols.json.gz"),
    );
    let inherited = archive(
        &files(&r("inherited-summary")),
        root,
        &art.join("inherited-summary.json.gz"),
    );
    let raw = files(&r("perf"))
        .into_iter()
        .chain(files(&r("stat")))
        .map(|p| (p.display().to_string(), hash(&p)))
        .collect::<BTreeMap<_, _>>();
    save(art.join("local-perf-pins.json"), &json!(raw));
    save(
        art.join("publication-review.json"),
        &json!({"source":src,"receipts":receipt,"proofs":proof,"symbols":symbols,"inherited_summary":inherited,"all_original_helpers_unchanged_after_capture":true,"production_changed":false,"adopted":false,"ledger":24943}),
    );
    let accepted = read(r("inherited-summary/data.json"));
    let output = Command::new("jq")
        .arg(".rows")
        .arg(r("inherited-summary/data.json"))
        .output()
        .unwrap();
    assert!(output.status.success());
    fs::write(dest.join("accepted-rows.json"), output.stdout).unwrap();
    let mut md=String::from("# Full SDK qualification of balanced G2 windows\n\n30 October2026. Complete verified requests across all nine circuits at32workers plus primary at16workers. The candidate follows a passed complete G2-query gate; its query gain is not assumed to transfer to proving. Production remains unchanged.\n\n");
    md+=&format!("Declared full-request gate **{}**; diagnostic qualification **{}**. Failed checks: `{}`.\n\n",a["gate_passed"],a["diagnostics_qualified"],a["failed_checks"]);
    md+="## Implementation and qualification\n\nG2Phi2 uses eleven variable-width spans covering127bits: `[11,12,11,12,11,12,11,12,11,12,12]`, starts `[0,11,23,34,46,57,69,80,92,103,115]`. All use the existing ordinary borrowed Window, high component then phi then low, with original early-stopping LIFO queue and152/200entry batches. This replaces the narrow indexed top's routing, point-tile copies and reduction layers. The exact accepted K/H offset Booth reader avoids a digit array. Field arithmetic, scalar split, key operands and two-part geometry remain. No previous failed queue-scan, queue-drain, GLS4 or arithmetic candidate is mixed in.\n\nThe earlier count model reports15.26%fewer bucket output slots per part, but more ordinary affine/projective work; it does not model all original indexed-top arithmetic or inversion costs. In the complete-query trial primary32 improves3.110% and the two-shape32aggregate3.467% versus the API-only frozen baseline, both31/31wins. Primary16query latency is inconclusive despite lower CPU service. These are query results, not proof results. [Count screen](../2026-10-30-g2-balanced-count175/README.md), [complete-query trial](../2026-10-30-g2-balanced175/README.md).\n\nSDK integration retains the exact qualified balanced child and two parent cfg/module changes, restoring the original lib.rs and removing the unused private query-probe flag/export. Native source176 has173exact originals; inverse edits recover all175. All17field files and workspace forecasts remain exact. Existing forecasts already charge eleven maximum-sized parents plus indexed-top extras; each new parent/leaf fits that envelope, without a new scalar vector, point/key table, duplicate plan, pool or retained state. Span descriptors request176stackbytes; compiler frames, minimum RAM and pressure behavior are not proven. Unsupported sizes/targets and disabled borrowed-prefix paths retain their original route.\n\nFresh nativeOFF/ON each pass127unfiltered core and8prover tests; portableON passes101core and8prover tests on generic x86-64/no_ifma. The three actual-curve tests retain whole sums, empty/full/tails, identities/doubling/opposites/sparse values, input point immutability and1/4/16/32custom pools. All33accepted field tests are inherited after exact source checks, without new field arithmetic or redundant execution. Original SDK driver and its perf helper are unchanged. [Source review](artifacts/final-source-review.json), [design](artifacts/design.md).\n\n## Fixed measurement scope\n\nRyzen9 9950X,16physical cores/32logical CPUs,128GiB RAM, shared host. Nine circuits at32workers onCPUs0–31, primary also at16on0–15. Accepted is the retained exact20fde production executable; OFF/ON are coherent fresh SDK builds. Thirty-one fixed rotated/reversed blocks retain all10cases and all3variants. Each process primes three times, then times151primary or31other requests. Timers include fresh witness solve, randomized proving, SDK verification and serialization. Loading/preparation are outside latency and inside lifetime process RSS. Reported latency is the median of31process medians; CPU is median summed service/request, RSS the largest process high-water across blocks, not minimum RAM or incremental workspace.\n\nThirty preflight processes independently pass Gnark before latency. All930latency processes precede six grouped PMU panels and six fixed-period profiles, primary16/32across all3variants. All captures precede the remaining942independent proof verifications, ownership transfer, decode and analysis. Whole-process taskset includes the coordinator. Every helper, including this publisher/summary/source reviewer, was compiled and pinned before capture. No owned build/helper/hash/archive/analyzer or independent CPU workload overlaps captures.\n\n|Case|Variant|Median ms|CPU ms/request|Peak RSS MiB|\n|---|---|---:|---:|---:|\n";
    for (k, c) in a["cells"].as_object().unwrap() {
        let (case, v) = k.rsplit_once('/').unwrap();
        md += &format!(
            "|{case}|{v}|{:.6}|{:.3}|{:.2}|\n",
            c["median_ms"].as_f64().unwrap(),
            c["CPU_ms"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap()
        );
    }
    md += "\n|Paired comparison|Ratio|Bounds|Wins /31|Pass|\n|---|---:|---|---:|---|\n";
    for (k, p) in a["pairs"].as_object().unwrap() {
        let s = &p["statistics"];
        md += &format!(
            "|{k}|{:.9}|[{:.9}, {:.9}]|{}|{}|\n",
            s["median"].as_f64().unwrap(),
            s["low"].as_f64().unwrap(),
            s["high"].as_f64().unwrap(),
            s["wins"],
            p["passed"]
        );
    }
    md +=
        "\n|Original-eight aggregate versus|Ratio|Bounds|Wins /31|Pass|\n|---|---:|---|---:|---|\n";
    for (k, p) in a["original8_aggregates"].as_object().unwrap() {
        let s = &p["statistics"];
        md += &format!(
            "|{k}|{:.9}|[{:.9}, {:.9}]|{}|{}|\n",
            s["median"].as_f64().unwrap(),
            s["low"].as_f64().unwrap(),
            s["high"].as_f64().unwrap(),
            s["wins"],
            p["passed"]
        );
    }
    md+="\nPrimary32and equal-log original-eight aggregate require upper ON/control ratios below1 and≥21/31wins versus BOTHaccepted/OFF. Every other case, including16and54→1, requires upper≤1.02. All ten OFF/accepted calibration intervals must fit factor1.03; all ten maximum RSS ON/control ratios must be≤1.05versus both controls. This relative guard is not an architecture cap. Sorted ranks10–22give nominal97.055%individual median coverage under independent/exchangeable blocks, without a familywise guarantee. Ratios pair blocks and need not equal ratios of displayed medians. No minimum gain floor, exclusions, extensions, changed thresholds, favorable subsets or unchanged reroll. Diagnostic qualifications also remain required. Passing performance selects separate resource/admission and final-artifact checks, not adoption or objective completion. The accepted build's historical physical16calibration exception is not inherited.\n\n## Automatic counters and profiles\n\nEach grouped PMU panel has31requests after3primes and requires≥99.9%event running coverage. Profiles use251requests after3primes with fixed16million cycles:u period, explicit CPU/time/period and executable/task metadata. FIFO gates exclude setup/primes and final exports/checking. OnePID, valid affinity/mappings/events, zero loss,≥1000samples and≤5%unknown period are required. These single panels are diagnostic, not repeated-trial confidence intervals or isolated causes. The known absolute-period coverage deficit remains unresolved: flat shares are qualitative, not exclusive costs, local CPI or removable proof wall time.\n\n|Panel|Instructions/request M|Cycles/request M|Qualified|\n|---|---:|---:|---|\n";
    for (k, c) in a["counters"].as_object().unwrap() {
        let n = |s: &str| {
            c["per_request"][s]
                .as_f64()
                .map(|v| format!("{:.3}", v / 1e6))
                .unwrap_or_else(|| "unavailable".into())
        };
        md += &format!(
            "|{k}|{}|{}|{}|\n",
            n("instructions:u"),
            n("cycles:u"),
            c["qualified"]
        );
    }
    md +=
        "\n|Profile|Samples|Balanced-namespace period|Qualified|Lost|\n|---|---:|---:|---|---:|\n";
    for (k, p) in a["profiles"].as_object().unwrap() {
        md += &format!(
            "|{k}|{}|{}|{}|0|\n",
            p["samples"], p["new_balanced_context_period"], p["attribution_qualified"]
        );
    }
    md+="\n## CPU and Metal reference comparisons\n\nM5 timings are supplied historical references, not new matched-hardware Apple measurements. ON remains experimental. There is no M5reference for54→1.\n\n|Circuit|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× achieved|\n|---|---:|---:|---:|---:|---:|---|\n";
    for (k, c) in a["new_reference_comparisons"].as_object().unwrap() {
        let v = &c["variants"]["ON"];
        md += &format!(
            "|{k}|{:.6}|{}|{:.3}×|{}|{:.3}×|{}|\n",
            v["ms"].as_f64().unwrap(),
            c["reference"]["cpu_ms"],
            v["CPU_speedup"].as_f64().unwrap(),
            c["reference"]["metal_ms"],
            v["Metal_speedup"].as_f64().unwrap(),
            v["CPU10x"]
        );
    }
    md+="\n## Verification and retained records\n\nAll1974steps complete. All55,788SDKrequests verify internally, including primes. Each process exports its last timed proof after the clock/FIFO gate. All972fresh164-byte exports independently pass Gnark with public/image bindings, common per-shape verification-key identities and uniqueness against23,971prior proof hashes. The persisted union is24,943; proof/public archives retain1944members. Repeated verification is not counted as fresh proof output. Source/receipt/proof/symbol archives round-trip byte-exactly; original source/helper/binary/input pins close. Raw perf and ELF files stay local with hashes.\n\nThe accepted original nine local/AWS rows remain in [accepted-rows.json](accepted-rows.json), copied without decimal reserialization from the pre-capture summary. Accepted local primary remains8.022165ms/134MiB and54→1remains162.250464ms/1831.87MiB. Latest AWSprimary remains Zopro11.915591ms at32workers versus pinned AeglosSorted7.959184ms at96; no new AWSmeasurement. These local results do not establish phone/ARM/TEE, forester250/4999, memory pressure/swap, minimum RAM or concurrent proof throughput.\n\n[Analysis](artifacts/analysis.json), [observations](artifacts/rows.json), [source review](artifacts/final-source-review.json), [design](artifacts/design.md), [diagnostic policy](artifacts/analysis-diagnostics.md), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [sources](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proof bytes](artifacts/proofs.json.gz), [proof inventory](artifacts/proof-inventory.json), [new union ledger](artifacts/proof-sha256-after.json), [symbols](artifacts/profile-symbols.json.gz), [inherited summary](artifacts/inherited-summary.json.gz), [archive checks](artifacts/publication-review.json).\n";
    fs::write(dest.join("README.md"), md).unwrap();
    let sums = files(dest)
        .iter()
        .map(|p| format!("{}  {}\n", hash(p), p.strip_prefix(dest).unwrap().display()))
        .collect::<String>();
    fs::write(dest.join("SHA256SUMS"), sums).unwrap();
    println!("PublishedbalancedG2SDK31blocksall9+primary16;ledger24943,productionunchanged");
}
