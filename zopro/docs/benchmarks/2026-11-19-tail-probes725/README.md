# Separating generation and verification after the observed 1×2 curve calls

Four paired blocks per public input and scalar-grouping configuration compare the production SDK with a byte-identical traced copy on 32 Ryzen logical workers. The new sites identify return from the canonical solver, return from rerandomized proof generation, and entry and return of the verification final exponentiation. Each measured SDK request satisfied the ordering needed to partition the interval from the later of the canonical G1 and exclusive G2 returns through SDK return. Intervals include concurrent worker activity. Only the final exponentiation interval directly measures that arithmetic operation. Every recording reported zero lost records. The 48 exports independently verified. These local timings establish probe coverage for the corresponding AWS diagnostic.

| Input | Configuration | Plain ms | Trace effect | Observed tail ms | To canonical return / to generation return / to final exponentiation / final exponentiation / to SDK return ms |
|---|---|---:|---:|---:|---:|
| catalog | ordinary | 7.286 | +0.94% | 0.583 | 0.014 / 0.142 / 0.203 / 0.206 / 0.014 |
| catalog | direct | 7.788 | +1.31% | 2.395 | 1.861 / 0.135 / 0.187 / 0.200 / 0.013 |
| heldout | ordinary | 7.016 | +1.84% | 0.591 | 0.016 / 0.141 / 0.196 / 0.213 / 0.013 |
| heldout | direct | 7.554 | +2.94% | 2.176 | 1.643 / 0.135 / 0.188 / 0.201 / 0.012 |
| v27 | ordinary | 7.062 | +1.41% | 0.559 | 0.012 / 0.138 / 0.193 / 0.204 / 0.013 |
| v27 | direct | 7.600 | +1.34% | 2.020 | 1.482 / 0.134 / 0.189 / 0.200 / 0.013 |

[Sampled intervals](artifacts/rows.json.gz) · [Exact sites](artifacts/symbols.json) · [Previous C7a timeline](../2026-11-19-sdk-tail724/README.md)
