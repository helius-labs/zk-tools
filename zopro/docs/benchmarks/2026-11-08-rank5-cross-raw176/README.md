# Five raw products shared across three field outputs

**This implementation is slower and is not adopted.** Candidate/fresh-OFF paired median ratios are 1.166–1.190 with one worker and 1.408–1.452 with 32 workers. All four contexts reject the fixed recipe. Both candidate profiles also fail the prospective quality rule. No weighted-point caller or SDK integration is selected; production remains unchanged.

The experiment implements the rank-five polynomial identity in section 2 of [Aranha and Barreto, 2026/2667](https://eprint.iacr.org/2026/2667). For six field inputs, X=AB−CD, Y=ED+FB and Z=CF+AE share five raw limb convolutions before three existing R256 Montgomery reductions. Linear operands use existing mod-2p normalization. This tests a different representation of the intermediate products, rather than five separately reduced multiplies. The paper's complete projective addition still requires 11 multiplications, or 10 for mixed addition; five products do not describe a complete point addition. No paper-level speedup is attributed to this implementation. The primary PDF identity and SHA are retained in design.md.

The integer model checks 111,744 six-input tuples across Fq/Fr, producing 335,232 field outputs, including boundary Cartesian products, fixed random inputs and dependent chains. A focused free-variable identity test also passes. Two new native tests check 335,232 raw results and 335,232 canonical exports against independent integer references and the original fused operations. Existing field function bodies, core and prover stay exact. The private source contains 174 exact originals, two reversible include parents and two new children. Existing unchanged field tests are inherited, not replayed. This is native arithmetic qualification, not new SDK proof verification or portable-runtime validation.

Static decoding corrects the initial 400-IFMA proxy: the unchanged full-convolution reduction retains five quotient-generation IFMAs beyond its 50 modulus-product IFMAs. The literal dynamic block therefore uses **415 IFMAs versus 450** for three original fused operations, a 7.8% reduction. Candidate functions have 305 static IFMA sites because one 55-site reducer body runs three times; 305 is not the executed count. Candidate code also contains a libc memset call. Prototype one-state OFF/ON instruction rows are 1,203/1,432, with 192/353 ZMM stack-addressed rows; four-state rows are 1,167/1,441 and 109/355. Static rows do not measure dynamic spills, stack peaks or latency. Both prototype and actual linked harness functions are decoded separately once.

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. The fixed screen compares the frozen accepted-source arithmetic library, fresh OFF and candidate ON. Both fresh arms share the new field library. This is an arithmetic block benchmark, not a proof or complete point-addition benchmark. All six input vectors change; eight SIMD lanes and one/four independent states are checked. A scalar accepted-field oracle independently generates 320 final-state digests covering all workers, primes and round counts. The three actual binaries use native O3, fat LTO and one codegen unit.

Twelve checked 64-round preflights precede seven rotated/reversed blocks of 8,192 rounds across four contexts and three arms. Three checked 128-round primes and a reset precede each clock; fixture creation, pool setup and final checks are outside timing. Solo pins to CPU 2; 32 workers pin individually to CPUs 0–31, with diagnostic 512 KiB worker stacks. All 84 latency calls precede four 16,384-round PMU panels and four 65,536-round profiles; 20 offline commands follow all recordings. Total: 125 commands and 104 checked caller processes. No owned build/helper/hash/analysis/report/push overlaps recording, including preflights. No observation is excluded, extended or repeated.

The prospective rejection rule uses sorted paired-ratio ranks 1/3/5 of seven. Any candidate/OFF lower bound above 1.02 rejects; a candidate/frozen lower bound above 1.02 also rejects when the full OFF/frozen interval lies within factor 1.05. There is no gain floor or early adoption claim. The one-state 32-worker frozen comparison fails calibration and remains visible; the other three calibrate. Every candidate/OFF comparison rejects, independently of frozen calibration.

|States / workers|OFF ns/wave|Candidate ns/wave|Candidate / OFF, fixed rank interval|Wins / 7|Frozen calibrated|
|---|---:|---:|---:|---:|---|
|1 / 1|84.183|98.528|1.166287 [1.159009, 1.174667]|0|true|
|1 / 32|216.892|308.445|1.408000 [1.333442, 1.586836]|0|false|
|4 / 1|330.145|394.078|1.190397 [1.189091, 1.194419]|0|true|
|4 / 32|827.912|1187.955|1.452007 [1.434085, 1.521609]|0|true|

A wave executes the stated number of independent blocks on every worker; four-state wave times are not single-block times. All output hashes/oracles match and timed allocations are zero. Retained fixture capacities are equal across arms: 7,744 bytes for one worker and 247,808 bytes for 32. The shared fixture enum reserves four-state capacity even in one-state runs. Candidate raw accumulators alone occupy a 1,920-byte fixed payload, with inputs, linear temporaries and compiler stack/spills in addition. Fixture equality does not bound total stack or RAM; no process peak-RSS or minimum-RAM result is added. There is no per-key table, key copy, retained solver plan, new production allocator/pool, C or assembly rewrite.

|32-worker panel|Instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|1 states|1.611688|1.539481|
|4 states|1.576775|1.404325|

All four grouped-counter panels run at 100% coverage. They show increased total instructions despite fewer IFMAs. Additional normalization, raw sharing and materialization are plausible costs; these single diagnostic panels do not isolate their individual causal contribution or prove a DRAM bottleneck.

|Profile|Samples|Unknown %|Target %|Quality|
|---|---:|---:|---:|---|
|cycles-S1-w32-OFF|2936|0.000|99.591|true|
|cycles-S1-w32-ON|4095|13.065|86.593|false|
|cycles-S4-w32-OFF|13100|0.053|99.878|true|
|cycles-S4-w32-ON|17813|18.924|81.003|false|

All 37,944 samples belong to assigned workers and all four recordings have zero LOST. Sampling uses cycles:u, fixed period 500,000, 512 mmap pages and acknowledged FIFO gating after primes. PID/TID/CPU/MMAP and raw symbols are retained. Both candidate profiles exceed the ≤5% unknown limit and miss the ≥95% target-symbol share; those failures remain failures. Unknown samples are not attributed to memset or another function. No profile-quality rescue, rerun or alternate recipe follows.

The independent reader reconstructs all 455 pins, 125 command receipts, 104 outputs, ratio rows, PMU panels and profiles. Its pass confirms evidence consistency, not performance qualification. Preparation failures and repairs are retained: a native test-only integer annotation, adapter occurrence count, builder compilation/template fixes, and null environment fields rejected by xtask before any process started. Successful builds, decodes, oracles and observations are reused. The original model prediction and both original/fixed plans remain archived.

This trial adds zero SDK requests, proofs or cloud resources. Canonical inventory stays 35,171. All following proof results are **inherited**, including the latest 1×2 from the [preceding SDK report](../2026-11-08-common-z-sdk176/README.md) and its seven earlier full-campaign rows. The same accepted binary previously measured 7.915–7.929 ms locally; the latest 8.002770 ms is a later observation, not an adopted code regression. Variation between runs remains unexplained. Supplied M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

The original verified CPU 10× goal remains active and unmet, **0/8**. Merge54 remains inherited at 162.240109 ms without an M5 reference. No new AWS/Aeglos, giant-circuit, mobile/ARM/TEE, pressure/swap or minimum-RAM claim is made. This fixed implementation closes without advancement or adoption.

The [evidence archive](artifacts/evidence.json.gz) retains prospective designs/plans/pins, source changes and helpers, all native/model/harness receipts, linked decodes, raw latency outputs, decoded profiles, PMU CSVs and the complete reviewed analysis. Decompression yields relative paths mapped to {utf8,bytes,sha256}; every extracted payload is verified. The public analysis omits only its duplicated caller map; its complete original bytes remain archived. Binaries/libraries, raw perf data, keys/images, proof payloads, private ledgers and full source copies are excluded; relevant identities are retained.
