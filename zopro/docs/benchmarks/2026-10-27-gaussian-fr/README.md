# Gaussian integer representation for BN254 Fr

The native component fails its screen. Gaussian radix-four butterflies take **19.020 ns per lane versus 8.459 ns** for ordinary arithmetic; the paired ratio is **2.247**, with 0/31 wins. The candidate is not integrated or adopted. The all-eight CPU 10× objective remains active and unmet, 0/8.

The BN parameter gives r=a²+b², a=6u²+3u and b=3u+1. The prototype represents Fr as Gaussian integers modulo a+bi, with Montgomery radix 2^128 and two signed 128-bit coordinates. Fixed correction bounds keep each coordinate within H=floor(a/2)+b. The stored pair occupies 32 bytes; expanded SIMD uses six vectors versus the ordinary field's five. Multiplication by i becomes a sign change and swap. A radix-four butterfly can therefore use three general products instead of four, while general products, corrections and conversions can cost more. No key table, duplicate plan, per-key cache, new worker pool, C or assembly backend is added.

The [primary-source review](artifacts/research.md) distinguishes the Gaussian residue-ring idea from earlier Gauss Fq2 arithmetic and already reviewed SIMD papers. Source-level product savings are not CPU or prover savings. This targets FFTs; primary's small FFT profile share gives no2×primary prediction.

## Native component results

Ryzen 9950X, CPU 2, one OS thread. There are 31 fixed rotated/reversed fresh-process blocks, with 100,000 iterations after three 2,000-iteration primes. Identical ordinary-copy processes use the same executable, function and inputs. No owned build/helper/hash/archive/profile or other benchmark overlaps latency. Input and output conversions are outside timers; one result-vector allocation is inside each process timer. Each result is checked after timing by a scalar affine-recurrence closed form or independent 5×5 matrix power, with every field output consumed. Chain/streams units are ns per field-lane update; butterfly units are ns per four-point butterfly lane. These are component costs, not proof latency.

|Pattern|Mode|Median ns/unit|
|---|---|---:|
|butterfly|gaussian-r2|21.1386|
|butterfly|gaussian-r4|19.0199|
|butterfly|ordinary-r2|8.4586|
|butterfly|ordinary-r2-copy|8.4454|
|butterfly|ordinary-r4|8.6922|
|chain|gaussian|5.5482|
|chain|ordinary|2.8718|
|chain|ordinary-copy|2.8625|
|streams4|gaussian|5.0901|
|streams4|ordinary|1.9529|
|streams4|ordinary-copy|1.9551|

|Comparison|Paired median ratio|Individual median interval|Wins/31|
|---|---:|---|---:|
|butterfly:gaussian-r2/ordinary-r2|2.499357|[2.492151,2.526143]|0|
|butterfly:gaussian-r4/gaussian-r2|0.899782|[0.888267,0.902295]|31|
|butterfly:gaussian-r4/ordinary-r2|2.246863|[2.243005,2.257718]|0|
|butterfly:gaussian-r4/ordinary-r4|2.186907|[2.177996,2.198783]|0|
|butterfly:ordinary-r2-copy/ordinary-r2|0.998255|[0.996588,1.003053]|20|
|butterfly:ordinary-r4/ordinary-r2|1.029770|[1.022088,1.033013]|4|
|chain:gaussian/ordinary|1.930466|[1.925320,1.935749]|0|
|chain:gaussian/ordinary-copy|1.938393|[1.932510,1.942484]|0|
|chain:ordinary-copy/ordinary|0.995910|[0.993466,0.998746]|22|
|streams4:gaussian/ordinary|2.605396|[2.602181,2.608223]|0|
|streams4:gaussian/ordinary-copy|2.604218|[2.600213,2.607623]|0|
|streams4:ordinary-copy/ordinary|1.000597|[0.998931,1.002936]|14|

