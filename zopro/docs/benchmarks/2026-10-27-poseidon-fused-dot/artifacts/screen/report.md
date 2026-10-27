# Verified SDK screen

Three rotated/reversed fresh-process blocks, 17 timed complete SDK requests after three primes. Every final export passes the configured independent verifier, public-input binding and freshness checks. No cycle profiles are collected in this short screen. It does not qualify adoption or replace the full circuit matrix, resource checks and profiles. There is no minimum gain cutoff.

|Case|Variant|Control|SDK median ms|Paired geometric ratio|Paired median saving µs|Winning blocks|
|---|---|---|---:|---:|---:|---:|
|1x2-w16|fresh|accepted|10.254917|0.960906967|248.138|3|
|1x2-w16|fused|accepted|10.237584|0.915562078|132.981|2|
|1x2-w32|fresh|accepted|8.606477|1.000693066|35.638|2|
|1x2-w32|fused|accepted|9.152437|1.083007152|-606.554|1|
|1x2-w16|accepted|fresh|10.503055|1.040683473|-248.138|0|
|1x2-w16|fused|fresh|10.237584|0.952810323|-115.157|1|
|1x2-w32|accepted|fresh|8.642115|0.999307414|-35.638|1|
|1x2-w32|fused|fresh|9.152437|1.082257076|-384.916|0|
|1x2-w16|accepted|fused|10.503055|1.092225229|-132.981|1|
|1x2-w16|fresh|fused|10.254917|1.049526832|115.157|2|
|1x2-w32|accepted|fused|8.642115|0.923354936|606.554|2|
|1x2-w32|fresh|fused|8.606477|0.923994882|384.916|3|

Savings use matched blocks; positive means faster. Independent medians need not reproduce paired ratios. Raw samples, resource summaries, proof exports and command/hash receipts are retained. Investigate unstable controls before interpreting a small difference; this report makes no automatic adoption decision.
