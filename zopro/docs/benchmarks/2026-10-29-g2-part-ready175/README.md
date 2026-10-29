# Partition-ready G2 SDK experiment

The private schedule passes correctness qualification and does not establish a full-proof gain. Its fixed performance gate is **false**. For 1×2 at 32 workers, ON/OFF paired ratio is 0.999766 [0.993583, 1.003620], 4/7 wins; ON/accepted is 1.007302 [0.995887, 1.020998], 2/7 wins. All measurements and failed checks below are retained. Production is unchanged; no candidate is adopted by this screening study.

## Change and scope

Each of the existing two G2 input partitions decomposes its scalars in place and immediately releases its eleven window fills. Each window's second arrival merges part 0 then part 1 and computes the original weighted reduction. Arithmetic, point arrays, partition count, width 12, Horner assembly and eligibility 1024≤n<65536 remain unchanged. The original global decomposition join remains in OFF. No lock is held across merge, reduction or Rayon work; no worker waits for its missing publisher. No extra pool, unsafe code, per-key table, point/scalar copy or public API is introduced. Serial per-part preparation can also reduce parallelism; the measurements decide the tradeoff.

A checked type-size bound ensures the per-window Mutex enum payload fits the existing forecast's two bucket-vector headers plus projective sum. Otherwise the implementation falls back before mutating scalars. Existing bucket and scratch forecasts remain unchanged. This is a source-level accounting check, not a minimum-RAM or universal-memory claim.

The snapshot contains 176 files: 174 accepted originals are exact, one original contains the private module/dispatch edit, and one child module is new. Reversing the dispatch/module edit and removing the child recovers all 175 originals. All 17 field files are exact; their 33-test qualification is inherited. OFF and ON each pass 127 unfiltered core and eight prover tests. Three new tests cover both arrival orders, owned-input panic cleanup and independent generator/scalar oracles at 0, 1, 17, 1024, 4097 and 30,089 inputs with one/four workers; they also compare the original schedule and final decomposed words.

The initial build stopped at compilation, before any test or SDK request, because module placement preceded inner docs and the panic-test pool needed an unwind-safety wrapper. Both corrections and the initial failure receipt are archived. No successful test or capture is rerun. Accepted native flags plus private cfg `zopro_phi_part_ready` build a single candidate SDK; exact `ZOPRO_CPU_PHI_PART_READY=1` enables ON. OFF/ON share binary identity; accepted uses the retained production SDK `20fde…`. All linked dependencies, compiler flags and input identities are pinned.

## Full-proof measurements

Ryzen 9950X, 16 physical/32 logical cores. All nine circuits use 32 workers/CPUs0–31, plus primary with 16 workers/CPUs0–15. Seven fixed blocks rotate/reverse both case and variant order. Primary 151 requests, others 31, after three primes; 30 preflights each 1+3 requests are independently verified before latency. Timers include fresh assignment/witness solving, fresh randomness, proving, internal verification and serialization; load and primes are excluded. CPU is summed user+system milliseconds/request. RSS is maximum lifetime process RSS across blocks, including load/preparation; it is not incremental workspace, cgroup peak or minimum RAM.

|Case|Accepted ms|OFF ms|ON ms|Accepted CPU ms|OFF CPU ms|ON CPU ms|Accepted/OFF/ON max RSS MiB|
|---|---:|---:|---:|---:|---:|---:|---|
|1x2-w32|7.991129|8.070709|8.065609|182.165980|182.466927|182.717768|132.50/133.00/131.50|
|2x2-w32|11.326836|11.291729|11.342473|261.046129|262.186032|261.135419|274.50/268.75/265.00|
|4x2-w32|15.835604|16.198605|16.141968|413.927613|418.012000|418.079097|309.00/301.25/315.75|
|8x2-w32|24.230714|24.197126|24.081823|671.370387|668.287516|668.880968|503.25/505.75/501.25|
|12x2-w32|33.868762|33.912341|33.935302|931.169419|934.363226|942.583839|713.25/696.75/718.50|
|16x2-w32|40.129128|40.018736|39.957360|1120.696548|1120.115935|1116.089581|810.00/803.75/824.75|
|24x2-w32|66.028306|65.819080|66.587585|1899.005258|1886.808387|1902.271839|1293.22/1309.90/1260.66|
|merge-24x1-w32|65.308064|65.060707|65.204564|1806.227645|1804.839194|1808.727226|1173.94/1174.62/1169.46|
|merge-54x1-w32|163.406119|163.236584|162.661963|4449.099290|4445.595903|4451.640968|1821.13/1848.63/1829.92|
|1x2-w16|9.639223|9.677748|9.862227|119.544450|120.426589|119.870675|89.75/89.50/89.25|

