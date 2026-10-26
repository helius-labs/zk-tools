# Deferred FFT correction in complete SDK requests

**The fixed full-proof screen fails; the candidate is not adopted.** The earlier [component](../2026-10-26-deferred-dif32-component/README.md) reduced isolated DIF32 tile time by about 32%, but the original-eight paired geometric SDK aggregate is essentially flat: 1.000160998 versus accepted and 1.000013540 versus fresh OFF. Each aggregate wins 3/7 blocks. Production remains unchanged.

Primary process-median latency is 8.580045 ms accepted, 8.590735 ms OFF and 8.569553 ms ON. Paired primary ratios improve by 0.45% and 0.18%, with 4/7 and 5/7 wins. Those small primary improvements do not pass the complete gate: one primary block exceeds 5% regression, the original-eight aggregates fail improvement, and the physical-16 control and one candidate block fail separately. All six failed checks and all observations are retained. There is no unchanged retry or selective adoption.

## Integration

The two field files are byte-identical to the pushed component. A native Plan samples `ZOPRO_CPU_DEFERRED_DIF32=1` once at construction; omission and zero are OFF. Only ordinary canonical roots can enable the new DIF32 path. Empty and multiplicatively twisted roots remain OFF: multiplication can leave twisted roots above p, outside this routine's stronger precondition. DIT, shorter expanded leaves, portable/generic algorithms and the original OFF body remain.

The callback borrows exactly the existing cached or uncached Quad roots. It adds no 31-root array, key data, retained root cache, preparation, worker or per-proof heap buffer. Standard environment parsing may allocate an ephemeral flag string during Plan construction. The arithmetic returns normalized values below 2p before existing compact storage. Removing the additions recovers both original core files byte for byte.

Native layout probes explicitly link the actual accepted and candidate libraries: both Plans are 1,728 bytes with 32-byte alignment. The candidate Twiddles type is 448 bytes and Pair is 320 bytes. The existing Plan footprint is preserved in these native builds; this is not a whole-process memory or minimum-RAM claim.

## Complete SDK timing

Ryzen 9950X, 16 physical cores/32 logical CPUs. Seven rotated/reversed separate-process blocks compare accepted, fresh OFF and ON. Every 32-worker case measures 31 fresh requests after three primes; primary-16 measures 101 after three primes, pinned to CPUs 0–15, one logical CPU per physical core. All latency finishes before profiles. Each request includes fresh solving/randomness/proving, SDK verification and serialization. Key loading and primes are excluded from latency. No other owned build, profile, archive or benchmark overlaps timing.

| Case/workers | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | 8.580045 | 8.590735 | 8.569553 | 0.995463544 | 0.998233595 |
| 2x2-w32 | 11.726016 | 11.694908 | 11.679797 | 0.999340419 | 0.997126613 |
| 4x2-w32 | 16.106504 | 16.229896 | 16.118517 | 1.002613789 | 0.994991157 |
| 8x2-w32 | 24.491386 | 24.230481 | 24.808428 | 1.002907141 | 1.005634134 |
| 12x2-w32 | 34.853869 | 34.555339 | 34.285069 | 0.988006296 | 0.993944970 |
| 16x2-w32 | 40.861940 | 40.893080 | 40.829698 | 1.006034329 | 1.003061031 |
| 24x2-w32 | 68.866157 | 68.939306 | 68.363464 | 0.990981365 | 0.997636425 |
| merge-24x1-w32 | 67.608589 | 67.542462 | 67.789058 | 0.995354881 | 0.996641799 |
| merge-54x1-w32 | 176.381251 | 177.750341 | 176.980118 | 0.996509319 | 0.997147505 |
| 1x2-w16 | 11.032182 | 10.418992 | 10.476766 | 0.949654928 | 1.006534696 |

Primary-32 and original-eight aggregate must improve against both controls with at least 4/7 winning blocks. Every case must stay within 2% paired median and 5% in every block of both controls; OFF calibration must stay within a factor 1.03. There is no minimum positive-gain floor or sample exclusion. Primary block 0 has ON/OFF ratio 1.055719315. Physical-16 OFF calibration is 1.059895196, and its block 1 ON ratios are 1.085776969/1.090784326. These failures remain alongside the two flat original-eight aggregate failures. The overall decision is not caused solely by physical-16 variability.

| Unprofiled resource observation | Accepted | OFF | ON |
| --- | ---: | ---: | ---: |
| Primary CPU ms/request | 200.146 | 199.114 | 198.624 |
| Primary maximum process RSS MiB | 115.250 | 113.000 | 114.000 |
| Merge54 CPU ms/request | 4864.632 | 4871.662 | 4855.840 |
| Merge54 maximum process RSS MiB | 1923.406 | 1898.594 | 1890.750 |

CPU entries are medians of per-process means. RSS includes key loading, setup and primes. Finite RSS observations are not minimum RAM, live scratch bounds or pressure qualification.

