# Mixed binary and triple G1 caller

A single inversion shared by ordinary additions and sparse three-point additions improves seven of ten modeled collision cases against a preselected two-layer control. The 200-entry cases save 0.61–7.31%; 352-entry cases are mixed, including regressions. **This is a qualified caller opportunity, not a production switch or proof-time gain.** Accepted local 1x2 remains 8.324853 ms at 134.50 MiB peak RSS.

The [real-request census](../2026-10-28-msm-geometry/README.md) supplied batch sizes 200/352 and the need to distinguish matching, repeated and unmatched queue entries. This fixture models those properties; it does not replay a captured production queue. The [three-point derivation](../2026-10-28-triple-affine-screen/README.md) remains the arithmetic basis.

## Native comparison

Ryzen 9950X, CPU 2, one active caller and Rayon setup pool one. Both algorithms use one executable. Thirty-one rotated/reversed process blocks cover twelve cases and both variants: 744 latency processes plus 24 preflights. Each runs 4096 rounds after three eight-round primes, with bucket reset outside timing. Every output and remaining queue order is checked after timing. All samples and outliers remain.

The candidate classifies queued entries, partitions pending entries, performs arithmetic and compacts the queue inside the timer. The control receives its independently preselected collision list and compaction mask before timing, then calls the unchanged signed-borrowed stream twice. Both include the same per-round fixture queue restore. This control is deliberately optimistic about selection cost. Real production can merge queued entries into later full batches, so two explicit layers do not establish the actual number of inversions saved there.

Times are median ns per round; change uses the median within-block mixed/binary ratio. Intervals use ranks 10..22 of 31, nominal 97.055% under independent/exchangeable blocks, without a familywise guarantee.

| Pending / matches | Binary ns | Mixed ns | Change | Mixed/binary interval | Wins |
|---|---:|---:|---:|---:|---:|
| 200 / 0 | 4510.30 | 4508.61 | calibration | 0.9974–1.0003 | 21/31 |
| 200 / 5 | 5657.39 | 5242.72 | +7.31% | 0.9264–0.9278 | 30/31 |
| 200 / 8 | 5348.28 | 5139.89 | +3.90% | 0.9603–0.9618 | 30/31 |
| 200 / 13 | 5841.73 | 5467.30 | +6.37% | 0.9358–0.9387 | 31/31 |
| 200 / 16 | 5474.00 | 5322.42 | +2.77% | 0.9708–0.9733 | 28/31 |
| 200 / 32 | 5796.12 | 5762.69 | +0.61% | 0.9929–0.9954 | 30/31 |
| 352 / 0 | 7375.91 | 7371.03 | calibration | 0.9982–0.9998 | 23/31 |
| 352 / 5 | 8516.85 | 8407.24 | +1.19% | 0.9872–0.9885 | 31/31 |
| 352 / 8 | 8202.87 | 8280.21 | -0.99% | 1.0091–1.0106 | 0/31 |
| 352 / 13 | 8681.18 | 8606.92 | +0.81% | 0.9914–0.9928 | 30/31 |
| 352 / 16 | 8368.60 | 8508.69 | -1.56% | 1.0147–1.0171 | 1/31 |
| 352 / 32 | 8705.13 | 8952.13 | -2.83% | 1.0265–1.0292 | 0/31 |

Both zero-collision calibration intervals pass the declared factor-1.03 bound. Seven nonzero cases pass the upper-ratio-below-one and at-least-21-wins condition, with no minimum gain floor. The three regressions remain visible. The gate supports a scoped production experiment; it does not support a universal threshold, unsampled collision counts or adoption.

## Implementation and validation

The reusable integer index has next-power-of-two capacity at least twice the pending count, without a dense per-bucket array or per-key table. It selects one queued update per pending destination, newest first. It leaves repeated and unmatched entries in their original order. Ordinary entries retain the existing five-vector record arithmetic; triple records retain eight vectors. Both sets accumulate into the same eight inversion chains. After one nonexceptional inversion stage, reverse passes write distinct destinations. Padded lanes have nonzero dummy denominators and never write output.

