use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-clean-fft-qualification");
    let old = "/tmp/zopro-composed-qualification";
    let mut s = fs::read_to_string(Path::new(old).join("close.rs"))
        .unwrap()
        .replace(old, r.to_str().unwrap())
        .replace(
            "2026-10-27-composed-qualification",
            "2026-10-27-clean-fft-qualification",
        )
        .replace("9154", "11660")
        .replace(
            "/tmp/zopro-grouping-module/production-source-manifest.json",
            "/tmp/zopro-composed-adoption/production-source-manifest.json",
        )
        .replace(
            "assert_eq!(files.len(), 170)",
            "assert_eq!(files.len(), 171)",
        );
    let begin = s.find("    let mut md=String::from(").unwrap();
    let end = begin + s[begin..].find("    for c in inputs").unwrap();
    let intro = r##"    let mut md=String::from("# Clean FFT compatibility and memory qualification\n\nThe exact clean FFT candidate passes the declared portable, fallback, small-stack, concurrent-call and constrained-memory cases. The large table below gives the actual Forester250/4999 load, cold/warm proof and memory observations under a95GiB hard limit with swap disabled. This establishes feasibility in these scenarios, not minimum RAM or a matched speed comparison. No source is adopted by this resource study.\n\nThe [complete latency study](../2026-10-27-clean-fft-leaves/README.md) retains its original failed full gate: primary strict-gain uncertainty and an8x2 candidate/fresh nonregression upper bound above1.02. All calibration and aggregate checks pass, with original-eight paired gains0.856%/0.419% against accepted/fresh. This qualification does not change that verdict. Any later adoption requires a separate explicit engineering review. Original all-eight supplied M5 CPU10x goal remains active0/8.\n\n## Sources and validation\n\nNative resource drivers link the exact measured clean14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a and current accepted824164 libraries; no native arithmetic is rebuilt. The legacy resource label composed denotes this clean candidate and its exact receipts. A separate fresh portable x86-64/noIFMA/noASM target uses the identical source and passes field/core/prover suites recorded in analysis.json. Native116fresh/119candidate core plus raw2/cubic6 qualification is inherited from the exact latency-study libraries, not a new execution here.\n\nRust xtask runs frozen cases serially. Two controller checks cover memory-event parsing and a nonfaulting mincore residency probe. Every request, verifier, source/library/input pin and lifecycle receipt remains in the archive. No unrelated owned workload overlaps qualification requests. All171 accepted production files remain unchanged. Kernel and scenario checks do not establish physical-phone/ARM/TEE or universal memory guarantees.\n\n## Functional and concurrent cases\n\nAll37 functional processes pass: nine portable standard shapes; portable raw/preparation-denied primary/54; native1/4workers x512KiB/1MiB requested worker stacks xprimary/54 xbasis/raw/preparation-denied. Every process makes three primes and one fresh internally verified request, then independently verifies its final export. RUST_MIN_STACK requests a stack size; no watermark is measured. Raw/denied cases exercise the existing fallback.\n\nPortable standard-image diagnostics below are one request after three primes at four workers. They are correctness/resource observations without acceptance confidence intervals; RSS includes loading and primes.\n\n|Shape|Observed request ms|Process peak RSS MiB|Independent proof|\n|---|---:|---:|---|\n");
