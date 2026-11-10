# Fused incoming quarter and signed Y difference

**The fused operation improves G1, but G2 at32 workers slows by about0.7% against the accepted-source control; the complete candidate fails7/48 checks and is not adopted.** All18 nonregression, six RSS and twelve diagnostic-quality checks pass. One G1 calibration and all six primary/combined gain checks fail. Accepted local1×2 full-proof latency remains **7.783594ms at32 workers**, with **9.016864ms at16 workers**.

The change combines incoming-Y modular quartering with signed subtraction in the untagged persistent-quarter bucket format. For canonical numerical b=4a+r<p, p≡3mod4 and T=(p+1)/4, quarter(b)=a+rT<p. Positive difference is2p+a+rT-ay; negative is3p-a-rT-ay, with ay<2p. Both fit the existing inclusive Wide3 bound. Unnormalized per-digit sums stay below4·2⁵²; one signed carry chain replaces separate quarter normalization and subtraction chains. Keys, solver plans, queues, layouts, scalar tails and allocation sites are unchanged. There is no lookup table or new pool.

The preliminary tagged raw-difference proposal was rejected before implementation: final Y still needs the converted bucket value, so conversion would survive. This trial prospectively uses the earlier untagged quarter snapshot and control to isolate a different fusion mechanism. It does not select a favorable group from the closed deferred-tag trial.

Independent BigUint and old-path exact comparisons pass on **196,608 lanes**, covering all256 sign masks,96 changing groups per mask,256 boundary/residue inputs, lazy ay including0/p/2p−1, output-fed inputs and negative-zero output exactly3p. Inputs remain unchanged. Four existing tests pass on new coherent libraries:1,695,744 independent Arkworks prefix point comparisons,5,018,112 bucket comparisons and220 threaded Windows including1/16/32 workers and twelve callers through one eight-worker pool. Two changed private Rust children reverse exactly to their181-member parent;179 members remain byte-identical and all176 production manifest members remain exact. Portable fallback evidence is inherited from unchanged guards and inactive source paths.

Twenty-four conversion cases preserve pointer/capacity without allocation/reallocation/deallocation. Complete single-Window requested-live peaks are191,360/303,360bytes forG1/G2, equal to the original path in this panel. Bucket sizes remain64/128bytes; records remain1,600/640bytes. These are requested allocation counters delegating to the production allocator, not minimum RAM, stack, allocator overhead or RSS claims.

The combined incoming+dy probe includes bucket load and numeric export. Native fat-LTO static instructions fall **154→142**, with zero calls and conditional branches in both builds. Static reduction is not a dynamic cost or proof-speed claim.

Four arms: REF retains the API-only accepted-source build; BASE selects the original Window path in the new coherent build; OFF retains the fixed-mask untagged quarter candidate; ON uses the fused operation. BASE and ON share an executable. BASE/REF calibrates equivalent original algorithms; OFF is a mechanism control and intentionally differs from the accepted baseline. Driver selectors and arm labels are frozen separately.

Ryzen9950X,16 physical/32 logical CPUs, shared host. Six contexts useG1/G2,15,045points,width12,column0,batch200 and1/16/32 workers. Worker i pins to CPU i; solo usesCPU2; stacks request512KiB. Independent-Arkworks fixtures are reused byte-for-byte. Three checked primes precede seventeen timed waves. Each wave includes fresh Window construction, feeds, queues, scalar paths, normalization, complete output comparison, checksum and drop. Loading, pool setup and input hashing are excluded. **A wave performs one Window per worker, not a complete MSM, proof or server-throughput measurement.**

Fifteen fixed rotated/reversed context/arm blocks are retained. Displayed times are medians of process medians; paired ratios need not equal ratios of displayed times. Fixed ranks4–12 of15 give nominal96.484375% individual median coverage under independent exchangeable blocks, without a familywise guarantee.

