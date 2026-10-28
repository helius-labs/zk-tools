# Relation to earlier arithmetic trials

Read the local22Oct fused mul-sub README and retained Ifma8 source, and24Oct
canonical-Y README. Their SHA256 pins are in history-audit.json. The generic
idea of fusing multiplication/subtraction is old and already failed an SDK
performance condition; this experiment makes no broad novelty claim.

The22Oct implementation normalized delta=2p-c, injected delta*R before final
Montgomery cancellation, then normalized its biased output and reduced modulo
2p. Its original3percent performance gate stays failed. The24Oct routine
canonicalized final Y through a subtraction AFTER normalized multiplication.
Neither result is reinterpreted or rerun unchanged here.

The current new child copies the accepted product/cancellation/radix-packing
prefix without changes and replaces only the final normalization with the
existing signed subtraction and negative repair, consuming unnormalized REDC
coefficients directly. It introduces no delta normalization or pre-cancellation
injection. Its exact lazy output must match the original multiply/sub sequence.
Bounds and native independent checks are required before real-caller timing.

This narrower dataflow distinction justifies a caller test; it does not promise
that the compiler avoids spills or that fewer carry links improve proof time.
The caller includes inversion, packing and writes at actual200/352 batch sizes.
Do not substitute a standalone field-product gain for the complete caller or
for verified full-proof/all-eight performance.
