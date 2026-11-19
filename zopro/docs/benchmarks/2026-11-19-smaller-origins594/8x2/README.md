# Preconverted public row offsets on 8x2

Five randomized blocks use 32 pinned Ryzen workers and measure 21 single verified SDK calls after three warmups in each process. The candidate uses the same executable for its ordinary and preconverted keys. The release and warm Aeglos prover provide independent controls. All exported proofs verify with the original key and expected public outputs. The input row dictionary reconstructs every original public reference pair. The derived key has the original private slots and the flat and blocked files match byte-for-byte before the dictionary.

| Request | release / ordinary / blocked / aeglos / grouped / both ms | both vs grouped | both vs release | Max both / Aeglos MiB | both vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 24.055 / 24.112 / 24.103 / 85.130 / 22.989 / 22.862 | -0.56% (4/5) | -3.90% (4/5) | 591.2 / 2297.4 | 8.27× / 3.24× |
| 153 | 24.280 / 24.577 / 24.305 / 85.869 / 23.406 / 23.149 | -0.38% (3/5) | -5.16% (4/5) | 566.0 / 2298.1 | — |
| 156 | 24.262 / 24.844 / 24.704 / 85.717 / 23.355 / 22.892 | +0.39% (2/5) | -5.29% (5/5) | 569.2 / 2336.1 | — |

All 91 exports verified. [Samples](artifacts/samples.json.gz) · [Pins](artifacts/pins.json) · [Verification](artifacts/verification.json)
