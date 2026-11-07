# Descriptor-mask complete G1/G2 caller screen

**The declared caller gate fails; no production change is adopted.** G1 improves about 2.1–2.4% at one and sixteen workers, with fifteen wins in every comparison against both controls. G2 shows no clear gain. Wide G1/32 control intervals prevent a reliable same-build conclusion. All seven failed checks remain recorded.

The [qualified helper](../2026-11-07-sign-mask176/README.md) replaces shifted-bit horizontal reduction with a vector sign-mask extraction. Existing loads were already vectorized. This stage changes only `signs`: native signed calls use the helper; other descriptor strides retain the exact original fallback. Field arithmetic, records, packing, scalar tails, inversion, point ownership and Window/query scheduling remain unchanged. No table, point copy, heap buffer, key plan, runtime pool or arbitrary RAM cap is added.

## Complete caller results

Ryzen 9950X, sixteen physical cores and 32 logical CPUs. Synthetic changing-state signed borrowed batches contain 200 or 352 additions to distinct destinations. Workers pin to CPU 2 solo, CPUs 0–15 or 0–31, with 512 KiB stacks. Three checked 128-round primes precede 4,096 observed batches per fixture. Ref assembly, dispatch/wait, original prefix/inverse/field/scatter work are timed; pool/fixture construction and final generator checks are outside. Intermediate timed states are not each independently checked. These are caller waves, not Window or proof latency or concurrent-proof throughput.

Fifteen fixed rotated/reversed blocks compare an immutable original caller, fresh OFF and fresh ON. The original control is reused from the earlier high-borrow control after all 176 sources recover and runtime fixture/driver/support/native flags/binary identities agree. OFF/ON reuse the original accepted field library coherently; both new cores and final O3/native/fatLTO/oneCGU consumers are built once. No rejected field candidate is imported.

Times are median microseconds per wave. Paired ratios need not equal ratios of marginal medians.

|Context|Original µs|OFF µs|ON µs|ON/OFF [low, high]|Wins/15|
|---|---:|---:|---:|---|---:|
|G2-200-w1|7.668|7.669|7.673|0.999824 [0.999319, 1.001185]|9|
|G2-200-w16|8.896|8.931|8.925|0.999843 [0.995838, 1.005253]|9|
|G2-200-w32|14.554|14.339|14.344|0.999501 [0.945819, 1.022940]|8|
|G2-352-w1|12.977|12.985|12.981|0.999892 [0.999137, 1.000508]|8|
|G2-352-w16|15.179|15.214|15.185|0.998942 [0.998140, 1.000140]|11|
|G2-352-w32|25.258|25.066|25.094|1.000619 [0.993985, 1.010865]|7|
|G1-200-w1|4.213|4.218|4.120|0.977209 [0.975707, 0.977999]|15|
|G1-200-w16|4.776|4.774|4.671|0.976212 [0.970108, 0.985536]|15|
|G1-200-w32|6.881|6.857|6.795|0.991753 [0.830694, 1.078584]|11|
|G1-352-w1|6.928|6.938|6.772|0.976152 [0.975264, 0.977235]|15|
|G1-352-w16|7.929|7.927|7.777|0.979439 [0.976441, 0.983625]|15|
|G1-352-w32|11.955|11.667|11.601|0.996371 [0.968834, 1.220604]|9|

|32-worker two-size aggregate|Median ratio [low, high]|Wins/15|
|---|---|---:|
|ON versus G1/OFF|0.996997 [0.922173, 1.058278]|8|
|ON versus G1/frozen|0.980210 [0.917367, 0.994678]|13|
|ON versus G2/OFF|0.999837 [0.980528, 1.008797]|8|
|ON versus G2/frozen|0.996783 [0.977327, 1.008181]|9|

The prospective gate requires all twelve OFF/original intervals within factor 1.03, all twenty-four ON/control nonregression upper bounds ≤1.03, and both curves' 32-worker aggregate upper ratios <1 with at least ten wins against both controls. Ordered ranks 4–12/15 give nominal 96.484375% marginal median coverage under independent exchangeable blocks, without simultaneous coverage. There is no gain floor, excluded block, sample extension or reroll.

Failures are two G1/32 control calibrations, two corresponding ON/OFF nonregression bounds, G1 aggregate gain versus OFF, and both G2 aggregate gains. G1/32 OFF/original upper bounds 1.0677 and 1.0361 describe uncertainty, not measured 6.8% and 3.6% slowdowns. G1 gains at 1/16 workers are independently calibrated; G2 near-parity estimates are retained without a gain claim.

## CPU, allocation and validation

