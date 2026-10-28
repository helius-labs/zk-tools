# Paired cancellation in affine and FFT callers

The selected paired-cancellation dispatcher passes correctness checks but fails the actual-caller performance gate. Against fresh ordinary arithmetic, G2 batches regress0.41–0.53% and FFT leaves regress0.09–0.15%. **The integration is rejected; no SDK trial or adoption follows.** Accepted local1x2 remains8.324853ms at134.50MiB peak RSS.

This follows the [positive primitive component](../2026-10-28-paired-cancel/README.md). Only eight-lane mixed fused and uniform ordinary products select the paired method. Mixed ordinary, uniform fused and all four-lane dispatches retain their original implementation. The method is generated from the four-lane template using the unchanged official generator. No representation, key, point table, image, allocator, pool or caller algorithm changes.

## Actual-caller results

Private adapters call the existing G1 signed/unsigned stream, G2 prefix/norm stream and serial FFT DIF/DIT leaves. They do not reproduce those arithmetic bodies. G1 is a retained-path control. FFT rounds consist of an inverse/forward leaf pair on two independent polynomials; they do not include the complete product FFT or outer parallel scheduling.

Ryzen9950X, CPU2, one active caller, Rayon setup pool1;31 fixed rotated/reversed process blocks and three variants. Frozen uses original fields plus adapters. Fresh OFF and selected ON share source, with a compile-time dispatcher enabled only for ON. Point rounds are4096/2048/512 for32/128/512 pairs; FFT rounds4096/512/128 for domains128/1024/4096. Three8-round primes precede each timer, with reset outside timing. All samples and outliers remain; there are no extensions, exclusions or unchanged rerolls.

Cells are median ns per caller round. Positive paired change means faster; it is computed from per-block selected/fresh ratios, not the ratio of displayed medians.

| Caller | Frozen | Fresh | Selected | Paired change |
|---|---:|---:|---:|---:|
| G1 32 | 1188.30 | 1188.99 | 1190.60 | -0.146% |
| G1 128 | 2781.57 | 2780.11 | 2781.05 | -0.061% |
| G1 512 | 9131.81 | 9128.57 | 9132.01 | -0.047% |
| G1 signed32 | 1205.45 | 1206.50 | 1205.79 | +0.018%, uncertain |
| G1 signed128 | 2844.39 | 2843.03 | 2844.52 | -0.013%, uncertain |
| G1 signed512 | 9393.48 | 9391.44 | 9392.54 | -0.005%, uncertain |
| G2 prefix32 | 1840.20 | 1804.62 | 1813.99 | -0.527% |
| G2 prefix128 | 5125.74 | 5092.04 | 5115.40 | -0.410% |
| G2 prefix512 | 18253.06 | 18217.88 | 18300.41 | -0.443% |
| FFT128 | 4156.41 | 4157.18 | 4163.59 | -0.128% |
| FFT1024 | 48365.66 | 48367.28 | 48447.65 | -0.151% |
| FFT4096 | 233924.64 | 233860.46 | 234098.25 | -0.085% |

The equal-weight12-case aggregate is0.999490 versus frozen, interval[0.998794,0.999856],28/31 wins; versus fresh it is1.001756,[1.001394,1.001934],4/31 wins. The necessary gain condition fails against fresh. All calibration and nonregression intervals remain within factor1.03; neither passing condition establishes a gain. There is no minimum gain floor. The aggregate is not weighted by real prover work.

Intervals use individual ranks10..22 of31, nominal97.055% under independent/exchangeable blocks, without a familywise guarantee. Fresh/frozen G2 differences remain visible in the table; selecting only the favorable frozen comparison would give an incomplete conclusion. The prior primitive gains remain valid for their declared contexts, but do not justify this caller integration.

## Correctness, storage and profiles

Three new adapter tests pass on original fields, then pass again within the changed candidate's122 native core tests. All33 native field tests pass on the changed dispatcher. These are three new test cases applied to different binaries, not six new independent cases. The prior paired integer model and native raw-lane qualification are inherited without rerunning them.

