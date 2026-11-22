# Zopro and Mopro on midrange and high-end phones

**Interim results.** The AWS server catalog covers all 41 shapes. The full phone catalog is in progress. AWS Device Farm recorded 36 verified process blocks and 12/410 complete shape, device, and backend combinations.

| Device | Tier | OS |
|---|---|---|
| iPhone 15 | Mid | — |
| iPhone 17 Pro | High | 26.3.1 |
| Galaxy A56 | Mid | 15 |
| Galaxy S25 | High | 15 |

Zopro used PR #15 head `64d1274`. Mopro used the native Gnark adapter at `c1071f9`, linked to Gnark v0.15.0. Both ran the same pinned key and witness for each shape. Mopro reopens the proving files on every call and reopens the constraint system and verifying key for verification. Warm Mopro latency includes those reads. The CSV reports Mopro setup separately from the time Zopro takes to load its prepared key. The adapter reported a Gnark v0.16.3 constraint file in the 2×2 smoke test. Every recorded Mopro proof passed verification with its original verifying key. [Mopro native adapter](https://github.com/sergeytimoshin/mopro/commit/c1071f96fa5a28dca8e567dd3944331047450a2b)

The server columns use the published c7a.24xlarge catalog request benchmarks with matching key hashes. Server latency includes request-level witness construction. The phone runs start with the pinned witness fixture and check the resulting public inputs and proof.

Warm latency is the median of three process means. Each process primes three proofs and times five. First response includes setup and the first verified proof. Memory is the largest peak over the three processes. Process RSS and Apple physical footprint are separate counters.

[Every shape with warm time, first response, and memory](phone-aws-comparison.csv) · [Verified raw blocks](artifacts/phone-comparison-blocks.jsonl) · [AWS measurement details](README.md#measurement-details)

On 1×2, Zopro CPU took 307 ms on the Galaxy A56 and 134 ms on the Galaxy S25. Mopro took 1,552 ms and 634 ms, respectively. iPhone 17 Pro Metal took 45 ms on 1×2 and 560 ms on merge-54×1. Its merge peak RSS was 1,755 MiB. AWS Gnark took 1,204 ms and peaked at 4,907 MiB on the merge, while AWS Zopro took 105 ms and Aeglos took 175 ms.

## Representative shape results

| Shape | Phone | Zopro CPU ms / RSS MiB | Zopro Metal ms / RSS MiB | Mopro ms / RSS MiB | AWS Zopro ms / RSS MiB | AWS Aeglos ms / RSS MiB | AWS Gnark ms / RSS MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | iPhone 15 | — | — | — | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 1×2 | iPhone 17 Pro | 106.2 / 110.9 | 45.2 / 149.8 | — | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 1×2 | Galaxy A56 | 307.1 / 107.6 | — | 1552.2 / 111.0 | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 1×2 | Galaxy S25 | 134.2 / 109.0 | — | 634.1 / 107.3 | 7.8 / 40.8 | 8.3 / 452.9 | 49.8 / 264.9 |
| 2×2 | iPhone 15 | — | — | — | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 2×2 | iPhone 17 Pro | — | — | — | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 2×2 | Galaxy A56 | — | — | 2520.4 / 212.2 | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 2×2 | Galaxy S25 | — | — | 1201.3 / 174.9 | 10.3 / 60.9 | 12.2 / 803.8 | 83.1 / 433.1 |
| 8×2 | iPhone 15 | — | — | — | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 8×2 | iPhone 17 Pro | — | — | — | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 8×2 | Galaxy A56 | — | — | — | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 8×2 | Galaxy S25 | — | — | — | 18.7 / 304.2 | 31.1 / 2418.9 | 253.2 / 903.5 |
| 24×2 | iPhone 15 | — | — | — | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 24×2 | iPhone 17 Pro | — | — | — | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 24×2 | Galaxy A56 | — | — | — | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 24×2 | Galaxy S25 | — | — | — | 47.5 / 882.0 | 87.4 / 6664.4 | 660.1 / 2518.1 |
| 49×2 | iPhone 15 | — | — | — | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| 49×2 | iPhone 17 Pro | — | — | — | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| 49×2 | Galaxy A56 | — | — | — | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| 49×2 | Galaxy S25 | — | — | — | 108.3 / 1552.5 | 170.0 / 12410.4 | 1100.7 / 4615.1 |
| merge-54×1 | iPhone 15 | — | — | — | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |
| merge-54×1 | iPhone 17 Pro | 2150.2 / 1245.2 | 559.8 / 1754.6 | — | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |
| merge-54×1 | Galaxy A56 | 9290.0 / 1162.2 | — | — | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |
| merge-54×1 | Galaxy S25 | 4412.3 / 1165.3 | — | — | 105.2 / 1725.7 | 174.6 / 12878.1 | 1203.7 / 4907.1 |

A dash denotes an unavailable backend or fewer than three verified process blocks.
