# Overlapping the product with fused K/H proving queries

The fixed speed gate fails; the candidate is not adopted. Primary paired latency regresses 0.267% against accepted and 1.066% against fresh OFF, with 3/7 and 1/7 wins. The original-eight aggregate regresses 0.481% and 0.258%, winning one block against each control. All OFF calibration checks pass, including the physical-16 case. Production remains unchanged.

## Scheduling change

The current small-proof path finishes the Euler product before starting A, fused K+H and G2. The prototype starts A and G2 while a scoped branch computes the product and then fused K+H. Arithmetic, scalar grouping, bucket widths, worker pool and field representations remain. Earlier four-query overlap trials were marginal and one failed a warm memory-pressure test; their results remain unchanged. This trial uses the current combined K/H stream, Euler product and G2 prefix implementation, with one fixed H-first schedule.

The corrected path requires the existing fused-query eligibility, an Euler query, sufficient query points and explicit `ZOPRO_CPU_FUSED_PRODUCT_OVERLAP=1`. A process OnceLock samples the flag once; omission and zero are OFF. An active allocator reclamation guard selects the sequential path. This decision is advisory at branch entry and does not reschedule a proof if pressure arrives later. Three existing core files gain additions; removing them recovers the originals byte for byte in both source versions. The other 167 accepted files, including field arithmetic and the workspace forecast, match.

The code moves existing input vectors and borrows existing key/scalar slices. It adds no key table, point copy, auxiliary heap buffer, worker or Key field. The unchanged forecast already sums FFT fields, all MSM parents and per-worker leaf scratch. More live buffers can coexist despite an unchanged upper forecast. Actual pressure behavior would require qualification before adoption.

## Complete SDK comparison

Ryzen 9950X, 16 physical cores and 32 logical CPUs. Seven rotated/reversed process blocks compare accepted, fresh OFF and ON. Every 32-worker case measures 31 requests after three primes; physical-16 primary measures 101 after three primes on CPUs 0–15. Requests include fresh solving, randomness, proving, SDK verification and serialization. Loading and primes are excluded. All latency finishes before profiles; no other owned build, profile, archive or benchmark overlaps latency.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | 8.656291 | 8.572243 | 8.670757 | 1.002668560 | 1.010655789 |
| 2x2-w32 | 11.712734 | 11.702034 | 11.639867 | 1.000374457 | 0.992597305 |
| 4x2-w32 | 16.109013 | 16.103748 | 16.234542 | 0.997617013 | 1.004400466 |
| 8x2-w32 | 24.780247 | 24.391450 | 24.629511 | 0.979286603 | 1.014947762 |
| 12x2-w32 | 34.442273 | 34.977863 | 34.610800 | 1.002632675 | 0.992319221 |
| 16x2-w32 | 41.031470 | 41.128801 | 41.501937 | 1.009020710 | 1.004943121 |
| 24x2-w32 | 68.402157 | 68.582964 | 68.668280 | 1.003081324 | 1.002553517 |
| merge-24x1-w32 | 67.369762 | 67.321777 | 68.015410 | 1.009808863 | 1.007896810 |
| merge-54x1-w32 | 176.776680 | 175.677290 | 177.759006 | 1.010570164 | 1.014863128 |
| 1x2-w16 | 10.550553 | 10.566153 | 10.757221 | 1.029470513 | 1.028947134 |

Times are medians of process medians; paired ratios can rank differently. The gate requires primary and original-eight improvement against both controls with at least 4/7 winning blocks, each case within 2% paired median and 5% in every block, and OFF calibration within a factor of 1.03. No minimum positive-gain floor, observation exclusion or unchanged retry applies. Thirteen checks fail: two primary-improvement checks, two aggregate checks, two physical-16 median checks, six physical-16 block checks and one 8x2 block check. The primary and aggregate failures independently reject the change. The branch is inactive on larger shapes; their variation does not demonstrate a causal effect of overlap.

| Unprofiled resource observation | Accepted | OFF | ON |
| --- | ---: | ---: | ---: |
| Primary CPU ms/request | 199.397 | 199.358 | 200.280 |
| Primary maximum process RSS MiB | 109.250 | 114.250 | 124.000 |
| Merge54 CPU ms/request | 4850.881 | 4830.446 | 4879.487 |
| Merge54 maximum process RSS MiB | 1908.324 | 1891.582 | 1907.449 |

CPU entries are medians of process means. RSS includes loading and primes; it is not minimum RAM or a live-scratch bound. Primary ON maximum RSS is higher in this finite observation.

## Profiles and generated code

After latency, nine warm user-cycle captures cover all variants at primary/101, 12x2/51 and merge54/9 requests. The audit verifies all tuples, round counts, FIFO warm control, monotonic clock, `--no-buildid-cache`, one sampled PID per capture, 297,412 samples and zero lost records. One capture per tuple does not establish a causal phase-cost decomposition.

