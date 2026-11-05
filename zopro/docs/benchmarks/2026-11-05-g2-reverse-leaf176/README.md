# G2 complete reverse-loop function boundary

The candidate is **not adopted** and does not advance to full-query or full-SDK qualification. Against fresh OFF, its paired aggregate is **0.711% slower** over all six contexts and **1.045% slower** at 32 workers. All six control calibrations pass; one non-regression check against FROZEN and all four aggregate gain checks fail. These are complete curve-kernel caller measurements, not proof timings.

## Change and correctness

The entire original borrowed-G2 eight-lane reverse loop moves byte for byte into one never-inline function called once per batch, after the unchanged forward pass, scalar tails and inversion. OFF retains the original loop. Mathematical operations, signs, record widths, point inputs, grouping, batch/window geometry and scheduling remain original. There is no extra arithmetic pass, coordinate repack, heap buffer, table, plan, pool or unsafe operation. The inverse value crosses a compiler ABI boundary, which can change copies, frames and live ranges. This is a code-generation experiment, not an arithmetic reduction or a RAM-saving claim.

The private snapshot has 178 files: 173 unchanged original files, three reversible original edits, the exact inherited caller-adapter child and the reverse-loop child. All 176 original hashes recover; all 17 field sources and generated arithmetic remain exact. The field qualification is inherited and not rerun. OFF and the correctly flagged ON each pass 130 unfiltered core tests: 128 production tests, the inherited caller adapter and the reused complete-generator case adapted for child-path activation. Its cases cover FAST false/true, vector inverse false/true, signed inputs, zero/full/tail lengths through 511, three-call reuse, immutable source points and stable capacities. Existing small-stack/threaded and all-sign-mask tests also run. These are integration/compiler qualifications of existing mathematical cases.

## Preserved preparation failures

The first setup accidentally retained the previous late-Y ON flag. Original OFF tests, library build and freeze succeeded and remain valid. The initial ON-labelled tests and library build also succeeded, but ran OFF; they are not candidate qualification. The freezer correctly rejected their flag mismatch before copying ON libraries or preparing a final caller link. A separate three-step workflow sets the actual reverse-leaf flag, uses a separate target directory, passes all 130 core cases and freezes coherent candidate libraries. Successful valid OFF stages are reused. Source bodies, performance gates and measurements are unchanged. [Repair record](artifacts/candidate-repair.md).

Two direct consumer-repair helper failures also precede capture: a raw-string delimiter compile error, then an exact replacement-count assertion before writing any consumer. Their initial source, available output and scope records are retained. The initial compiler output was truncated by the tool, so no complete filesystem stderr receipt is claimed for it. Some unchanged consumer helpers were recompiled during this failed preparation; no successful test, final link or capture repeats. The corrected repair changes only unexecuted consumer/reviewer bookkeeping. [Direct-helper scope](artifacts/initial-repair-consumers/scope.md).

The linker emits the reverse leaf at 24,154 bytes and inlines the forward/tail remainder into BorrowedG2::run. The initial family-name-only census therefore omitted the inlined forward body. Its original metadata/pins are retained; a supplemental pre-capture audit records the actual caller, its single direct leaf call site and all four selected disassemblies. Combined ON IFMA sites are 250 in the caller plus 1,200 in the leaf, matching 1,450 in each control. Captures start only after closing these supplemented pins. No candidate, gate or timing changes, and no preparation or capture repeats for this audit. [Supplemental audit](artifacts/seal-forward-caller.log).

## Fixed complete-caller measurements

Ryzen 9 9950X, 16 physical/32 logical CPUs. Solo workers pin to CPU 2; physical16 to 0–15; logical32 to 0–31. The coordinator is unpinned. Exact inherited API-only FROZEN, fresh OFF and true ON share unchanged driver/adapter bytes and coherent libraries. Actual optimized direct-link argv has -O, target-cpu=native, fat LTO and one codegen unit. These independent kernel workers do not reproduce the full MSM scheduler or concurrent-proof throughput.

Eighteen 32-call preflights precede 15 rotated/reversed matched blocks of 8,192 calls per worker, after three checked 1,024-call primes. Timed allocations are zero; complete generator/scalar outputs, reference payloads and scratch capacities match across arms. Wave clocks include synchronization and all caller arithmetic, inversion, packing and scatter. Scalar output checks follow worker clocks.

| Pairs | Workers | FROZEN µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF [bounds] | Wins/15 |
|---|---:|---:|---:|---:|---|---:|
| 200 | 1 | 7.867900 | 7.857818 | 7.895335 | 1.006934223 [0.994465519, 1.010777002] | 6 |
| 200 | 16 | 9.077324 | 9.014889 | 9.075167 | 1.005913975 [1.002545716, 1.009911479] | 3 |
| 200 | 32 | 14.679107 | 14.731921 | 14.831663 | 1.008668510 [1.002688558, 1.011876208] | 2 |
| 352 | 1 | 13.291878 | 13.266099 | 13.349064 | 1.003731408 [0.997590580, 1.010582159] | 6 |
| 352 | 16 | 15.333238 | 15.353565 | 15.666678 | 1.010973095 [1.004873286, 1.021976253] | 2 |
| 352 | 32 | 25.143046 | 25.135711 | 25.384463 | 1.014147789 [1.002930871, 1.017892844] | 2 |

