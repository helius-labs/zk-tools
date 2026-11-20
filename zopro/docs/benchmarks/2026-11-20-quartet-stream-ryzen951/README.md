# Compact quartet input stream for 4×2 proving

The compact index stream fell short of the paired full-proof requirement. It took **13.329 / 13.330 ms** on Ryzen with 24 main and eight frontend threads.

Each prepared query retains the indices outside all candidate quartets, under the existing optional workspace policy. A proof adds one summed point index for each equal nonzero quartet and the four original indices for every other quartet. The classifier scans only those entries. This removes the per-proof marker array. It changes class encounter order by processing the outside indices first.

The frozen SDK, same-build ordinary path, [earlier direct executable](../2026-11-20-quartet-origin-runtime-ryzen948/README.md), and same-build marker path are the paired controls. Each input uses eight counterbalanced blocks with three primes and seventeen measured proofs per process.

| Input | Frozen ms | Ordinary ms | Prior direct ms | Marker ms | Stream ms | Stream vs marker | Stream vs prior | Stream / marker peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | 13.652 | 13.585 | 13.303 | 13.332 | 13.329 | -0.03% (4/8) | +0.47% (3/8) | 220.8 / 227.2 |
| heldout | 13.686 | 13.546 | 13.308 | 13.305 | 13.330 | +0.02% (4/8) | +0.17% (4/8) | 216.2 / 220.2 |

| Input | K entries scanned | B2 entries scanned | Marker vs prior calibration |
|---|---:|---:|---:|
| catalog | 44,242 | 42,704 | +0.10% (4/8) |
| heldout | 46,366 | 44,828 | -0.14% (6/8) |

All three native quartet tests and all 88 independent proof exports verified. Route checks confirmed exact accepted memberships and nonzero counts, plus zero optional capacity and an invalid K plan with valid B2. The timing applies to local full proofs.

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
