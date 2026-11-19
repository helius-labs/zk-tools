# Preconverted public row offsets on 4x2

Five randomized blocks use 32 pinned Ryzen workers and measure 21 single verified SDK calls after three warmups in each process. The candidate uses the same executable for its ordinary and preconverted keys. The release and warm Aeglos prover provide independent controls. All exported proofs verify with the original key and expected public outputs. The input row dictionary reconstructs every original public reference pair. The derived key has the original private slots and the flat and blocked files match byte-for-byte before the dictionary.

| Request | release / ordinary / blocked / aeglos / grouped / both ms | both vs grouped | both vs release | Max both / Aeglos MiB | both vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 15.201 / 15.354 / 15.359 / 37.751 / 15.120 / 14.870 | -1.99% (5/5) | -2.04% (5/5) | 334.2 / 1275.7 | 7.33× / 3.23× |
| 153 | 15.490 / 15.622 / 15.469 / 38.034 / 15.468 / 15.379 | +0.89% (2/5) | -0.18% (3/5) | 348.5 / 1268.7 | — |
| 156 | 15.589 / 15.504 / 15.357 / 37.656 / 15.444 / 15.472 | -0.35% (3/5) | +0.17% (2/5) | 355.0 / 1269.1 | — |

All 91 exports verified. [Samples](artifacts/samples.json.gz) · [Pins](artifacts/pins.json) · [Verification](artifacts/verification.json)
