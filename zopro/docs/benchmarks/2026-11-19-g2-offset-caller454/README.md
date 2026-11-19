# Fixed public offsets in the actual 1×2 B2 query

Five randomized blocks tested each of four held-out scalar vectors at 16 and 32 Ryzen workers. A fixed offset vector derived from five public training inputs selected 1,168 positions. The offset MSM was prepared outside the timed loop in both arms. Each timed residual call includes subtracting the selected scalars, the original B2 kernel and adding the correction point. Both arms restore the input before starting each timer and check the exact affine result against the original query afterward. Three warmups precede 21 timed queries per process. These processes measure the B2 component, with workers sharing the process affinity mask.

| Workers | Input | Ordinary ms | Residual ms | Paired change | Wins / 5 |
|---:|---:|---:|---:|---:|---:|
| 16 | 5 | 4.454 | 3.059 | -5.50% | 5 |
| 16 | 6 | 3.109 | 3.001 | -3.29% | 3 |
| 16 | 7 | 3.118 | 3.097 | -2.64% | 3 |
| 16 | 8 | 3.137 | 3.587 | +13.52% | 2 |
| 32 | 5 | 2.675 | 2.577 | -3.44% | 5 |
| 32 | 6 | 2.647 | 2.576 | -2.56% | 5 |
| 32 | 7 | 2.660 | 2.556 | -4.10% | 5 |
| 32 | 8 | 2.674 | 2.567 | -3.85% | 5 |

Across the four held-out inputs, the median paired improvement was 3.64% at 32 workers (four witness wins) and 2.97% at 16 workers (three witness wins). The next check is a full SDK proof with a key-bound correction and the optional memory allowance enforced.

Every computed point matched the ordinary affine result, including warmups. The components are timed in isolation on public fixture scalars. Full-proof timing is required before adoption.

[Component caller](artifacts/caller.rs) · [Measurements](artifacts/processes.json.gz) · [Input and build identities](artifacts/identities.json) · [Offset selection](../2026-11-19-g2-public-offset453/README.md)
