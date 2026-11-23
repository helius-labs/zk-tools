# Zopro and Mopro on midrange and high-end phones

**Interim results.** Zopro CPU completed all 41 shapes on both Android phones. Mopro completed 22 shapes on each Android phone, 11 on the iPhone 15 and 21 on the iPhone 17 Pro. The full iOS Zopro catalogs and the remaining Mopro batches are pending. The AWS server catalog covers all 41 shapes. AWS Device Farm recorded 3,984 verified proofs in 498 process blocks, covering 166 of 410 shape, device, and backend combinations.

| Device | Tier | OS |
|---|---|---|
| iPhone 15 | Mid | 18.5 |
| iPhone 17 Pro | High | 26.3.1 |
| Galaxy A56 | Mid | 15 |
| Galaxy S25 | High | 15 |

Zopro used PR #15 head `64d1274`. Mopro used the native Gnark adapter at `c1071f9`, linked to Gnark v0.15.0. Both ran the same pinned key and witness for each shape. Mopro reopens the proving files on every call and reopens the constraint system and verifying key for verification. Warm Mopro latency includes those reads. The CSV reports Mopro setup separately from the time Zopro takes to load its prepared key. The adapter reported a Gnark v0.16.3 constraint file in the 2×2 smoke test. Every recorded Mopro proof passed verification with its original verifying key. [Mopro native adapter](https://github.com/sergeytimoshin/mopro/commit/c1071f96fa5a28dca8e567dd3944331047450a2b)

The server columns use the published c7a.24xlarge catalog request benchmarks with matching key hashes. Server latency includes request-level witness construction. The phone runs start with the pinned witness fixture and check the resulting public inputs and proof.

Warm latency is the median of three process means. Each process primes three proofs and times five. First response includes setup and the first verified proof. Memory is the largest peak over the three processes. Process RSS and Apple physical footprint are separate counters.

[Every shape with warm time, first response, and memory](phone-aws-comparison.csv) · [Verified raw blocks](artifacts/phone-comparison-blocks.jsonl) · [AWS measurement details](README.md#measurement-details)

Across the 22 shapes with results for both phone provers on each Android device, Zopro CPU was **5.21× faster** at the median on the Galaxy A56 and **4.62× faster** on the Galaxy S25. On merge-54×1, Zopro took **9.17 s** with **1,162 MiB** peak RSS on the A56 and **6.10 s** with **1,165 MiB** on the S25. Mopro took **50.17 s** with **2,967 MiB** on the A56 and **25.29 s** with **2,955 MiB** on the S25. On the iPhone 17 Pro, Zopro Metal took **559.8 ms** with **1,755 MiB** peak RSS and Mopro took **23.20 s** with **2,782 MiB**. Where a shape ran in both a smoke test and the full catalog, the table uses the full-catalog result.

## Representative shape results

| Shape | Phone | Zopro CPU ms / RSS MiB | Zopro Metal ms / RSS MiB | Mopro ms / RSS MiB | AWS Zopro ms / RSS MiB | AWS Aeglos ms / RSS MiB | AWS Gnark ms / RSS MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | iPhone 15 | 158.8 / 108.9 | 111.3 / 163.9 | 738.8 / 173.8 | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 1×2 | iPhone 17 Pro | 106.2 / 110.9 | 45.2 / 149.8 | 579.5 / 175.3 | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 1×2 | Galaxy A56 | 308.8 / 108.2 | — | 1552.2 / 111.0 | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 1×2 | Galaxy S25 | 134.7 / 109.6 | — | 634.1 / 107.3 | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 2×2 | iPhone 15 | — | — | 1316.1 / 246.0 | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 2×2 | iPhone 17 Pro | — | — | 986.3 / 231.6 | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 2×2 | Galaxy A56 | 507.6 / 109.5 | — | 2520.4 / 212.2 | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 2×2 | Galaxy S25 | 240.3 / 114.0 | — | 1201.3 / 174.9 | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 8×2 | iPhone 15 | — | — | — | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 8×2 | iPhone 17 Pro | — | — | — | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 8×2 | Galaxy A56 | 1505.3 / 179.2 | — | 7929.1 / 604.0 | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 8×2 | Galaxy S25 | 868.6 / 183.5 | — | 5410.5 / 624.0 | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 24×2 | iPhone 15 | — | — | — | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 24×2 | iPhone 17 Pro | — | — | — | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 24×2 | Galaxy A56 | 4561.3 / 564.0 | — | 25800.9 / 1573.5 | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 24×2 | Galaxy S25 | 2883.8 / 568.6 | — | 16930.8 / 1477.3 | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 49×2 | iPhone 15 | — | — | — | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| 49×2 | iPhone 17 Pro | — | — | — | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| 49×2 | Galaxy A56 | 8871.0 / 1125.4 | — | 48517.1 / 3206.0 | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| 49×2 | Galaxy S25 | 6434.9 / 1129.3 | — | 34127.6 / 3027.9 | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| merge-54×1 | iPhone 15 | 6134.6 / 1234.3 | 4603.5 / 2241.5 | — | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |
| merge-54×1 | iPhone 17 Pro | 2150.2 / 1245.2 | 559.8 / 1754.6 | 23199.4 / 2781.9 | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |
| merge-54×1 | Galaxy A56 | 9165.3 / 1162.1 | — | 50173.7 / 2967.1 | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |
| merge-54×1 | Galaxy S25 | 6101.0 / 1164.9 | — | 25291.4 / 2955.0 | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |

A dash denotes an unavailable backend or fewer than three verified process blocks.
