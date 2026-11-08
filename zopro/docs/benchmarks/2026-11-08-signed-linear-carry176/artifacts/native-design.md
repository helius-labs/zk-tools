# Native selection after mathematical screen

result.json and review.json pass the first two-command mathematical workflow.
This declares a single native candidate before its source/build/execution.
Retain original multiply, canonical add, generic sub, signed scaled G1 delta,
fused negative complement and all other accepted methods. Only add two callable
extensions: parallel_sub_canonical for both radix tags, and
parallel_signed_difference_canonical for R256, in the existing native-gated
signed_delta module. Core routing and generated ifma8 stay untouched at this
stage. The new methods are not selected by any production caller yet.

Use one independent high/low transfer and a single all-lane high-bit predicate
over digits1..3, followed by three signed ripple repairs if necessary. Keep top
as s4+h3 until the final mask, avoiding a redundant top low-mask. For signed dy,
masked 64-bit subtraction forms -by directly in negative lanes; offset selection
remains p/2p and the original strict<2p final reducer remains unchanged. No
complement+initialcarry in this new method. All five output limbs normalize
before any multiplication. Adversarial cascades take exact repair; no frequency
assumption is used for correctness or a timing gate.

Build a private19-member field snapshot: original18 members with only a cfg
include suffix in signed_delta.rs, plus one new child. Reversing that suffix
recovers the original; other17members and all176production hashes exact.
No field/radix/key/ABI/table/heap/pool change. Raw constructor/normalizer/export
APIs only under a separate diagnostic cfg. Old methods remain exact references.
No original field/core suite or model is replayed for bookkeeping.

Native tests: signed raw-normalization tuples with positive and negative
cascades, mixed lanes/forced repair; canonical Fq/Fr raw-word boundaries/random
and dependent normalized-state inputs; exact original method digits; independent
Arkworks residue/host export; all256sign masks, negative zero and subsequent
unchanged multiply consumption. Test sub onR256/R260, signed onR256. Generic
no_asm/no_ifma compile with selectors present. New actual wrappers for both
original/new operations (four sub pairs, two signed pairs) must be checked and
decoded once each. Original references are newly linked consumer artifacts,
not old unchanged artifacts rebuilt. Record comparisons, IFMAs, branches and
actual complete-body counts without assuming a speedup or removable wall share.

Only passing native/cost qualification selects new coherent G1/G2 caller
integration through FqBatch wrappers. The accepted source hashes and production
selection remain unchanged. Full goal/old negatives/resources/recording rules
from design.md remain in force. This is not a whole-proof performance result.
