# Fixed BN254 parameter addition/subtraction chain

The actual-library component passes its declared gate. Production remains unchanged; this does not establish a whole-proof speedup.

The paired improvement is 17.6% against frozen and 17.5% against fresh OFF, with seven wins against each. Displayed medians differ by 32.5 µs; paired ratios use all seven within-block ratios and therefore differ from the ratio of displayed medians. This is about 0.4% of the historical 8.56 ms primary proof time, before SDK/code-generation/measurement effects. It cannot explain or close the remaining multi-millisecond gap.

This tests Gnark Crypto v0.21.0's fixed Expt schedule:60 subgroup squares,17 full extension products and5 conjugations for X=4965661367192848881. The accepted binary method uses63 squares/28 products. Three powers per final exponentiation remove9 subgroup squares and33 full extension products at source level. The general exponent, Miller loop, base-field kernels and hard-part formula are unchanged. No heap table, retained input-dependent state or prepared proof is added. Stack footprint and complete SDK memory remain unqualified.

The mathematical schedule is independently translated from [Gnark Crypto's source](https://github.com/Consensys-Incorporated/gnark-crypto/blob/v0.21.0/ecc/bn254/internal/fptower/e12_pairing.go). The previous signed-window4 candidate and its failed production/library comparisons remain closed; this is a different fixed schedule with fewer operations, selected once at the full function entry.

The frozen control is the actual accepted170-source native library. OFF/ON use a freshly built candidate library, linked with the identical fat-LTO consumer. Seven rotated/reversed fresh-process blocks, CPU2,31timed calls after3primes. Every process compares275 deterministic raw/edge/actual-Miller fixtures with the frozen public function before timing; expected outputs are checked outside every timer. All inputs are regenerated, with matching block indices across modes. Inputs/reference setup are excluded; call and output materialization are included. No successful capture is excluded or repeated.

|Mode|Median of block medians µs|Whole-process peak RSS MiB|
|---|---:|---:|
|frozen|164.591000|2.50|
|off|164.390000|2.50|
|on|132.079000|2.50|

RSS includes fixtures/references and is not minimum RAM.

The policy flag is an eight-byte process-latched configuration object in this executable, inspected with `nm -S`. It retains no input-dependent value. Its one-time environment lookup happens during fixture checks, before primes; warm calls avoid per-call environment allocation. Before any timings, this replaced the first draft's repeated environment lookup, followed by a new native qualification and link. Both drafts and their test/pin records are retained under [pre-capture-config](artifacts/pre-capture-config). The earlier snapshot repair is documented in [snapshot-repair.md](artifacts/snapshot-repair.md).

|Comparison|Paired geometric ratio|Wins of7|
|---|---:|---:|
|off_frozen|0.999359838|5|
|on_frozen|0.824243625|7|
|on_off|0.824771612|7|

Calibration requires OFF/frozen within factor1.03 each block and1.02 paired median. ON must improve against both controls with4/7 wins; no gain floor.

calibration_failures:

- None.

failures:

- None.

All35 native field tests pass, including symbolic exact-chain operation counts, independent general powers, complete final exponentiation and existing Arkworks references. Native tests enable ON. The library consumer uses both public modes and all275 fixture comparisons in every process. Production170 source hashes are checked unchanged.

All latency finishes before three warm FIFO-gated199Hz user-cycle profiles,512timed+3primes per mode. Profile rows are descriptive and excluded from timing ratios. perf uses --no-buildid-cache.

|Mode|Samples|Lost|PIDs|
|---|---:|---:|---:|
|frozen|16|0|1|
|off|15|0|1|
|on|6|0|1|

Only 37 profile samples were collected across the three short captures. That is insufficient for fine symbol attribution; no hotspot or causal conclusion is drawn from them, and no longer capture was substituted after observing this limitation.

There are2259 complete final-exponentiation timed/priming calls,6600 fixture comparisons and6600 separate frozen reference setup calls. Zero SDK/Gnark proofs; prior6562 exports unchanged. No source adoption. A component pass only motivates separate full-nine SDK/Gnark, consumer and memory qualification. [Latest full local table](../2026-10-26-g1-quarter-current/README.md) retains supplied CPU and Metal references; component microseconds must not be substituted for whole-proof timings. No AWS/Aeglos/phone/TEE/large/pressure/throughput qualification or claim of the all-eight10× goal.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [provenance](artifacts/provenance.json), [artifact index](artifacts/artifact-index.json). Binaries and raw profiles stay local with hashes.