Separate single PMU panels show the following ON/OFF ratios. They do not provide replicated counter uncertainty or isolate an exclusive cause of latency.

|32-worker panel|Instructions|Cycles|
|---|---:|---:|
|G1-200-w32|0.995705|0.987639|
|G1-352-w32|0.995353|0.986615|
|G2-200-w32|0.998884|1.004213|
|G2-352-w32|0.998829|1.000618|

All eight fixed-period cycles profiles qualify: 62,434 samples, at least 3,863 each, maximum unknown share below 0.080%, zero LOST, owned PID/TIDs, worker CPUs, mappings/task/header/events and 16M-period checks. Actual G1/G2 kernels are sampled. Flat bins include inlined work and do not identify exclusive critical-path cost. All 32 PMU events run at least 99.9%. Thirty-six preflights and all 540 latency processes finish before eight counter panels and eight profiles; forty offline ownership/decode/loss steps follow all recordings. No owned build, analysis, hashing or publishing overlaps capture.

All 592 caller processes have zero timed allocations and stable scratch capacities. Per-worker records remain G1 40,000/70,400 bytes and G2 16,000/28,160 at 200/352 additions. These are record capacities, not whole-process peaks or minimum RAM. Lifetime RSS and faults remain in raw observations; fixture/reference arrays and primes are included, so small RSS differences do not prove production memory savings or pressure behavior. There is no new mobile/ARM/TEE, huge-circuit, swap or concurrency qualification.

Three new targeted tests pass, with 125 old core tests filtered. They cover both curves across all 256 masks, eighteen zero/full/tail lengths through 353, three changing updates and 1/16/32-worker 512 KiB stacks; complete bucket arrays match independent generator-coefficient references. Source arrays remain unchanged and scratch reuse is stable. A non-Copy descriptor test preserves the parent's Borrow-only interface; the helper's unused Copy bound is removed without changing its loads or arithmetic. Prior helper/field populations are inherited rather than rerun. The whole prover also compiles with generic/no_asm/no_ifma flags.

The 179-file snapshot recovers all 176 accepted sources: 174 are byte exact, and only the two diagnostic parents plus one dispatch insertion differ. The helper reverses to its qualified source after imports, visibility and unused-bound adaptation. All field sources and the original accepted field library remain unchanged. No complete SDK request or Gnark proof occurs; the distinct-proof ledger stays 32,857. Benchmark loops execute 57,135,872 observed plus 3,809,280 prime batches, not proofs.

Preparation records two tooling issues. Initial pin finalization tried to hash a core library before its producing command; a separate finisher retained the source audit and excluded only declared future outputs. The three new tests then passed, but inherited workflow metadata still required an older test name and an unfiltered suite. A reader validates the existing three successes and 125 filtered tests without rerunning them, then executes the two pending build/fallback commands. Raw success receipts and the workflow-policy failure are retained. All caller captures and analysis pass on first execution; no successful test, build, recording, decoder or prior verifier is replayed for bookkeeping. Review checks 641 raw-successful workflow commands, including the cached-test reader and 633 capture/offline steps.

## Next measurement and original goal

The caller gate remains **failed**, and this screen does not authorize production adoption. A separately declared complete-SDK study is selected as an engineering exception: four calibrated G1 contexts show repeatable gains, G1 counters decrease, and G2 passes nonregression with no clear gain. No caller reroll or G1-only variant follows. The next candidate retains this exact helper for **both curves**, removes only diagnostic adapters, and must pass complete-SDK correctness, independent Gnark verification, resource/fallback checks and matched proof measurements with control calibration. This tests the actual objective; it does not revise the failed caller gate. [Decision](artifacts/decision.json).

Accepted SDK 6d83 and proving timings remain unchanged. The latest matched primary observation is 8.027583 ms for the same binary previously measured below 8 ms. These all-shape adoption results and supplied M5 references are inherited, not remeasured here.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 /4.667×|24 /3.027×|3.7|
|2×2|11.151932|59 /5.291×|30 /2.690×|5.9|
|4×2|16.086386|109 /6.776×|48 /2.984×|10.9|
|8×2|24.179453|189 /7.817×|74 /3.060×|18.9|
|12×2|33.884455|284 /8.381×|101 /2.981×|28.4|
|16×2|40.077381|335 /8.359×|124 /3.094×|33.5|
|24×2|65.850777|528 /8.018×|176 /2.673×|52.8|
|merge24×1|65.060937|507 /7.793×|175 /2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. The all-eight verified CPU 10× goal remains active and unmet, **0/8**. No new AWS/Aeglos or proof-level gain is claimed.

[Design and fixed protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [source/receipts/decoded profiles](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
