# Accepted Zopro and pinned Aeglos on C7a

Current accepted Zopro measures **11.243892 ms** for 1×2 at 32 workers, versus pinned Aeglos **11.870012 ms** at the same count. The paired Zopro/Aeglos ratio is **0.950116 [0.942381, 0.952890]**, with 15/15 wins. Aeglos reaches **7.902802 ms** at 96 workers; Zopro's fastest tested count remains 32. At each engine's fastest tested count, Zopro has the lower displayed median on six of nine circuits; Aeglos leads on 1×2, 2×2 and 4×2. Selecting the fastest count is descriptive; the paired comparisons below use matched worker counts.

Peak RSS at the respective fastest counts is **122.52 MiB for Zopro** and **479.98 MiB for Aeglos** on 1×2. For 54→1, Zopro at 64 workers measures **120.757128 ms / 2,003.02 MiB**, versus Aeglos at 96 workers at **171.908504 ms / 13,102.11 MiB**. RSS includes loading, preparation and primes. These observations do not establish minimum RAM requirements.

The complete CPU AVX single-proof 10× goal remains active and unmet: **0/8 supplied M5 CPU targets**. Production is unchanged.

This is a fresh Zen4 build of all 176 accepted source files, using Rust 1.98.1 / LLVM 22.1.8, `target-cpu=znver4`, native O3, fat LTO, one codegen unit and the accepted manual flags, including dense busy reset. SDK SHA-256 is `d5e1b2c981fb9bcab2beb69e3b2af5608d298d792c7e3814e87273b04509662e`. The accepted driver fixes each Zopro worker's CPU mask and uses default worker stacks. The local Zen5 result of 7.772288 ms remains a separate observation. The rejected G2 filter is excluded.

Aeglos reuses pinned commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, its Sorted engine, qualified native archive, pool-aware adapter and existing setup/cache policy. It is not asserted to be latest upstream. Its existing native preparation limit is 24 GiB. Both engines use the established matching keys, requests and public-input bindings. Zopro adds no expanded key table or solver plan.

One c7a.24xlarge provides 96 physical cores without SMT. Processes pin to CPUs 0…N−1 for 32/64/96 workers; Zopro checks its individual worker masks before and after timing. Aeglos retains its pool-aware driver under the same process affinity. Physical-host exclusivity is not established. The network-disabled container has CPU quota 96, memory 30 GiB, no swap and a PID limit of 2,048. This memory bound is a harness setting, not a prover architectural cap or minimum.

Fifteen fixed blocks rotate/reverse context and engine order. Primary processes time 101 requests after three primes; other circuits time 17 after three. A displayed latency is the median of 15 process medians. Paired ratios divide corresponding process medians by block; bounds use sorted zero-based ranks 3/7/11, with nominal 96.484375% marginal median coverage conditional on independent exchangeable blocks. There are no familywise or stationarity guarantees, extra blocks, sample exclusions or timing reruns.

Timers include fresh assignment/witness solving, proof randomness, proving, internal verification and serialization. They exclude loading, key preparation and primes. CPU columns are medians of per-process mean user+system CPU milliseconds per request. RSS is the maximum lifetime process RSS across the 15 processes, including loading and preparation.

