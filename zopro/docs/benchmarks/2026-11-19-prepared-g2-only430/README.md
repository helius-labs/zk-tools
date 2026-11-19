# Reusing prepared G2 point sums in the rerandomized prover

The candidate checks whether the ordinary G2-only B query can use the existing prepared B layout. When the layout is available it checks the live scalars and charges the temporary vectors to the optional preparation budget. The regular G2 MSM handles a declined group. No additional key tables are stored. The route probe used three warmups and one timed full proof at 32 Ryzen threads per shape.

| Shape | Accepted calls / 4 | G1 B omitted / prepared witness / prepared B layout | Layout rows | Layout groups | Scalar groups returned |
|---|---:|---|---:|---:|---:|
| 8x2 | 0 | true / false / false | — | — | — |
| 12x2 | 0 | true / false / false | — | — | — |
| 16x2 | 0 | true / false / false | — | — | — |
| 24x2 | 0 | true / false / false | — | — | — |
| merge-24x1 | 0 | true / false / false | — | — | — |

The zero-budget process on 8x2 produced a verified proof with zero accepted calls.

All five shapes used the ordinary fallback across all four calls. The proposed reuse produced no grouped calls to time under these profiles. No production change was selected.

[Protocol](artifacts/protocol.md) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Prototype patch](artifacts/source.patch)