## Fixed paired checks

Ratios use all seven paired blocks; bounds are their extrema, nominal98.4375% individual median coverage under independent/exchangeable blocks, not familywise guarantees. Every 32-worker OFF/accepted interval must lie within[1/1.03,1.03]. ON/OFF andON/accepted upper ratios must be≤1.03 for every 32-worker context, with strict upper<1 and≥5/7 wins for primary with 32 workers and the equal-log aggregate of all eight reference circuits. Physical 16 is descriptive and retains prior calibration uncertainty. No gain floor, post-capture waiver, observation exclusion, sample extension or subset promotion. Passing selects separate resource/portability/final-artifact qualification; it is not adoption or the full 10× goal.

|Comparison|Ratio|Bounds|Wins/7|Paired saving µs|
|---|---:|---|---:|---:|
|12x2-w32/offset/accept|0.997886|[0.977352, 1.009862]|5|72.907|
|12x2-w32/ready/accept|1.002697|[0.971606, 1.013978]|3|-91.291|
|12x2-w32/ready/offset|1.003122|[0.986479, 1.022356]|2|-105.618|
|16x2-w32/offset/accept|0.998915|[0.971006, 1.029857]|4|43.563|
|16x2-w32/ready/accept|0.999897|[0.969517, 1.024594]|4|4.148|
|16x2-w32/ready/offset|0.999059|[0.988074, 1.010356]|4|37.440|
|1x2-w16/offset/accept|1.003281|[0.917157, 1.040209]|3|-31.630|
|1x2-w16/ready/accept|1.009338|[0.925669, 1.385648]|3|-89.258|
|1x2-w16/ready/offset|1.020768|[0.970322, 1.371520]|2|-200.649|
|1x2-w32/offset/accept|1.007538|[0.999703, 1.017316]|1|-60.354|
|1x2-w32/ready/accept|1.007302|[0.995887, 1.020998]|2|-58.470|
|1x2-w32/ready/offset|0.999766|[0.993583, 1.003620]|4|1.884|
|24x2-w32/offset/accept|0.994697|[0.972860, 1.017112]|5|350.130|
|24x2-w32/ready/accept|1.005477|[0.969456, 1.020133]|3|-361.510|
|24x2-w32/ready/offset|1.008321|[0.995898, 1.024072]|2|-547.682|
|2x2-w32/offset/accept|1.001267|[0.985987, 1.019331]|3|-14.387|
|2x2-w32/ready/accept|1.005877|[0.992223, 1.012421]|2|-65.733|
|2x2-w32/ready/offset|1.009782|[0.979430, 1.018184]|3|-110.358|
|4x2-w32/offset/accept|1.014059|[0.991988, 1.028721]|2|-224.904|
|4x2-w32/ready/accept|1.018432|[0.989763, 1.025124]|2|-289.225|
|4x2-w32/ready/offset|1.001189|[0.988670, 1.011331]|3|-18.785|
|8x2-w32/offset/accept|0.996089|[0.971956, 1.021192]|4|94.009|
|8x2-w32/ready/accept|0.992193|[0.968211, 1.050761]|4|187.645|
|8x2-w32/ready/offset|0.997739|[0.986932, 1.034331]|4|54.522|
|merge-24x1-w32/offset/accept|0.999734|[0.978904, 1.022419]|4|17.474|
|merge-24x1-w32/ready/accept|0.999216|[0.986974, 1.015811]|4|51.478|
|merge-24x1-w32/ready/offset|0.999482|[0.980675, 1.012506]|4|34.004|
|merge-54x1-w32/offset/accept|0.997403|[0.981076, 1.008998]|4|425.031|
|merge-54x1-w32/ready/accept|0.993892|[0.984864, 1.018063]|4|999.652|
|merge-54x1-w32/ready/offset|0.996480|[0.986317, 1.011988]|4|574.621|

Failed checks: `["1x2-w32/ready/offset","1x2-w32/ready/accept","8x2-w32/ready/offset","8x2-w32/ready/accept","original8/ready/accept","original8/ready/offset"]`.

