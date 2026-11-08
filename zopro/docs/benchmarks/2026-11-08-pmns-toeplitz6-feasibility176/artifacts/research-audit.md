# Prior work and primary-source audit

Current production176/SDK6d83 remains accepted. Existing full-field census
2026-11-07-whole-field-census176 reports21,231,136Fq vector lane products
versus1,070,736Fr for primary; these are diagnostic method counts, not cycle
or critical-path shares. The G2 field-volume census prioritizes streaming
accumulation over another weighted-tree-only arithmetic change. No census
or profile was rerun for this study.

Reviewed completed reports before selecting the new private root:
- 2026-10-23-pmns-field-component: schoolbook36 and pairwise21 products;
  fully paired version105IFMAs/234vector-add sites and poor performance.
- 2026-11-08-pmns-truncation-width176: dense coefficient/word template only;
  Toeplitz/polynomial fast multiplication explicitly outside its exclusion.
- 2026-11-06-toom5-work176: widened nine-point integer limb Toom5, rejected.
- 2026-11-08-fused-scale-callers176: fewer static sites but more executed
  instructions and slower complete callers, rejected and pushed4218b774cd.

Primary paper inspected again, not newly discovered:
https://arxiv.org/html/2610.09767v1 , sections2.1/2.2/2.3.2 and6.
Its discussion connects PMNS reduction with Toeplitz matvecs and warns
that non-multiply costs can dominate. Its large-modulus coefficient-lane
benchmarks are not BN254 eight-independent-field measurements.

Authors' source inspected using the connected GitHub fetch tool:
https://github.com/francoispalma/PMNS/blob/master/karatoep/toeplitzmacros.h
Git blob SHA1 0caad4cb1c779977cade7339c61847264858475d.
The private downloaded text is pinned by SHA256. TOEP22TOP uses3subproducts;
TOEP33TOP uses6. The Rust study evaluates these standard block identities
and independently checks them against dense matrix/polynomial oracles.
No mathematical novelty or external implementation speedup is claimed.
No C code is compiled. No complete external source/paper is published.

New local contribution: exact6coefficient18-product schedule; explicit
Fq-trinomial row correction; current parameter field/range checks using
the new factored algorithm; fixed small-binomial eligibility and determinant/
uniform-bound certificate screen. The square/cube flag is only an obvious
factor guard, not a complete irreducibility or feasibility decision.