|Circuit|Workers|Zopro ms|Aeglos ms|Zopro CPU ms|Aeglos CPU ms|Zopro RSS MiB|Aeglos RSS MiB|Zopro/Aeglos [bounds], wins|
|---|---:|---:|---:|---:|---:|---:|---:|---|
|1x2|32|11.243892|11.870012|247.072|327.646|122.52|428.12|0.950116[0.942381,0.952890],15/15|
|1x2|64|11.567772|8.852565|432.758|428.870|124.72|445.51|1.285063[1.265591,1.317852],0/15|
|1x2|96|13.502813|7.902802|771.838|518.202|118.04|479.98|1.711440[1.693327,1.740419],0/15|
|2x2|32|15.263659|18.574454|343.181|536.961|223.98|725.20|0.822767[0.809091,0.826758],15/15|
|2x2|64|16.214128|13.140633|631.251|681.470|223.78|752.74|1.235636[1.193169,1.244625],0/15|
|2x2|96|18.416164|11.679913|1095.825|831.031|214.80|810.55|1.584437[1.551735,1.596794],0/15|
|4x2|32|19.648714|30.778427|504.262|911.380|272.52|1257.91|0.638439[0.633860,0.641463],15/15|
|4x2|64|19.121785|20.871367|881.273|1142.282|268.83|1280.09|0.922530[0.893927,0.926474],15/15|
|4x2|96|21.297714|18.240823|1470.066|1392.436|283.82|1350.33|1.162406[1.140540,1.178891],0/15|
|8x2|32|27.347865|54.574932|753.469|1653.467|466.35|2311.38|0.499854[0.495530,0.518695],15/15|
|8x2|64|25.894916|35.399268|1319.592|2069.819|458.53|2381.57|0.729232[0.723040,0.738316],15/15|
|8x2|96|28.751237|30.951086|2165.413|2575.393|470.99|2467.30|0.930833[0.913002,0.948533],15/15|
|12x2|32|35.822934|87.049778|1005.536|2641.868|643.13|3438.30|0.412832[0.409825,0.426193],15/15|
|12x2|64|33.778290|56.072003|1688.772|3329.409|657.43|3522.01|0.606768[0.596509,0.613222],15/15|
|12x2|96|36.647819|48.320431|2651.011|4124.206|662.32|3722.73|0.755233[0.745555,0.768114],15/15|
|16x2|32|41.956174|104.945834|1195.580|3160.794|801.33|4335.46|0.400828[0.394144,0.406741],15/15|
|16x2|64|37.977699|65.117610|1895.459|3862.278|766.29|4413.95|0.583497[0.576315,0.587619],15/15|
|16x2|96|41.937149|56.830844|2947.662|4882.137|783.02|4618.36|0.734419[0.729043,0.752428],15/15|
|24x2|32|60.656766|170.564406|1782.102|5170.000|1271.06|6481.65|0.356375[0.351469,0.362491],15/15|
|24x2|64|53.297583|104.927354|2612.471|6192.447|1223.63|6471.72|0.511228[0.497894,0.513103],15/15|
|24x2|96|57.663959|86.494125|3869.749|7407.500|1221.12|6877.96|0.667928[0.661162,0.675173],15/15|
|merge-24x1|32|61.736049|164.943925|1725.477|5023.130|1094.82|6172.62|0.373136[0.368007,0.382405],15/15|
|merge-24x1|64|56.324672|101.016373|2592.544|5966.855|1114.06|6257.07|0.558641[0.552034,0.565433],15/15|
|merge-24x1|96|61.869262|84.047696|3992.681|7222.138|1122.89|6599.03|0.735945[0.733222,0.748602],15/15|
|merge-54x1|32|138.695323|345.936677|3672.023|10657.675|2175.59|12596.56|0.401397[0.393593,0.405777],15/15|
|merge-54x1|64|120.757128|206.945251|4709.187|12156.837|2003.02|12612.08|0.583617[0.578314,0.599423],15/15|
|merge-54x1|96|125.244950|171.908504|6591.391|14890.209|2075.50|13102.11|0.729394[0.725099,0.742096],15/15|

Zopro wins all nine matched 32-worker comparisons, seven at 64 and six at 96. Each winning comparison has 15/15 wins and bounds below parity; each complementary comparison has 0/15 wins. Zopro's fastest tested counts are 32 for 1×2/2×2 and 64 for the other seven circuits. Aeglos's fastest tested count is 96 for all nine.

The supplied M5 CPU and Metal references are historical observations on different hardware, not devices remeasured here. This table uses each circuit's fastest tested Zopro count. That selection does not establish a portable production policy.

