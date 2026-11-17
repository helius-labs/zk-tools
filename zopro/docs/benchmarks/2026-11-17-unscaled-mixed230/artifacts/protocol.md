# Unscaled G1 mixed Jacobian addition230

The active goal remains all eight verified CPU AVX shapes <=supplied M5CPU/10,
with CPU/Metal comparisons; merge54 has no supplied M5 reference. Baseline is
accepted227 SDK b70a9472fa64ac6b6b41b227ca64ddfcf61c6443879c9187331eae6d5e22c153,
source186. No earlier negative experiment or gate is relabeled.

Hypothesis: stop choosing mixed-add formulas on M/S counts that ignore linear
work. Fq::square calls the same backend::multiply as Fq multiplication. The
current G1 mixed add is7M+4S with9add+3double+one quadruple; the standard HMV
formula is8M+3S with6add+one double. In this backend both total11 product calls;
the second uses eight fewer field additions/doublings. Source: EFD Jacobian
a=0, madd-2004-hmv, https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html .
No mathematical novelty claim or externally copied implementation. Derivation:
h=Bx*z²-X; r=By*z³-Y; hh=h²; hhh=h*hh; v=X*hh;
X'=r²-hhh-2v; Y'=r*(v-X')-Y*hhh; Z'=z*h.
Compared with old ordinary output, coordinates rescale by2:old=(4X',8Y',2Z').
The original identity/equal/opposite guards and double stay unchanged.

Apply only when Base has Fq's32-byte size; G2/Fq2 retains the old method because
its square and multiply have different costs. This is type-based, not a circuit,
worker-count or fixture selector. A private cfg zopro_unscaled_mixed_g1 gates
the prototype. No extra runtime option, table, key copy, heap allocation, pool,
state or memory policy. Only curve.rs plus a cfg(test) module declaration change;
one new test child. Removing those edits must recover all186 accepted files.

Current229 primary16 flat profile puts G1 add_affine at~3.13% of user cycles;
main SIMD affine streams bypass this scalar method. This is not an exclusive
critical-path/removable bound. A modest improvement is plausible; this cannot
credibly explain a twofold proof improvement alone. History review covers
negative unit-Z, XYZZ/persistent scaled coordinates, RNS/Toom, direct gather,
ternary and static queue designs. This formula changes ordinary scalar algebra,
not those guards, representations, codecs or queue algorithms. Do not repeat
those recipes unchanged. Paper2608.13948 concerns different isogeny primitives;
its SIMD speedups are not transferred here.2025/1068PDFreturned403; no claim
from unseen contents. Research/history review precedes this protocol's clock.

Native ON runs new independent Arkworks full-width/scaled/identity/sign and
changing-chain G1 oracles, the existing field suite and unfiltered core suite;
coherent OFF/ON libraries and exact accepted driver/support are linked. Generic
noASM/noIFMA field suite tests the new math too. Inherit unchanged field kernel,
generator and unrelated compatibility evidence; fuller integration/portable/
resource qualification remains necessary if promising. Do not repeat successful
tests to repair metadata. Pin compiler, source, dependency/link inputs and SDKs.

REF exactb70; OFF fresh source with cfg absent; ON cfg present. Three independent
executable backings per arm, all participate. Three4worker smokes,1timed+3prime,
then three matched blocks of1x2/16,1x2/32,2x2/32,merge54/32 ×three arms.
Primary251timed,2x2 51,merge54 7, each3primes. Rotate/reverse cases/arms as227.
39freshGnarkexports/5160SDKcalls/5043timedrecords. All capture before lateGnark;
no owned compiler/hash/analyzer/report/profile cojob during any SDK recording.
Actual default SDK worker stacks and accepted fixed worker/main affinity/optins.

Freeze before clocks. Both raw smoke readers must pass before guarded main.
Descriptive paired medians and individual3block ratios, no CI or adoption claim.
Useful signal: ON/control median<1 with>=2/3wins against both controls at either
primary count; no minimum gain floor. OFF/REF median factor1.03 calibration,
RSS ON<=each control*1.10+8MiB across all cases. Report CPU and all other latency/
memory effects, no extension/exclusion/relink/threshold/shape rescue. No signal
closes this exact composition locally. A promising exact build receives fresh
prospective all-eight+merge54 qualification and diagnostics before adoption.
Startcanonical61104 from229, SHA40c7d386963090fd69829225e51b35a7256ee7301348e4025957ca372bfa3e6c;
afterpreflight61107, aftermain61143. No AWS or push. Commit completed evidence
locally. Preparation30min, recording15min; record actual elapsed/overruns.
