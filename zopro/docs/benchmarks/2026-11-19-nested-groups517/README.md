# Two levels of prepared B2 groups on 12×2

A broad partition trained on eight adjacent public requests is refined with four scattered training requests. Within each parent group, rows with identical residual scalars on all four scattered requests form a child group when at least three positions agree. Each child is replaced by its point sum. Parent groups are projected onto the new child-and-singleton basis. Group representatives use the same exact difference and voting rules as the adaptive prover. In this arithmetic model each level requires at most one exception per sixteen grouped rows. Direct fallback preserves the input to that level. The model excludes the preliminary 32-group sample used by the SDK. Distinct coefficients are counted across both MSMs and corrections pooled, which may overstate opportunities for runtime deduplication.

All four scattered evaluation requests and three adjacent requests were held out from both levels of training. The original is also reported. Each capture was allowed only for these public fixtures, and its proof verified independently. The model checked exact per-row reconstruction modulo the scalar field at both levels. Term counts exclude zeros. They do not measure proof time or RSS.

| Request | Nonzero terms dense / broad / fine / nested | Distinct nonzero in same order | Exceptions broad / fine / upper |
|---|---:|---:|---:|
| Original | 235609 / 46857 / 144880 / 46567 | 41425 / 41425 / 41425 / 41426 | 4077 / 1873 / 1899 |
| 57 | 236094 / 46876 / 145083 / 46531 | 41408 / 41409 / 41409 / 41410 | 4058 / 1852 / 1855 |
| 59 | 236176 / 46815 / 145175 / 46542 | 41414 / 41415 / 41415 / 41415 | 4002 / 1900 / 1817 |
| 64 | 235853 / 46975 / 145035 / 46643 | 41396 / 41396 / 41396 / 41396 | 4185 / 1923 / 1921 |
| 141 | 240069 / 240069 / 149431 / 149431 | 126688 / 126688 / 126931 / 126931 | 103064 / 4588 / 97744 |
| 142 | 240121 / 240121 / 149222 / 149222 | 125721 / 125721 / 125964 / 125964 | 98567 / 4318 / 93375 |
| 143 | 240055 / 240055 / 149406 / 149406 | 126413 / 126413 / 126656 / 126656 | 102287 / 4632 / 96963 |
| 144 | 240030 / 240030 / 148703 / 148703 | 126701 / 126701 / 126704 / 126704 | 100625 / 3835 / 96084 |

[Geometry](artifacts/geometry.json) · [Counts](artifacts/counts.json) · [Proof identities](artifacts/capture.json)
