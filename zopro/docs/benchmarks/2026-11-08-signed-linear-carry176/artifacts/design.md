# Signed linear-coordinate carry normalization: mathematical screen

Previous goal turn PROGRESS: overwide52 completecaller comparison and report
2283bb49560ad88a352d4e5a687f4f07f07fb1eb pushed/exactremoteverified. All capture,
analysis, push and other owned handles terminal0; no live recording/cloud job.
Accepted176/SDK6d83 and canonical37023 unchanged. Full verified single-proof
CPU AVX >=10x supplied M5 CPU on ALL8, reporting CPU+Metal, active/unmet0/8.

This fixed screen is declared BEFORE a model/native execution. No native
candidate is selected until the signed identity and consumer bounds pass.
Do not rerun closed quotient or CIOS-normalization variants.

Current source scope: canonical_sub in core/msm.rs forwards canonical FqBatch
inputs to Ifma8::sub_canonical. G2's canonical signed dy uses signed_delta.rs
signed_difference_canonical. Their normalized results feed unchanged IFMA
products. These input linear operations retain serial per-limb carries in
accepted source176. Generic add/sub, multiplication/reduction, fused products,
FFT, inverse, key representation, point formulas and schedules stay original.

Let B=2^52. For signed independent coefficients s[0..4] in [-2B,3B), define
h[j]=floor(s[j]/B), r[j]=s[j] mod B, so h in[-2,2], r in[0,B).
Simultaneously form v0=r0, vj=rj+h[j-1] for j1..3, and v4=s4+h3.
This preserves the represented integer because the terms B*h[j] telescope
into the next coefficient. Coefficients1..3 are in[-2,B+1]. If all three are
in[0,B), they are already normalized and top v4 modulo B gives exactly the
original serial result modulo B^5. Otherwise ripple v1->v2->v3->v4 with signed
floor carries, masking each emitted digit, then mask top. The represented
integer and exact serial digits remain equal; arbitrary negative/overflow
tuples are compared modulo B^5. Real canonical consumers have total[0,2p]
and2p<B^5, so the represented nonnegative integer is preserved as well.

An unsigned OR of v1,v2,v3 has any bit outside the low52 iff at least one
coefficient is negative or >=B. A negative two's-complement word has high bits
set. This one predicate admits only exact no-cascade states; no random-input
assumption is needed. Repair covers all lanes when any lane needs it. All
intermediates fit signed64, including repair carries; shifts are arithmetic.

Canonical subtraction has independent s[j]=p[j]+a[j]-b[j], a,b<p. Its integer
is p+a-b in(0,2p). Signed dy uses an independently signed limb, rather than
52-bit complement plus a serial initial carry: positive s[j]=p[j]+by[j]-ay[j],
negative s[j]=2p[j]-by[j]-ay[j]. The latter sums exactly to2p-by-ay. Each fits
[-2B,3B). The original complement scheme differs by exactly B^5 in negative
lanes, so all five normalized digits agree modulo B^5. Preserve its original
strict<2p correction, especially the negative-zero result2p. Sign masks can
differ per lane. No overwide intermediate is passed to IFMA.

History boundary: conditional parallel normalization after CIOS was tested on
24October (parallel-normalize-component, prefix-controlled), and mask-prefix
CIOS normalization on29October/8November. All remain rejected. This transfers
that existing carry idea to a DIFFERENT input-operation consumer and extends
the proof to signed negative coefficients; it is not a new carry algorithm or
a retest of CIOS. Earlier uncarried-X-sum work eliminates a different producer/
consumer boundary and remains separate. No novelty claim or paper speedup.

Potential cost case, not emitted code or timing: remove the four dependent
signed-carry transfers in each coordinate difference, replacing them with
independent shifts/adds plus two ORs, one high-bit test and a conditional repair.
Normalize all outputs before any field multiply. Multiply count is unchanged;
additional predicates, branches, live values, spills and pathological repair
can outweigh shorter dependencies. No twofold projection. Old instruction
reductions already failed in complete callers, so static counts are insufficient.

Prospective mathematical tests: independently compare serial, parallel/repair
and BigInt represented integers, both cascade and fast paths, positive/negative
carry, radix8/256/2^52, mixed lanes and chains. Exhaust every five-tuple over
the declared boundary alphabet for the three radices; include constructed
signed cascades and deterministic random tuples. Test every finite sign mask
at canonical Fq/Fr digit boundaries and random pairs, deriving original
complement digits separately; check strict2p correction, negative zero and exact
raw output. Finite checks are implementation evidence, not the all-input proof.

Only a passing screen selects a private extension outside generated arithmetic:
new methods in an appended signed_delta child, with all176accepted original
files reversible and the original multiply untouched. New native functions,
exact original/Arkworks oracles, generic guards and actual linked opcodes must
be qualified before any timing. No unchanged native test/decoder replay. Any
new caller stage covers BOTHcurves/all200/201/352/353/1/16/32 and fixedcontrols/
resources/calibration/nonregression/gain/automaticprofiles with independent
prospective review. Only fullcaller benefit selects realSDK ALL8+54/primary16/
Gnark/generator-or-extensionconsistency/architecture/portable/resources/concurrency.

No heap/table/keycopy/pointcopy/retainedplan/newproductionpool/artificialRAMcap,
Rust1.98.1/serializedxtask, noquestions/subagents/budget. DuringANYrecording
inclpreflight ONLYPOLLconfirmedlivehandle. CommitANDpush completed experiments,
including negative. Canonical37023 ledger MUSTseedfutureexports from
tree-policy-sdk176/proof-sha256-after-capture.json. No new proofs/timings yet.

Research this turn: current complete proof profile still lists G1/G2 MSM and
inversion prominently; samples are not removable wall shares. Prior GLV/KH,
epoch pin, inverse broker/arrivals, phased inverse, endpoint, float quotient,
lookup and 32-bit divider work was checked and not replayed. The current FFT
already saves C's forward transform; that proposed saving is not new.
Primary SQIsign arXiv2608.13948v1 was read for representation/dependency ideas:
https://arxiv.org/html/2608.13948v1 . It uses another prime and protocol; none
of its numerical gains or hardware forecasts is transferred to BN254.

Latest accepted local32 ms ALL8:8.040649245/11.149096784/16.069415294/24.131177863/
33.940627784/40.164083059/66.187704392/65.361571353(lastmerge24x1),54 160.890372137.
M5CPU37/59/109/189/284/335/528/507;Metal24/30/48/74/101/124/176/175;
CPU10xtarget3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7. No54M5reference. Same accepted
historical7.915095valid; current~8ms isn't adoptedregression. Goal remains0/8.
