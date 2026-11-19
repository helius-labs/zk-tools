# Why similar transfer key sizes can produce very different curve workloads

On the two original catalog requests, 5×16 supplied **51,707 distinct nonzero G2 scalars** to the grouped multiplication, while 6×2 supplied **35,323**. That is 46.4% more distinct scalars for 4.7% more B2 points. The two grouped G1 queries grew by 46.5% and 46.7% in distinct scalars. Both shapes used the same grouped entry points. This identifies substantially different remaining curve work despite their common FFT size. The archived 96-core whole-proof times differed by 3.299 ms for Zopro and 0.342 ms for Aeglos. Local cardinalities do not allocate that time difference to a specific phase. [Matched AWS comparison](../2026-11-19-c7a-crossover601/README.md)

Two independent observer runs replayed the exact archived SDK, keys and original public fixtures on 32 Ryzen workers. Every process warmed three times and produced three measured proofs. The first observer counted function entries. The second also recorded slice lengths, with identical entry counts. Disabling proof-local scalar deduplication removed both grouped G1 calls and the grouped G2 call on each shape. Lower-level point-combination calls remained in that control. The table shows ordinary deduplication, with G1 queries ordered by original base count. Counts repeated exactly on all three requests per process.

| Transfer | Curve | Original bases | Nonzero inputs | Distinct nonzero scalars | Distinct / original |
|---|---|---:|---:|---:|---:|
| 5x16 | G1 | 101,510 | 93,170 | 34,543 | 34.03% |
| 5x16 | G1 | 148,727 | 138,806 | 51,983 | 34.95% |
| 5x16 | G2 | 145,951 | 137,425 | 51,707 | 35.43% |
| 6x2 | G1 | 98,091 | 89,640 | 23,576 | 24.03% |
| 6x2 | G1 | 141,711 | 132,780 | 35,425 | 25.00% |
| 6x2 | G2 | 139,420 | 131,943 | 35,323 | 25.34% |
| 4x2 | G1 | 67,154 | 61,387 | 20,386 | 30.36% |
| 4x2 | G1 | 97,075 | 90,985 | 30,631 | 31.55% |
| 4x2 | G2 | 95,537 | 90,417 | 30,544 | 31.97% |
| 8x2 | G1 | 129,064 | 117,866 | 25,236 | 19.55% |
| 8x2 | G1 | 186,411 | 174,578 | 37,973 | 20.37% |
| 8x2 | G2 | 183,347 | 173,466 | 37,850 | 20.64% |

The pinned G1 and G2 entry instructions compare the base slice length against the grouping record’s original length and pass slice lengths to point combination and the downstream raw MSM. The observer reads the lengths only. It checks that the number of distinct values equals the number of classes, the starts slice has one extra sentinel, nonzero inputs fit in the original bases and each G2 base count equals the original key header. This is one public input per shape. The GPU, other witnesses and current AWS candidate were outside this observation. Concurrent offline conversion makes these captures unsuitable for latency comparisons. All 16 exports verified twice against the original verification keys.

[Reader](artifacts/routes.py) · [Symbols](artifacts/pins.json) · [Instructions](artifacts/grouped-entry.asm) · [Dimensions](artifacts/observations.json) · [Independent entry counts](artifacts/entry-counts.json) · [Verification](artifacts/verification.json)
