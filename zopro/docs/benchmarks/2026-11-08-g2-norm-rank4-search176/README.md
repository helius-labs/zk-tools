# Four-product search for a coupled G2 packet

No new BN254 arithmetic candidate is selected. An exhaustive search finds no four-product identity for the coupled norm/numerator packet over the three tested fields where −1 is nonsquare. The split-field control finds identities, and a separately implemented dense-rank search reproduces all four domains. **This is finite-field evidence, not a proof of impossibility over BN254 or a proving-time improvement.**

For denominator components a,b and signed numerator components c,d, the targets are N=a²+b², R=ac+bd and I=ad−bc. The accepted [G2 code](../../../crates/zopro-core/src/msm.rs) uses six elementary products inside three fused operations. This study seeks all three outputs from at most four homogeneous products of arbitrary linear forms, followed by arbitrary linear output combinations. It differs from the earlier two-output complex and free-six-variable rank-five cross maps. No existing negative kernel is rebuilt or retimed.

## Search and independent validation

|Prime|−1|Projective linear forms|Unordered product pairs, including squares|At most four products?|
|---:|---|---:|---:|---|
|3|nonsquare|40|820|None in this model|
|5|square (control)|156|12246|Yes; three products suffice|
|7|nonsquare|400|80200|None in this model|
|11|nonsquare|1464|1072380|None in this model|

The four declared domains contain **1,165,646 distinct unordered product pairs** in total. Each domain needs two enumeration passes; that total counts pairs once, not repeated visits or cryptographic proofs. All nonzero projective linear forms and all unordered pairs with repetition are included.

Let T be the three-dimensional span of the desired quadratics. A product span containing T has dimension three or four. In dimension three, the products already lie in T. In dimension four, all products outside T project to a single projective line in the seven-dimensional quotient. The search groups products by that quotient line and includes every product lying in T before checking each full lift rank. This covers the declared model without enumerating every four-element combination. It permits squares and absorbs nonzero input-product scaling into the output coefficients. It does not cover arbitrary rational circuits, divisions, other polynomial packets, representations or native instruction schedules.

The producer uses three-coordinate affine lift differences. A separate reader reconstructs every product and quotient group using ten-coordinate polynomial elimination, with a different linear-form enumeration order. It agrees on product counts, target-only span, group counts, full lift-rank and group-size histograms, and existence decisions. The two new tests also check projective coverage, symbolic coefficient reconstruction, direct polynomial evaluation and the output solver. The four-command Rust/xtask search workflow and reader pass on first execution, with no failed run or excluded domain.

The positive F5 control has a square root of −1; BN254 Fq does not. The first emitted three-product and four-product control packets satisfy all 30 target coefficient equations modulo five. Their centered input forms have nonzero augmented integer minors **20 and 80**, checked separately by permutation expansion. These minors stay nonzero modulo BN254, ruling out a lift of those particular forms even with arbitrary BN254 output coefficients. This does not rule out different BN254 forms, and the search does not lift every possible F5 program.

## Complete-caller costs remain relevant

Even an exact four-product identity would require a separate range and native cost qualification. The accepted norm/numerator packet has a 450-IFMA source proxy. Four full raw products plus three standalone reductions have a prospective proxy of 365 before all linear operations, signs, carries, packing and correction. These are source counts, not emitted instructions, cycles or speed estimates. The [earlier top-cross caller result](../2026-11-08-top-cross52-callers176/README.md) already demonstrates that fewer IFMAs can accompany more executed work and slower callers.

Moving this packet into the prefix pass would retain four FqBatch values per group instead of two: **640 → 1,280 bytes**, or **16,000 → 32,000 bytes** for a 200-point batch. The fixed minimal-record layout also adds two coordinate pack calls per group. No such buffer or kernel is implemented. These prospective bytes are not prover RSS, minimum RAM or mobile qualification. No key/point copy, retained solver plan, production pool or lookup table is added.

The [research audit](artifacts/research-audit.md) records source/history checks and the exact primary sources consulted. The new search is derived from current polynomial expressions; it imports no external implementation or reported hardware gain. Two ePrint PDFs were unavailable with HTTP403, so only their abstracts are discussed. Full third-party papers and code are excluded from the archive.

No native, full-proof, AWS, Aeglos, Gnark, huge-circuit, device, memory-pressure, swap or throughput result is added. Accepted production176/SDK6d83 and the canonical **36,477** verified proofs remain unchanged. No SDK proof or Gnark export was requested. The fixed four-product packet search is closed at this screen; no five-product, larger-prime or favorable-parameter native rescue is selected.

## CPU and Metal comparisons, unchanged

Inherited latest accepted local 32-worker full-proof observations from the [default-carry comparison](../2026-11-08-uniform-carry-default-performance183/README.md), with supplied historical M5 references on different hardware:

|Shape|Accepted Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.018438|37 / 4.614×|24 / 2.993×|3.7|
|2×2|11.199751|59 / 5.268×|30 / 2.679×|5.9|
|4×2|15.957517|109 / 6.831×|48 / 3.008×|10.9|
|8×2|24.250900|189 / 7.794×|74 / 3.051×|18.9|
|12×2|33.735233|284 / 8.418×|101 / 2.994×|28.4|
|16×2|40.165709|335 / 8.340×|124 / 3.087×|33.5|
|24×2|65.802777|528 / 8.024×|176 / 2.675×|52.8|
|merge24|64.926690|507 / 7.809×|175 / 2.695×|50.7|

The full verified all-eight CPU10× objective remains active and unmet, **0/8**. Merge54 remains 162.442299 ms without an M5 reference. Earlier accepted 1×2 observations include 7.915095 ms; private experiments include 7.859756 and 7.869400 ms. Sub-8 ms was already measured. Cross-campaign absolute variation does not establish an adopted regression or new record.

[Result and emitted packets](artifacts/result.json), [independent review](artifacts/review.json), [fixed design](artifacts/design.md), [source cost/context](artifacts/field-context.json), [search](artifacts/search.rs), [reader](artifacts/review.rs), [source, pins, plans and command receipts](artifacts/source-and-receipts.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS).

All archived UTF8 member hashes and the gzip roundtrip pass. Executables, libraries, unchanged full production sources, key/image/witness/proof bytes, the private proof ledger and full external source bodies are excluded. Relevant artifact identities remain in pins. Manual preparer/reviewer compiler invocations lack complete xtask compiler receipts; the report makes no claim that they have them.
