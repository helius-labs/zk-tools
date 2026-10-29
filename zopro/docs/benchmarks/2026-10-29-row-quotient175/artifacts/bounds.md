# Row-local cancellation digit by distributivity

Let B=2^52, I=-p^-1 mod B, and A,C have normalized five-limb representations. At entry to CIOS row i, let t0 be the existing unnormalized low coefficient BEFORE adding A*b_i (+C*d_i for fused mode). The accepted cancellation digit is
q_i = ((t0 + a0*b_i + fused*c0*d_i)*I) mod 2^s,
with s=52 except the final mixed row s=48. Only low s bits matter; IFMA masks each multiplicand to52bits, which is sufficient for this congruence even when t0 has accumulated higher carry bits.

Prepare A0I=(a0*I)modB and C0I similarly. Prepare five row contributions w_i=lo52(A0I*b_i)+fused*lo52(C0I*d_i). Then
q_i = (lo52(t0*I)+w_i)mod2^s.
For row0, t0=0, so q0=w0 modB without another multiplication. Each w_i<2B, and adding one low52 product stays<3B; all unsigned64 computations are exact. Using modB preparations is also valid for the final mod2^48 digit.

The product rows themselves, exact cancelled-low carry identity, remaining reduction IFMAs, mixed48 terminal repacking, final normalization and fused2p correction are copied unchanged from current product_cios. Every q_i must match the original row digit, so every postrow integer and final lazy output representation match exactly. Global quotient equals X*(-p^-1) modR, R=2^256 or2^260, independently checked using BigUint. Existing inputs<=2p yield raw ordinary<2p, mixed fused<3p and uniform fused<2p; existing final correction restores<2p. The copied working-coefficient envelope remains<2^61; new quotient-only partials<3B never enter coefficients except through normalized q_i.

This prepares only proof-local/current-operation low-digit products in registers, including its cost on EVERY multiplication, with no key table, root table, heap buffer or changed representation. It does not prepare constants once per key or assume later reuse. For ordinary multiplication: one preparation+five row contributions+four old-state quotient products replace five original quotient products, so105IFMA instead100. Fused: two+ten+four replacefive, so161instead150. Four/five-future quotient dependencies remain; only dependence on the CURRENT raw product row is removed. This differs from paired/triple-prefix lookahead and does not use a104/156-bit inverse, extra modulus constants, more raw input rows or a new global reduction order.

Potential cost: five row contribution vectors plus up to two prepared low digits lengthen live registers; LLVM may spill, recompute or reorder. Source dependency graphs/static counts are not latency or spill evidence. No speedup predicted. Do not infer CPU10x progress from component-only gains.
