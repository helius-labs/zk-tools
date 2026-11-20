# Chunk-local bucket merges for four-range G2

Paired full-proof changes ranged from 0.26% faster to 0.22% slower across 1×8 and 1×16 on Ryzen. The larger 1×16 shape won at most half its blocks in any request and placement. The existing merge remains the baseline.

Each G2 window fills the same four point ranges as the existing C7a profile. Applying all three range merges to a chunk in one Rayon job replaces the barriers between range merges and reuses the pair buffer. The subsequent weighted reduction uses the existing arithmetic. The control and candidate share an executable and run with the same eleven-bit windows and four ranges.

| Shape | Request | Main + G2 | Ordinary / chunked proof ms | Paired change | Ordinary / chunked G2 window phase ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×8 | catalog | 8+8 | 12.019 / 11.966 | -0.19% (5/6) | 6.048 / 5.837 | 100.8 / 100.0 |
| 1×8 | catalog | 16+16 | 9.814 / 9.821 | +0.11% (3/6) | 4.969 / 5.266 | 124.0 / 124.2 |
| 1×8 | heldout | 8+8 | 11.992 / 11.983 | -0.08% (3/6) | 6.285 / 6.017 | 102.5 / 103.0 |
| 1×8 | heldout | 16+16 | 9.801 / 9.782 | -0.16% (5/6) | 5.295 / 5.391 | 116.8 / 125.2 |
| 1×16 | catalog | 8+8 | 15.149 / 15.176 | +0.22% (3/6) | 7.326 / 7.642 | 120.0 / 122.0 |
| 1×16 | catalog | 16+16 | 12.539 / 12.586 | -0.26% (3/6) | 6.298 / 6.026 | 135.8 / 151.6 |
| 1×16 | heldout | 8+8 | 15.216 / 15.148 | +0.12% (2/6) | 7.475 / 7.324 | 124.2 / 123.1 |
| 1×16 | heldout | 16+16 | 12.612 / 12.663 | +0.14% (3/6) | 6.697 / 6.287 | 151.5 / 149.9 |

All 112 process exports passed independent verification. The targeted curve tests passed, including a direct projective oracle for identity and opposite points at chunk boundaries and an uneven tail. Six blocks prime three requests and time thirteen. The two-request clock diagnostics run separately. The 8+8 case uses sixteen physical cores. The 16+16 case includes sibling threads. Four ranges are forced on both arms to exercise the server geometry on Ryzen.

[Summary](artifacts/summary.json) · [Samples and diagnostics](artifacts/rows.json.gz) · [Candidate patch](artifacts/source.patch.gz)