Calibration failures: **0**; required butterfly-gain failures: **2**. The frozen gate requires ordinary-copy calibration intervals within factor1.03 for all three patterns, Gaussian radix4 strict gain upper bounds below1 against both ordinary radix2/radix4 and at least21/31wins. Chain and streams regressions remain diagnostic and cannot be ignored in any eventual fullFFT integration. Individual distribution-free median intervals use rank10..22 at31blocks under independent/exchangeable block assumptions, not familywise guarantees. No minimum gain floor, sample exclusion, extension or unchanged reroll.

## Correctness, conversion and limits

Radix four improves the Gaussian implementation itself by a paired **10.02%**, with 31/31 wins over Gaussian radix two. That positive result does not overcome the new representation's arithmetic cost: the dependent chain is about 1.93× and four-stream arithmetic about 2.61× the ordinary field's cost. Both required butterfly-gain checks fail; all identical-control calibration checks pass. This rejects the present representation and correction schedule, not every possible Gaussian arithmetic implementation.

Three exact Rust-model tests pass the modulus/isomorphism, all coordinate corners, centered cancellation, closure inequalities, rotation, changing fullwidth chains and host import model. Two native tests pass exact integer-coordinate and scalar-field comparisons, signs/bounds and changing chains. One further native test checks every output of both butterfly graphs and the independent affine-power oracle. Successful tests were not rerun. The existing ordinary scalar-field source is unchanged; its inherited field qualification is separate. New generated constants, sources, actual Cargo libraries and compiler identities are pinned before capture and checked afterward.

The butterfly/chain benchmark excludes representation conversion. Its output conversion uses an exact model after timing; there is no optimized conversion/fullFFT/prover integration. A component pass would still require these costs, root preparation/storage, memory forecasts, native/portable fallbacks, concurrency and pressure qualification. Native working vectors grow from five to six;32-byte stored coordinates do not prove equal process RSS or a universal memory bound. Scalar Gaussian correction is generally noncanonical; this stage checks its field interpretation rather than changing serialization.

The four separate 199 Hz profiles follow all latency and include the whole process: setup, primes, timed loop and post-timer reference. Their 950 samples contain no lost records and one sampled PID per capture. They are not FIFO-gated warm intervals, useful-work fractions or isolated critical-path measurements. Raw captures/dumps remain local with hashes; decoded symbols are archived. The initial offline reader expected separate PID/TID columns; the [reader repair](artifacts/parser-repair.json) handles the actual combined token using a separate finish helper. Frozen sources, binaries and pins remain unchanged, and no successful capture, test or output check is repeated.

There are 341 latency processes, one preflight and four profiles: 34.1 million unprofiled measured iterations, 40 million profile iterations and 17 preflight iterations, plus 2,076,000 prime iterations. Every process checks all final fields; loop iterations are not independent verification samples. No SDK or Gnark proofs were generated; the independent proof ledger remains 12,467. No AWS, Aeglos, phone, TEE, pressure, large-circuit or full-proof gain is claimed.

## Accepted CPU and Metal comparison

The [accepted all-eight report](../2026-10-27-clean-fft-adoption/README.md) remains authoritative. The following unchanged local accepted observations use supplied historical M5 CPU/Metal references; they are not results of this component.

|Shape|Accepted ms|CPU reference ms|CPU/accepted|Metal reference ms|Metal/accepted|
|---|---:|---:|---:|---:|---:|
|1x2|8.324853|37|4.445×|24|2.883×|
|2x2|11.460675|59|5.148×|30|2.618×|
|4x2|15.854365|109|6.875×|48|3.028×|
|8x2|24.084626|189|7.847×|74|3.072×|
|12x2|33.693397|284|8.429×|101|2.998×|
|16x2|39.981636|335|8.379×|124|3.101×|
|24x2|66.010670|528|7.999×|176|2.666×|
|merge-24x1|65.086624|507|7.790×|175|2.689×|

Accepted merge54 remains161.341835ms with no supplied M5 reference. Production172files and accepted14daSDK are unchanged.

[Declaration](artifacts/timing-declaration.json), [exact model bounds](artifacts/model-gate.json), [all results](artifacts/analysis.json), [source pins](artifacts/pins.json), [Cargo artifacts](artifacts/native-cargo-artifacts.jsonl), [artifact hashes](artifact-index.json).
