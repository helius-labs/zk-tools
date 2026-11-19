# Feeding the medium-sized H query directly to the raw MSM

The general prover normally checks whether H scalars can be grouped and canonicalizes them on the ordinary path. The candidate compares two direct representations on 4×2 and 8×2. One converts to canonical words in place. The other uses Montgomery words and corrects their common field factor on the resulting curve point. Both reuse the existing raw MSM. Four screening blocks selected a representation per shape; nine fresh blocks timed 21 verified requests per process.

| Stage | Shape | H scalars | Representation | Median ms | CPU-ms | Peak MiB | M5 CPU / Metal speedup |
|---|---|---:|---|---:|---:|---:|---:|
| screen | 4x2 | 98,674 | canonical | 15.293 | 402.2 | 293.2 | 7.13× / 3.14× |
| screen | 4x2 | 98,674 | montgomery | 15.200 | 401.3 | 278.8 | 7.17× / 3.16× |
| screen | 4x2 | 98,674 | off | 15.186 | 400.6 | 282.2 | 7.18× / 3.16× |
| confirm | 4x2 | 98,674 | canonical | 15.237 | 402.3 | 288.0 | 7.15× / 3.15× |
| confirm | 4x2 | 98,674 | montgomery | 15.216 | 400.8 | 290.2 | 7.16× / 3.15× |
| confirm | 4x2 | 98,674 | off | 15.246 | 402.6 | 284.5 | 7.15× / 3.15× |
| confirm | 4x2 | 98,674 | release | 15.483 | 406.6 | 292.5 | 7.04× / 3.10× |
| screen | 8x2 | 189,618 | canonical | 24.353 | 659.5 | 497.2 | 7.76× / 3.04× |
| screen | 8x2 | 189,618 | montgomery | 23.453 | 648.9 | 474.5 | 8.06× / 3.16× |
| screen | 8x2 | 189,618 | off | 23.684 | 654.0 | 481.0 | 7.98× / 3.12× |
| confirm | 8x2 | 189,618 | canonical | 23.943 | 657.1 | 477.2 | 7.89× / 3.09× |
| confirm | 8x2 | 189,618 | montgomery | 23.698 | 652.1 | 490.8 | 7.98× / 3.12× |
| confirm | 8x2 | 189,618 | off | 23.643 | 654.0 | 486.2 | 7.99× / 3.13× |
| confirm | 8x2 | 189,618 | release | 23.825 | 657.2 | 476.2 | 7.93× / 3.11× |

On 4x2 the selected montgomery path changed paired latency -2.32% against release (7/9 wins) and -0.38% against the identical disabled binary (5/9).

On 8x2 the selected montgomery path changed paired latency -0.56% against release (6/9 wins) and -0.09% against the identical disabled binary (5/9).
Both shapes won five of nine pairs against the disabled control. Neither met the six-win criterion, so the existing H dispatch stays in the integrated prover.

[Protocol](artifacts/protocol.md) · [Source patch](artifacts/source.patch) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
