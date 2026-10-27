# Gaussian Fr with deferred coordinate corrections

The component fails its screen and is not adopted. The deferred-correction butterfly takes **11.581 ns per lane versus 9.039 ns** for ordinary radix-two arithmetic. Its paired ratio is **1.296**, with 1/31 wins. This follows the rejected [R128 experiment](../2026-10-27-gaussian-fr/README.md); neither study changes production or proves a whole-proof gain. The original all-eight CPU 10× goal remains active and unmet, 0/8.

The radix is fixed at 2^130, whose square matches the FFT's 2^260 domain. With P=2^126 and H=P+3b, exact bounds allow normal products without final correction. Canonical add/sub uses one correction per axis; a lazy radix-four graph defers correction for linear operands up to 4H until twiddle multiplication, and handles its final all-sum plus recurrence constant at 5H with three corrections per axis. Power-of-two thresholds use top coefficient bits instead of full lexicographic comparisons. Temporary signed130-bit coefficients require six native vectors; canonical stored coordinates still fit two signed128-bit words (32 bytes). These bounds do not prove equal process RSS or a universal memory limit.

## Native component comparison

Ryzen 9950X, CPU2, one OS thread.31 fixed rotated/reversed fresh-process blocks,100000 iterations after3×2000 primes. The ordinary field source remains the accepted implementation; its comparator uses the actual FFT radix R260 and CIOS cancellation-carry option. Ordinary-copy runs the identical executable/function/inputs in a separate process. No owned helper/build/hash/archive/profile or second benchmark overlaps latency. Conversion is outside the timer; one result-vector allocation is included per process. Chain/streams units are ns per field lane update; butterfly units are ns per four-point butterfly lane.

|Pattern|Mode|Median ns/unit|
|---|---|---:|
|butterfly|gaussian-r2|16.4911|
|butterfly|gaussian-r4-eager|15.7792|
|butterfly|gaussian-r4-lazy|11.5813|
|butterfly|ordinary-r2|9.0389|
|butterfly|ordinary-r2-copy|8.8897|
|butterfly|ordinary-r4|8.8274|
|chain|gaussian|3.9308|
|chain|ordinary|2.9178|
|chain|ordinary-copy|2.9069|
|streams4|gaussian|3.5010|
|streams4|ordinary|1.9871|
|streams4|ordinary-copy|1.9845|

|Comparison|Paired median ratio|Individual median interval|Wins/31|
|---|---:|---|---:|
|butterfly:gaussian-r2/ordinary-r2|1.914260|[1.830480,1.956898]|1|
|butterfly:gaussian-r4-eager/ordinary-r2|1.833950|[1.786679,1.867625]|1|
|butterfly:gaussian-r4-lazy/gaussian-r2|0.703782|[0.672009,0.745190]|29|
|butterfly:gaussian-r4-lazy/gaussian-r4-eager|0.728634|[0.702288,0.801502]|30|
|butterfly:gaussian-r4-lazy/ordinary-r2|1.295983|[1.275305,1.378364]|1|
|butterfly:gaussian-r4-lazy/ordinary-r4|1.290333|[1.255298,1.420102]|0|
|butterfly:ordinary-r2-copy/ordinary-r2|0.997368|[0.992493,1.001839]|19|
|butterfly:ordinary-r4/ordinary-r2|1.002727|[0.983228,1.023388]|14|
|chain:gaussian/ordinary|1.347616|[1.338314,1.352768]|0|
|chain:gaussian/ordinary-copy|1.351079|[1.346598,1.354974]|0|
|chain:ordinary-copy/ordinary|0.995718|[0.991702,1.002034]|19|
|streams4:gaussian/ordinary|1.766280|[1.759545,1.816810]|0|
|streams4:gaussian/ordinary-copy|1.769190|[1.759451,1.802659]|0|
|streams4:ordinary-copy/ordinary|0.998984|[0.992981,1.004321]|17|

Calibration failures: **0**; required gain failures: **2**. The frozen screen requires ordinary-copy calibration intervals within factor1.03 for all three patterns, and lazy-butterfly upper median bounds below1 against ordinary-r2 and ordinary-r4 with at least21/31wins. Individual distribution-free median intervals use rank10..22, conditional on independent/exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extensions or unchanged rerolls. All eager/internal positive estimates and regressions remain in [analysis](artifacts/analysis.json).

## Validation and limits

The same-binary lazy/eager paired ratio is **0.728634**, interval **[0.702288, 0.801502]**, with 30/31 wins: a supported internal improvement of about **27.1%**. Against ordinary radix-two, the ratio is **1.295983**, interval **[1.275305, 1.378364]**; against ordinary radix-four it is **1.290333**, interval **[1.255298, 1.420102]**, with 0/31 wins. All three identical-code calibration checks pass. The normal chain and four-stream arithmetic also remain slower than ordinary arithmetic, by paired medians of about 34.8% and 76.6%. The internal gain does not waive either failed baseline comparison. This rejects the tested implementation, not every arithmetic scheme using Gaussian integers.

Four exact model tests pass modulus/isomorphism/cancellation, the new headroom inequalities, correction-free normal products, lazy4H twiddle products,5H finishing corners and fullwidth changing chains/import modeling. Three native tests independently check exact coordinates, signs, wraps, corner values, bounds and lazy finishing. A further native test compares every complete butterfly output with scalar field arithmetic and an independent5×5matrix-power oracle. These are new tests of the changed invariant; completed R128 captures are not repeated. Generated constants, source, current compiler and actual Cargo libraries are pinned before capture and rechecked afterward.

All final fields in every process pass an independent scalar affine-recurrence closed form or5×5matrix-power check after timing. There are372latency processes, one17iteration preflight and five whole-process199Hz profiles with10million iterations each:37.2million latency iterations,50million profile iterations and17preflight iterations, plus2,268,000prime iterations. Loop iterations are not independent verification samples. No SDK or Gnark proofs are generated; the proof ledger remains12,467.

The five profiles follow all latency; decoding follows all captures. They include setup/primes/post-timer reference work and are not FIFO-gated warm intervals. Zero lost records and one sampledPID per capture are checked. Raw captures/dumps are local with hashes and decoded symbols are archived. Profiles and static code sizes do not isolate removable wall-time costs.

Conversion, complete FFTs, proving, portable whole-SDK behavior, concurrent callers, memory pressure, phone, TEE and250/4999 are unqualified. Any component win must survive these costs and full-proof validation before adoption. No key table, copied proving key, duplicate plan, extra worker pool, C or assembly backend is introduced.

## Accepted CPU and Metal comparison

The accepted172-file source and its [canonical all-eight CPU/Metal comparison](../2026-10-27-clean-fft-adoption/README.md) remain authoritative. These are inherited local observations against supplied historical M5 figures, not timings from this component.

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

Accepted merge54 remains161.341835ms; no supplied M5 reference. The old /tmp workspaces, rawcaptures and binaries are currently absent. Archived sources/results/verification records remain; this workspace is persistent under /home/ubuntu/src/zopro-perf-work/gaussian-fr-r130. No completed benchmark is repeated to restore missing temporary assets.

[Design](artifacts/design.md), [exact bounds](artifacts/model-gate.json), [frozen declaration](artifacts/timing-declaration.json), [all observations](artifacts/analysis.json), [source pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
