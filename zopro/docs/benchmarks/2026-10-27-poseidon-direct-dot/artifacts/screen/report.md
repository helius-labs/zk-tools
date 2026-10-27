# Verified SDK screen

Three rotated/reversed fresh-process blocks, 17 timed complete SDK requests after three primes. Every final export passes the configured independent verifier, public-input binding and freshness checks. No cycle profiles are collected in this short screen. It does not qualify adoption or replace the full circuit matrix, resource checks and profiles. There is no minimum gain cutoff.

|Case|Variant|Control|SDK median ms|Paired geometric ratio|Paired median saving µs|Winning blocks|
|---|---|---|---:|---:|---:|---:|
|1x2-w16|direct|accepted|11.876990|1.004734741|276.331|2|
|1x2-w16|fresh|accepted|11.920432|0.960709065|579.804|2|
|1x2-w32|direct|accepted|8.523445|0.989859579|93.566|2|
|1x2-w32|fresh|accepted|8.590331|0.992494129|62.427|2|
|1x2-w16|accepted|direct|13.160621|0.995287571|-276.331|1|
|1x2-w16|fresh|direct|11.920432|0.956181793|-43.442|1|
|1x2-w32|accepted|direct|8.570033|1.010244303|-93.566|1|
|1x2-w32|fresh|direct|8.590331|1.002661539|-37.100|1|
|1x2-w16|accepted|fresh|13.160621|1.040897849|-579.804|1|
|1x2-w16|direct|fresh|11.876990|1.045826231|43.442|2|
|1x2-w32|accepted|fresh|8.570033|1.007562635|-62.427|1|
|1x2-w32|direct|fresh|8.523445|0.997345526|37.100|2|

Savings use matched blocks; positive means faster. Independent medians need not reproduce paired ratios. Raw samples, resource summaries, proof exports and command/hash receipts are retained. Investigate unstable controls before interpreting a small difference; this report makes no automatic adoption decision.
