# Local Karatsuba tiles: matched primitive results

**The local-tile candidate fails the fixed performance screen and is not adopted.** Across all 48 contexts, it is 2.10% slower than fresh original arithmetic, 2.20% slower than the frozen original and 8.50% slower than the schoolbook control. All three aggregates lose every paired block. Correctness, allocation/capacity checks and all profiles pass; seven control-calibration checks fail and remain visible. No complete-caller or SDK integration follows from this failed screen.

The [native qualification](../2026-11-08-blocked-pair52-native176/README.md) established exact arithmetic and 96/142 IFMAs, versus original 100/150. This experiment measures the complete changing-state primitive loop. It retains useful positive observations, but lower IFMA counts do not offset added work across the declared recipe.

## Compared implementations

Four newly linked drivers use identical source and runtime flags. `accepted` links the exact frozen original field library; it is not the accepted SDK executable. Fresh `OFF`, schoolbook `OLD` and local-tile `ON` share one coherent new release field library.

ON contains the exact qualified local-tile child: only two 2×2 tiles use Karatsuba, with seven coefficient slots and signed carries. OLD expands those tiles into four schoolbook products each and otherwise retains that same new row-pair/reduction/packing schedule. It is a new causal source control, not an adoption or rerun of an older row-pair candidate. The original field methods remain unchanged. All controls have 100/150 IFMAs; ON has 96/142 in every actual linked kernel. There is no table, retained plan, key/point copy or production pool change.

The new OLD control passes one focused native test with 554,496 raw outputs, canonical exports and exact original lazy-digit comparisons. It reuses the existing Arkworks/BigInt oracle families on this different kernel. ON's successful native/model qualification is inherited without replay. Seventeen private original field files remain byte-identical; removing one parent suffix recovers the eighteenth; the qualified ON child is exact. All 176 production hashes remain unchanged.

The fixed SHA-derived changing-v1 fixture seed, recurrence, bank/lane/worker order and rounds are identical to the previous independent reference. Its 156 gold rows are reused byte-for-byte; the successful gold generator is not rerun. One round rotates `[a,b,c,d]` to `[b,c,d,op(a,b,c,d)]`. `mul4` traverses four distinct banks in a rolled slice, not four simultaneously resident register streams. The timed region includes field work, array movement/load/store, dispatch and join; construction, reset, hashes and final oracles are outside.

## Complete screen

Ryzen 9950X; solo CPU2, 16 workers on CPUs0–15 and 32 workers on CPUs0–31, individually pinned with requested 512 KiB stacks. Logical fixtures may execute on any allowed Rayon worker; output digest order is logical. Forty-eight contexts cover Fq/Fr × R256/R260 × mul1/mul4/sum1/diff1 × 1/16/32 workers.

There are 192 checked 64-round preflights, then 15 fixed rotated/reversed blocks totaling 2,880 latency processes, each with 262,144 rounds per worker. Each process first performs three checked 128-round primes and resets. Thirty-two grouped PMU panels and 32 FIFO-gated cycle profiles follow. All 3,136 recordings finish before 160 ownership/decoder/metadata/event/loss commands: **3,296 commands total, 324 pins**. No owned build/helper/hash/analysis/report/push overlaps any recording, including preflights. No observation is excluded, extended or repeated.

The original analyzer completes once. After the preserved offline JSON comparison repair, the separate reader independently reconstructs all 3,296 receipts/argv/environments/stream hashes, 3,136 complete gold outputs, timing checks, counters and profiles. It confirms the failed gate. Native builds, gold generation, decodes, recordings and the successful analyzer are not replayed.

All aggregate values below are ON/control ratios with sorted zero-based ranks3/7/11 of15. They have nominal96.484375% marginal coverage under exchangeable blocks, without familywise coverage. Each aggregate has **0/15 wins**. Field/radix aggregates contain their four operation contexts at 32 workers; ALL includes all 48.

|Aggregate|ON/frozen [lower, upper]|ON/OFF [lower, upper]|ON/OLD [lower, upper]|
|---|---|---|---|
|ALL|1.022042 [1.019571,1.025540]|1.021001 [1.018568,1.027359]|1.085000 [1.081187,1.087945]|
|Fq/R256/w32|1.040135 [1.025188,1.050458]|1.046317 [1.028072,1.055294]|1.097509 [1.085737,1.110791]|
|Fq/R260/w32|1.108508 [1.097101,1.116905]|1.107511 [1.098089,1.119704]|1.106548 [1.095428,1.116197]|
|Fr/R256/w32|1.036950 [1.031907,1.052313]|1.037038 [1.031353,1.053095]|1.098107 [1.091840,1.121250]|
|Fr/R260/w32|1.107931 [1.098924,1.119307]|1.117167 [1.109854,1.128561]|1.107794 [1.100387,1.117593]|

The gate has **120 failed checks: seven calibration, 98 nonregression and all 15 gain checks**. It requires every OFF/frozen interval within factor1.03, every ON/control upper bound≤1.02, and each declared aggregate upper<1 with≥10/15 wins against all three controls. There is no percentage gain floor. Failed calibrations are Fq256 mul4/solo; Fq260 mul1/32 and mul4/solo; Fr256 mul4/solo; Fr260 mul1/32, mul4/solo and mul4/32. Their causes remain unresolved; no failed calibration is discarded or repaired through source identity alone.

