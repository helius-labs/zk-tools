# Matching nonadjacent G2 windows for shared point sums

For two signed digits (a,b), group points whose digit pairs agree up to overall sign. Adding k signed points once and feeding that sum to both windows saves k−1 bucket additions. A bitmask dynamic program chooses the largest total saving over disjoint pairs of windows. It can leave windows unpaired. Each of the two GLV components selects its own matching. The experiment also varies the signed radix width and the largest group of consecutive input points over which matches can be found.

The first, middle and last available fixture by ID were chosen from each of two public B-query panels. Exact and opposite scalars are grouped before the scan, preserving first occurrence of each scalar class. The program checks signed-digit reconstruction on every input. For each selected pair, it independently accumulates the original and transformed buckets using model point weights modulo 2⁶¹−1 and checks equality at every bucket. The six archived Groth16 exports also passed the independent verifier again. The model transformation has no native prover implementation or latency measurement here.

The ordinary count includes arrivals less one seed per nonempty bucket, plus additions in a descending weighted scan that visits each bucket from its highest occupied index. Each component has its own bucket set. The model excludes doubling, endomorphisms, digit extraction, hash lookups, synchronization, separate point partitions and the cost of selecting a matching. Each percentage uses the ordinary width-12 cost on that same witness. The width with the fewest modeled additions is selected separately for each witness and tile capacity, giving the method hindsight.

| Shape | Fixture | Scalar classes | Ordinary width-12 adds | Best width / saving at 256 points | At 4,096 points | At whole query |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 0 | 26,139 | 615,344 | 12 / 0.01% | 12 / 0.09% | 12 / 0.55% |
| 1x2 | 4 | 26,789 | 629,401 | 12 / 0.01% | 12 / 0.09% | 12 / 0.56% |
| 1x2 | 8 | 26,788 | 629,412 | 12 / 0.01% | 12 / 0.09% | 12 / 0.56% |
| 2x2 | 33 | 29,199 | 682,324 | 12 / 0.01% | 12 / 0.10% | 12 / 0.61% |
| 2x2 | 37 | 29,201 | 682,375 | 12 / 0.01% | 12 / 0.10% | 12 / 0.61% |
| 2x2 | 40 | 29,197 | 682,321 | 12 / 0.01% | 12 / 0.10% | 12 / 0.61% |

Across six witnesses, even whole-query grouping with hindsight saved only 0.55–0.61% of modeled additions. Limiting the grouping to 4,096 points saved 0.09–0.10%. Width 12 had the lowest count in every case. These results rule out a substantial improvement from this matching strategy on the measured inputs and leave the runtime unchanged.

To reproduce a row, run `python3 artifacts/components.py INPUT.scalars PARTS`, compile `artifacts/matching.rs` with `rustc --edition 2021 -O`, and pass the parts file to the executable. Each output row gives width, tile size, arrivals, seeds, reduction additions, saved additions, maximum distinct pair keys in an examined tile and the chosen pairs. The raw counts record the digest of each parts file.

[Signed GLV fixture reader](../2026-11-19-trained-digits541/artifacts/zp_glv513.py) · [Earlier contiguous blocks](../2026-11-19-g2-digit-blocks414/README.md) · [Matching and bucket checks](artifacts/matching.rs) · [Selected window pairs, counts and verified fixture hashes](artifacts/counts.json)
