# Exact experimental G2 scalar-grain follow-up

The earlier experimental candidate remains **pending adoption**, with its successful 12-check primary campaign preserved. Its separate unconditional default variant failed broader qualification; that does not reject this candidate. This follow-up reuses the exact measured SDK and runtime source, reviews the retained all-shape control evidence, and prepares the missing prospective adoption qualification. The new target run has not started; production is unchanged.

Candidate PARENT is `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`, accepted REF is `d5e1b2c981fb9bcab2beb69e3b2af5608d298d792c7e3814e87273b04509662e`. No prover/SDK is rebuilt. Native 127 core, two prover and six cubic tests, generic 102 core/two prover tests and prior all-nine functional checks are reused after source/frozen-library/consumer hash checks. The original [primary campaign](../2026-11-11-phi-grain-c7a176/README.md) and [separate default-build campaign](../2026-11-11-phi-grain-clean176/README.md) retain their original rules and decisions.

The following **retrospective** PARENT/REF comparisons use validated retained control samples from the later nine-block campaign. These were not declared adoption gates for PARENT and do not retroactively qualify it. Latency is the median of nine process means; paired ratios use same-context/block ratios. Rank intervals are nominal 96.09375% marginal under independent exchangeable blocks, with no joint-confidence or stationarity guarantee.

|Context|REF ms|PARENT ms|Paired median ratio|Rank interval|CPU ratio|PARENT peak RSS MiB|
|:---|---:|---:|---:|:---|---:|---:|
|12x2-w64|34.051144|33.963240|0.997418|[0.990570, 1.008330]|0.980659|702.31|
|16x2-w64|38.794493|39.129346|1.008923|[0.991066, 1.016276]|0.988647|821.26|
|1x2-w16|16.206219|16.207738|0.999856|[0.995616, 1.004154]|1.000221|82.79|
|1x2-w32|11.436294|11.324571|0.990250|[0.982720, 0.998566]|0.990811|125.48|
|1x2-w64|11.379093|11.596753|1.009944|[0.965121, 1.040497]|0.967296|128.43|
|1x2-w96|13.545724|12.802860|0.946937|[0.924584, 0.956665]|0.875672|124.07|
|24x2-w64|54.147074|54.339839|0.999528|[0.996526, 1.013400]|0.990066|1292.34|
|2x2-w32|15.271225|15.166862|0.996520|[0.988046, 0.999296]|0.994002|266.61|
|2x2-w96|18.335000|17.349968|0.946276|[0.931722, 0.965148]|0.889703|249.90|
|4x2-w64|19.254783|19.107621|0.995154|[0.980690, 1.010520]|0.980143|296.02|
|8x2-w64|26.365444|26.288512|0.997435|[0.989814, 1.001643]|0.974323|497.57|
|merge-24x1-w64|57.568461|57.734686|0.993717|[0.984785, 1.015557]|0.983480|1172.55|
|merge-54x1-w64|124.312911|123.169610|0.999629|[0.976509, 1.020851]|0.998090|2130.14|

The retained evidence supports useful 96-worker improvements for both 1×2 and 2×2, with lower CPU use. Primary64 and merge54 have intervals too broad to establish a 2% nonregression bound. This motivates a separate, fixed 31-block qualification across all thirteen contexts against accepted REF only; the failed default campaign is neither extended nor reinterpreted. The [frozen protocol](artifacts/protocol.md) requires all forty latency/CPU/RSS/primary-gain checks, with no gain floor, exclusions, threshold changes or sample extensions. Sorted ratio ranks 9/21 give nominal 97.055062651634% marginal coverage, verified by independent binomial/Pascal models. No fresh Aeglos comparison is planned.

Production activation is staged separately: retain the exact experimental `g2_phi.rs` body and cfg branches, then register/emit `zopro_phi_grain` beside the existing native-IFMA dense-reset selector. This follows the [busy-reset promotion](../2026-11-09-busy-reset-adoption176/README.md): the measured manual SDK remains the performance artifact. Ten finite build-script contexts preserve existing dense/GPU selection, and an actual Cargo core check selects phi without an explicit phi flag. These are build-selection/type checks, not a newly timed default Cargo SDK. Production has not been edited.

The source review preserves the same pure per-item operation on disjoint mutable scalar slices. Rayon's minimum-grain producer delegates iteration, splitting and folding; no new buffer, allocation, lookup table, key plan, pool, shared mutable state, unsafe code, arithmetic, forecast or RAM cap is introduced. Accepted functional concurrency qualification (two shared-image cases and one mixed-key case in a single 32-worker pool) is inherited with this explicit source argument; no new concurrent workload or throughput claim is made. There is no physical ARM/phone/TEE, huge-circuit, pressure/swap or minimum-RAM result.

The new local harness check uses the two retained primary consumers at four workers: eight SDK calls and two fresh externally Gnark-verified 164-byte exports, with an independent binding/freshness/sample/placement reader. Canonical verified exports advance **50,752 → 50,754**. Previous all-nine correctness work is reused. The future target contains 57,268 calls / 832 fresh exports / 54,772 timed samples including preflight, for a final ledger of 51,586 only if every output verifies. Target latency and all-eight CPU/Metal comparisons remain pending. The full all-eight CPU 10× goal remains unmet, 0/8.

Preparation failures are retained: an exact text rewrite initially expected different line wrapping; later a Docker workflow was started before a freeze helper had finished, and input pin validation rejected the overlap. Resume rejected its changed protocol before any command. After the freeze helper's authoritative completion, the initial no-command attempt was preserved and the final image workflow ran successfully. All of these failures preceded new SDK calls; no successful science was replayed. A source review also corrected a median-arm omission before preflight, without changing the frozen statistical method.

All SDK preflight work is serialized, with only the confirmed live handle polled until terminal. The final image is built and pinned. No new AWS instance or transfer resource has been created at publication. The next step is credential/resource revalidation and one scoped C7a run under the frozen protocol, followed by cleanup and an explicit adoption decision.

[Retrospective analysis](artifacts/retrospective.json), [source review](artifacts/source-review.json), [release review](artifacts/release-review.json), [selector contexts](artifacts/selection-cases.json), [actual Cargo selection](artifacts/selection-cargo.json), [statistical models](artifacts/models-review.json), [local proof/sample review](artifacts/local-review.json), [publication checks](artifacts/preparation-publication.json), [sources, receipts and local public proofs](artifacts/preparation-sources-receipts-public-proofs.tar.gz), [checksums](SHA256SUMS). The archive round-trips exact bytes and hashes. Executables, compiled libraries, key images/private inputs and credential-bearing files are excluded. Reproduction needs the pinned dependencies and private inputs.

Local root: `/home/ubuntu/src/zopro-perf-work/phi-grain-parent176`; inherited SDK: `/home/ubuntu/src/zopro-perf-work/phi-grain-c7a176/sdk-candidate-znver4`.