| Primary profile | CPU ms/request | G1 + G2 sample % | Epoch + scheduler sample % | FFT sample % |
| --- | ---: | ---: | ---: | ---: |
| accepted | 200.170 | 67.91 | 13.08 | 4.51 |
| off | 200.218 | 67.15 | 13.80 | 4.69 |
| on | 202.074 | 66.74 | 13.99 | 4.58 |

Both actual G2 prefix specializations retain identical static counts between accepted and candidate: the true specialization has 50,288 bytes, 8,326 instructions, 1,450 IFMA sites and 5,696 bytes of literal stack subtractions. The false specialization also matches its accepted counts. ON and OFF share one executable.

| Actual canonical-wire caller | Bytes | Instructions | Literal stack subtraction bytes |
| --- | ---: | ---: | ---: |
| accepted | 6030 | 1054 | 1976 |
| candidate | 7701 | 1320 | 2072 |

The candidate additionally emits a separate fused helper. Static code/stack counts exclude dynamic execution, alignment, pushes and callees; they do not isolate a cause of the timing result. Overlapping these stages did not produce a reliable reduction in latency or CPU work. This experiment does not establish a general memory-bandwidth bottleneck.

## Validation and retained failures

The corrected candidate passes 110 unfiltered core tests in both modes, two raw-prover tests and six cubic API tests. A nested reclamation-guard test and existing independent combined-query generator tests pass. All nine shapes pass corrected OFF/ON preflight. Trace checks confirm four primary ON calls and no OFF or ineligible calls. Field validation is inherited from unchanged source and pinned reference artifacts; no new field-suite run is claimed.

The complete campaign contains 9,228 completed benchmark SDK requests, 246 distinct exports independently verified by Gnark and 492 proof/public files. This includes 36 successful initial OFF preflight requests and nine exports, followed by 9,192 corrected-candidate/control requests and 237 exports. The failed ON attempt completed no SDK request or export. Proofs have the 164-byte compressed format, distinct hashes and expected fixture public inputs. Freshness is checked against 4,321 earlier identities, then 4,330 before corrected preflight and 4,348 before performance. Two fixed source fixture members are separate. All 361 performance pins pass the closing audit.

The first enabled preflight failed an assertion added by this prototype: the Euler product returns 30,922 actual rows rather than the padded 32,768-point domain. The repair restricts overlap to Euler products and uses max(a.len(), b.len()), matching the existing product routine. Source, binary and receipts from the failure are retained; both full native modes and preflight were repeated for the changed source. No comparative latency block ran before the repair. Initial preflight SAMPLE observations remain archived and are used only as correctness evidence.

The summary command initially omitted the frozen reader's two required arguments after all measurements had completed. The corrected invocation reads a byte-identical configuration alias; no gate, source or timing rerun follows. Rebuilding also reused one scheduler-metadata pathname after the first preflight terminated. Its original JSON is preserved separately, reconstructed from retained Cargo receipts and required to match its original pinned SHA exactly; original source, libraries, executables and proofs remain unchanged. The current performance pins are unchanged in place.

Further portable, public/generic/admission-denied, concurrent SDK and cold/warm pressure qualification was not run after the failed speed gate. No mobile, huge-circuit, AWS, Aeglos or minimum-RAM result is claimed. The tested schedule is rejected without adoption or an unchanged retry.

## Supplied M5 references

Candidate diagnostics below do not replace accepted production results. CPU and Metal references are the supplied historical measurements, not fresh Apple measurements.

| Shape | ON ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.670757 | 37 | 4.27× | 24 | 2.77× |
| 2x2 | 11.639867 | 59 | 5.07× | 30 | 2.58× |
| 4x2 | 16.234542 | 109 | 6.71× | 48 | 2.96× |
| 8x2 | 24.629511 | 189 | 7.67× | 74 | 3.00× |
| 12x2 | 34.610800 | 284 | 8.21× | 101 | 2.92× |
| 16x2 | 41.501937 | 335 | 8.07× | 124 | 2.99× |
| 24x2 | 68.668280 | 528 | 7.69× | 176 | 2.56× |
| merge-24x1 | 68.015410 | 507 | 7.45× | 175 | 2.57× |

No merge54 M5 reference was supplied. The all-eight 10× CPU objective remains unmet. Accepted historical primary remains 8.594267 ms and merge54 178.891799 ms in the [production checkpoint](../2026-10-25-grouping-module/README.md).

Research review: [SQIsign AVX-512](https://arxiv.org/html/2608.13948v1) uses a special-prime reduction; [MORPH](https://arxiv.org/html/2604.17808v1) targets TPU arithmetic/dataflow. Neither supplies a measured BN254 CPU speedup for this prover. The schedule tested here is derived from the local dependency graph.

[Protocol](artifacts/experiment.json), [repair](artifacts/repair.json), [all samples](artifacts/perf/rows.json), [decision](artifacts/pilot-result.json), [activation](artifacts/activation-audit-v2.json), [profiles](artifacts/profile-analysis.json), [actual code](artifacts/codegen.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json), [metadata relocation](artifacts/pin-relocation.json), [hashes](artifacts/SHA256SUMS). Nine raw captures and nine expanded symbol streams remain hash-indexed locally and omitted from Git.
