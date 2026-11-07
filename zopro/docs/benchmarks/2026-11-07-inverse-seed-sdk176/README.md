# Initial-scale inversion: complete SDK pilot

The candidate does **not** improve complete-proof latency and is not adopted. At 32 workers it is **0.57% slower than the original control** by paired geometric mean, with 95% bootstrap bounds entirely above parity. The [native scalar gains](../2026-11-07-inverse-seed-native176/README.md)—3.75% for Fq and 1.89% for Fr—remain valid in their measured component, but do not carry through this complete SDK trial. The fixed build is closed without an unchanged retry, parameter/compiler/layout sweep or favorable subset promotion.

Only the qualified four expressions in `inverse.rs` change. Initial modular coefficients incorporate h=2⁶⁴, ordinary Euclid finish omits its final scale multiplication, and both early returns correct by h⁻¹. All 176 originals recover exactly; 175 remain byte-exact. Original architecture/feature guards, arithmetic outside the inverse, solver, packing, queues, query geometry, resource forecasts and scheduler remain unchanged. No heap buffer, point/key table, retained plan, pool or new unsafe arithmetic is added. Whole-prover generic/no-assembly/no-IFMA compilation passes.

The new SDK links eight coherent libraries at O3/native/fatLTO/oneCGU and uses the unchanged accepted benchmark driver/support. Candidate SHA starts `fe15cf32`; accepted remains `6d83f1d6`. The original control `64bfa94a` is reused from the earlier separately built accepted-source control. It is not a fresh rebuild; current calibration measures its agreement with accepted. All original component/field qualifications are inherited without replaying successful old suites.

Both control and candidate pass all nine shapes at 32 workers, plus primary cases at 1 and 16 workers: 88 SDK requests including primes and 22 fresh exports independently verified by Gnark before the matched trial. These preflight clocks are functional observations and are excluded from performance inference.

Thirty-one fixed rotated/reversed blocks compare accepted/control/candidate at 32 logical workers on CPUs 0–31 and 16 physical workers on CPUs 0–15. Each fresh process performs three primes and 151 observed complete SDK requests at either topology. The 151-request, 16-worker setting was declared before this candidate's first clock; earlier candidates used 51. Solving, fresh blinding/proving, internal verification and serialization are included. Loading/primes/export are excluded. All 186 processes and 28,086 observed request times are retained. No other owned CPU workload overlaps recording.

|Workers|Accepted median process mean,ms|Original control,ms|Candidate,ms|Candidate/control pairedGM,95%bootstrap|Wins/31|
|---|---:|---:|---:|---|---:|
|32|8.027583|8.004420|8.055422|1.005699 [1.003261,1.008204]|6|
|16|9.642093|9.599550|9.927266|1.060366 [1.013782,1.106564]|7|

Candidate/accepted is 1.001194 [0.997668,1.004596] at 32 workers, with 14 wins: no established gain or regression against that arm. At 16 it is 1.047908 [0.999637,1.097639], with 12 wins. Paired ratios need not equal ratios of the displayed medians.

The declared 32-worker gate requires both candidate upper bounds below 1. The 16-worker guard requires both upper bounds ≤1.02. Control/accepted calibration bounds must fit [1/1.03,1.03]. The 32-worker calibration passes at [0.992598,0.998249]; the 16-worker calibration fails at [0.959076,1.017442]. Both gain checks and both nonregression checks fail: **five failed checks**. The wide 16-worker calibration limits attribution there; it does not erase the calibrated 32-worker candidate/control regression. There is no gain floor, exclusion, extension or threshold change. Analyzer labels containing “fresh” refer to the reused original control above.

|Workers|Maximum process RSS,MiB accepted/control/candidate|Median CPU ms/request accepted/control/candidate|
|---|---|---|
|32|134.00 /135.00 /134.50|179.555 /179.136 /179.551|
|16|91.00 /89.25 /90.25|116.945 /117.389 /116.153|

RSS includes loading, primes and allocator/runtime state. It is not minimum RAM, incremental scratch or pressure qualification. CPU usage surrounds the request loop before export. No prover memory saving is established.

