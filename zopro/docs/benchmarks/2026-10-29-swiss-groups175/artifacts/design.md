# Index-only scalar dictionary with grouped probing

Distinct experiment after the closed index-groups175 trial (afeeb00ea7).
That scalar-linear-probe dictionary regressed primary A/B by 3.09%/29.42%.
This implementation replaces its entire probe/storage mechanism with safe
hashbrown 0.16.1 HashTable<u32> entry/equality/rehash APIs. Each slot stores a
class index, with grouped control-byte probing. Full four-word equality and
rehashing read the existing owned values. This is not a rerun, fingerprint
width adjustment, compiler annotation or post-capture subset selection.

hashbrown is MIT OR Apache-2.0, version-pinned and compiled offline with
default features disabled and inline-more enabled. It is already present in
the repository lockfile but is not an existing direct core dependency.
The private standalone component dependency does not change production's
Cargo files. Both benchmark variants use unchanged frozen accepted field/core/
SDK libraries, native targeting, fat LTO and one codegen unit. No new unsafe
code is written; the third-party crate retains its own internal unsafe code.

The original constructor, real-input harness, three existing algorithm oracles
and two FIFO tests are reused from index-groups175. Existing memory-bound tests
are adapted from eight bytes per bucket to four bytes plus control bytes and
group padding. The full constructor source extraction, original sampling,
admission, first-occurrence class order, zeros and final cutoff stay unchanged.
Only the dictionary allocation/insertion hooks select the new implementation.
No point array, witness reuse, retained key table, plan, ISA or pool changes.

Fixed protocol: accepted raw-key and solver-image scalar solutions must agree
for all nine fixtures. Test A/B for all18 contexts, canonical words for first
two shapes and Montgomery words for others as in production entry geometry.
All samples and their subsequent accept/decline outcomes are retained. Main
thread CPU2; three primes per context/variant,31rotated/reversed blocks,
8constructor calls per cell, matching deterministic AHash seeds. Every prime
and timed output must equal an independent BTreeMap whole-output oracle.
Constructor clocks include all temporary dictionaries, growth, destruction and
successful output construction, excluding sampling/admission and output checks/
destruction. This is not end-to-end proving or concurrency qualification.

Gate: both primary A/B upper paired ratios<1 and at least21/31wins; all18 upper
ratios<=1.03; equal-log aggregate upper<1 and at least21/31wins. Sorted ratio
ranks10..22 nominal97.055%individual median coverage under independent blocks.
No gain floor, exclusion, extension, unchanged reroll or threshold adjustment.
This fixed candidate closes on any gate failure; individual positive cells do
not justify subset integration. Only a pass selects a later full-SDK trial.

All latency precedes two separate FIFO199Hz cycles:u profiles of primary B,
8192calls per variant after three primes, and post-profile output checks.
Profiles are not added to latency samples. Allcaptures precede loss auditing
and owner-user decoding. All measurement helper builds/pins precede captures;
no owned helper/build/hash/archive/other CPU workload overlaps capture.
Source/library/key/request guards close; no private scalars are persisted.

No SDK/Gnark proof is created here; ledger20881 unchanged. No production,
AWS/phone/TEE/giant-circuit/pressure/concurrency or minimum-RAM qualification.
Original all-eight verified CPU10x goal remains active/unmet0/8; accepted
CPU/Metal full-proof comparisons must remain in the completed report.
