# Late G2 bucket-Y materialization

The fixed complete-caller gate **fails**. The candidate is closed without an unchanged rerun or full-SDK promotion. Production remains unchanged. This is an isolated curve-kernel caller experiment, not a proof timing or a new AWS/Aeglos comparison.

The reverse borrowed-G2 kernel packs bucket Y for the signed numerator and retains that value across inverse-prefix multiplication, slope calculation and X finishing. The candidate introduces a late immutable-slice `black_box` and gathers/packs the unchanged bucket Y again immediately before the final subtraction. This tests a shorter SIMD live range against extra loads, packing, bounds checks and barrier costs. There is no new field formula, representation, buffer, record width, table, copied key/point array, execution plan, pool or unsafe operation. The eight-lane reverse loop, forward pass, tails, signs, batch geometry and scheduling remain original. Source-level live ranges do not establish dynamic spills or a speed benefit.

The prior [owned-finish](../2026-10-26-g2-owned-finish/README.md), [phased-inverse](../2026-10-28-g2-phased-inverse174/README.md) and [staged operators](../2026-10-30-g2-staged175/README.md) tested different whole-batch or tiled multipass changes and remain rejected under their original gates. No calibration exception or PGO profile transfers here.

## Correctness and identity

Fresh OFF and ON each pass **130 unfiltered native core tests**: 128 production tests, one inherited caller-adapter test and one new independent generator-sum test. The new case covers FAST false/true, vector inverse false/true, positive/negative inputs, zero/full/tail lengths through 511, three-call reuse, immutable source points, stable scratch capacity and an ON-only test counter proving route execution. Existing all-sign-mask and small-stack threaded tests also run. All 17 field source files remain byte exact and inherit their qualification; field suites are not rerun. This is not a prover, portable, device or memory-pressure qualification.

The private snapshot has 177 files, 173 original files unchanged, three reversible original edits and one caller-adapter child. Reversing the edits recovers all 176 original hashes. The adapter and driver are byte exact to the inherited API-only original control. OFF retains the original Y expression. Coherent frozen libraries and transitive/proc-macro sets are checked; actual direct-link argv includes `-O`, `target-cpu=native`, `lto=fat` and `codegen-units=1`. Six native and two link workflow steps succeed. Test-count metadata was aligned before its consumers ran: 128 production + one inherited adapter + one new oracle is 130; no test or successful stage was repeated.

## Complete-caller measurements

Ryzen 9 9950X, 16 physical/32 logical CPUs. Each worker pins to CPU 2 for solo, 0–15 for physical16 or 0–31 for logical32; coordinator unpinned. These independent ordinary-kernel workers do not reproduce a full MSM or concurrent-proof schedule. Eighteen 32-call preflights precede 15 rotated/reversed matched blocks, 8,192 calls/worker after three checked 1,024-call primes. Complete generator/scalar output checks follow worker clocks. Every cross-arm output/reference/capacity signature must match, and timed allocations must be zero. The wave clock includes worker synchronization and complete caller arithmetic/packing/inversion/scatter.

| Pairs | Workers | FROZEN µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF [bounds] | Wins/15 |
|---|---:|---:|---:|---:|---|---:|
| 200 | 1 | 7.852839 | 7.970722 | 8.066356 | 1.014755565 [1.003921075, 1.020358749] | 0 |
| 200 | 16 | 9.038936 | 9.144650 | 9.269011 | 1.012853760 [1.009806442, 1.016412827] | 1 |
| 200 | 32 | 14.654917 | 14.897312 | 15.044820 | 1.012424941 [1.001868902, 1.018878540] | 1 |
| 352 | 1 | 13.300029 | 13.418081 | 13.643042 | 1.016274411 [1.011260408, 1.027440960] | 0 |
| 352 | 16 | 15.383452 | 15.522446 | 15.753690 | 1.020747578 [1.016847212, 1.027290968] | 1 |
| 352 | 32 | 25.118897 | 25.375332 | 26.013199 | 1.020240926 [1.012557834, 1.029236456] | 1 |

Values are medians of wave observations; paired ratios need not equal ratios of the displayed medians. Original-link calibration requires all six intervals within [1/1.03, 1.03]; all twelve candidate/control upper bounds must be ≤1.03. Equal-log all-six and logical32-two aggregates require upper <1 and ≥10/15 wins against both controls. Bounds use ranks 4 and 12 of 15, nominal 96.484375% marginal coverage conditional on exchangeable blocks, without a familywise guarantee. No gain floor, excluded observation, extension, threshold change, favorable subset, parameter sweep or unchanged reroll applies.

| Aggregate / control | Paired median | Bounds | Wins/15 |
|---|---:|---|---:|
| all6/on/frozen | 1.027613183 | [1.021860548, 1.028755287] | 0 |
| all6/on/off | 1.015695575 | [1.014106601, 1.020192018] | 1 |
| logical32_2/on/frozen | 1.029077742 | [1.024177591, 1.032291622] | 0 |
| logical32_2/on/off | 1.016295065 | [1.011146086, 1.019181381] | 1 |

Failed checks, retained exactly:

```json
[
  "n200-solo/on/frozen",
  "n200-logical32/on/frozen",
  "n352-solo/on/frozen",
  "n352-physical16/on/frozen",
  "n352-logical32/on/frozen",
  "all6/on/frozen/gain",
  "all6/on/off/gain",
  "logical32_2/on/frozen/gain",
  "logical32_2/on/off/gain"
]
```

All 317 capture commands complete once: 296 caller processes (270 latency), four grouped counter panels, four cycle profiles, preflight checking and postcapture decoding. Measured latency calls total 36,126,720; all measured calls including diagnostics total 62,088,384; primes add 15,040,512, giving 77,128,896 kernel calls. These are not proofs. No owned compiler/helper/hash/analyzer/archive/verifier or second workload overlaps recording.

## CPU and memory diagnostics

All latency precedes counters and fixed-period user-cycle profiles. FIFO gates exclude primes and final validation, retaining synchronization/accounting edges. Single counter panels have no repeated-panel uncertainty estimate. Raw cache/branch counters remain available without unsupported cache-level, bandwidth or causality claims.

| Panel | Instructions/batch | Cycles/batch |
|---|---:|---:|
| stat-n200-physical16-off | 111170 | 42223 |
| stat-n200-physical16-on | 114145 | 42554 |
| stat-n200-logical32-off | 111170 | 61917 |
| stat-n200-logical32-on | 114145 | 62463 |

| Actual linked kernel | Static instruction sites | Static IFMA sites |
|---|---:|---:|
| frozen | 8587 | 1450 |
| off | 8587 | 1450 |
| on | 8688 | 1450 |

Static instruction/stack sites are not executed traffic, measured spills or RAM bounds. They describe the actual linked caller rather than a standalone field wrapper.

| Profile | Samples | Unknown % | Sampled workers | Qualified |
|---|---:|---:|---:|---|
| cycles-n200-physical16-off | 11038 | 0.027 | 16 | true |
| cycles-n200-physical16-on | 11151 | 0.018 | 16 | true |
| cycles-n200-logical32-off | 32221 | 0.037 | 32 | true |
| cycles-n200-logical32-on | 32495 | 0.037 | 32 | true |

Profile checks bind the executable, PID/FORK membership, worker TID/CPU pairs, 16,000,000 user-cycle period and zero lost records. Attribution requires ≥1,000 samples and ≤5% unknown; unqualified diagnostics remain retained. Flat sampled shares have unvalidated absolute-period coverage and are not exclusive critical-path fractions.

| Pairs/logical32 | OFF / ON scratch bytes per worker | FROZEN / OFF / ON lifetime RSS MiB |
|---|---:|---:|
| 200 | 16000 / 16000 | 4.250 / 4.500 / 4.500 |
| 352 | 28160 / 28160 | 11.250 / 11.250 / 11.250 |

Prefix records remain two FqBatch vectors per eight points, 16,000/28,160 bytes at 200/352 pairs. Caller scratch/reference capacities match across all three arms and remain stable. Payload sizes exclude other arrays, stacks and allocator state. RSS is a lifetime diagnostic including setup/primes, not an isolated proof peak, minimum RAM or a pressure test. No memory saving is established.

## Independent review and limits

The independent Rust reviewer imports no campaign/parser code and checks **593 source/input pins and 325 successful workflow commands**, normalized protocols, actual argv/stream hashes, original-source recovery, exact field/adapter identity, all 270 raw integer latency observations and paired gates, all output/capacity signatures, CPU/RSS and separately parsed PMU/profile ownership/loss. The report uses those reconstructed results.

No new SDK proof, independent Gnark export, AWS/Aeglos, phone/ARM/TEE, 250/4999, swap/pressure, minimum-RAM or concurrent-proof result is added. Canonical proof ledger stays **30,708**. A component pass only selects further complete-query/full-SDK/resource qualification; it never adopts a compiler artifact or completes the goal.

## Accepted CPU and Metal comparisons

Inherited accepted 32-worker local results and supplied historical M5 references, not measurements of this candidate. [Accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Latest separate full-SDK evidence is the [composed PGO comparison](../2026-11-05-bundle-proof-pgo-full176/README.md), whose control-calibration failure remains closed without adoption.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 4.667× | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.291× | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 7.817× | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 8.359× | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 8.018× | 2.673× | 52.8 |
| merge-24x1 | 65.060937 | 507 | 175 | 7.793× | 2.690× | 50.7 |

Merge54 remains 161.883899 ms without a supplied M5 reference. All-eight CPU10× remains active and unmet, **0/8**. Comparing machines does not establish equal hardware efficiency.

## Artifacts

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [edits](artifacts/edits.json), [reproduction](artifacts/reproduce.md), [artifact index](artifact-index.json), [checksums](SHA256SUMS). Source reconstruction hashes, helper source, exact native/link/capture UTF-8 receipts, decoded profiles and linked assembly are retained. Gzip receipts round trip member paths, lengths, bytes and SHA-256 hashes. Executables, libraries, raw perfdata and private key/image/witness/scalar/point arrays are excluded.
