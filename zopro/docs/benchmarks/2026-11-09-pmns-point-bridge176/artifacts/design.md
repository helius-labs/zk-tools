# PMNS point ranges and compact-coordinate bridge

Frozen question: can the qualified six-coefficient binomial Toeplitz recipe
remove useful complete-caller work while retaining current compact R256 key
and bucket coordinates? Do not rebuild/retest prior PMNS/Toeplitz kernels.
No native timing or code selection belongs to this screen.

Source audit: derive G1/G2 actual eight-point full-group imports/exports and
ordinary/fused product populations, preserving both G1 group constructors.
Expected G1=4imports/2exports/6ordinary; borrowed G2=12imports/4exports/
7ordinary+5fused. The deliberately optimistic retained-bucket case still
imports2G1 or6G2 incoming key coordinate vectors. It excludes all bucket
normalization/storage/export costs and is not implemented.
Also report an idealized G2 floor of4distinct incoming coordinate imports,
assuming free reuse of prefix X conversions. Every declared bridge has at
least one54-IFMA nominal internal reduction;4*54>190saved G2IFMAs and
2*54>60saved G1IFMAs even before forming any conversion linear combination.
This is a bound for this fixed bridge family, not every possible converter.

Audit point coefficient ranges assuming each PMNS product returns to rho.
Treat additions/subtractions as literal coefficient operations. G2 square's
imaginary component doubles a product. Derive first-update output x bounds
3rho/4rho, y2rho, and asymmetric finishing-product bound9w*rho². Derive
retained-bucket second-update norm coefficient41w, plus a recurrence for
repeated updates. These are conservative conditional interval certificates,
not actual valid-curve trajectories or impossibility results.

Use ALL SIX existing Fq X6-2 parameter candidates from the prior exact input.
Read raw p/gamma decimal tokens exactly; serde_json's floating conversion
must not supply those integers. Check equations and mod2^52 inverse again
only as preconditions of the NEW conversion, not rerun old finite suites.

Fixed compact import family: digit widths43..52, one or two PMNS internal
reductions, all six candidates;120 configurations. Let B=2^52,H=2^256.
For r reductions, precompute small polynomials C_i with
C_i(gamma)=2^(ki)*H^-1*B^(r+1) modp.
Compute T=sum digit_i*C_i; apply r reductions. Preserve obvious small
power-of-two constant polynomials rather than hide needless multiplies.
Generate remaining constants with five BigInt internal reductions of a
scaled scalar residue; check their evaluations and rho bounds.
Derive a rigorous per-coefficient import bound from exact constants, digit
widths, and M row sums. Price nonzero/non-power-of-two coefficient products,
plus18low+36full-product-halves per inherited Toeplitz internal reduction.
All source counts exclude sign/carry/ALU/constant/packing costs and are not
native instructions or measured speed.

Per configuration check16boundary+256deterministicfullwidth raw R256 inputs
and64output-fed inputs against independent field evaluation and roundtrip.
Report uncertified bounds and finite overruns without dropping rows.
Explicit export: sum six signed coefficients times fixed constants
gamma^j*H²*B^-1 modp, then five mixed52/52/52/52/48-bit Montgomery rows.
Check literal rows against a closed integer Montgomery quotient and direct
polynomial evaluation. Prove signed pre-reduction range and one correction
each side for |coefficient|<=8rho. Check64corners+128random extra exports
per parameter; outputs already produced by import also export once.

Compare complete source-product ledgers using ALL normalized-bound-certified
rows. Retain the optimistic unnormalized imports separately. If imports
alone exceed the available product saving, or full compact bridge has no
product-volume advantage, close this fixed bridge before native timing.
This is not a lower bound for every conversion or all PMNS implementations.
Do not hide conversions in a key copy, point cache, unpriced setup, new key
format or retained state. No memory/throughput/pressure claims from counts.

Independent reader must reconstruct source ledgers, constants/evaluation,
range inequalities and symbolic recurrences; inspect actual receipts and
all accepted source hashes. Do not replay successful finite populations.
Publish factual report and complete inputs/source/receipts; commit AND push.
Canonical37023 proofs unchanged. Full CPU AVX single-proof>=10xM5CPU all8
plus CPU/Metal comparisons remains active0/8; no reduced goal.
