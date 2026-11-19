# Complete proofs with lossless public Euler origins and existing K slots

The fixed row reference is represented by a lossless dictionary of public fields, a dictionary of field pairs and an index per row. The encoder checks file bounds and every decoded pair was hashed independently against the uncompressed reference. The active prover holds a row index stream and a Montgomery pair dictionary. The shape-specific metadata records both disk bytes and retained payload. Conversion scratch is additional.

The fold variant propagates the key coefficient of every otherwise-unqueried prefix sum backwards through its defining linear expression. Original private wires and previously retained slots consume all the resulting coefficients. The extra variant keeps new slots in the K query. Both consume exactly the same public reference; each process prepares a single key for every request of its shape. The same input order and CPU placement run between two ordinary processes. A native oracle compares original and rewritten point commitments for empty, alternating and complete slot retention on fresh and reloaded plans. Whole SDK timings come from one process per variant and shape. Setup time and process peak RSS are included in the raw samples.

| Shape | Request | Ordinary / fold / extra / second ordinary ms | Extra private slots, fold / extra | Lossless file KiB | Fold vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2 | 32 | 6.954 / 6.821 / 7.456 / 6.973 | 0 / 9,842 | 110.7 | — |
| 1x2 | 29 | 6.897 / 6.730 / 7.441 / 6.911 | 0 / 9,842 | 110.7 | — |
| 1x2 | Original | 6.969 / 7.051 / 7.441 / 6.994 | 0 / 9,842 | 110.7 | 5.25× / 3.40× |
| 2x2 | 40 | 9.520 / 10.146 / 11.290 / 9.504 | 0 / 17,266 | 173.9 | — |
| 2x2 | 37 | 9.660 / 10.176 / 11.045 / 9.511 | 0 / 17,266 | 173.9 | — |
| 2x2 | Original | 9.512 / 10.179 / 11.057 / 9.497 | 0 / 17,266 | 173.9 | 5.80× / 2.95× |
| merge-24x1 | 122 | 67.268 / 67.848 / 69.759 / 68.697 | 0 / 170,797 | 1677.2 | — |
| merge-24x1 | 124 | 70.397 / 68.221 / 71.825 / 70.179 | 0 / 170,797 | 1677.2 | — |
| merge-24x1 | Original | 64.171 / 61.759 / 65.068 / 64.868 | 0 / 170,797 | 1677.2 | 8.21× / 2.83× |
| merge-24x1 | 112 | 68.893 / 66.933 / 70.852 / 68.971 | 0 / 170,797 | 1677.2 | — |
| merge-24x1 | 109 | 67.396 / 63.235 / 67.648 / 67.721 | 0 / 170,797 | 1677.2 | — |

All 44 exports verified with the original verification key. [Samples](artifacts/samples.json) · [Origin sizes](artifacts/metadata.json) · [Changes](artifacts/source.patch) · [Verification](artifacts/verification.json)
