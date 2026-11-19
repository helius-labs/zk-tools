# Preconverted public row offsets on 2x2

Five randomized blocks use 32 pinned Ryzen workers and measure 21 single verified SDK calls after three warmups in each process. The candidate uses the same executable for its ordinary and preconverted keys. The release and warm Aeglos prover provide independent controls. All exported proofs verify with the original key and expected public outputs. The input row dictionary reconstructs every original public reference pair. The derived key has the original private slots and the flat and blocked files match byte-for-byte before the dictionary.

| Request | release / ordinary / blocked / aeglos ms | blocked vs ordinary | blocked vs release | Max blocked / Aeglos MiB | blocked vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 9.594 / 9.601 / 10.316 / 20.812 | +7.37% (0/5) | +7.34% (1/5) | 122.8 / 740.9 | 5.72× / 2.91× |
| 41 | 9.655 / 9.570 / 10.357 / 20.955 | +8.96% (0/5) | +6.93% (0/5) | 122.0 / 735.0 | — |
| 48 | 9.609 / 9.614 / 10.361 / 20.896 | +7.77% (0/5) | +7.83% (0/5) | 122.8 / 734.0 | — |

All 61 exports verified. [Samples](artifacts/samples.json.gz) · [Pins](artifacts/pins.json) · [Verification](artifacts/verification.json)
