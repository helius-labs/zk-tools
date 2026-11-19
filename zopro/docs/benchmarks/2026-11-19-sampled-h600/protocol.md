# Sample nonzero H coefficients before grouping

The 2×2 prover previously built 46,345 one-element scalar groups on the original centered request. A stack sample of 64 H coefficients chooses direct curve multiplication when more than half the sample is nonzero and those nonzero coefficients are distinct. All other cases use the existing scalar grouping decision. The change applies only to the canonical H query. A poor sample can change the execution time but both paths evaluate the same curve sum.

Five randomized blocks on 32 Ryzen workers compare ordinary and centered keys, each with the candidate disabled and enabled, against the prior executable forced to stream the centered H query directly. Each process warms three times and measures 31 full verified proofs. The test uses three 1×2 and four 2×2 inputs, including ones absent from the earlier 2×2 routing diagnostic. Every exported proof must verify with the original key. A native test covers unique, repeated, sparse and empty-valued samples.

[Fixed process order](artifacts/schedule.json)
