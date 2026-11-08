# Overwide radix-52 quotient: prospective native qualification

This is a distinct consumer-truncation implementation, declared before any
native build, test or timing. The prior masked-quotient complete-caller trial
remains rejected. Production 176, accepted SDK 6d83 and the 37,023-proof ledger
remain unchanged. The previous goal turn only clarified historical sub-8 ms
results and was no progress; current source and the next safe action have been
revalidated. No owned recording is live.

For any unsigned 64-bit words x and v, let B=2^52, U=2^64,
q=(x*v mod U), and r=((x mod B)*(v mod B)) mod B. Because B divides U,
q mod B=r. IFMA low and high consumers multiply only their operands' low
52 bits, then add either product bits 51:0 or 103:52 to the full 64-bit
accumulator modulo U. Thus replacing r by q in either multiplicand position
leaves each consumer bit-for-bit identical for every other operand and
accumulator, including accumulator wrap. This is an algebraic all-input proof;
the finite native tests below check its implementation, not its universality.

In the actual normal-row product_cios macro, m occurs only as a multiplicand
in reduction's madd52lo/madd52hi or the cancellation-carry madd52hi. Neither
carry expression, t-array shift, normalization nor output bounds inspect m
directly. Start both executions with identical t. Data/fused products are
unchanged; q mod B equals old m; every reduction/carry accumulator is therefore
identical; the next t is identical. Induction proves exact raw state equality
after every normal row and the same final output, for both carry cfg paths,
both FUSED choices, both primes and both radices. It does not need a new lazy
bound: every raw accumulator and returned digit is the original one. In R260,
row(4) is another normal row. In R256, the final mixed step is unchanged.

The mixed final quotient MUST retain its 48-bit mask. The shifted-by-four
consumer sees only 48 original bits, but unshifted p1..p4 consumers see 52.
For example q=2^48, multiplier=1, accumulator=0: masked-48 IFMA low is zero,
unmasked IFMA low is 2^48. This is a real counterexample to blanket deletion.

Implementation: start with the already qualified 18-file private parent field
snapshot, change only the normal-row quotient expression under a distinct
zopro_quotient_wide cfg. Enable the parent's zopro_quotient_mullo for the still
masked mixed step. Preserve the original reference/diagnostic suffix exactly.
Reverse the one new replacement to recover the complete parent source, then
reverse its audited changes to recover accepted source. The other 17 field
files must match both parent and production. No table, retained plan, heap
allocation, point/key copy, pool, new scalar arithmetic or production change.

Fixed mechanism/cost case: retain five VPMULLQs per product while eliminating
four explicit quotient ANDs in R256 or five in R260, compared with the masked
parent. Total multiply count is unchanged. Generated code may schedule or
combine operations differently, so static opcode counts must be inspected;
no latency or complete-proof gain is predicted from removed ANDs alone.
Intel target policy and generated-source integration remain unresolved.

Native qualification: a new source-pinned Rust 1.98.1 / serialized xtask
workflow builds only the new ON artifact, links and runs its new tests, and
checks no_asm/no_ifma gating with all selectors present. Reuse the qualified
original/OFF and masked-parent libraries unchanged, never rebuild them for
bookkeeping. Test both Fq/Fr, R256/R260, mul/sum/difference, original raw digits,
independent Arkworks residues/exports, loose inputs through 2p, edges, random
and dependent chains. New consumer tests exercise arbitrary full-width words,
native low/high IFMA in both operand positions, arbitrary wrapping
accumulators, retained 48-bit equality and a negative 48-bit counterexample.
Build/check twelve actual variable-input native O3/fatLTO wrappers and decode
each once after one symbol listing. Compare to the saved parent decodes and
count instructions, ANDs, IFMAs, VPMULLQs, calls and stack operands.

Only successful native qualification selects new coherent complete G1/G2
caller integration and a prospectively frozen full comparison of both curves,
200/201/352/353 points and 1/16/32 workers, original frozen/fresh OFF/ON controls,
outputs/resources and automatic PMU/profiles with independent review. Only
complete-caller evidence can select a full SDK ALL8+merge54 trial, primary16,
independent Gnark proof verification, generator, architecture/portable/resource
and concurrency checks. No change to old gates or favorable-subset promotion.
During any recording, only poll its confirmed live handle.

Primary semantics consulted before implementation:
- https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_madd52hi_epu64.html
- https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_madd52lo_epu64.html
- Intel 319433-049, pp. 2-47/2-48, VEX per-lane operand truncation formulas:
  https://cdrdv2-public.intel.com/782879/architecture-instruction-set-extensions-programming-reference.pdf
Rust's linked 512-bit intrinsics and the actual new native opcodes/tests are
the directly relevant width; the Intel VEX formulas agree but alone are not
evidence about a 512-bit binary.

Full objective stays active: verified single-proof CPU AVX at least 10x supplied
M5 CPU on all eight shapes, reporting CPU and Metal comparisons. Latest
accepted local milliseconds: 8.040649245 / 11.149096784 / 16.069415294 /
24.131177863 / 33.940627784 / 40.164083059 / 66.187704392 / 65.361571353.
CPU targets: 3.7 / 5.9 / 10.9 / 18.9 / 28.4 / 33.5 / 52.8 / 50.7. Zero of
eight met. No AWS, Aeglos, mobile, huge-circuit or memory-pressure result added.
