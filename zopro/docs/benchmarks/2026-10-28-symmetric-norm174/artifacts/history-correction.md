# History correction

The pre-capture history audit missed the existing
[2026-10-24 G2 norm experiment](../../2026-10-24-g2-norm-squares/README.md).
It already implemented the same sum of symmetric squares, with a shared
Montgomery reduction and six live coefficients. Its source includes the same
exact cancellation-carry sequence and mixed-radix correction. The later
specialized Fq/R256 child module changes integration and test context, but
does not supply a new arithmetic mechanism.

The original design and captured helpers are preserved to keep their pins and
the measurement record valid. Their novelty implication is superseded by this
correction. The report's observation that the earlier G1-square trial left
norms unchanged was true but incomplete: a separate prior G2-norm trial existed.
This is an experiment-history audit failure, not a field-arithmetic or proof
verification failure.

The latest 9672 SDK calls,108 independent Gnark exports, all timings, the
43.803µs primary OFF saving, failed gate and non-adoption remain unchanged.
The earlier trial remains rejected under its own declared conditions. The
latest study must not be counted as evidence for a novel work-reduction idea
or used to justify another unchanged norm rerun.

No build, test, SDK request, profile, counter, decoding or verification was
repeated for this correction. Production and accepted174/SDK81033 are unchanged.