|Context|REF ms|BASE ms|OFF ms|ON ms|ON/REF [bounds], wins|ON/OFF [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|G1-w1|0.328079|0.326866|0.307881|0.299805|0.913799 [0.911711,0.915574], 15/15|0.973747 [0.972146,0.975072], 15/15|
|G1-w16|0.404863|0.400114|0.391579|0.379876|0.942927 [0.935215,0.946776], 15/15|0.975491 [0.970577,0.982343], 15/15|
|G1-w32|0.597958|0.585694|0.599761|0.586516|0.990636 [0.970562,0.995977], 12/15|0.980521 [0.964538,0.997411], 12/15|
|G2-w1|1.233606|1.229167|1.222344|1.223487|0.994257 [0.992399,0.995525], 15/15|1.003006 [0.999402,1.003863], 5/15|
|G2-w16|1.504496|1.499737|1.501080|1.501441|0.996827 [0.993686,1.005420], 10/15|1.001142 [0.995552,1.008919], 7/15|
|G2-w32|2.400696|2.400305|2.391678|2.415113|1.007058 [1.002440,1.018772], 2/15|1.014532 [1.000386,1.018222], 3/15|

All G1 contexts improve against OFF. At32 workers G1 ON/BASE remains uncertain, and BASE/REF calibration is0.982276 [0.970384,1.003772], with the lower bound just outside the factor1.03 range (1/1.03≈0.970874). This limits attribution of that context's exact differences solely to the fused algorithm. Five other calibrations pass. G2 at32 workers has all three primary gain checks fail; no worker/group subset is promoted.

Equal-log G1/G2 32-worker ratios:

- ON/BASE: 1.005989 [0.995888,1.013332], 4/15.
- ON/OFF: 0.999952 [0.980259,1.003438], 8/15.
- ON/REF: 0.997862 [0.989888,1.004774], 8/15.

All three aggregate gain checks fail. Seven failures are one calibration, three primary G2 gains and three aggregate gains. The fixed recipe closes without an unchanged rerun, parameter rescue or SDK trial.

|32-worker group/arm|Maximum process RSS MiB|Median CPU ms/Window|Minor faults/Window|
|---|---:|---:|---:|
|G1-w32/BASE|26.00|0.592208|0.080882|
|G1-w32/OFF|25.50|0.615706|0.080882|
|G1-w32/ON|25.75|0.608846|0.088235|
|G2-w32/BASE|36.00|2.363563|0.281250|
|G2-w32/OFF|36.00|2.349836|0.240809|
|G2-w32/ON|36.00|2.383689|0.244485|

Major-fault medians are zero. Short process high-water observations include fixtures/mappings and establish neither minimum RAM nor pressure/swap behavior or full-proof memory savings.

After all latency runs, six separate PMU panels and six separate cycle profiles run2,048waves behind FIFO warm gates. Grouped counters report100% scheduled coverage. Profiles use period16million cycles and512pages; all pass PID/TID/CPU/executable-MMAP, zeroLOST and sample/unknown limits. They contain156,952samples, with unknown shares below0.49%.

|Per complete Window,32workers|G1 BASE|G1 OFF|G1 ON|G2 BASE|G2 OFF|G2 ON|
|---|---:|---:|---:|---:|---:|---:|
|Instructions|4670124|4806944|4766014|19221897|19341138|19157179|
|User cycles|2615398|2623295|2551186|10256694|10120888|10299843|
|Branch misses|4745|5747|5672|7259|9242|9129|
|Generic cache misses|4149|3949|3788|22610|25092|25806|

G1 uses fewer instructions and cycles than OFF. G2 also uses fewer instructions, but more cycles; reducing the instruction count is insufficient here. Instruction scheduling, execution ports and register pressure are hypotheses for follow-up, not established causes. These are separate single diagnostic panels, not replicated causal estimates or proof of a particular cache/DRAM bottleneck.

All **448 workflow commands succeed once**:15native,5consumer/tool,426capture and2post-analysis. Capture contains24preflights,360latency processes,6PMU panels,6profiles and30offline steps;396callers perform906,752independent-oracle-checked Windows. Frozen producer and separate same-agent raw reader pass first execution, reconstructing all48 decisions and resource rows; PMU/sample schema parsers are shared. All607 capture pins remain exact. No owned helper/build/analysis/report overlaps recording. No failing observation is excluded or successful scientific step replayed.

No new SDK request, Gnark export, proof, AWS/Aeglos comparison, ARM/mobile, huge-circuit, pressure/swap or minimum-RAM result is added. Canonical proof inventory remains44,265. Accepted local32 full-proof observations and supplied M5 references below are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 / 4.754×|24 / 3.083×|3.7|
|2×2|10.932731|59 / 5.397×|30 / 2.744×|5.9|
|4×2|15.757088|109 / 6.918×|48 / 3.046×|10.9|
|8×2|23.973295|189 / 7.884×|74 /3.087×|18.9|
|12×2|33.827269|284 / 8.396×|101 /2.986×|28.4|
|16×2|39.900917|335 / 8.396×|124 /3.108×|33.5|
|24×2|66.041083|528 / 7.995×|176 /2.665×|52.8|
|merge-24×1|65.717994|507 / 7.715×|175 /2.663×|50.7|

The full verified CPU10× goal stays active and unmet, **0/8**. Accepted16-worker primary remains9.016864ms; merge54 remains162.170158ms without a supplied M5 reference. Next: inspect the G2 emitted hot path against its sampled addresses to identify whether multiply scheduling or spills explain the cycle increase, then choose a distinct arithmetic/scheduling mechanism before new clocks.

[Analysis](artifacts/analysis.json), [raw reader](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [arithmetic](artifacts/arithmetic.json), [allocations](artifacts/allocations.json), [structural qualification](artifacts/structural.json), [fused operation](artifacts/incoming-dy.rs), [prospective freeze](artifacts/prospective-review.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [commands](artifacts/receipts.json), [symbols](artifacts/symbols.json), [failure record](artifacts/failures.md), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Raw rows/full commands/source edits remain in the UTF-8 archive. Private fixtures, binaries, raw perf payloads, unchanged source snapshots and full proof ledgers are excluded; reproduction needs the pinned private artifacts.
