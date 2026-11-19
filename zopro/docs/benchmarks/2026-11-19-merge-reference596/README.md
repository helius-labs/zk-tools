# Selecting a public merge reference on independent inputs

Four existing public merge requests fix six references. The modal reference requires three equal pairs and otherwise uses zero. Plurality takes the most frequent pair at each row, giving the earliest training request priority on ties. The four remaining choices each use one complete training request. Each of twelve different public inputs supplies its current row pairs for this diagnostic. A selector samples 64 equally spaced pairs to choose one whole reference. A separate oracle reads all current pairs and maximizes exact matches. Both break ties in the order modal, plurality, 105, 106, 107, 108. Neither uses measured H counts to make its choice.

The native prover computes both the ordinary and residual Euler transforms. The table gives exact nonzero residual H scalar counts. Each number comes from a complete run on the original key, returning the ordinary product for the independently verified proof. The figures exclude key conversion, switching, memory use, the cost of subtracting references and downstream curve time. The separate fixed plurality policy requires just one converted key. The five shared modal cases reproduce the earlier native counts.

| Request | Modal | Plurality | Fixed 105 | Sampled choice / count | Exact-pair oracle / count |
|---|---:|---:|---:|---:|---:|
| Original | 254,483 | 240,600 | 240,646 | plurality / 240,600 | 108 / 240,375 |
| 109 | 150,605 | 146,458 | 146,431 | 106 / 136,680 | 106 / 136,680 |
| 110 | 184,990 | 180,938 | 181,194 | plurality / 180,938 | 107 / 171,065 |
| 111 | 219,857 | 205,959 | 206,015 | 106 / 215,834 | 108 / 205,327 |
| 112 | 254,450 | 250,270 | 250,433 | 106 / 240,375 | 106 / 240,375 |
| 121 | 322,338 | 300,997 | 300,997 | plurality / 300,997 | plurality / 300,997 |
| 122 | 322,773 | 309,990 | 310,098 | plurality / 309,990 | plurality / 309,990 |
| 123 | 323,014 | 318,868 | 318,856 | 107 / 318,942 | 105 / 318,856 |
| 124 | 357,526 | 353,437 | 353,492 | 108 / 353,333 | 108 / 353,333 |
| 129 | 323,106 | 301,853 | 301,839 | plurality / 301,853 | 105 / 301,839 |
| 133 | 391,941 | 387,811 | 387,862 | 107 / 387,941 | 108 / 387,743 |
| 136 | 495,971 | 491,813 | 491,800 | 107 / 491,926 | 105 / 491,800 |

All 59 exported proofs verified. [Native counts](artifacts/counts.json) · [Selectors](artifacts/select.rs) · [Identities](artifacts/identities.json) · [Verification](artifacts/verification.json)