Positive observations remain in [all 48 timing rows](artifacts/results.md) and the [complete analysis](artifacts/analysis.json). Fq/R256 ordinary mul1 improves9.36% at solo and9.68% at16 versus OFF, each15/15 wins. Four-bank R256 multiplication improves2.66% for Fq/32 and2.91% for Fr/32, with13/15 and15/15 wins. These do not establish a universal field or proof gain. At32, Fq/R256 sum2/difference regress7.34%/14.19% versus OFF. No favorable operation/field/worker subset is promoted from this screen.

## Executed work, profiles and memory

Separate PMU panels show additional retired instructions despite fewer IFMAs. Values below are ON/OFF ratios in Fq/R256; one panel per case is diagnostic, not a replicated uncertainty estimate. Summed worker CPU cycles are distinct from joined wall latency.

|Pattern/workers|Retired instructions ON/OFF|User cycles ON/OFF|
|---|---:|---:|
|mul1/1|1.083718|0.900523|
|mul1/32|1.081136|1.134265|
|mul4/1|1.085204|0.900101|
|mul4/32|1.085297|0.914479|
|sum1/1|1.254520|0.922713|
|sum1/32|1.254063|1.078170|
|diff1/1|1.227992|0.939384|
|diff1/32|1.226993|1.108819|

All four counter events are nonnegative with ≥99.9% running coverage. Actual Fq/R256 linked loop-kernel instruction sites are OFF 234/314/349, OLD 221/302/335, ON 252/385/417 for mul/sum/difference. Corresponding RSP/RBP-addressed rows are OFF 6/12/30, OLD 6/12/30, ON 6/22/32; all have no calls. These are whole benchmark-loop static counts, not spills or traffic measurements; RBP can also hold ordinary data. Extra sign/recombination and compiled work are material tradeoffs; the measurements do not isolate every regression's cause or establish a RAM/bandwidth limit.

All **32 profiles qualify**, totaling346,573 samples, minimum1,100 per profile, maximum unknown0.06429%, minimum assigned-worker99.9904% and exact-target98.2938%, with zero LOST. Profiles use cycles:u period50,000, 512 mmap pages and acknowledged FIFO warm gating. Binary mappings, PID/TID/CPU ownership, inheritance and user-only metadata are checked. Flat sample locations do not establish a causal critical path or removable proof-time fraction. [Counter/profile summary](artifacts/metrics.json).

All 3,136 timed regions make zero allocation requests. Initial/current capacities are stable and equal across arms: `[1280,1280]` bytes per one-bank logical fixture or `[5120,5120]` for four banks. ON process high-water RSS ranges2.50–3.25MiB. These small synthetic-process observations are not prover peak RAM, minimum RAM, stack peaks, phone or huge-circuit qualification.

## Exploratory schoolbook-control observation

The fixed local-tile candidate remains rejected. A separate **post-capture descriptive calculation** gives OLD/OFF ALL48 ratio0.942967 [0.941232,0.944690],15/15 wins:5.70% lower joined primitive time. OLD/frozen is0.941775 [0.939730,0.947538],15/15. Raw block-time calculations and same-block aggregate-ratio quotients agree. R256/32 descriptive aggregates favor OLD, while R260/32 intervals include parity. No new selection gate is applied to OLD and it is not adopted or relabeled as this campaign's passing candidate. [Every descriptive ratio](artifacts/descriptive-control.json).

This suggests inspecting the control's exact reduction/packing/source differences and prior row-pair/packing history before selecting a distinct complete-caller experiment. The older negative component and SDK results stay closed. The observation is not a proof-speed estimate, an unchanged retry authorization or a selected favorable-subset rescue.

## Preserved offline repair

The original reader failed on exact JSON equality for a derived counter rate:233.02130889892575 versus233.02130889892578. The separate fixed reader changes only that comparison to its existing numerical tolerance, with explicit null handling. Raw counts/coverage, timing/calibration/nonregression/gain thresholds and profile-quality gates remain fixed. Original pinned reader source/binary and failure streams are intact, and the exact one-expression difference is verified for publication. [Repair](artifacts/review-repair.md).

## Accepted full-proof comparisons, inherited

No new SDK requests or proofs occur. Production176/SDK6d83 and canonical35,171 remain unchanged. These local32 accepted SDK observations and supplied historical M5 references are inherited; they are not timings of the primitive candidate. The same accepted binary previously measured7.915/7.929/7.996ms locally for1×2; latest8.003ms is another observation, with variation cause unconfirmed and no adopted regression. Different machines do not establish equal hardware efficiency.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 /4.623×|24 /2.999×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|merge-24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Source: [preceding inherited comparison](../2026-11-08-blocked-pair52-model176/README.md). Seven nonprimary rows remain from the earlier full campaign. Merge54 remains162.240109ms, without a supplied M5 reference. The verified CPU10× goal stays active and unmet, **0/8**. No fresh AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Protocol](artifacts/protocol.md), [driver](artifacts/driver.rs), [source audit](artifacts/source-audit.json), [release qualification](artifacts/qualification.json), [linked code](artifacts/codegen.json), [independent review](artifacts/review.json), [pins](artifacts/pins.json), [exact sources/raw observations/receipts/decoded profiles](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) retain positive and negative evidence. Public analysis removes only the duplicated caller map; its complete original is archived. Every UTF-8 payload and gzip roundtrip is verified. Binaries/libraries, raw perf data, unchanged full sources, keys/images/witnesses/proofs and full third-party text are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/blocked-pair52-native176/bench`.