Host identity and first-denominator checks, plus unpacked combined-denominator zero checks, precede inversion/output writes. Complete scalar fallback computes the same selected logical work. Signed Q and R operands are independent. An empty queue uses the unchanged binary path. No field backend, key representation, scheduler or production memory policy changes.

Two new independent Arkworks tests pass once on full-width inputs, signed operands, empty/tail sizes, repeated chains, complete exception combinations and leftover order. A third targeted test passes on all twelve contexts after adding the optimistic preselected control. The two earlier successful tests were not repeated; their sources, binary hash and logs remain. The mixed arithmetic body is unchanged by this amendment. The first test build lacked the Arkworks `Field` import and stopped before tests; failed source/log and repaired build remain archived.

The initial protocol used a common classification step. Before any capture, the final protocol moved classification and R-list assembly out of the control's timer while retaining them in the candidate's. Both protocols remain archived, and every capture uses the final declared protocol. No timing was discarded or repeated for this change.

Every measured loop reports zero allocation calls. Included retained vector capacities are 54,750 bytes for mixed versus 44,480 for binary at 200/13, and 93,348 versus 76,032 at 352/16. These sums cover the measured mixed scratch, binary record scratch and a retained reference-list allocation; they exclude other fixture arrays, preselected data, reference scalars and allocator metadata. They are not complete process RSS, minimum RAM or a qualified production reservation. Resource admission, portable/mobile/TEE behavior and memory-pressure qualification remain required before adoption. No universal memory cap or point lookup table is proposed.

## Profiles and next requirement

All latency precedes four whole-process cycles:u profiles at 199 Hz, each with 262144 rounds. The 200/13 binary/mixed recordings contain 315/300 samples; 352/16 contain 444/453, 1512 total, one user PID per recording and zero lost records. Setup, primes and checks are included, so these are not warm-gated profiles. Raw recordings remain local with hashes; decoded symbols and static assembly are archived. No owned build/helper/hash/archive or second workload overlaps capture.

Scalar inversion receives about 20% versus 7% of sampled period at 200/13, and 16% versus 7% at 352/16. The candidate structurally saves one inversion stage, but its outer `run` method also receives about 10%/13% of sampled period, including classification, partitioning, restore and compaction. These shares do not isolate removable wall time. The larger case regresses despite the saved inversion, consistent with the need to charge additional arithmetic and bookkeeping. The static mixed symbol has 2150 IFMA sites across its two loops; this is not a dynamic operation or spill count.

All 444 pinned paths match before and after capture. Only two private original files add module/reexport suffixes; removing them recovers accepted bytes, and 170 other private originals and all 172 production files remain unchanged. There are no SDK requests, new proof exports, independent Gnark checks, AWS/Aeglos comparisons or new peak-RAM observations. The independent proof ledger remains 12,527.

Next: integrate the qualified mechanism into actual pending/queue handling with a declared policy that respects the regressions and charges retained scratch. Measure whether it reduces later flushes or projective work and complete SDK latency, internal verification and independent Gnark checks. Do not transfer the modeled two-layer percentage to proof latency, add an outcome-specific lookup policy, or replace weighted binary reductions with ternary reductions.

## Inherited CPU and Metal comparison

The [accepted report](../2026-10-27-clean-fft-adoption/README.md), including its full-performance-gate failure and engineering exception, remains authoritative. M5 figures are supplied historical references. The original all-eight CPU 10x goal remains active and unmet, 0/8.

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

Accepted merge54 remains 161.341835 ms without a supplied M5 reference.

[Final protocol](artifacts/design.md), [all results and profile symbols](artifacts/analysis.json), [mixed implementation](artifacts/mixed_probe.rs), [independent tests](artifacts/tests.log), [preselected control check](artifacts/control-tests.log), [pins](artifacts/pins.json), [source audit](artifacts/source-audit.json), [workflow completion](artifacts/runs/result.json), [artifact hashes](artifact-index.json).
