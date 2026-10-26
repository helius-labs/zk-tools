# Clean fixed-parameter power integration

The short screen did not advance to full qualification. At 32 workers, the clean candidate improves by a paired 0.362% against the fresh build, but regresses 0.699% against the accepted executable and loses all three accepted comparisons. One 16-worker fresh/accepted control differs by 23.5%. Production is unchanged; the candidate is retained.

Small gains remain useful and have no minimum acceptance floor. This screen neither discards the earlier positive arithmetic result nor resolves its failed full qualification. It tests a distinct integration, without the earlier runtime selector.

The Rust change calls the existing fixed power schedule from the canonical final-exponentiation body. It removes the environment lookup, OnceLock policy and duplicate final-exponentiation implementation. The mathematical schedule remains 60 cyclotomic squares, 17 full extension products and five conjugations per parameter power, removing nine squares and 33 full extension products per final exponentiation. The [previous study](../2026-10-26-pairing-x-chain-sdk/README.md) records provenance and earlier measurements.

Only pairing.rs and a new private x_chain.rs change; 169 original files remain byte-identical. There is no new table, input cache, heap workspace, thread pool, field/MSM/solver change or memory cap. Compiler output and the integration are part of the comparison; timings do not isolate arithmetic alone.

## Verified SDK screen

AMD Ryzen 9 9950X. Three rotated/reversed fresh-process blocks, 17 timed complete requests after three primes. Workers use CPUs 0–15 or 0–31. Variants are the exact accepted executable, a fresh unmodified build, and the clean candidate. The standard SDK driver, allocator policy and inputs are identical.

Timers include witness construction, fresh blinded proving, verification and serialization; loading and primes are excluded. Paired ratios compare process medians within blocks. Displayed medians are independent and need not reproduce those ratios. CPU time includes scheduling, and RSS includes loading and primes.

|Workers|Accepted ms|Fresh ms|Clean ms|Clean/accepted|Clean/fresh|Paired saving vs fresh µs|Wins vs fresh|Clean peak RSS MiB|Clean CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|16|10.385817|12.515634|10.246784|0.917842835|0.859455435|2268.850|3/3|76.25|129.787|
|32|8.497706|8.547550|8.555135|1.006985200|0.996378422|60.884|2/3|92.75|198.488|

These maxima do not establish minimum RAM or a memory saving. The apparent 16-worker improvement is affected by divergent controls. Its fresh/accepted block ratios are 0.990810, 1.235103 and 0.995271. Accepted, fresh and clean processes all show slower 16-worker cases, so the source cleanup did not eliminate that variation. The [scheduling diagnostic](../2026-10-26-control-rescheduling/README.md) gives context; those instrumented runs do not identify this screen’s cause.

The clean primary result, 8.555135 ms, is 4.325× faster than the supplied M5 CPU reference of 37 ms and 2.805× faster than the supplied Metal reference of 24 ms. These are historical supplied references, not new Apple measurements. The previous full SDK report retains comparisons for all eight supplied shapes; the CPU 10× goal remains unmet.

## Gate and validation

The predeclared advancement gate requires fresh/accepted calibration within factor 1.05 per block and 1.03 paired median; primary32 improvement against both controls with at least two winning blocks; and both cases within 1.02 of both controls. It fails the 16-worker calibration and primary32 improvement against accepted. The nonregression checks pass. All first-capture results are retained; no unchanged retry or full-stage measurement follows.

Native field tests (35), core tests (109), raw/cubic prover tests (2/6), generator consistency, portable/no-IFMA field tests and consumer checks pass. The symbolic operation-count test and complete final-exponentiation comparisons exercise the actual canonical function against the frozen general-power reference; independent Arkworks checks remain intact.

All nine shapes pass fresh/clean preflight with four workers: 72 SDK requests and 18 independently Gnark-verified exports. The screen adds 360 requests and 18 exports. Combined: **432 complete verified SDK requests, 36 distinct exported proofs and 72 proof/public members**. Every export passes pinned public-input binding, 164-byte proof length, freshness against 6,814 prior identities and independent Gnark verification. Next prior ledger: **6,850**. All proof members are archived and hash-checked.

All 170 production source files and source, exact-library, binary, input and tool pins pass the closing audit. Builds and hashes occur before SDK capture or after every capture. The short screen collects no new cycle profile; previous profiles do not characterize this integration. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, pressure, minimum-RAM or throughput measurement is claimed. Full qualification and the original all-eight objective remain unproven.

[Design](artifacts/design.md), [source audit](artifacts/implementation-audit.json), [screen analysis](artifacts/screen-analysis.json), [raw SDK rows](artifacts/screen/rows.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifact-index.json).
