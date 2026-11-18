# Repeat of combined private and H queries on 16 physical cores

On an independent seven-block repeat at 16 physical cores, combining the original K and H streams took 11.871 ms versus 12.050 ms for separate queries. The median paired change was -1.70% with 7/7 faster blocks. The prespecified requirement was 2% in at least five blocks.

| Shape | Workers | Mode | Median ms | Peak RSS MiB |
|---|---:|---|---:|---:|
| 2x2 | 16 | retained | 12.101 | 110 |
| 2x2 | 16 | off | 12.050 | 110 |
| 2x2 | 16 | raw | 11.871 | 104 |

All 21 distinct exported proofs verified against their pinned public witnesses.

[Protocol](artifacts/protocol.md) · [Measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
