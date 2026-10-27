use std::{fs, path::Path};
fn swap(text: &mut String, before: &str, after: &str) {
    assert_eq!(
        text.matches(before).count(),
        1,
        "missing report text: {before}"
    );
    *text = text.replace(before, after);
}
fn main() {
    let r = Path::new("/tmp/zopro-clean-fft-leaves");
    let mut s = fs::read_to_string("/tmp/zopro-parallel-scalar-groups/close.rs")
        .unwrap()
        .replace("/tmp/zopro-parallel-scalar-groups", r.to_str().unwrap())
        .replace(
            "2026-10-27-parallel-scalar-groups",
            "2026-10-27-clean-fft-leaves",
        )
        .replace("11110", "11575")
        .replace("27660", "43410");
    s = s
        .replace(
            "candidate/crates/zopro-core/src/msm_affine/grouping.rs",
            "candidate/crates/zopro-core/src/fft/pair.rs",
        )
        .replace(
            "candidate/crates/zopro-core/src/msm_affine/grouping/parallel.rs",
            "candidate/crates/zopro-core/src/fft/pair/leaf.rs",
        );
    let start = s.find("        if p == changed {").unwrap();
    let end = start + s[start..].find("        } else {").unwrap();
    s.replace_range(start..end,r#"        if p == changed {
            let new=fs::read_to_string(candidate).unwrap();
            let recovered=new.strip_suffix(edits["module_suffix"].as_str().unwrap()).unwrap()
                .replace(edits["after"].as_str().unwrap(),edits["before"].as_str().unwrap());
            assert_eq!(recovered,fs::read_to_string(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)).unwrap());
"#);
    s = s.replace(
        "# Parallel exact scalar grouping",
        "# Clean bounded serial FFT leaves",
    );
    swap(&mut s,"The candidate stably partitions nonzero scalar indices by their low three bits and builds eight disjoint exact maps in the existing Rayon pool. Each map records the original first index. Summed cardinality preserves the existing cutoff; reconstruction traverses original positions to reproduce the exact values, counts, starts and stable input-index lists. The original sample, zero-aware policy and initial-capacity rule remain. Small inputs and single-worker pools retain the original code. No point arithmetic, scalar representation, bucket geometry, key preparation, queue implementation or persistent table changes. One original file changes and one private module is added; the other 170 originals are exact. Inverse edits recover all 171 production files.","The candidate replaces the stored FFT transform with one implementation that carries a fixed cutoff through the existing 32-way outer recursion. Each expanded leaf still holds at most 1024 vectors. Inside it, binary forks stop at floor_power_of_two(clamp(vectors/(2*workers),64,1024)); terminal DIF/DIT loops use the existing quad and within-pair butterflies. The cutoff is computed once per stored transform, with saturated arithmetic. Root/column caches, packed storage, expansion allocation, load/store/drop and field operations remain. One original file changes and one private leaf module is added; the other 170 originals are exact. Inverse edits recover all 171 production files.");
    swap(&mut s,"The temporary index/representative buffer uses eight bytes per nonzero input. Before allocating it, checked bounds include the sample set, all map-growth envelopes, descriptors and reconstruction arrays; both peaks must fit the existing grouping reservation. Otherwise the untouched reservation returns to the serial fallback. Maps drop before reconstruction and pairs before final list allocation. Consumer layout and inherited bounds remain. This is no universal RSS bound; allocator retention, stacks, concurrent callers and actual pressure still need qualification. See memory-review.md. There is no new unsafe code, runtime flag, ISA setting, worker pool, synthetic universal cap or shard-count sweep.","The earlier serial-leaf trial kept a runtime dispatcher and a duplicate outer-transform body. This clean source removes both. No runtime option, experimental cfg, OnceLock, second transform implementation, new buffer, key table, plan, pool, unsafe code or cutoff sweep is introduced. Existing terminal out-of-line annotations are inherited without tuning. New frames carry only references and a scalar cutoff. The allocation forecast is unchanged for the same storage, but scheduling can change lifetimes and RSS; actual resource/concurrency qualification is still required. See artifacts/integration-review.md.");
    swap(&mut s,"Retained stage recordings showed serial primary A/G2 grouping maps that ultimately decline. This prototype targets their critical serial work rather than task-search policy. It differs from earlier exact-prefix/bitmap/borrowed-key trials; their original outcomes remain. No new C7a latency or causal prediction follows from those older recordings. The reviewed thesis clustering mechanism overlaps an already unfavorable search census and supplies no new performance claim here.","The earlier 2026-10-26 serial-leaf experiment passed every 32-worker condition but failed overall physical16 OFF calibration. Its original positive estimates and failed gate remain unchanged. This is a different implementation and new complete-proof study; no previous captures are repeated or pooled. Cheap-pass grain, queue and retained-deque outcomes remain separate evidence. No arithmetic or universal twofold gain is inferred from the earlier measurements.");
    swap(&mut s,"Both primary cases use 151 requests after three primes, other cases 31 after three.","Primary32 uses 151 requests after three primes; physical16 uses 501 after three; other cases 31 after three. The longer physical16 observation was declared before capture because its earlier controls varied; its calibration/nonregression bounds remain unchanged.");
    s=s.replace("Fresh passes all 116 native core tests and candidate 119, including three new parallel-grouping tests over exact stable metadata, cutoff boundaries, adversarial shard populations, zero/full-word inputs, one/two/four-worker pools, nested/concurrent callers and checked memory/overflow fallback. Both pass two raw-image and six cubic tests; candidate portable/no-IFMA consumers pass. Existing complete-curve grouping oracles remain unfiltered. Field source and the accepted 33-test native field qualification are inherited, with no fresh field-suite claim. Ordinary RSS and the conservative workspace review do not establish minimum RAM, pressure or concurrent-server behavior.","Fresh passes all 116 native core tests and candidate 119. Three reused independent oracle tests check cutoff bounds/available parallelism, scalar-polynomial inverse coefficients before ordinary/twisted forward outputs, cached/uncached roots, packed/unpacked storage, and concurrent shared-root callers with 512 KiB worker stacks. These are previous oracle cases applied to a different binary, not new distinct mathematical cases. Both pass two raw-image and six cubic tests; candidate portable/no-IFMA consumers pass. Field source and the accepted 33-test native qualification are inherited, with no fresh field-suite claim. Kernel concurrency tests do not establish whole-SDK pressure/concurrency/huge-circuit behavior.");
    s = s
        .replace("latency adds 26,100/450", "latency adds 41,850/450")
        .replace("**27,660 SDK calls", "**43,410 SDK calls")
        .replace("ledger **10,645→11,110**", "ledger **11,110→11,575**");
    s = s
        .replace(
            "complete parallel scalar-grouping experiment",
            "complete clean bounded FFT-leaf experiment",
        )
        .replace("Closed parallel scalar grouping", "Closed clean FFT leaves");
    fs::write(r.join("close.rs"), s).unwrap();
    println!("Prepared clean FFT closure:119candidate tests/43410SDK/465Gnark/ledger11575");
}
