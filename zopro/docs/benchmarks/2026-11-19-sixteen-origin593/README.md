# Combining public Euler origins and curve groups on 16×2

The offline conversion folds a fixed public reference into the K points, retains the original private slots and exports a proof verified with the original key. A streaming reader verifies that only the appended dictionary changes when the compact block layout replaces the flat dictionary. All Zopro settings use identical external wire offsets. The grouped modes prepare the same A/K/B2 indices against the actual loaded points. Five shuffled blocks use 32 pinned Ryzen workers, three warmups and eleven measured calls. The two additional public transfers are held out. The Aeglos arm uses its prepared key on all measured calls. An independent verifier checks the public outputs and proof of every process.

| Request | Release / grouped / blocked / combined / Aeglos ms | Combined vs grouped / Aeglos | Max combined / Aeglos RSS MiB | Combined vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| Original | 39.551 / 36.971 / 39.009 / 36.888 / 237.336 | +0.44% (2/5) / -84.50% (5/5) | 753.2 / 4299.2 | 9.08× / 3.36× |
| 75 | 39.387 / 36.186 / 38.584 / 36.572 / 238.379 | +0.66% (2/5) / -84.80% (5/5) | 733.7 / 4321.1 | — |
| 80 | 39.270 / 36.636 / 39.287 / 36.613 / 238.304 | +0.44% (2/5) / -84.69% (5/5) | 733.1 / 4315.5 | — |

All 76 exports verified. [Complete process samples](artifacts/samples.json.gz) · [Conversion and key identities](artifacts/inputs.json) · [Verification](artifacts/verification.json)
