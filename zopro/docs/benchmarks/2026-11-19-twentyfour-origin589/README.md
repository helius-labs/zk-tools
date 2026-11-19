# Combining public Euler origins and curve groups on 24×2

The offline conversion folds a fixed public reference into the K points, retains the original private slots and exports a proof verified with the original key. A streaming reader verifies that only the appended dictionary changes when the compact block layout replaces the flat dictionary. All Zopro settings use identical external wire offsets. The grouped modes prepare the same A/K/B2 indices against the actual loaded points. Five shuffled blocks use 32 pinned Ryzen workers, three warmups and eleven measured calls. The two additional public transfers are held out. The Aeglos arm uses its prepared key on all measured calls. An independent verifier checks the public outputs and proof of every process.

| Request | Release / grouped / blocked / combined / Aeglos ms | Combined vs grouped / Aeglos | Max combined / Aeglos RSS MiB | Combined vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| Original | 64.702 / 59.956 / 63.081 / 58.838 / 457.680 | +0.03% (2/5) / -87.13% (5/5) | 1093.7 / 6355.8 | 8.97× / 2.99× |
| 99 | 64.506 / 57.953 / 62.191 / 58.338 / 455.937 | -0.83% (3/5) / -87.24% (5/5) | 1082.7 / 6340.6 | — |
| 103 | 63.699 / 57.990 / 63.100 / 58.644 / 458.561 | +0.91% (1/5) / -87.22% (5/5) | 1121.5 / 6338.5 | — |

All 76 exports verified. [Complete process samples](artifacts/samples.json.gz) · [Conversion and key identities](artifacts/inputs.json) · [Verification](artifacts/verification.json)
