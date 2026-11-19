# Hash-based grouping on the centered 2×2 H query

The canonical-wire CPU path can group equal H coefficients before its curve query. The raw setting streams those same canonical H coefficients directly to the ordinary MSM. All other curve queries keep their configured grouping and each process uses the same 32 main workers and 12 frontend workers as its matched control. Nine untimed diagnostic processes count nonzero H coefficients and the number of groups admitted. A missing group count means the standard grouping code declined. The flat and blocked encodings must produce identical observations. Five shuffled timing blocks use three warmups and 31 complete proofs per process. Only the diagnostic arm counts scalar values. Two frozen SDK controls measure code-layout changes independently.

| Request | Nonzero ordinary / centered H | Groups ordinary / centered | Frozen ordinary / ordinary / ordinary raw / frozen block / block / block raw / flat raw ms | Block raw vs block / ordinary | Block raw vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 53,150 / 46,345 | None / 46345 | 9.737 / 9.587 / 9.626 / 10.426 / 10.605 / 9.522 / 9.530 | -10.74% (5/5) / -1.23% (5/5) | 6.20× / 3.15× |
| 41 | 53,162 / 46,368 | None / 46368 | 9.618 / 9.690 / 9.672 / 10.359 / 10.641 / 9.513 / 9.508 | -10.96% (5/5) / -2.03% (5/5) | — |
| 48 | 53,160 / 46,373 | None / 46373 | 9.623 / 9.686 / 9.712 / 10.466 / 10.656 / 9.549 / 9.603 | -12.17% (5/5) / -1.41% (5/5) | — |

All 114 exported proofs verified with the original key. [Samples and routes](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Binary and key identities](artifacts/pins.json) · [Verification](artifacts/verification.json)
