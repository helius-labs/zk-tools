# How much more Euler work could choosing a reference remove?

A fixed public reference pair for each R1CS row allows the prover to move a linear correction into the key. A residual pair of zero field elements guarantees the corresponding H scalar vanishes. Could choosing a different reference per request yield enough additional zero rows to justify a second preconverted key? Each shape supplies four archived, distinct public witnesses. Each serves once as the input while the three others supply references. Input digests, proof identities, ordinary support counts and majority-reference support counts match the previous independent reader.

In each fold the six choices are a row-wise majority with zero fallback, each of the three other complete input vectors, zero, and a row-wise majority with the first training pair as fallback. The sampled selector chooses one whole vector by comparing the current pairs at 64 equally spaced rows. The whole-request oracle knows all rows before choosing. The block and row oracles can switch between the same six choices every 256 rows or every row, respectively. They are upper bounds within this family and include no storage or correction cost.

The entries are the median percentage of the baseline's still-live rows additionally removed, over four folds. A positive entry means more zero pairs. A nonzero residual pair can still produce a zero H scalar. These counts measure neither curve operations nor wall time.

| Shape | Fixed majority, first fallback | Sampled choice | Best whole reference | Best per block | Best per row |
|---|---:|---:|---:|---:|---:|
| 1x2 | +0.000% | -0.070% | +0.075% | +0.168% | +1.724% |
| 2x2 | +0.001% | -0.112% | +0.035% | +0.208% | +1.818% |
| 4x2 | -0.000% | -0.009% | +0.057% | +0.278% | +2.062% |
| 8x2 | -0.000% | +0.000% | +0.047% | +0.227% | +2.048% |
| 12x2 | +0.002% | +0.057% | +0.057% | +0.295% | +2.154% |
| 16x2 | +0.002% | +0.012% | +0.018% | +0.212% | +2.027% |
| 24x2 | +0.002% | +0.045% | +0.102% | +0.309% | +2.165% |
| merge-24x1 | +4.407% | +4.246% | +9.989% | +10.808% | +20.241% |

[All counts and digests](artifacts/counts.json) · [Streaming Rust reader](artifacts/choice.rs) · [Prior row-support oracle](../2026-11-19-residual-support581/README.md)