|Circuit|Zopro workers / ms|M5CPU ms / speedup|M5Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|32 / 11.243892|37 / 3.291x|24 / 2.134x|3.7|
|2x2|32 / 15.263659|59 / 3.865x|30 / 1.965x|5.9|
|4x2|64 / 19.121785|109 / 5.700x|48 / 2.510x|10.9|
|8x2|64 / 25.894916|189 / 7.299x|74 / 2.858x|18.9|
|12x2|64 / 33.778290|284 / 8.408x|101 / 2.990x|28.4|
|16x2|64 / 37.977699|335 / 8.821x|124 / 3.265x|33.5|
|24x2|64 / 53.297583|528 / 9.907x|176 / 3.302x|52.8|
|merge-24x1|64 / 56.324672|507 / 9.001x|175 / 3.107x|50.7|

No M5 reference is supplied for 54→1. The 24×2 result, 53.297583 ms, is close to its 52.8 ms CPU target but does not satisfy it.

Primary Zopro latency worsens from 11.243892 to 13.502813 ms as workers rise from 32 to 96, while CPU service rises from 247.072 to 771.838 ms per request. Aeglos latency falls from 11.870012 to 7.902802 ms, while CPU service rises from 327.646 to 518.202 ms. Zopro uses less CPU and has lower latency at 32 workers; the larger-pool results identify a scalability problem worth investigating.

The October 29 Zopro best-count observation was 11.915591 ms. The new 11.243892 ms is about 5.6% lower descriptively. Differences in campaign, build and worker placement prevent attributing that difference solely to the source change.

After all 810 latency processes and their independent verifications, four FIFO-gated primary profiles run 251 requests each after three primes, at 32/96 workers for both engines. All recordings finish before decoding. The profiles contain 97,089 samples, zero LOST records and one sampled PID each. Absolute sample-period coverage is unvalidated.

|Profile|Samples|Sampled TIDs|Rust epoch %|Rust scheduler %|Unknown period %|
|---|---:|---:|---:|---:|---:|
|cycles-1x2-w32-accepted|13039|33|8.77|5.31|2.52|
|cycles-1x2-w32-aeglos|17214|40|n/a|n/a|1.06|
|cycles-1x2-w96-accepted|39821|97|53.08|11.93|2.10|
|cycles-1x2-w96-aeglos|27015|104|n/a|n/a|1.26|

Zopro's combined epoch/scheduler residence rises from 14.08% to 65.01%. Across equal 251-request windows, G1 stream flat period totals remain 47.29→47.50 billion, and G2 borrowed totals 36.62→37.57 billion; coordinator/epoch symbols grow sharply. This supports investigating work availability, scheduling and work amplification. Samples are not exclusive wall-time fractions, and their removal would not necessarily remove the same fraction of latency. They do not isolate frequency, cache, DRAM capacity, fences or syscalls as the cause.

The inherited category rules name Rust symbols. Aeglos therefore appears in `other`; its epoch/scheduler entries above are unclassified, and `other` does not mean unresolved. Resolved C++ symbols are retained. A descriptive grouping after capture places `Pool::work` at about 16.06% for 32 workers and 25.87% for 96. That function may include work and coordination; it is not an idle-time measurement or a category directly equivalent to Rust epoch residence.

The whole-container workflow records 46 throttled periods and 341,760 throttled microseconds. This scope includes loading, preparation, primes, latency, profiles and verifiers, so it cannot be attributed to one engine's timed proof. All memory/swap/OOM event deltas are zero. The cgroup peak is 13,896,298,496 bytes across the entire workflow. These observations provide no evidence of a RAM-capacity limit causing the gap, and do not rule out cache or bandwidth costs.