Values are medians of wave observations; paired ratios need not equal ratios of the displayed medians. Original-link calibration requires all six intervals within [1/1.03, 1.03]; all twelve candidate/control upper bounds must be ≤1.03. Equal-log all-six and logical32-two aggregates require upper <1 and ≥10/15 wins against both controls. Bounds use ranks 4 and 12 of 15, nominal 96.484375% marginal coverage conditional on exchangeable blocks, without a familywise guarantee. No gain floor, excluded observation, extension, threshold change, favorable subset, parameter sweep or unchanged reroll applies.

| Aggregate / control | Paired median | Bounds | Wins/15 |
|---|---:|---|---:|
| all6/on/frozen | 1.008124588 | [1.005624110, 1.011294040] | 1 |
| all6/on/off | 1.007106524 | [1.003309641, 1.016932905] | 3 |
| logical32_2/on/frozen | 1.009780644 | [1.008933466, 1.014902294] | 0 |
| logical32_2/on/off | 1.010449250 | [1.004792235, 1.013596046] | 2 |

Failed checks, retained exactly:

```json
[
  "n352-physical16/on/frozen",
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
| stat-n200-physical16-off | 111170 | 41561 |
| stat-n200-physical16-on | 110767 | 41658 |
| stat-n200-logical32-off | 111170 | 61029 |
| stat-n200-logical32-on | 110767 | 61404 |

| Actual linked kernel | Static instruction sites | Static IFMA sites |
|---|---:|---:|
| frozen | 8587 | 1450 |
| off | 8587 | 1450 |
| on | 3686 | 1200 |
| on | 5140 | 250 |

The ON caller scope includes the inlined forward body and adapter while the controls retain a named combined kernel. Selected whole-symbol counts therefore have different ownership boundaries. Static instruction/stack sites are not executed traffic, measured spills, cache misses or RAM bounds. The measured counter panels show about 0.36% fewer instructions for ON, but slightly more cycles; they do not isolate a causal cost.

| Profile | Samples | Unknown % | Sampled workers | Qualified |
|---|---:|---:|---:|---|
| cycles-n200-physical16-off | 10873 | 0.046 | 16 | true |
| cycles-n200-physical16-on | 10910 | 0.027 | 16 | true |
| cycles-n200-logical32-off | 31679 | 0.035 | 32 | true |
| cycles-n200-logical32-on | 31955 | 0.028 | 32 | true |

Profile checks bind the executable, PID/FORK membership, worker TID/CPU pairs, 16,000,000 user-cycle period and zero lost records. Attribution requires ≥1,000 samples and ≤5% unknown; unqualified diagnostics remain retained. Flat sampled shares have unvalidated absolute-period coverage and are not exclusive critical-path fractions.

| Pairs/logical32 | OFF / ON scratch bytes per worker | FROZEN / OFF / ON lifetime RSS MiB |
|---|---:|---:|
| 200 | 16000 / 16000 | 4.500 / 4.250 / 4.000 |
| 352 | 28160 / 28160 | 11.250 / 11.250 / 11.000 |

Prefix records remain two FqBatch vectors per eight points, 16,000/28,160 bytes at 200/352 pairs. Caller scratch/reference capacities match across all three arms and remain stable. Payload sizes exclude other arrays, stacks and allocator state. RSS is a lifetime diagnostic including setup/primes, not an isolated proof peak, minimum RAM or a pressure test. No memory saving is established.

## Independent review and limits

The independent Rust reviewer imports no campaign/parser code and checks **616 source/input pins and 327 successful workflow commands and one preserved failed freeze**, normalized protocols, actual argv/stream hashes, original-source recovery, exact field/adapter identity, all 270 raw integer latency observations and paired gates, all output/capacity signatures, CPU/RSS and separately parsed PMU/profile ownership/loss. The report uses those reconstructed results.

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

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [edits](artifacts/edits.json), [reverse-loop source](artifacts/reverse_leaf.rs), [reproduction](artifacts/reproduce.md), [artifact index](artifact-index.json), [checksums](SHA256SUMS). All 178 source reconstruction hashes, helper source, exact native/link/capture UTF-8 receipts, decoded profiles and linked assembly are retained. Gzip receipts round trip member paths, lengths, bytes and SHA-256 hashes. Executables, libraries, raw perfdata and private key/image/witness/scalar/point arrays are excluded.
