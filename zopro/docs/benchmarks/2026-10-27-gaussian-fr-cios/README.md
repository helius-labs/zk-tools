# Coupled Gaussian CIOS component

Component screen: **fail**. The coupled CIOS butterfly is **74.0% slower than current ordinary arithmetic** and **35.4% slower than parent Gaussian Comba**, with 0/31 paired wins against either. Both new model tests, three native tests and the complete butterfly test pass; all timed final outputs verify. Calibration passes. The candidate is rejected, with no production change or verified proof speedup.

The original goal of 10× the supplied CPU reference across all eight shapes remains unmet, 0/8. This changes the Gaussian reduction recurrence while retaining the [parent headroom and lazy butterfly](../2026-10-27-gaussian-fr-headroom/README.md).

The base field uses two short Gaussian coordinates with six expanded vectors. Three coupled Montgomery cancellation rounds of52,52,26bits replace the full product and inverse convolution. Unsigned cancellation digits reconstruct the same Gaussian multiplier as the parent Comba oracle; the final digit identifies centering. Both input and modulus Gauss sum products need an explicit53-bit digit carry. Stored canonical coordinates remain32bytes; no proving-key table, duplicated plan, extra workers or C/assembly backend is introduced. This does not prove process memory equivalence.

## Measurements

Ryzen 9950X, CPU 2, one OS thread. There are 31 fixed rotated/reversed fresh-process blocks, with 100,000 iterations after 3×2,000 primes. The exact qualified parent Comba binary is reused as a contemporaneous control, alongside its ordinary field baseline. The new binary has unchanged ordinary field arithmetic, allowing a cross-binary calibration. The actual FFT field radix is 2^260 with CIOS cancellation carry enabled. Conversion is outside timing; one result-vector allocation is included. No owned helper, build, hashing, archiving or second CPU workload overlaps capture.

|Pattern|Label|Median ns/unit|
|---|---|---:|
|butterfly|cios|14.745939|
|butterfly|comba|10.912933|
|butterfly|ordinary-current|8.477519|
|butterfly|ordinary-current-r4|8.729157|
|butterfly|ordinary-parent|8.481901|
|chain|cios|4.652843|
|chain|comba|3.906945|
|chain|ordinary-current|2.875045|
|chain|ordinary-parent|2.882510|
|streams4|cios|4.034903|
|streams4|comba|3.464640|
|streams4|ordinary-current|1.966988|
|streams4|ordinary-parent|1.962214|

Chain/streams units are ns per field lane update; butterfly units are ns per four-point butterfly lane. `comba` and `cios` use the same Gaussian representation and lazy butterfly graph; labels identify different reduction algorithms.

|Comparison|Paired median ratio|Individual median interval|Wins/31|
|---|---:|---|---:|
|butterfly:cios/comba|1.353653|[1.337649,1.359895]|0|
|butterfly:cios/ordinary-current|1.739507|[1.726670,1.747926]|0|
|butterfly:cios/ordinary-current-r4|1.688757|[1.674763,1.702226]|0|
|butterfly:comba/ordinary-current|1.287816|[1.279198,1.291689]|0|
|butterfly:ordinary-current/ordinary-parent|1.000000|[0.997978,1.003258]|15|
|chain:cios/comba|1.191712|[1.187531,1.197370]|0|
|chain:cios/ordinary-current|1.616546|[1.614422,1.629593]|0|
|chain:comba/ordinary-current|1.358505|[1.355317,1.362954]|0|
|chain:ordinary-current/ordinary-parent|0.996654|[0.994334,1.001768]|18|
|streams4:cios/comba|1.164706|[1.161484,1.166753]|0|
|streams4:cios/ordinary-current|2.055084|[2.046690,2.056761]|0|
|streams4:comba/ordinary-current|1.761729|[1.760163,1.764884]|0|
|streams4:ordinary-current/ordinary-parent|1.000772|[0.997772,1.003968]|13|

Calibration failures: **0**; required gain failures: **2**. Frozen calibration requires the ordinary current/parent individual median interval within factor1.03 for all3patterns. The lazy CIOS butterfly must have an upper median bound below1 and≥21/31wins against both current ordinary-r2 and parent Comba. Ordinary-r4 comparison is diagnostic. Individual rank10..22 intervals have97.055% nominal coverage conditional on independent/exchangeable blocks, without a familywise guarantee. All31blocks/outliers retained; no gain floor, extensions or unchanged rerolls.

## Validation and limits

The new lazy butterfly function occupies 20,407 bytes of native code versus 15,537 for parent Comba; both ordinary FFT functions retain their parent sizes of 8,093 and 7,860 bytes. The four profiles contain 717 samples in total, with no lost records. These observations describe the generated implementation; they do not isolate which instructions or dependencies cause the regression. Fewer conceptual multiplication stages did not produce lower measured latency. This result does not justify another Gaussian representation sweep or a whole-proof integration.

Two new exact integer model tests validate the coupled recurrence, a separate IFMA-half coefficient interpreter, final26-bit repacking and signed corrections against unchanged Comba REDC on canonical/lazy corners,4096fullwidth changing pairs and digit boundaries. The first model build had a test-code type error; the initial interpreter then failed stage checks because it omitted the input-row53-bit sum carry. Original source and failure logs are retained. Both tests passed after correction, before native code or timing. [Coefficient bounds](artifacts/coefficient-bound.md) conservatively keep accumulation below2^59. Four unchanged parent modulus/range tests are inherited with byte-hash checks, not rerun or counted as new.

Three native tests pass exact coordinates, canonical bounds, negative/wrapped inputs,4096changing chains and lazy finishing. One additional native butterfly test checks the complete graph against scalar field arithmetic and a5×5matrix-power oracle. All403latency processes,17iteration preflight and four10million-iteration profiles independently verify all final fields after timers. These are80,300,017measured loop iterations and2,448,000prime iterations; iterations are not independent proofs. No SDK/Gnark proofs are generated; the ledger stays12,467. Source/compiler/actual Cargo libraries and both control binaries are pinned before capture and rechecked afterward.

All latency precedes four separate199Hzwhole-process profiles; all captures precede decoding. Profiles include setup/primes/post-timer oracle and are not FIFO-gated warm intervals. Each has positive samples, onePID and zero lost records; raw capture/dump hashes remain local and decoded symbols are archived. Static code size and sampled shares do not establish removable wall-time costs.

Conversion, complete FFTs, whole proving, portableSDK, concurrency, pressure, phones, TEE and250/4999 remain unqualified. Any component win must survive those costs before adoption.

## Inherited CPU and Metal comparison

The accepted172-file source and its [canonical comparison](../2026-10-27-clean-fft-adoption/README.md) remain authoritative. These are inherited local observations against supplied historical M5 figures, not observations from this experiment.

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

Accepted merge54 remains161.341835ms; no supplied M5 reference. The prior full-FFT adoption has an explicit engineering exception and its full performance gate remains false. Old /tmp SDK/rawcapture assets are unavailable; historical observations remain archived. This study uses persistent local storage. No AWS resources were started.

[Design](artifacts/design.md), [frozen declaration](artifacts/timing-declaration.json), [all observations](artifacts/analysis.json), [source pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
