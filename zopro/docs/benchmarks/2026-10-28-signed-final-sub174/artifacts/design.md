# Signed final-carry subtraction caller experiment

Declared before captures. Source174 private derivative. Fold subtraction into
the final carry normalization after unchanged CIOS/REDC and radix packing.
Read bounds.md and history-audit.md; the generic fusion family already has
failed experiments. This tests a specifically different dataflow, with no
claim that fusion itself is new.

Integrate only the two native8 G1 streaming output expressions (x and y).
Original expressions remain in an inline wrapper for OFF and other backends.
Field method lives in a new cfg-only child; original product body is untouched.
G2, weighted trees, solver, keys, scheduler, forecasts and allocation source
remain unchanged. No new heap scratch, per-key table or retained plan.

Fresh OFF and ON have coherent frozen library sets and the same diagnostic
caller adapter. Reuse the prior modeled real-batch-size adapter byte-for-byte:
G1 owned unsigned, G1 borrowed signed and G2 prefix at200 and352 entries.
Production stream routing, packing, inverse, tails and writes are included.
Destination permutation and points are modeled, not recorded key/witness data.

Native unfiltered field/core suites for both builds. ON adds two field tests
for exact coefficient folding, lazy bounds and independent Arkworks outputs.
The existing caller's three tests cover13 lengths, tails, signs, resets and
multiple repeated calls. Field oracle checks both fields and both radices;
this is an8-lane experiment, with no claim of a4-lane change or qualification.

31 rotated/reversed process blocks, six cases and two variants, CPU2 and one
Rayon worker;372 latency processes plus12 correctness preflights. Each process
has3x8 checked primes, then4096 timed calls (200) or2048 (352). Final outputs
are checked and must match across variants. Count timed allocations, require
zero and identical endpoint scratch capacities. Profiles come AFTER all timing:
four whole-process cycles:u/199Hz recordings, G1 borrowed signed at200/352,
both variants,1048576 calls each. Include setup/checks in profile scope.

Per-case gain: paired ON/OFF upper ratio (ranks10..22 of31) below1, at least21
wins. No minimum gain floor. Integrate into SDK only if ALL4 G1 cases pass;
G2 negative controls must have paired medians within3% of parity. Preserve
every observation. No outcome-driven threshold sweep or unchanged reroll.
Any SDK integration needs its own whole-proof controls/verification/admission
checks; this component cannot establish proof speed or all-eight10x success.

No owned build/helper/hash/archive/second benchmark during capture. Pin source,
libraries, tools, driver and protocol before and after. Preserve exact captured
bytes. Raw profiles and hexadecimal dumps remain local with hashes; publish
decoded profiles and analyzed loss checks. Commit and push completed evidence.
