# Exact-binary FFT and G2 overlap on C7a

Three traced and three paired plain processes per width and setting each time 51 full SDK calls. Probes bracket the product FFT, exclusive G2 query, canonical core and SDK entry. Overlap measures the intersection of FFT and G2 elapsed intervals within each request. The observed control baseline and probe overhead are reported for each cell.

| Physical cores | Mode | Trace vs plain | Traced SDK ms | FFT ms | G2 ms | FFT / G2 overlap ms | Requests overlapping | After both until SDK return ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 64 | wait | +2.68% | 8.406 | 0.895 | 4.366 | 0.000 | 0/153 | 0.890 |
| 64 | g2 | +0.31% | 11.872 | 4.573 | 4.577 | 4.373 | 153/153 | 4.700 |
| 96 | wait | +2.00% | 8.395 | 0.861 | 4.110 | 0.000 | 0/153 | 1.110 |
| 96 | g2a | +2.11% | 11.599 | 3.502 | 4.239 | 3.502 | 153/153 | 4.909 |

[All traced intervals and plain controls](artifacts/stages.json.gz)
