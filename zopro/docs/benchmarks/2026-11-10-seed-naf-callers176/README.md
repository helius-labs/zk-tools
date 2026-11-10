# Complete subgroup-call comparison

The signed-seed candidate was faster in the recorded component comparisons, but all six fresh-control calibration gates failed. The exact comparison is closed without adoption. This experiment does not measure proof latency. Production remains unchanged.

For valid points at one worker, the median of the 15 process medians was **29.496 µs** for the accepted library, **33.703 µs** for the fresh unchanged control, and **27.713 µs** for the candidate. The paired candidate/accepted ratio was **0.93850** (95% interval **0.93608–0.94086**, 15/15 wins); candidate/fresh-control was **0.82274** (0.82100–0.82472, 15/15 wins). The inflated fresh control makes the latter unsuitable as an estimate of the arithmetic change alone.

| Population | Workers | Fresh control / accepted | Candidate / accepted | Candidate / fresh control |
|---|---:|---:|---:|---:|
| valid | 1 | 1.14070 | 0.93850 | 0.82274 |
| valid | 16 | 1.12853 | 0.94133 | 0.83412 |
| valid | 32 | 1.06343 | 0.94281 | 0.88657 |
| mixed | 1 | 1.14232 | 0.94296 | 0.82549 |
| mixed | 16 | 1.12794 | 0.94052 | 0.83384 |
| mixed | 32 | 1.06471 | 0.94283 | 0.88552 |

Ratios below 1 are faster. These are complete public `G2Affine::is_in_subgroup` calls with materialized, checked boolean outputs. The serial valid population is primary; the standard-thread cases measure component concurrency, not concurrent proofs or Rayon behavior. Six calibration intervals had to fit [1/1.03, 1.03]; none did. The remaining 38 of 44 gates passed: both primary gain gates, 12 nonregression gates, six RSS guards and 18 diagnostic gates. No gate was changed and no timing was repeated.

The serial counter panel recorded approximately equal retired instructions for accepted and fresh control (15.235529 versus 15.235507 billion), but branch misses rose from 72,156 to 762,675 while cycles rose from 5.0115 to 5.7909 billion. The candidate recorded 14.3990 billion instructions, 71,551 branch misses and 4.7089 billion cycles. This supports investigating branch prediction and executable placement; it does not identify the responsible branch or prove a placement cause. The accepted and fresh-control subgroup disassemblies have equal line counts, but equal line counts do not establish identical machine code. No assembly was edited.

Latency-process peak RSS maxima were equal across the three arms in every context: 2.0 or 2.25 MiB. These small standalone processes do not establish full-prover peak memory or minimum RAM. The production change under investigation has no new heap allocation site or point table. Its independently qualified exact-integer seed representation replaces four mixed additions; see the [qualification report](../2026-11-10-seed-naf176/README.md).

The serialized Rust xtask workflow completed all 18 caller preflights, 270 latency processes (15 fixed rotated/reversed blocks, 31 samples per process, 1024 calls per worker), nine PMU panels and nine cycle profiles on the first execution. Timing excludes allocation, oracle construction, startup, primes, barriers and output checks; coordinator wave time is retained separately. Every worker clock stops before any output check. Process resource accounting includes primes, barriers and checking. Nine grouped PMU panels had 100% running time. All nine profiles passed fixed-period, ownership, PID/TID/CPU, unknown-symbol and zero-loss checks. These are aggregate sampled-cycle profiles, not a causal branch study or absolute wall-time attribution.

The original frozen secondary reader stopped on exact JSON number equality after a one-ulp serialization difference in a ratio. Its source is preserved. A separate corrected reader permits at most eight machine epsilons when comparing recomputed numbers with their serialized representation. Raw medians, bootstrap code, intervals and gate thresholds were unchanged. That reader reconstructed all process medians, all 18 contrasts and 18 diagnostic panels. This is a separate reader by the same agent, not an external audit. No benchmark or native qualification was rerun for this correction.

The public [analysis](artifacts/analysis.json), [reader result](artifacts/review.json) and [evidence archive](artifacts/evidence.tar.gz) retain all latency samples, command receipts, source, fixed plans, pins, counter data, event/loss records, disassemblies and profile summaries. [Public member hashes](artifacts/public-members.json) were checked after archive extraction. Large raw profile traces, perf data, exact native binaries and synthetic point payloads remain at `/home/ubuntu/src/zopro-perf-work/seed-naf-callers176`; their [hash inventory](artifacts/private-capture-hashes.json) records this limitation. Full symbolic profiles have not been reproduced independently from the public archive.

No AWS resources were started, no SDK requests or Gnark proof exports were produced, and the canonical verified-proof ledger remains at 47,437. Accepted 1×2 proof observations remain approximately 7.86 ms locally and 11.24 ms on c7a; neither is a measurement of this candidate. The original performance objective remains unmet.

Next: inspect the control discrepancy using the already captured counters and disassembly before designing a different experiment. Do not rerun this recipe, relax its calibration band or report its component gain as a proof improvement. A later full-SDK trial requires coherent field/core/prover libraries and the existing verified ALL8 plus 54 protocol.