Automatic diagnostics follow all latency. Six FIFO-gated panels cover 31 requests after three primes; six separate `cycles:u` profiles cover 251 after three, with 16M periods/512 mmap pages. All 24 counter events run 100%. All six profiles qualify: 63,316 samples total, zero LOST, maximum unknown 1.6165%, owned PID/MMAP/TID/CPU/period/header/gate checks, actual G1/G2 callers and shared inverse samples in every context. This directly exercises the new inverse inside real threaded proof generation.

|Topology|Candidate/control instructions|Cycles|Cache misses|Branch misses|
|---|---:|---:|---:|---:|
|32workers|0.999135|1.007147|1.082515|1.023879|
|16workers|0.997511|1.000107|1.089761|1.007752|

These single panels and sampled user CPU periods do not isolate a critical-path, cache, RAM-capacity, code-size or scheduling cause. The source-level multiplication saving and slightly lower retired instructions do not establish a proof-latency gain. Candidate SDK codegen contains two Fq instantiations of 8709 bytes and one Fr of 8752, each retaining six width-alternative division sites; the larger early-return implementation is a tradeoff, not a proven cause.

All 220 fresh 164-byte exported proofs independently pass Gnark with image/public/VK/freshness bindings. Every one of 30,460 SDK requests also verifies internally. Canonical distinct-export inventory advances 32637→32659→32845→32857; copies and internal requests are not counted as additional exports. Independent review checks 479 successful workflow commands, the retained preparation failure, source/library/tool pins, all 28,086 raw samples and reconstructed means/ratios/geomeans/wins/CPU/RSS/gates, byte-exact qualified statistical utilities, proof bindings and inventories. No successful model, test, capture, bootstrap, verifier or decoder is rerun for bookkeeping.

One preparation reader initially expected two emitted inverse functions; this SDK emits three, including separate Fq instantiations. All builds and four new codegen commands had succeeded before the assertion. The repaired reader checks two Fq/one Fr and reuses those exact saved command/hash/decoded streams. No proof or performance capture had started. Original reader, failure and repair scope are preserved. Unused-import warnings in preparation helpers are retained; they are not arithmetic failures.

Accepted production remains unchanged. The accepted build has previous 1×2 observations below 8 ms—7.928660 at adoption, 7.915095 later and 7.991694 in the preceding primary trial. This trial's 8.027583 ms is a new observation of the **same accepted binary**; no slower build was adopted and the cause of run variation is not established.

The inherited accepted all-eight table and supplied historical M5 references remain:

|Shape|Accepted ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10×limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 /4.667×|24 /3.027×|3.7|
|2×2|11.151932|59 /5.291×|30 /2.690×|5.9|
|4×2|16.086386|109 /6.776×|48 /2.984×|10.9|
|8×2|24.179453|189 /7.817×|74 /3.060×|18.9|
|12×2|33.884455|284 /8.381×|101 /2.981×|28.4|
|16×2|40.077381|335 /8.359×|124 /3.094×|33.5|
|24×2|65.850777|528 /8.018×|176 /2.673×|52.8|
|merge24|65.060937|507 /7.793×|175 /2.690×|50.7|

These eight accepted values are inherited, not new all-shape timings. Merge54 remains 161.883899 ms without a supplied M5 reference. This failed primary pilot does not advance to all-nine candidate timing/adoption. The original all-eight CPU 10× objective remains active/unmet, 0/8. No fresh Apple/AWS/Aeglos, huge-circuit, mobile, concurrent-throughput, pressure/swap or minimum-RAM result is added.

Next seek a distinct reduction in dominant streaming work using these saved profiles and the existing experiment history. No new runtime candidate is selected by this result. [Protocol](artifacts/design.md), [latency](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [review](artifacts/review-result.json), [source audit](artifacts/source-audit.json), [exact helpers/sources/receipts](artifacts/source-and-receipts.tar.gz), [publication](artifacts/publication.json). Absolute local dependency paths need adaptation elsewhere. Private root: `/home/ubuntu/src/zopro-perf-work/inverse-seed-sdk176`.
