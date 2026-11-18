# Executed general solver operations on all eight shapes

The diagnostic counts calls that reached the general solver interpreter during one complete request after three primes. Specialized Poseidon tasks can execute outside this interpreter. The table gives the most frequent exact pattern among the operations observed. Counts describe frequency. They do not measure the CPU time of each pattern.

| Shape | General ops | Distinct patterns | Most frequent pattern | Count | Share |
|---|---:|---:|---|---:|---:|
| 1x2 | 16,445 | 35 | output / wire, wire, zero / 0 | 4,693 | 28.5% |
| 2x2 | 28,688 | 33 | output / wire, wire, zero / 0 | 8,222 | 28.7% |
| 4x2 | 52,265 | 35 | output / wire, wire, zero / 0 | 14,974 | 28.7% |
| 8x2 | 100,046 | 37 | output / wire, wire, zero / 0 | 28,667 | 28.7% |
| 12x2 | 148,066 | 37 | output / wire, wire, zero / 0 | 42,413 | 28.6% |
| 16x2 | 195,506 | 35 | output / wire, wire, zero / 0 | 55,939 | 28.6% |
| 24x2 | 292,732 | 35 | output / wire, wire, zero / 0 | 83,693 | 28.6% |
| merge-24x1 | 277,534 | 35 | output / wire, wire, zero / 0 | 80,249 | 28.9% |

Patterns show assignment target, the three expression kinds, and the number of sum entries. All eight exported proofs verified.

[All counts](artifacts/operations.json) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
