# Balanced K/H portability and resource checks

The measured 175-file candidate passes the additional compatibility and resource checks: 101 portable core tests, 8 portable prover tests, 248 internally verified SDK requests and 89 independently verified Gnark exports. All eight cold/warm cgroup cases finish under the declared limits without OOM or maximum-limit events. Production remains unchanged; the candidate is not adopted.

The [full performance qualification](../2026-10-28-balanced-kh-qualification175/README.md) remains **failed** solely on physical-16 identical-baseline calibration. At 32 workers it measured 8.039940 ms versus accepted 8.105814 ms, paired 0.813% faster. This study adds no latency measurements, repeats no successful capture and changes no performance gate.

## Coverage

The source is copied byte for byte from the measured candidate: 173 of 174 original files are unchanged, one parent has the private balanced K/H hook, and one child implements the variable-width windows. No runtime arithmetic, scheduler, key, allocation forecast, native flag or library changes are made here. Native 122/124 core tests, 8 prover tests per variant and 33 field tests are inherited. Portable field source and manifests match the earlier qualified snapshot; its 11 field tests are inherited, not rerun. The new portable x86-64/no-IFMA/no-ASM build includes both balanced scalar/point oracle tests in its unfiltered 101-test core suite.

- 13 portable processes cover all eight reference shapes plus 54→1 with basis images, and primary/54 with raw and denied optional-preparation paths.
- 24 native candidate processes cover 1/4 workers, 512 KiB/1 MiB worker stacks, primary/54 and basis/raw/denied paths. Eight accepted/candidate processes disable packed, direct, canonical-batch or vector-inverse processing individually.
- Four concurrent processes cover accepted/candidate × primary/54, with four callers making two proofs each. They check 33 idle and 37 loaded TIDs, consistent with the existing global 32-worker pool plus callers. All 32 exports verify independently.
- Eight resource processes cover accepted/candidate × cold/warm × default/retained allocator settings. Each charges its own private image copy inside its owned transient service. Swap is disabled.

Resource maximum is derived from accepted primary peak RSS: rounding 1.25 × 133.5 MiB upward to 64 MiB gives 192 MiB. Warm high is 0.9 × 133.5 MiB rounded upward to 8 MiB, giving 128 MiB. Warm cases prime at 512 MiB, add 32 MiB ballast and lower high/maximum to 128/192 MiB before two fresh proofs. These are controlled test conditions, not an architectural cap or minimum-RAM estimate. No larger-limit retry is allowed.

Raw and basis images retain their recorded identities and public bindings. Independent Gnark verification uses the corresponding Euler image for its verifying key; raw-loader correctness does not require changing the verifier parser.

## Resource observations

| Case | Passed | Cgroup peak MiB | High events | Max events | OOM/kill | Swap limit |
|---|---|---:|---:|---:|---|---|
| cold-accepted-default | true | 61.46 | 0 | 0 | 0/0 | 0 |
| cold-accepted-retained | true | 62.97 | 0 | 0 | 0/0 | 0 |
| cold-on-default | true | 60.17 | 0 | 0 | 0/0 | 0 |
| cold-on-retained | true | 61.29 | 0 | 0 | 0/0 | 0 |
| warm-accepted-default | true | 102.12 | 0 | 0 | 0/0 | 0 |
| warm-accepted-retained | true | 102.86 | 0 | 0 | 0/0 | 0 |
| warm-on-default | true | 102.63 | 0 | 0 | 0/0 | 0 |
| warm-on-retained | true | 105.34 | 0 | 0 | 0/0 | 0 |

All cases recorded **zero memory.high events**. These runs establish operation under the configured limits; they do not establish behavior under active throttling or reclaim. Cold peaks were 60.17–62.97 MiB and warm peaks were 102.12–105.34 MiB.

Peaks include the private image, controller, SDK and any ballast. High events identify observed throttling; complete pressure, event, CPU, memory and thread receipts are retained. The controller stops/resets only uniquely prefixed owned units and removes only their control directories. No global memory/kernel settings are changed.

## Verification and limits

All 248 SDK requests verify internally. The 89 fresh 164-byte exports pass independent Gnark verification, expected public/key binding, member hashing and uniqueness against 16,411 previous proof identities. The ledger reaches 16,500; 178 proof/public members represent 89 proofs. All build, helper and source pins close. Helpers and consumer builds complete before proof/resource captures; no owned build, hash, archive or second workload overlaps them. Unrelated host work is not excluded.

This result does not authorize adoption or turn the failed full-performance gate into a pass. No fresh mobile, TEE, AWS/Aeglos, swap, 250/4999 or minimum-RAM claim is made. The giant images are unavailable here. There is no new lookup table, duplicate key plan or prover memory cap.

## Inherited CPU and Metal comparisons

These are the preceding full performance qualification and supplied historical M5 references, not measurements from these functional/resource checks.

| Shape | Accepted ms | Candidate ms | M5 CPU ms | M5 Metal ms | CPU/candidate | Metal/candidate |
|---|---:|---:|---:|---:|---:|---:|
| 12x2-w32 | 33.840266 | 33.734127 | 284 | 101 | 8.419× | 2.994× |
| 16x2-w32 | 40.034518 | 40.096854 | 335 | 124 | 8.355× | 3.093× |
| 1x2-w32 | 8.105814 | 8.039940 | 37 | 24 | 4.602× | 2.985× |
| 24x2-w32 | 66.310989 | 66.143755 | 528 | 176 | 7.983× | 2.661× |
| 2x2-w32 | 11.291047 | 11.313311 | 59 | 30 | 5.215× | 2.652× |
| 4x2-w32 | 16.020905 | 16.021489 | 109 | 48 | 6.803× | 2.996× |
| 8x2-w32 | 24.082256 | 24.042427 | 189 | 74 | 7.861× | 3.078× |
| merge-24x1-w32 | 65.345371 | 65.132194 | 507 | 175 | 7.784× | 2.687× |

The original all-eight CPU 10× goal remains active and unmet, **0/8**. Primary requires 3.7 ms. There is no supplied M5 reference for 54→1.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source reuse](artifacts/source-reuse.json), [build receipts](artifacts/build/result.json), [functional receipts](artifacts/functional/result.json), [resource receipts](artifacts/resource-result.json), [proof inventory](artifacts/proof-inventory.json). Native and portable binary payloads remain local with hashes and frozen-library receipts.
