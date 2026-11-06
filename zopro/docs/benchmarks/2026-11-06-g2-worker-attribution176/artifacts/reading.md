# Arithmetic alternatives and prior experiments

Read the relevant sections of two primary arXiv PDFs, not just their abstracts:

- [Costello–Smith, Montgomery curves and their arithmetic](https://arxiv.org/pdf/1703.01863), sections3.1 and6, PDF pages7–8/20–21. Differential addition needs the difference point as well as its two operands. The cited Brown construction takes(3^n−1)/2 signed-combination x-values for n bases. Its full-arity input requirement is unsuitable for our large universal MSM. These observations reject a direct replacement of arbitrary bucket additions by xADD, not all possible x-only methods.
- [Chung–Costello–Smith, Fast, uniform, and compact scalar multiplication](https://arxiv.org/pdf/1510.03174), section5.3/Table4, PDF pages13–14. The paper supports short Weierstrass scalar multiplication and full-point recovery; x-only arithmetic is not inherently impossible on that curve model. Its formulas still require differential inputs and recovery work. It does not supply a cheap replacement for the current arbitrary bucket accumulator. No speed claim transfers to BN254. This is a scope assessment, not a native experiment or lower bound over every alternative algorithm.

The direct ePrint2015/983 and IACR Eurocrypt2014 PDF fetches returned403; the arXiv PDFs above were accessible. The multidimensional-encoding paper's introduction and differential-addition discussion at https://pmc.ncbi.nlm.nih.gov/articles/PMC7334993/ were read, but many formulas render as images and were not visually inspected; no numerical bound is taken from that page.

History read before choosing a different prefix question:

- 2026-10-22-universal-g1-prefix-chains-sdk: two/four G1 chains; rejected under its then-declared3%gain floor, with the positive two-chain1.35%estimate retained. Do not present prefix striping as a novel algorithm or waive that old gate.
- 2026-10-28-g2-phased-inverse174: four G2 chains, extra passes/materialized inverse norms and nine extra vector products per batch; primary full-proof gain failed.
- 2026-10-31-g2-denominator-tree176: balanced heap, three fewer vector products than original, but native caller instructions/cycles rose; gate false.
- 2026-11-05-g2-denominator-record176: larger retained denominator records, slower across the complete kernel screen.
- 2026-10-26-bounded-steal-search-c7a resolves the stale local report's pending target stage: the target screen passed64/96workers but no production adoption followed; local failure remains. This is historical, not a new AWS comparison.

A possible distinct next prototype is exactly two interleaved borrowed-G2 prefix chains kept in the current streaming forward/point-finishing passes, without the four-chain trial's inverse-norm materialization passes. It adds three root-combination vector products per full batch, keeps two-vector prefix records, original point-finishing/tails/inversion, and adds fixed local vector state. This changes a dependency graph while adding work; it could lose to instruction/register costs. No native code or speed/memory qualification is claimed by this reading note. Freeze a separate design and inspect the previous runtime bodies before implementing. Do not silently revive the prior four-chain/heap/record recipes.
