# Native late-centered qualification, declared before native builds/execution

Previous goal turn PROGRESS: exact signed-affine and late-compensation models
passed/reviewed/reported/pushed5a3df9e1f6. This stage implements that fixed recipe;
it does not rerun those successful models or their original oracles. Full goal
remains verified CPUAVX single-proof >=10x suppliedM5CPU ALL8 plusCPU/Metal.
Production176/SDK6d83/canonical35171 stay unchanged; no performance clocks.

Private field-source copy, one append-only ifma8.rs module declaration and one
child. Existing field functions/generator bytes remain exact. Use separate Signed
and Loose types; low4normalized52-bit digits and signed top for Signed, bounded
unnormalized linear coefficients for Loose. Raw construction is unsafe and only
qualifier fixtures with independently checked bounds use it. No signed values
escape into Fp/Ifma8/get/compact. Public legacy contracts are not widened.

Unsigned six-coefficient CIOS uses the qualified canceled-low-product schedule.
Encode signed operands modulo2^260, capture input signs, retain final quotient
sign, compensate input signs at the end, subtractp for centeredquotient, normalize
low4digits and sign-extend the top52-bit digit. Source100/150IFMA recipe remains;
actual codegen is inspected separately. Difference2 pays a third-input negation
and normalization. G1/G2 actual callers use R256; R260 tests are a cross-check.

Import uses acceptedIfma8::new. Rootboundary normalizes signedvalue< p by one
conditional addp, then uses the exact original five-vector-product/scalarproduct/
scalarinverse bridge. Normalize the bridge's final lazy inverse belowp in vector
form before returningSigned. Finalexport for |value|<4p adds4p, normalizes,
uses the existing one-digit quotient estimate on[0,8p), and canonically reduces.
R256 uses the original direct transpose/store block on canonical digits; R260
uses its original radix-converting get after canonicalization. Extra R260 export
correction is included in that cross-check, not hidden as R256 cost.

New diagnostic complete G1/G2 streaming callers preserve the original group
geometry, interleaved8chains, scalar tails, signed arrivals and G2 coordinate
rereads. G1 records retain five320-byte fields (xsum is Loose); G2 records retain
two320-byte fields. Vector-inverse conversion/canonicalization is part of these
callers. A diagnostic checked adapter detects identity/equalX/duplicate destinations
BEFORE publication and uses unchanged scalar curve addition as whole-call fallback.
That adapter is not a new production Window/dispatcher or speed-qualified guard.

New focused native tests only: bothprimes/radices/ordinary/fusedsum/difference,
signedboundaries through5p and deterministic full-width tuples/chains; exact
signed output against independent BigInt fullradixREDC and independentArkworks
meaning; wideexports strictlywithin4p and rootinverse boundary including aliases;
completeG1/G2 full/tails/signed/reuse/fallback/keypreservation; zero warm-allocation
and fixed scratchcapacities on ordinary paths; sharedfixture1/4/16/32workers on
512KiBstacks. Existing field/core/prover suites remain inherited, not replayed.
Population is fixed in native-tests.rs before execution, no outcome-basedextension.

Pin176productionhashes/newsource/compiler/dependencies/plans. Builds use -O,
target-cpu=native,fatLTO,oneCGU and inherited fastinverse/canceledcarry cfgs. Generic
field compile must keep the new child native-gated. No core/SDK build is claimed.
Run newqualification once through serializedxtask. Preserve failures and repair
onlyfailed/unstartedsteps. No width/radix/layout/compiler/worker rescue after
futuremeasurement. Actual linkedloop decodes ONCE after successfulqualification.

Pass selects a separately frozen matched completecaller performance/profile/
resource experiment only. No timings, newSDK/proofs/Gnark, minimumRAM/pressure/
mobile/AWS/Aeglos or goalcompletion claim follows. No production keycopy/table/
input-sizedcache/solverplan/pool/CASMrewrite. Publish andcommitANDpush finished
stage positiveornegative. Keep all8CPU10x goal active unless fullyverified.
