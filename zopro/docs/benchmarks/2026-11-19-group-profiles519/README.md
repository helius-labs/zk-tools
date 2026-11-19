# Warm hardware counters and cycle profiles for grouped proofs

Three shuffled blocks each use separate counter and sampling processes. The SDK enables Linux perf through an acknowledged FIFO after three warmups. The 1×2 production build uses 32 main workers and eight frontend workers with 61 timed calls. The experimental 12×2 build uses 30 main workers with 21 timed calls on the original and one scattered request. Counters cover the proof loop, SDK verification and serialization. Each process exports one independently verified proof.

G1 counts only symbols naming the streaming G1 bucket implementation. G2 counts only the borrowed and prefix bucket implementations. Grouping counts the explicit group evaluators and scalar grouping. FFT counts symbols naming the FFT and scheduler counts symbols naming Crossbeam or the Rayon core. Unclassified symbols and inlined instructions may include additional work in any category. Sample shares refer to CPU execution across workers.

| Shape | Request | Mode | Counter-run ms | Million cycles / instructions per proof | Million cache misses | Switches / minor faults | G1 / G2 / grouping / FFT / scheduler samples % |
|---|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | stable | 7.116 | 647.4 / 1213.8 | 2.43 | 556 / 42 | 23.0 / 21.9 / 0.3 / 3.0 / 11.5 |
| 1x2 | Original | frequent | 7.153 | 647.8 / 1212.8 | 2.43 | 557 / 43 | 22.7 / 22.3 / 0.4 / 2.7 / 11.5 |
| 12x2 | Original | off | 30.972 | 3748.2 / 4707.7 | 15.42 | 840 / 2707 | 18.3 / 6.1 / 1.6 / 28.2 / 9.8 |
| 12x2 | Original | broad | 29.970 | 3560.5 / 4434.5 | 13.82 | 882 / 1885 | 19.4 / 6.5 / 2.6 / 29.7 / 10.2 |
| 12x2 | Original | nested | 29.635 | 3539.3 / 4441.8 | 13.82 | 884 / 2136 | 19.7 / 6.3 / 2.7 / 29.6 / 9.8 |
| 12x2 | 141 | off | 46.604 | 5422.9 / 7112.6 | 20.19 | 1301 / 6420 | 18.0 / 13.3 / 1.9 / 18.9 / 8.2 |
| 12x2 | 141 | broad | 46.825 | 5432.2 / 7114.2 | 20.09 | 1304 / 6686 | 18.5 / 13.1 / 2.0 / 18.8 / 8.2 |
| 12x2 | 141 | nested | 45.476 | 5342.5 / 7369.0 | 19.23 | 1258 / 6562 | 18.8 / 15.1 / 1.7 / 19.4 / 8.1 |

[All counters and symbol counts](artifacts/processes.json.gz) · [Leading symbols](artifacts/summary.json) · [Proof verification](artifacts/verification.json)
