# Native core compilation repair, before any SDK capture

The native field suite passes41 tests, including both new arithmetic/raw-Y
tests. Native core compilation then exits101: the copied kernel's concrete
reference type no longer brings Borrow into scope, and one inherited diagnostic
counter compares typed LazyFq directly to canonical Fq. No core test executes.

Preserve the initial source copies, plan, pins and failed compile receipt.
Replace only three redundant reference.borrow() calls with direct dereferences,
and canonicalize Y in the diagnostic equal-X counter as in the actual Window
guard. The field source/numerator/raw pack/scale arithmetic are unchanged, so
the successful41-field suite is inherited without replay. New ON-v2 plan runs
only remaining core tests/libraries/link. OFF/generic bind the same repaired
source through new v2 pins. Screen preparation is updated before compilation
or freezing/capture to review the composite build receipts explicitly.