|Primary engine|Workers|Median minor faults/request|Median major faults/request|
|---|---:|---:|---:|
|accepted|32|181.832|0.000|
|accepted|64|195.614|0.000|
|accepted|96|199.050|0.000|
|aeglos|32|34.584|unreported|
|aeglos|64|71.851|unreported|
|aeglos|96|91.317|unreported|

Minor faults can include demand-zero, first-touch or mapping work. They do not by themselves establish disk paging. Aeglos's driver does not report major faults here. This campaign adds no pressure/swap, mobile/ARM/TEE, huge 250/4999-circuit, minimum-RAM or concurrent-throughput qualification.

Fresh Zen4 qualification passes 35 field, 127 core and eight prover tests. Container preflight completes 72 SDK requests and 18 fresh Gnark exports across all nine shapes and both engines at four workers. AWS completes 24,776 requests and 814 fresh exports: 810 latency processes and four profiles. All 832 new exports are 164 bytes, have expected public bindings, are independently verified, and are disjoint from prior/new proof identities. The retrieved archive's 6,174,518 bytes and SHA-256 match remote output. All 814 transported proofs are independently reverified locally once; this adds no new proofs. The canonical count advances **45,335→46,167**. Every future export must seed `accepted-current-c7a176/proof-sha256-after-capture.json`.

Both result readers were compiled and pinned before preflight. The producer passes. The original independent-rank reader fails direct in-memory-versus-reparsed JSON equality on ten floating values, with maximum relative difference 1.991×10⁻¹⁶. A comparison-only repair after capture retains the original ranking computation and requires its serialized output bytes to equal the producer's file exactly. That comparison passes. The original source, binary and failure logs remain retained. No timing, performance threshold or ranking logic changes, and successful producer/proof checks are not replayed. The readers share parsing and are a same-agent cross-check, not an external audit.

The validated receipt inventory contains 1,697 commands: 1,694 successes and three retained failures. All 1,640 cloud and 36 local-container workflow commands pass. Host failures are the initial typed link, first container permission attempt and original rank reader. The link referenced an old artifact path and default Rust 1.99; its replacement explicitly uses Rust 1.98.1 and the newly frozen dependencies. Successful builds/tests are retained without replay. The first container attempt stops before SDK calls because capability dropping prevents root from writing the Ubuntu-owned output directory; adding its existing group fixes local access. Two initial typed-plan parse errors and a premature plan lookup execute no command or SDK request. Direct preparer/compiler/lifecycle invocations are separate. During preflight and recording, only the confirmed live handle is polled, without overlapping owned builds, helpers, hashes, analysis or reports. The AWS command takes 44 minutes.

The prior qualification subnet, security group and profile no longer exist. Launch resolves to existing resources: a no-ingress group, public subnet and SSM/ECR-read-only instance profile. No network or IAM resource is created or modified. Exactly one owned instance, `i-0a15ad9ae313aa976`, uses encrypted storage deleted on termination and a 90-minute termination timer. After SHA-checked retrieval it is terminated; the volume is absent, the temporary ECR tag and private S3 object/bucket are removed, and local Docker auth/signed URL files are absent. Cleanup is complete and production resources are unchanged.

Next work should investigate useful work and coordination at larger pools while preserving the current memory behavior. This benchmark changes no production algorithm or acceptance policy.

[All results](artifacts/analysis.json),[best-count selection](artifacts/best-counts.json),[profiles and faults](artifacts/quality.json),[reader](artifacts/review.json),[reader audit](artifacts/reader-failure-audit.json),[proof checks](artifacts/post-verification.json),[build identity](artifacts/build-identity.json),[source audit](artifacts/source-audit.json),[protocol](artifacts/design.md),[container hashes without ledger](artifacts/container-manifest-without-ledger.json),[receipts](artifacts/receipts.json),[cleanup](artifacts/cleanup.json),[raw perf data](raw-perf-data.tar.gz),[sources and raw driver streams](sources-receipts.tar.gz),[archive index](artifacts/archive-index.json),[checksums](SHA256SUMS).
