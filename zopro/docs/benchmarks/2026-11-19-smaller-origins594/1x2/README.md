# Preconverted public row offsets on 1x2

Five randomized blocks use 32 pinned Ryzen workers and measure 21 single verified SDK calls after three warmups in each process. The candidate uses the same executable for its ordinary and preconverted keys. The release and warm Aeglos prover provide independent controls. All exported proofs verify with the original key and expected public outputs. The input row dictionary reconstructs every original public reference pair. The derived key has the original private slots and the flat and blocked files match byte-for-byte before the dictionary.

| Request | release / ordinary / blocked / aeglos ms | blocked vs ordinary | blocked vs release | Max blocked / Aeglos MiB | blocked vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 7.097 / 7.081 / 7.018 / 12.912 | -0.13% (4/5) | -0.78% (5/5) | 63.2 / 418.4 | 5.27× / 3.42× |
| 29 | 6.852 / 6.940 / 6.834 / 12.931 | -1.90% (5/5) | -1.13% (4/5) | 64.2 / 423.0 | — |
| 32 | 7.034 / 7.021 / 6.897 / 12.968 | -2.31% (5/5) | -1.43% (4/5) | 65.5 / 418.3 | — |

All 61 exports verified. [Samples](artifacts/samples.json.gz) · [Pins](artifacts/pins.json) · [Verification](artifacts/verification.json)