Point outputs are checked over every destination against generator scalar formulas, including signed entries, retained scratch and boundary lengths. The FFT adapter checks every inverse coefficient against sparse scalar polynomials before any forward pass can hide an error, then checks repeated inverse/forward rounds against scalar N^rounds. All1116 latency processes and12 preflights check their final outputs after timing. Qualified profiles do the same. Every timed process reports zero allocation calls; source-level scratch capacities agree across all variants. This is not minimum-RAM or whole-prover RSS evidence.

The initial four-lane template translation retained incorrect intrinsic suffixes. The failed sources/builds are preserved; a repair guard also initially confused expanded compiler errors with source occurrences. The naming repair leaves the generated eight-lane source byte-identical to its pre-repair version. No arithmetic test or timing was rerun for this repair. Generator check passes. The source audit checks all172 accepted production hashes,165 unchanged private files and inverse edits recovering the original four-lane template and five adapter host files.

All latency completed before profiling. The first recording attempt stopped during caller construction because sudo discarded the explicit G2 environment; it ran no priming or timed kernel loop. Its failed workflow, stderr, exit record and raw-data hash remain. A separate16-step workflow passes the same environment explicitly after sudo and collects all four qualified profiles in new paths. It repeats no latency or test. Original controller/pins stay intact; an offline controller reads the qualified profile paths. Both workflow histories are retained.

Qualified whole-process cycles:u profiles at199Hz have45/42 G2 samples and166/163 FFT samples (fresh/selected),416 total, one PID each and zero lost records. Setup, primes and oracles are included; these are not warm-gated profiles. Raw data remains local with hashes; decoded samples and loss dumps are archived. The small G2 sample counts limit fine attribution. The separate failed setup recording has10 samples and is excluded from these qualified totals by its failed-construction status.

| Actual caller symbol, fresh to selected | IFMA sites | Instruction sites | RSP operand lines | Literal frame allocation |
|---|---:|---:|---:|---:|
| Canonical G2 prefix | 1450 to1470 | 8326 to8473 | 1396 to1418 | 5696 to5888bytes |
| FFT DIF leaf | 200 to208 | 806 to842 | 71 to77 | 280 to440bytes |
| FFT DIT leaf | 300 to312 | 920 to979 | 69 to73 | 376 to504bytes |

These are static whole-symbol counts, not executed instructions, spills or removable wall-time costs. They show additional work and larger frames in these callers; they do not isolate a causal fraction of the latency difference. G1 selected symbol counts match the controls. Full disassembly hashes and selected symbol assembly are retained.

No owned build, helper/hash/archive or second workload overlaps captures. All production files remain unchanged. No SDK requests, fresh proof exports or independent Gnark verification are added; the proof ledger remains12524. No fresh AWS, Aeglos, portable-device, pressure, concurrency or250/4999 result is claimed. The specified paired recipe is closed for these callers; no unchanged inlining/layout/flag reroll or complete-proof timing is selected.

## Inherited CPU and Metal comparison

The [accepted all-eight report](../2026-10-27-clean-fft-adoption/README.md) remains authoritative, including its full-performance-gate failure and engineering exception. M5 figures are supplied historical references. The original all-eight CPU10x goal remains active and unmet,0/8.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445x | 24 | 2.883x |
| 2x2 | 11.460675 | 59 | 5.148x | 30 | 2.618x |
| 4x2 | 15.854365 | 109 | 6.875x | 48 | 3.028x |
| 8x2 | 24.084626 | 189 | 7.847x | 74 | 3.072x |
| 12x2 | 33.693397 | 284 | 8.429x | 101 | 2.998x |
| 16x2 | 39.981636 | 335 | 8.379x | 124 | 3.101x |
| 24x2 | 66.010670 | 528 | 7.999x | 176 | 2.666x |
| Merge24x1 | 65.086624 | 507 | 7.790x | 175 | 2.689x |

Accepted merge54 remains161.341835ms without a supplied M5 reference.

[Protocol](artifacts/design.md), [complete results](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [template integration](artifacts/integration-audit.json), [profile repair](artifacts/profile-repair-receipt.json), [pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
