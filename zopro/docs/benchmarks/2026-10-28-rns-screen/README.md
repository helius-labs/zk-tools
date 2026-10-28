# Structured RNS arithmetic and conversion screen

The exact RNS construction works for both BN254 fields, including fused two-product sums and differences. Regularly spaced moduli replace a dense CRT sum with a degree-four polynomial evaluation. The resulting recipe still adds substantial conversion work and doubles the working residue planes. **No native kernel or SDK trial is selected for this recipe, and no speedup is claimed.**

Accepted local 1×2 remains **8.324853 ms / 134.50 MiB**. The original all-eight CPU10× goal remains active and unmet, 0/8.

## Construction and validation

The local basis is `m_i=2^52-(3+2520i)`, indices0–9, split into two groups of five. All ten moduli are pairwise coprime; compositeness is allowed. The basis products exceed64p and8p for both fields. Modulus constants are independent of circuits and proving keys.

Normal values lie in `[0,4p)`. A single linear operation may form an input in `[0,8p)`. Left-basis preweighted residues form an approximate CRT quotient using only their sum and a shift; its underestimate is at most one. The resulting Montgomery output returns to `[0,4p)`, including fused sum2 and a difference2 with the field-zero64p² offset. An upward64bit estimate makes the reverse conversion exact under a checked range inequality. [Derivation and bounds](artifacts/bounds.md) retain the full assumptions.

Both conversions evaluate a degree-four polynomial through finite differences. Two26bit coefficient halves keep signed intermediates within64bits. Recombining those outputs can require66bits and must retain the high part. Input inverse weights, output factors, per-modulus reductions and conversion to the current binary field format remain costs.

| Field | Boundary reductions | Random reductions | Changing chain steps | Total reductions |
|---|---:|---:|---:|---:|
| Fq | 3,888 | 12,288 | 1,024 | 17,200 |
| Fr | 3,888 | 12,288 | 1,024 | 17,200 |

Every result agrees with independent BigInt Montgomery arithmetic and the field interpretation. Both bases agree on each reconstructed output, and all outputs meet the4p bound. Five new tests cover forward/backward polynomial conversion, quotient carries, small-prime fused arithmetic, invalid channel/integer ranges and two analytically constructed quotient boundaries. The latter exercise the extra-p case that the fixed full-width sample sets do not encounter. Successful tests are not repeated.

An initial Rust compilation required an explicit array return type; no test ran before that repair. The failed source/compiler log and the attempted invocation of the absent test binary are retained. The initial three tests then pass once; the two added tests each run once with exact filters. Model and source/library pins are checked before/after and at closure.

## Cost and storage assessment

The [Langowski/Devadas2025 paper](https://eprint.iacr.org/2025/1068) studies vectorized single-integer latency, with its headline CPU result at much larger primes. Its [author code](https://github.com/SimonLangowski/RNSMont/tree/2aa19af8fcd4f2ed5ecebe9b6ad8d0245740a8c4) uses lanes for residue moduli; current Zopro uses lanes for eight independent field values. The paper's full PDF returned403; its abstract, inspected author sources and exact parameter calculations support this comparison, not a claim of reading the full paper or reproducing its benchmarks.

At five channels per basis, a direct eight-instance port of the inspected dense flow has200 IFMA sites, against100 in the current ordinary CIOS template. This excludes quotient-estimation and conversion work. Actual canonical BN254 inputs produce54bit first correction quotients in both fields, so the earlier tentative shortcut dropping their upper products is invalid for that normalized representation. The new structured construction has different preweighted inputs and proves both quotient corrections are in0–4; the two statements concern different domains.

| Partial ledger for the specified structured recipe | Sites |
|---|---:|
| IFMA | 65 |
| u64 products | 70 |
| u32 products | 10 |
| Other vector operations | 255 |
| Total before excluded work | 400 |

Excluded work includes polynomial splitting/recombination/folding, quotient sums/carries, correction lookup/addition/reduction, binary conversion, loads/stores and register spills. Fused sum/difference operations add further product/offset work. These are explicit recipe counts, not optimized machine instructions, timings or a lower bound on all RNS algorithms. Fewer IFMAs alone do not establish a complete latency advantage. This review does not justify building the specified native port.

Proposed universal constants total1040bytes, including400bytes of small correction constants. An unpadded state has80bytes of residues per field versus32bytes currently stored; eight-instance working planes occupy640bytes versus320. These representation sizes do not establish proof RSS, minimum RAM, spills or memory-pressure behavior. No persistent point table or key expansion is introduced.

The [Bajard/Didier/Kornerup2001 paper](https://webusers.imj-prg.fr/~jean-claude.bajard/MesPublis/ARITH15.pdf) provides RNS/base-extension background. [Selected RNS Bases2009](https://www.acsel-lab.com/arithmetic/arith19/papers/ARITH19_Bajard.pdf) motivates examining sparse inverse constants next; its full construction is not implemented here. The arithmetic-progression polynomial construction and4p bound above are local derivations. The [MORPH paper](https://arxiv.org/html/2604.17808v1) uses matrix engines and additional representation storage; its accelerator results are not CPU predictions.

Only independent Rust models are archived. The inspected upstream C++/Python files remain local with commit/blob/SHA256 references; none were compiled, executed, installed or adopted. All172 production source hashes remain unchanged. There are **zero new SDK requests, proof exports, verifier calls or timing/profile captures**. The independent proof ledger remains **12,524**.

No AWS, Aeglos, phone, TEE, pressure, concurrency or large-circuit result is added. A further RNS attempt needs a distinct reduction in inverse-weight/output-conversion work; this recipe is not scheduled for an unchanged timing trial.

## Inherited CPU and Metal comparison

The [accepted all-eight report](../2026-10-27-clean-fft-adoption/README.md) remains authoritative, including its full-performance-gate failure and engineering exception. The M5 references are supplied historical figures, not new measurements.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445× | 24 | 2.883× |
| 2x2 | 11.460675 | 59 | 5.148× | 30 | 2.618× |
| 4x2 | 15.854365 | 109 | 6.875× | 48 | 3.028× |
| 8x2 | 24.084626 | 189 | 7.847× | 74 | 3.072× |
| 12x2 | 33.693397 | 284 | 8.429× | 101 | 2.998× |
| 16x2 | 39.981636 | 335 | 8.379× | 124 | 3.101× |
| 24x2 | 66.010670 | 528 | 7.999× | 176 | 2.666× |
| Merge24x1 | 65.086624 | 507 | 7.790× | 175 | 2.689× |

Accepted merge54 remains161.341835ms without a supplied M5 reference.

[Design](artifacts/design.md), [exact model](artifacts/rns_model.rs), [all results](artifacts/analysis.json), [bounds](artifacts/bounds.md), [upstream identities](artifacts/upstream-references.json), [source pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