|All-eight aggregate|Ratio|Bounds|Wins/7|
|---|---:|---|---:|
|original8/ready/accept|1.002124|[0.992837, 1.010668]|2|
|original8/ready/offset|1.001541|[0.998563, 1.006347]|2|

## Automatic profiling and verification

All 210 latency processes finish before six separate 199 Hz cycles:u primary 16/32 profiles across all three variants (251 requests + three primes). FIFO enable/ack excludes loading/primes. All 216 captures finish before their independent Gnark verification, decoding and raw-loss audits. No owned build, verifier, hashing, archive or other CPU helper overlaps captures. Each profile contains one SDK PID and zero lost records. Complete sorted symbol weights and decoded streams are retained; inlining and scheduling affect attribution. These CPU samples do not establish exclusive critical-path wall fractions or a cache/memory/scheduler cause.

|Profile|Samples|
|---|---:|
|cycles-1x2-w16-accept|5428|
|cycles-1x2-w16-offset|5577|
|cycles-1x2-w16-ready|5747|
|cycles-1x2-w32-accept|9091|
|cycles-1x2-w32-offset|9057|
|cycles-1x2-w32-ready|9124|

The original workflow completes 498 steps, including every capture and independent proof verification, then its first offline `sudo perf script` decode fails because root differs from the transferred file owner. The archived owner-user repair completes the remaining 18 offline decode/evlist/raw-loss steps in a separate workflow. All 516 planned successful steps are accounted for; the failed decode and original protocol remain archived. No successful step is repeated and raw perf bytes are unchanged. **13,824 internally verified SDK requests and 246 distinct independently Gnark-verified 164-byte proof exports**, with exact image/public bindings and freshness against 20,509 prior proof identities. Ledger 20,755; 492 proof/public members. All closing pins and compressed member round trips pass. Private source, receipt streams, proof/public bytes and helpers are archived; SDK/dependency and raw-perf artifacts remain local with SHAidentities.

## Supplied M5 CPU and Metal references

Historical supplied references, not fresh Apple measurements. Each variant cell is ms; CPU/Metal speedup. All eight rows are newly measured here; 54→1 has no supplied M5 reference. Experimental results do not replace accepted timings.

|Shape|M5 CPU/Metal ms|Accepted ms; CPU/Metal ×|OFF ms; CPU/Metal ×|ON ms; CPU/Metal ×|CPU10× target ms|
|---|---:|---|---|---|---:|
|12x2-w32|284/101|33.868762; 8.385/2.982×|33.912341; 8.375/2.978×|33.935302; 8.369/2.976×|28.4|
|16x2-w32|335/124|40.129128; 8.348/3.090×|40.018736; 8.371/3.099×|39.957360; 8.384/3.103×|33.5|
|1x2-w32|37/24|7.991129; 4.630/3.003×|8.070709; 4.584/2.974×|8.065609; 4.587/2.976×|3.7|
|24x2-w32|528/176|66.028306; 7.997/2.666×|65.819080; 8.022/2.674×|66.587585; 7.929/2.643×|52.8|
|2x2-w32|59/30|11.326836; 5.209/2.649×|11.291729; 5.225/2.657×|11.342473; 5.202/2.645×|5.9|
|4x2-w32|109/48|15.835604; 6.883/3.031×|16.198605; 6.729/2.963×|16.141968; 6.753/2.974×|10.9|
|8x2-w32|189/74|24.230714; 7.800/3.054×|24.197126; 7.811/3.058×|24.081823; 7.848/3.073×|18.9|
|merge-24x1-w32|507/175|65.308064; 7.763/2.680×|65.060707; 7.793/2.690×|65.204564; 7.776/2.684×|50.7|

The authoritative accepted manifest remains unchanged (primary 8.022165 ms / max 134 MiB), with its prior full-performance-gate failure and physical-16 engineering exception retained. The original all-eight CPU 10× goal remains active and unmet (accepted 0/8). This study creates no AWS resources and supplies no new Aeglos, phone/ARM/TEE, huge-circuit, memory-pressure, swap, concurrent-throughput or minimum-RAM result.

[Design](artifacts/design.md), [history](artifacts/history-audit.md), [all blocks and profiles](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [proof inventory](artifacts/proof-inventory.json).
