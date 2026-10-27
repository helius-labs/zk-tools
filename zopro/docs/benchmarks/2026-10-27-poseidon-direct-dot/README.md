# Direct width-three Poseidon fusion

Primary32 shows a positive signal: **8.523445 ms**, with a paired median saving of **37.100 µs versus fresh** and **93.566 µs versus accepted**, winning 2/3 blocks against both controls. Paired geometric ratios are **0.997346 / 0.989860**, respectively. The screen does not advance because physical16 control calibration and nonregression fail. The positive candidate is retained; production is unchanged. No minimum positive-gain threshold is used.

The width-three dense matrix now forms each row with the existing fused two-product field operation plus its third ordinary product. The partial-round rest uses one fused pair directly. This removes the [previous generic helper](../2026-10-27-poseidon-fused-dot/README.md), its slice loop and its zero accumulator. It is a distinct source version; the preceding failed screen is not repeated or reclassified.

Only `solver/sparse_poseidon.rs` changes: two explicit width-three formula branches. Reversing those changes recovers every original byte; the other 169 originals remain identical. Other widths retain the original loops. All S-boxes, constants, prefixes, recognizer checks, original wire/slot/A/B/C writes, scheduling and dependency ordering remain unchanged. There is no new module, field primitive, runtime experiment flag, table, copied plan, heap workspace, pool, image-format or resource-policy change.

## Verified SDK comparison

The controls are the exact historical accepted SDK and the exact fresh baseline executable built in the preceding experiment. Its original 170-source baseline, linked libraries, consumer and binary pins are rechecked; this report claims no new baseline build. Its earlier all-nine-circuit independent proof validation is already in the prior 6,982-proof ledger and is not counted again. Only the direct candidate is rebuilt with the same compiler, native flags, fat LTO, one code-generation unit and standard SDK driver.

Fresh blinded proving, witness construction, self-verification and serialization are timed together; loading and three primes are excluded. Process medians pair within rotated/reversed fresh-process blocks. CPU includes scheduling; lifetime RSS includes loading/primes and is not minimum RAM.

|Case|Accepted ms|Fresh ms|Direct ms|Direct/accepted|Direct/fresh|Saving vs fresh µs|Wins vs fresh|Direct RSS MiB|Direct CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2-w16|13.160621|11.920432|11.876990|1.004735|1.045826|43.442|2|72.75|127.015|
|1x2-w32|8.570033|8.590331|8.523445|0.989860|0.997346|37.100|2|105.50|197.106|

The short screen has three blocks of accepted/fresh/direct at primary16 and primary32, with 17 timed requests plus three primes per process. Declared gates require fresh/accepted calibration within factor 1.05 in each block and factor 1.03 for paired medians; primary32 improvement against both controls in at least two of three blocks; and both cases within 2% nonregression against both controls. All failures and block ratios remain retained, without exclusions, threshold relaxation or successful repeats.

Calibration failures: `["1x2-w16 block1 fresh/accepted=0.9057651610816845","1x2-w16 median fresh/accepted=0.9576730870801967"]`.

Primary improvement failures: `[]`.

Nonregression failures: `["1x2-w16 direct/fresh=1.0458262307278445"]`.

Physical16 block0 medians are accepted 13.698235, fresh 13.118431 and direct 15.820077 ms; in block2 they are 10.292071, 10.520722 and 10.015740 ms. The first direct block has less summed CPU time but more involuntary switches than its controls. These observations describe the variability and do not establish external preemption or an arithmetic regression as the cause. Every block remains included. The physical16 geometric regression is retained even though the paired median saving there is positive.

The full stage and cycle profiles did not run. The [last complete all-shape SDK matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied M5 CPU/Metal comparisons and remains authoritative outside this short screen. No arithmetic-only timing was used for selection.

## Validation and limits

All 109 native core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. The existing independent all-width symbolic fixture covers 384 complete-buffer comparisons, 48 malformed-recognition rejections and 48 tail-error comparisons, including width-three, reversed operands and present/omitted C. No duplicate helper tests were added. Field source and qualification are inherited unchanged; no new field-suite run is claimed.

The actual linked direct SDK contains no generic `sparse_poseidon::dot::dot` helper symbol; the source has two explicit fused call sites. [Code-generation audit](artifacts/codegen-audit.json) retains that check. Removing the helper does not by itself establish a causal latency gain. No dynamic call count is claimed.

Candidate preflight covers all nine circuits at four workers, one timed request plus three primes: 36 SDK requests and nine fresh independently verified exports. Subsequent measured exports also pass independent Gnark verification, 164-byte length, pinned public-input binding and freshness against all prior identities. Internal primes are not counted as exported proof identities.

Total **396 SDK requests**, **27 distinct independently verified exports**, **54 proof/public members**; next proof-ledger count **7009**. Full stage: **false**; new cycle profiles: **0**. All 170 production files and accepted SDK are unchanged. The original all-eight CPU10× objective remains active and unmet; this report adopts no change.

No owned helper, build, hash or archive overlaps SDK timing/profile captures. Closing checks rehash all pinned sources, libraries, binaries, tools and proof members. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, memory-pressure, concurrent-prover or throughput qualification is claimed.

reused fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

direct executable: `/tmp/zopro-poseidon-direct-dot/native/direct-sdk`; SHA-256 `32c25a093502981c012f8eb083586cdf86df112d22829ec54bc66bae47afd68f`.

[Design](artifacts/design.md), [implementation audit](artifacts/implementation-audit.json), [screen analysis](artifacts/screen-analysis.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