"##;
    s.replace_range(begin..end, intro);
    s = s
        .replace(
            "exact accepted/combined libraries",
            "exact accepted/clean libraries",
        )
        .replace("\"combined\"", "\"clean\"");
    let begin = s
        .find("    md.push_str(\"\\nThese small cases remain")
        .unwrap();
    let end = begin + s[begin..].find("    for row in resources").unwrap();
    let intro = r##"    let high_total:u64=resources["rows"].as_array().unwrap().iter()
        .filter(|r|r["case"]["kind"]!="large").map(|r|r["memory_events"]["high"].as_u64().unwrap()).sum();
    writeln!(md,"\nSmall-case high-limit events total{high_total}; zero means the soft threshold did not trigger reclaim. The actual table and events retain the distinction between successful limit transitions and active pressure. Cgroup peaks include controller, copied image pages, SDK and warm ballast; they differ from standalone RSS and do not establish minimum RAM.\n\n## Forester250 and4999\n\nThe original pinned solver images are unchanged. Before each launch, a bounded streaming hash verifies the image, then fsync/POSIX_FADV_DONTNEED acts only on that owned image. A PROT_NONE mincore probe confirms zero residency without faulting pages. No global cache drop, key conversion, image copy or larger retry occurs. One cold and one warm core-prover call use different original witnesses; proofs and exact public bindings independently verify.\n\n|Batch|Load s|Cold proof s|Warm proof s|Process peak RSS GiB|Cgroup peak GiB|High / max events|OOM / kill|\n|---|---:|---:|---:|---:|---:|---:|---:|").unwrap();
"##;
    s.replace_range(begin..end, intro);
    let begin = s.find("    md.push_str(\"\\nBoth use95GiB").unwrap();
    let end = begin
        + s[begin..]
            .find("    save(\n        r.join(\"closeout.json\")")
            .unwrap();
    let ending = r##"    md.push_str("\nBoth large cases use95GiB hard limits and memory.swap.max=0. Actual memory peak, swap and reclaim/OOM events remain in the table and raw receipts. Max/high event counters are not page counts or removable wall time. These single cold/warm observations are not matched accepted/Aeglos/Gnark timing and do not establish minimum RAM or a large-circuit speed gain. No lower-cap search occurs.\n\n## Counts and scope\n\nAll216SDK calls plus four Forester core calls complete. All85distinct new exports independently verify, satisfy164-byte length/exact public binding/freshness against11575prior identities: ledger11575->11660,170proof/public members. Inventories/copies/reverification are not counted twice. The frozen workflow/finish helper streams image hashes before/after execution; closing rehashes remaining inputs and exports without an extra giant-image pass. Owned cgroup services and runtime overrides are removed; no cloud resource is created.\n\nThe [integration review](../2026-10-27-clean-fft-leaves/artifacts/integration-review.md) describes unchanged allocation geometry. These scenarios complement it without proving a universal allocator peak, physical phone/ARM/TEE behavior,193GiB key feasibility, giant concurrent proofs or minimumRAM. No new lookup table, key copy or duplicate execution plan is introduced. Small image copies belong only to the owned qualification harness.\n\nThe parent full performance gate remains false. A separate explicit engineering decision may weigh reliable aggregate/larger-case benefits against primary and8x2bound uncertainty after these resource results. This report does not adopt code, waive a gate, repeat acceptance timing or claim new AWS/M5/Aeglos performance. The all-eight CPU10x goal remains active0/8.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [jobs](artifacts/jobs.json), [resources](artifacts/resource-result.json), [portable tests](artifacts/portable/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).\n");
"##;
    s.replace_range(begin..end, ending);
    s = s
        .replace(
            "complete combined resource/compatibility qualification",
            "complete clean FFT resource/compatibility qualification",
        )
        .replace(
            "\"production_sources_checked_unchanged\":170",
            "\"production_sources_checked_unchanged\":171",
        )
        .replace(
            "\"portable_field_tests\":11",
            "\"portable_field_tests\":a[\"portable_field_tests\"]",
        )
        .replace(
            "\"portable_core_tests\":99",
            "\"portable_core_tests\":a[\"portable_core_tests\"]",
        )
        .replace(
            "\"portable_prover_tests\":8",
            "\"portable_prover_tests\":a[\"portable_prover_tests\"]",
        );
    fs::write(r.join("close.rs"), s).unwrap();
    println!("Prepared data-driven cleanFFT resource report;ledger11660/171unchanged sources;no stale timing/event constants");
}
