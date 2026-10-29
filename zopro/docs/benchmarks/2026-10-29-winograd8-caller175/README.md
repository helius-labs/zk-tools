# Eight-point factored NTT caller

The declared caller gate is **false**. This independently derived finite-field DFT8 replaces two eight-lane vector products with one, adding linear arithmetic and lane permutations. It transforms two independent eight-point inputs and returns the same bit-reversed outputs. No real FFT or SDK implementation is changed or adopted.

The research lead is [Ingonyama's Winograd NTT note](https://www.ingonyama.com/oldblogs/pushing-the-limits-on-ntt-computation). All six pages were reviewed. Its hardware counts are not CPU speed predictions. The upstream GPL implementation was not ported; this kernel follows the independently derived [DFT8 identity](artifacts/algebra.md). The current control composes the original DIF butterfly with the verbatim accepted dif_within function. Both link the exact accepted native Fr R260 library; all 175 production hashes remain exact. Earlier fixed/serial leaves, pass grain and Euler compaction/fusion studies remain separate results.

## Complete leaf arithmetic

Ryzen 9950X, CPU 2, one thread. Four primitive eighth roots, 31 fixed rotated/reversed process blocks per root, three labels. A and B use identical original code. Each process runs 192 priming calls, resets outside the timer, then performs 8,191 complete transformations on 64 changing preallocated fixtures. All loads, shuffles, arithmetic, stores and blackboxing are timed; construction and independent output checks are outside. All 1,024 final field outputs are checked. There is no owned build/helper/hash/archive/second workload during capture and no exclusive-host claim.

| Root power | A ns | B ns | Factored ns | Factored/A ratio [bounds] | Wins | Factored/B ratio [bounds] | Wins |
|---|---:|---:|---:|---|---:|---|---:|
| 1 | 54.205 | 54.333 | 60.924 | 1.124259505 [1.121145568, 1.131416409] | 0 | 1.120071474 [1.116484380, 1.128377037] | 0 |
| 3 | 54.273 | 54.289 | 60.977 | 1.122523783 [1.119122743, 1.127750906] | 0 | 1.122556688 [1.121340507, 1.131249647] | 0 |
| 5 | 54.221 | 54.268 | 60.908 | 1.122720358 [1.120436419, 1.124739579] | 0 | 1.120783590 [1.118807017, 1.124192813] | 0 |
| 7 | 54.218 | 54.150 | 61.045 | 1.124973195 [1.121099814, 1.132073480] | 0 | 1.125083071 [1.121835307, 1.136277273] | 0 |

Failed checks: `["p1/candidate/A","p1/candidate/B","p3/candidate/A","p3/candidate/B","p5/candidate/A","p5/candidate/B","p7/candidate/A","p7/candidate/B"]`. Each root must improve against both controls with upper ratio below one and at least 21/31 wins. All B/A intervals must fit [1/1.03,1.03]. Bounds use sorted ranks 10–22 of 31, nominal 97.055% per comparison under independent exchangeable blocks, without familywise coverage. No minimum gain floor, exclusions, sample extensions, threshold changes or successful-capture repeats. Failure closes this vector factorization; passing only permits a separate real-FFT caller assessment including twiddles, packing and dataflow before SDK qualification.

## Correctness and scope

Three new native tests pass: all 16 basis inputs for each primitive root (both independent sequences); 4,096 boundary/full-width fixtures against scalar direct DFT; and changing recurrence at seven lengths through 8,191 calls, with every final fixture checked against independently exponentiated DFT matrices. Basis equality proves this linear identity over the field assuming the inherited field arithmetic. It does not prove an unimplemented large-FFT decomposition. Existing 33-test field qualification is inherited through exact source/artifact identity, not rerun.

The kernel adds no unsafe code, assembly, heap allocation, key table or input-sized buffer. Both methods share a three-vector constant payload, approximately 960 bytes per root; compiler frames and caller buffers are additional. These are not process RSS, minimum RAM or pressure bounds. Root constants are computed before priming. Extra additions and shuffles can erase multiplication savings.

All 372 latency processes finish before four whole-process 199 Hz cycles profiles. Profiles include setup, primes and output checks, are sparse, and do not isolate kernel costs or removable proof wall time.

| Profile | Samples | PIDs | Lost records |
|---|---:|---:|---:|
| cycles-p1-A | 10 | 1 | 0 |
| cycles-p1-candidate | 11 | 1 | 0 |
| cycles-p3-A | 11 | 1 | 0 |
| cycles-p3-candidate | 11 | 1 | 0 |

All **3643528 complete leaf calls including primes** finish with matching checked outputs. There are **zero new SDK requests or proofs**; the independent-proof ledger stays **17,239**. No new full-proof, AWS/Aeglos, portable/phone/TEE, giant, active-pressure, concurrency, RSS or minimum-RAM result follows. The original all-eight CPU 10× objective remains active and unmet, 0/8.

## Inherited CPU and Metal comparison

Accepted source 175 measurements and supplied historical M5 references follow. These are not new measurements or candidate proof speeds. The accepted physical-16 baseline-calibration exception remains recorded in its adoption report.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU 10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 37.0 | 4.612× | 24.0 | 2.992× | 3.7 |
| 2x2 | 11.335969 | 59.0 | 5.205× | 30.0 | 2.646× | 5.9 |
| 4x2 | 15.945544 | 109.0 | 6.836× | 48.0 | 3.010× | 10.9 |
| 8x2 | 24.090485 | 189.0 | 7.845× | 74.0 | 3.072× | 18.9 |
| 12x2 | 33.844855 | 284.0 | 8.391× | 101.0 | 2.984× | 28.4 |
| 16x2 | 39.931557 | 335.0 | 8.389× | 124.0 | 3.105× | 33.5 |
| 24x2 | 66.233309 | 528.0 | 7.972× | 176.0 | 2.657× | 52.8 |
| merge-24x1 | 65.273224 | 507.0 | 7.767× | 175.0 | 2.681× | 50.7 |

Accepted 1×2 remains 8.022165 ms with 134 MiB peak RSS; its CPU target is 3.7 ms. Merge54 remains 162.250464 ms without a supplied M5 reference. [Design](artifacts/design.md), [algebra](artifacts/algebra.md), [analysis](artifacts/analysis.json), [native tests](artifacts/tests.log), [driver](artifacts/driver.rs), [commands](artifacts/run-plan.json), [capture completion](artifacts/runs/result.json), [research provenance](artifacts/research.json).
