# Repeated blocks of G2 scalar digits

Two previously verified B-query fixtures provide the scalar inputs. The ordinary width-12 decomposition has eleven signed windows per component. This model asks how often several adjacent nonzero digits repeat together, including patterns that differ only by overall sign. Combining k points with an identical d-nonzero-digit pattern costs k−1 additions and removes d(k−1) nominal bucket additions. The calculation explores all disjoint contiguous partitions of the eleven windows and reports the most favorable partition on each fixture.

| Shape | Effective rows | Component | Nominal bucket additions | Whole component saved | Best partition saved | With ≤16 patterns per block |
|---|---:|---|---:|---:|---:|---:|
| 1x2 | 30089 | 1 | 286,665 | 17,682 | 17,682 (6.17%) | 825 (0.29%) |
| 1x2 | 30089 | 2 | 286,154 | 17,760 | 17,760 (6.21%) | 910 (0.32%) |
| 2x2 | 28852 | 1 | 296,330 | 50 | 1,976 (0.67%) | 98 (0.03%) |
| 2x2 | 28852 | 2 | 296,293 | 60 | 1,963 (0.66%) | 98 (0.03%) |

Across both 1x2 components the best partitions save 35,442 of 572,819 nominal within-component bucket additions (6.19%).

Across both 2x2 components the best partitions save 3,939 of 592,623 nominal within-component bucket additions (0.66%).

The model pools all rows within each component, gives the algorithm the best partition and omits grouping, storage and scheduling costs. Actual code uses two point ranges, shares buckets across components, and handles the leading window separately. These counts do not predict wall time. The fixtures each represent one solved request. The Rust tool checks that all signed digits reconstruct their original component and checks the saving formula on a synthetic signed example. Existing point and scalar data stay in the local benchmark workspace.

[Rust model](artifacts/count.rs) · [Protocol](artifacts/protocol.md) · [Counts](artifacts/counts.json) · [Fixture digests](artifacts/inputs.json)
