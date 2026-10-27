use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-parallel-scalar-groups");
    let mut s = fs::read_to_string("/tmp/zopro-g2-scalar-grain/close.rs")
        .unwrap()
        .replace("/tmp/zopro-g2-scalar-grain", r.to_str().unwrap())
        .replace(
            "2026-10-27-g2-scalar-grain",
            "2026-10-27-parallel-scalar-groups",
        )
        .replace("10645", "11110");
    s=s.replace("&& s != \"candidate/crates/zopro-core/src/msm_affine/g2_phi.rs\"","&& s != \"candidate/crates/zopro-core/src/msm_affine/grouping.rs\"\n                && s != \"candidate/crates/zopro-core/src/msm_affine/grouping/parallel.rs\"");
    let start = s.find("        if p == changed {").unwrap();
    let end = s[start..].find("        } else {").unwrap() + start;
    s.replace_range(start..end,r#"        if p == changed {
            let new = fs::read_to_string(candidate).unwrap();
            let recovered = new.replace(edits["after"].as_str().unwrap(), edits["before"].as_str().unwrap())
                .replace(edits["module_after"].as_str().unwrap(), edits["module_before"].as_str().unwrap());
            assert_eq!(recovered, fs::read_to_string(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)).unwrap());
"#);
    let before = "    for mode in [\"fresh\", \"candidate\"] {\n        assert_eq!(";
    let after="    let added = edits[\"added_file\"].as_str().unwrap();\n    assert_eq!(hash(r.join(\"candidate\").join(added)), edits[\"added_file_sha256\"].as_str().unwrap());\n    for mode in [\"fresh\", \"candidate\"] {\n        assert_eq!(";
    assert_eq!(s.matches(before).count(), 1);
    s = s.replace(before, after);
    let testbefore = "                .len(),\n            116\n        );";
    let testafter="                .len(),\n            if mode == \"candidate\" { 119 } else { 116 }\n        );";
    assert_eq!(s.matches(testbefore).count(), 1);
    s = s.replace(testbefore, testafter);
    s = s.replace(
        "# G2 scalar decomposition task boundary",
        "# Parallel exact scalar grouping",
    );
    let before="Only the existing in-place `phi_split` parallel iterator adds `with_min_len(128)`. It applies the accepted canonical-conversion task boundary to this separate decomposition stage. The minimum covers 4 KiB of existing scalar storage and adds no buffer or memory cap. Per-scalar arithmetic, ordering, subgroup formulas, points, bucket geometry, dependencies, worker pool, queue, key preparation and resource forecast remain unchanged. No runtime flag, unsafe code, ISA change, lookup table or grain sweep is introduced. The other 170 files are byte-exact; reversing this edit recovers all 171 accepted sources.";
    let after="The candidate stably partitions nonzero scalar indices by their low three bits and builds eight disjoint exact maps in the existing Rayon pool. Each map records the original first index. Summed cardinality preserves the existing cutoff; reconstruction traverses original positions to reproduce the exact values, counts, starts and stable input-index lists. The original sample, zero-aware policy and initial-capacity rule remain. Small inputs and single-worker pools retain the original code. No point arithmetic, scalar representation, bucket geometry, key preparation, queue implementation or persistent table changes. One original file changes and one private module is added; the other 170 originals are exact. Inverse edits recover all 171 production files.\n\nThe temporary index/representative buffer uses eight bytes per nonzero input. Before allocating it, checked bounds include the sample set, all map-growth envelopes, descriptors and reconstruction arrays; both peaks must fit the existing grouping reservation. Otherwise the untouched reservation returns to the serial fallback. Maps drop before reconstruction and pairs before final list allocation. Consumer layout and inherited bounds remain. This is no universal RSS bound; allocator retention, stacks, concurrent callers and actual pressure still need qualification. See memory-review.md. There is no new unsafe code, runtime flag, ISA setting, worker pool, synthetic universal cap or shard-count sweep.";
    assert_eq!(s.matches(before).count(), 1);
    s = s.replace(before, after);
    let before="Historical C7a recordings showed late G2 release and increasing preparation/coordination costs. They did not isolate a removable decomposition cost or predict a speedup. Earlier weighted-reduction grain, partition, guard-reuse and shared-phase failures remain evidence; this changes a different iterator.";
    let after="Retained stage recordings showed serial primary A/G2 grouping maps that ultimately decline. This prototype targets their critical serial work rather than task-search policy. It differs from earlier exact-prefix/bitmap/borrowed-key trials; their original outcomes remain. No new C7a latency or causal prediction follows from those older recordings. The reviewed thesis clustering mechanism overlaps an already unfavorable search census and supplies no new performance claim here.";
    assert_eq!(s.matches(before).count(), 1);
    s = s.replace(before, after);
    let before="Fresh and candidate each pass all 116 native core tests, two raw-image and six cubic tests. Existing exact phi decomposition and complete subgroup-query generator oracles cover the unchanged per-scalar arithmetic. Candidate portable/no-IFMA consumer checks pass. Field source and the accepted 33-test native qualification are inherited; no fresh field-suite execution is claimed. The edit adds no allocations or storage and retains conservative forecasts, but scheduling can affect live allocations, queue reclamation and RSS. No minimum RAM or pressure benefit is established.";
    let after="Fresh passes all 116 native core tests and candidate 119, including three new parallel-grouping tests over exact stable metadata, cutoff boundaries, adversarial shard populations, zero/full-word inputs, one/two/four-worker pools, nested/concurrent callers and checked memory/overflow fallback. Both pass two raw-image and six cubic tests; candidate portable/no-IFMA consumers pass. Existing complete-curve grouping oracles remain unfiltered. Field source and the accepted 33-test native field qualification are inherited, with no fresh field-suite claim. Ordinary RSS and the conservative workspace review do not establish minimum RAM, pressure or concurrent-server behavior.";
    assert_eq!(s.matches(before).count(), 1);
    s = s.replace(before, after);
    s = s.replace("ledger **10,180→10,645**", "ledger **10,645→11,110**");
    let before="The first preparation helper uses an incorrect manifest field name after completing the two snapshots and exits before workflow construction, build, tests or SDK execution. A separate retained helper completes pins and the plan using those same snapshots. No prover change or successful test/capture is repeated for that repair. Original helper and correction remain archived.";
    let after="All initial setup, build and observer receipts are retained. No successful capture, test or verifier is repeated as part of analysis or publication. Raw perf.data files remain local with hashes; privileged capture ownership is transferred to the experiment owner after measurement for reading, without altering bytes or mode.";
    assert_eq!(s.matches(before).count(), 1);
    s = s.replace(before, after);
    s = s
        .replace(
            "complete G2 scalar task-boundary experiment",
            "complete parallel scalar-grouping experiment",
        )
        .replace(
            "Closed G2 scalar task boundary",
            "Closed parallel scalar grouping",
        );
    fs::write(r.join("close.rs"), s).unwrap();
    println!(
        "Prepared source/added-module,119test,ledger11110 closure with parallel-grouping report"
    );
}