## Profiles and linked code

After all latency, nine warm 999 Hz user-cycle captures cover every variant at primary/101, 12x2/51 and merge54/9 measured requests after three primes. All 301,520 samples have zero lost records and one process ID per capture. The closing audit independently checks all nine declared tuples and their round counts.

Primary sampled FFT share is 4.69% OFF and 5.04% ON; the targeted inverse stored-column wrapper is 1.015% and 1.213% of sampled user-cycle weight. G1/G2 arithmetic together remain about 67% in both captures. These percentages do not measure critical-path time, removable latency, or the causal effect of a particular load or spill. One capture per tuple does not establish a CPU-work improvement.

| Primary profile | Accepted | OFF | ON |
| --- | ---: | ---: | ---: |
| CPU ms/request | 200.321 | 199.455 | 199.898 |

The real Euler path has one packed inverse and one packed forward transform, plus separate weighting and product passes. Only eligible outer inverse column tiles change; expanded leaves and the forward transform retain their original work. Thus the isolated 32% tile result describes a much narrower workload than complete proving. It also lacks the full caller's packing, dispatch and other code.

| Actual packed DIF wrapper | Bytes | Instructions | Static IFMA sites | RSP operand lines |
| --- | ---: | ---: | ---: | ---: |
| accepted | 15569 | 2893 | 200 | 693 |
| candidate | 20423 | 3861 | 400 | 751 |

The candidate wrapper retains OFF/ON and cached/uncached alternatives. Its 400 static IFMA sites do not mean an executed tile doubles its multiplications. Literal prologue stack subtractions sum to 17,024 bytes in both versions, excluding alignment, pushes and callees. The forward wrapper's static counts match. Code growth and stack-reference counts are observations, not proof that instruction-cache or register costs explain the result. Exact disassemblies are retained.

## Validation and decision

Both native modes pass 113 unfiltered core tests; the raw prover passes two tests and the cubic API six. Four new tests check 12,800 actual column outputs against the existing four-lane reference, 1,209,320 actual root lanes for the canonical bound, 12 cached columns, full packed/legacy-quad transform agreement through 262,144 fields, default/explicit/empty/twisted dispatch policy, and four concurrent callers with 512 KiB stacks. Repeated OFF/ON runs reuse the same mathematical cases. The component's 33 field tests and six independent integer/Fourier oracles are inherited only after exact field-byte identity is verified; no new field-suite run is claimed here.

All nine Euler shapes pass OFF/ON preflight at four workers. The complete campaign contains **9,192 benchmark SDK requests, 237 distinct exports independently verified by Gnark, and 474 proof/public files**. Every exported proof has the 164-byte compressed format, distinct hash, expected fixture public inputs, and no collision with 4,084 prior proof identities. The two fixed source fixtures are separate. All 336 performance pins pass the closing audit.

The native build, preflight and fixed measurement sequence finish without a source, arithmetic or timing repair. Additional uncached/preparation-denied SDK compatibility, portable, concurrent whole-SDK, finite-pressure and large-circuit qualification are not run after the failed speed gate. No AWS, Aeglos or mobile comparison is added.

The tested integration is preserved without adoption or an unchanged reroll. The component result remains valid for its own scope. The next research priority is the G1/G2 arithmetic that dominates primary CPU work, with a derivation and actual linked-code/resource audit before another full trial. This result does not establish a general memory bottleneck or justify another FFT parameter sweep.

## Supplied M5 references

Candidate diagnostics below do not replace the accepted prover. CPU/Metal numbers are the supplied historical references, not fresh Apple measurements.

| Shape | ON ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.569553 | 37 | 4.32× | 24 | 2.80× |
| 2x2 | 11.679797 | 59 | 5.05× | 30 | 2.57× |
| 4x2 | 16.118517 | 109 | 6.76× | 48 | 2.98× |
| 8x2 | 24.808428 | 189 | 7.62× | 74 | 2.98× |
| 12x2 | 34.285069 | 284 | 8.28× | 101 | 2.95× |
| 16x2 | 40.829698 | 335 | 8.20× | 124 | 3.04× |
| 24x2 | 68.363464 | 528 | 7.72× | 176 | 2.57× |
| merge-24x1 | 67.789058 | 507 | 7.48× | 175 | 2.58× |

No merge54 M5 reference was supplied. The original all-eight 10× CPU objective remains unmet. Accepted historical primary is 8.594267 ms and merge54 is 178.891799 ms in the [production checkpoint](../2026-10-25-grouping-module/README.md).

[Protocol](artifacts/experiment.json), [full decision and failed checks](artifacts/pilot-result.json), [all samples](artifacts/perf/rows.json), [source audit](artifacts/source-audit.json), [actual linked code](artifacts/codegen.json), [profile observations](artifacts/profile-analysis.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Nine raw captures and nine expanded symbol streams remain hash-indexed locally and are omitted from Git.
