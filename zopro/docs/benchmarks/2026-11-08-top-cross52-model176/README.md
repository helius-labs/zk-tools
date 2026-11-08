# Shared top-limb products in six-slot Montgomery rows

**The exact arithmetic and range screen passes. Native qualification is next; performance is unmeasured.** A new fixed schedule shares the top diagonal across four ordinary CIOS rows, reducing the source IFMA recipe from 100→92 for ordinary products and 150→134 for fused products. The accumulator retains six slots. Production remains unchanged.

For i<4, let Di=ai·bi, D4=a4·b4 and Pi=(ai−a4)(b4−bi). Then Di+D4+Pi=ai·b4+a4·bi. Row i computes Di once, retains the other three lower-limb products and inserts the combined cross term at coefficient 4. That pulls ai·b4 forward from row 4; the last row inserts only D4. Four cross identities remove four of 25 elementary products. Fused operations apply the same schedule twice and retain one cancellation frontier.

|Recipe property|Accepted CIOS|Earlier local paired tiles|New top-limb sharing|
|---|---:|---:|---:|
|Ordinary source IFMAs|100|96|92|
|Fused source IFMAs|150|142|134|
|Accumulator slots|6|7|6|

D4 requires two persistent halves per elementary product in addition to the six accumulator slots. Inputs, signs, recombination, constants, normalization and compiler temporaries are additional. These are source-operation counts, not decoded instructions or a complete register/stack bound. Lower multiplication volume does not prove a speedup: the [earlier local tiles](../2026-11-08-blocked-pair52-screen176/README.md) and [factor-placement caller](../2026-11-08-g2-factor-placement-screen176/README.md) both retain their failed performance decisions. No unchanged implementation is retried.

The representation remains radix 52 with Montgomery R256/R260. The original final 48/52 cancellation and fused lazy correction remain required. The signed-pair arithmetic identity is inherited byte-exact from the prior local-tile model; this new schedule and population are executed once. Negative intermediate coefficients occur, so a native implementation must preserve signed carries rather than assuming every coefficient is unsigned.

## Executed checks

The pinned Rust 1.98.1 overflow-checked compile and new model run succeed on their first executions. There are 87,948 complete products, 439,740 cancellation steps and 586,320 cross identities across both primes and both radices. Each panel includes 47 boundary values crossed into 2,209 tuples, 4,096 new SHA-derived tuples and 1,024 changing steps for each of ordinary, sum2 and positive 2p-complement difference. Inclusive 2p inputs, zeros and negative pair products are retained.

Each cancellation quotient agrees with a separate full-product recurrence. Each six-slot partial integer agrees with an independently accumulated inserted-row integer. The complete quotient and final result agree with full-radix Euclidean-inverse REDC and modular multiplication. Tests use BigInt/BigUint rather than a new independent Arkworks population; they are not native field, curve or proof verification. No successful parent tests or arithmetic population is replayed.

Conservative interval propagation bounds pre-packing coefficients below 64B and all intermediates below 1024B, B=2^52. A separate coarse envelope below 801B also fits signed 64-bit. The offline reader independently reconstructs the 25 graded schoolbook monomials, totals and recipe counts, checks 185 pins and both command/stream receipts, and verifies the reported interval bounds against that coarse argument. It does not reexecute the successful arithmetic model or act as a second native decoder.

The fixed construction adds no proving-key table/copy, retained execution plan, production pool, new host field representation or input-sized runtime buffer. Big-integer arrays belong to the diagnostic model. No process peak, minimum-RAM, pressure/swap, phone, AWS/Aeglos, huge-circuit or concurrent-proof result is produced.

Next is one private native implementation, broad independent field-output tests and actual linked code inspection. Only subsequent complete changing-input threaded callers with automatic profiles can select an SDK experiment. Full all-eight proof, portable, resources and independent Gnark qualification remain required before adoption. No layout, width, compiler or worker sweep is selected.

## Full-proof comparison, inherited

Sub-8 ms was already measured: historical private candidates 7.86–7.87 ms and the same accepted build 7.92 ms. Its latest matched campaign measured 8.018438 ms; an unadopted default-carry candidate measured 7.897108 ms. Cross-campaign absolutes establish neither a regression nor a new record. The table below is inherited from the [latest full-proof campaign](../2026-11-08-uniform-carry-default-performance183/README.md), not a measurement of this arithmetic model. Supplied M5 references use different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.018438|37 / 4.614×|24 / 2.993×|3.7|
|2x2|11.199751|59 / 5.268×|30 / 2.679×|5.9|
|4x2|15.957517|109 / 6.831×|48 / 3.008×|10.9|
|8x2|24.250900|189 / 7.794×|74 / 3.051×|18.9|
|12x2|33.735233|284 / 8.418×|101 / 2.994×|28.4|
|16x2|40.165709|335 / 8.340×|124 / 3.087×|33.5|
|24x2|65.802777|528 / 8.024×|176 / 2.675×|52.8|
|merge-24x1|64.926690|507 / 7.809×|175 / 2.695×|50.7|

Accepted merge54 remains 162.442299 ms without a supplied M5 reference. The original verified CPU AVX 10× objective remains active and unmet, 0/8. Zero new SDK requests or independent Gnark exports; canonical inventory remains 36,477. No new proof speed is claimed.

[Fixed design](artifacts/design.md), [model](artifacts/model.rs), [results](artifacts/result.json), [review](artifacts/review.json), [evidence](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
