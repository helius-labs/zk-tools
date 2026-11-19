# Row support for an Euler FFT sum-and-difference transform

The Euler quotient product uses the linear operator E, where E(f) is X times f′. For S = A + B and D = A − B, the same product derivative is (S·E(S) − D·E(D))/2. If D has zeros, a sparse algorithm can skip calculations on its zero input pairs and omit derivative outputs at those positions.

The probe counts nonzero rows after a real solve, including nonzero rows where A equals B. It propagates the resulting support through a radix-2 DIF butterfly model. A pair with either input marked nonzero marks both outputs as potentially nonzero. This conservative model ignores later cancellations. The same far-to-near propagation describes support demanded by an output-restricted DIT when the desired outputs are those nonzero rows. The model counts butterflies and omits the costs of converting representations, implementing pruning, extracting the outputs and running the product.

The comparison uses all eight original requests and four additional transactions. Every exported proof independently verified. No witness values were saved by the probe. Production arithmetic remains unchanged.

| Shape | Input | Rows / domain | Equal nonzero A and B | Nonzero A / B | Nonzero S / D | Potentially full butterflies A+B / S+D | Empty butterflies A+B / S+D |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 30,922 / 32,768 | 18,642 | 29,780 / 28,769 | 30,528 / 11,889 | 484,473 / 457,781 | 60 / 10,620 |
| 2×2 | catalog | 53,874 / 65,536 | 32,347 | 51,757 / 49,986 | 53,142 / 20,803 | 1,018,874 / 965,005 | 373 / 26,444 |
| 4×2 | catalog | 98,674 / 131,072 | 58,989 | 94,509 / 91,281 | 97,186 / 38,213 | 2,146,253 / 2,039,872 | 5,617 / 61,607 |
| 8×2 | catalog | 189,618 / 262,144 | 113,037 | 181,286 / 175,170 | 186,618 / 73,613 | 4,541,774 / 4,325,613 | 8,986 / 128,922 |
| 12×2 | catalog | 280,242 / 524,288 | 166,699 | 267,726 / 258,708 | 275,840 / 109,189 | 9,410,292 / 8,991,721 | 29,038 / 306,204 |
| 16×2 | catalog | 371,122 / 524,288 | 220,365 | 354,207 / 342,081 | 364,886 / 144,585 | 9,590,934 / 9,208,985 | 18,250 / 213,382 |
| 24×2 | catalog | 554,226 / 1,048,576 | 328,077 | 528,063 / 510,157 | 544,358 / 216,377 | 19,851,411 / 19,117,138 | 61,177 / 517,965 |
| merge-24×1 | catalog | 525,759 / 1,048,576 | 314,079 | 502,673 / 486,144 | 517,710 / 203,679 | 19,800,251 / 18,964,981 | 62,934 / 630,776 |
| 4×2 | heldout | 98,674 / 131,072 | 58,987 | 94,351 / 91,590 | 97,257 / 38,286 | 2,146,445 / 2,040,290 | 5,576 / 61,333 |
| 8×2 | heldout | 189,618 / 262,144 | 113,035 | 181,045 / 175,728 | 186,787 / 73,784 | 4,542,250 / 4,326,407 | 8,827 / 128,468 |
| 8×2 | v153 | 189,618 / 262,144 | 113,035 | 181,151 / 175,579 | 186,752 / 73,749 | 4,542,198 / 4,326,283 | 8,836 / 128,522 |
| 8×2 | v156 | 189,618 / 262,144 | 113,036 | 181,222 / 175,476 | 186,728 / 73,724 | 4,542,124 / 4,326,091 | 8,878 / 128,665 |

[Counts by stage](artifacts/counts.json) · [Workflow](artifacts/workflow.json.gz) · [Diagnostic source](artifacts/source.patch) · [Verification](artifacts/verification.json)
