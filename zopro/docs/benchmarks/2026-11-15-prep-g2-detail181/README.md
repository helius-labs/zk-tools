# Preparation and G2 completion diagnostic

The local 74-slot observer passes all 16 calibration/backing checks. All 56
exported proofs independently verify. This qualifies the observer for the next
C7a measurement; it does not establish the cause of the C7a scaling loss or a
production improvement. Arithmetic, work geometry and joins remain unchanged.

Ryzen 9950X, primary 1x2, 16/32 fixed workers, nine rotated/reversed blocks,
three independent byte-identical executable backings per binary, 31 timed
requests after three primes. REF is the exact accepted SDK; OFF/ON share the new
SDK. All four paired interval gates and 12 backing guards satisfy the declared
[0.95,1.05] range. Calibration bounds total overhead, not individual-stage bias.

| Local observation | 16 workers | 32 workers |
|---|---:|---:|
| Accepted full request, median ms | 8.811371 | 7.657485 |
| ON full request, median ms | 8.845716 | 7.699636 |
| Preparation, mean ms | 0.125768 | 0.199735 |
| W canonicalization, mean ms | 0.039955 | 0.062669 |
| A/B gather span, mean ms | 0.083898 | 0.134799 |
| G2 scalar-group admission, mean ms | 0.780807 | 1.321011 |
| G2 splitting, mean ms | 0.214163 | 0.581551 |
| G2 window collection, mean ms | 5.416826 | 3.072522 |
| Collection return after last window, mean ms | 0.008579 | 0.007303 |

Preparation's reclamation and C drop are each below 0.1 microseconds here. G2
point grouping is absent in these traces: group admission returns no groups.
Its recorded cost includes that decision's work, including any rejected full
construction; it is not successful point reduction. The admission source is
serial; parallel splitting/window spans may include helping and waiting.

The observer records 11 windows, two parts per window, merge/weight intervals
and final combination, plus six preparation boundaries. It uses a fixed request
record and bounded stack formatting, without per-leaf traces or global/TLS
attribution. All nested intervals pass checks. A new concurrent native test
matches traced G2 results against an independent group oracle; all 135 core tests
pass. Complete SDK requests include trace formatting and internal verification.

There are 1,844 SDK calls and 56 new exports. Canonical ledger advances from
55,664 to 55,720, SHA256
`810baa30d05dcad89de5c6af5ceda9ce7ca9090807e25103b3f62240389891e5`.
Local evidence root: `/home/ubuntu/src/zopro-perf-work/prep-g2-detail181`.
No AWS measurement for this observer yet. Next run the frozen 32/96-core target
matrix, retain every control and then clean owned resources. No production
change or adoption. The broader goal remains unmet. No GitHub push authorized.

[Local measurements](artifacts/local-review.json), [protocol](artifacts/protocol.md).
