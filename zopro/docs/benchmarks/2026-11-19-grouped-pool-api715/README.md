# Routing grouped G2 windows through a caller-owned pool

The Rust prover exposes `set_grouped_g2_on_frontend` on Linux x86-64. An opted-in key runs its final 12-bit grouped G2 windows on the attached, smaller frontend pool when the resource policy admits the extra parent state and one scratch leaf per worker. The default executes on the main pool. The option creates no tables or worker pools.

The two shapes that crossed the Aeglos median on both inputs in the earlier placement study ran in six fresh paired blocks per transaction on 96 physical cores of the same c7a.24xlarge. At 4×4 the main pool used 64 cores and the frontend 32. At 4×8 they used 56 and 40. All four arms used their respective frozen settings. The direct SDK build and its disabled control differ only in the new option. Each process ran three warmups and thirteen verified requests. An independent verifier checked 96 AWS exports and 48 local exports covering enabled, disabled, detached and zero-budget configurations. The native core passed its 157 tests and the portable benchmark examples compiled.

| Shape | Input | SDK on / off / research / Aeglos ms | On vs off | On vs Aeglos | Peak MiB SDK on / Aeglos |
|---|---|---:|---:|---:|---:|
| 4x4 | catalog | 18.178 / 18.654 / 18.026 / 19.129 | -2.03% (6/6) | -4.67% (6/6) | 238.7 / 1390.6 |
| 4x4 | heldout | 18.282 / 18.537 / 18.308 / 19.008 | -1.07% (4/6) | -3.92% (6/6) | 234.2 / 1383.1 |
| 4x8 | catalog | 19.287 / 19.736 / 19.402 / 19.601 | -0.54% (3/6) | -1.15% (5/6) | 289.7 / 1454.3 |
| 4x8 | heldout | 19.418 / 19.620 / 19.460 / 19.738 | -0.84% (5/6) | -1.54% (5/6) | 276.3 / 1462.2 |

[Samples](artifacts/aws-rows.json.gz) · [Build](artifacts/build.json) · [Proof verification](artifacts/verification.json) · [Earlier placement study](../2026-11-19-tile-c7a703/README.md)
