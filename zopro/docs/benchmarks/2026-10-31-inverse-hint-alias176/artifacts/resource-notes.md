# Resource and qualification clarifications

The native operation size is72bytes at alignment4, matching the original hint
layout using the same accepted Step type. Hint grows inside existing Op slack
to56bytes. Initial hardcoded76byte guards were wrong: the compiler uses enum
layout niches. The original126pass/2fail execution and all source/receipts remain
preserved. The repaired snapshot changes only the layout guard/test and adds
the original-layout shadow types, with no runtime allocation for those types.

The first repair helper also assumed every source-manifest member was UTF-8;
binary fixtures invalidated that assumption before a new plan was emitted.
The original helper/partial copy are preserved locally. The second helper copies
bytes except for the edited Rust child. An attempted workflow invocation with
the absent intermediate plan exited before any test. Final qualification uses
the new complete snapshot and passes128core plus2prover+6cubic tests.

The snapshot's fourth counter is the discovery arrays' checked charge, including
two64byte allocation margins. It excludes the existing definite-assignment
check's temporary flags. That check separately reserves its own cost through
the existing resource policy while the array charge is held. No arrays have yet
been allocated during that check. Array capacity is verified against the charge
after fallible allocation. All discovery storage is dropped before scheduling;
the Boolean definite result remains in the existing OnceLock.

The candidate adds no retained allocation or larger Op/Plan field. This does not
establish equal process RSS, stack use, startup peak, or minimum RAM. Profiles
and RSS counters remain diagnostic; pressure/portable/all8shape qualification
is still required before any adoption. The screen exports204Gnark proofs but
does not export every SDK request's randomized proof.
