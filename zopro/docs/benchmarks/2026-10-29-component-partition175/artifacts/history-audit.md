# Scope against prior experiments

This keeps accepted two-component phi decomposition and changes the meaning of
the two existing per-window tasks. The 22 October GLS4 parallel trial changed
decomposition dimensions and ran2/4component jobs while retaining point parts;
it is rejected and supplies no transferred gain. The 24 October interleaved and
guarded phi feeders used a temporary transformed-X cache and one input traversal;
this candidate has no such cache and does not interleave into one mutable set.
The routed-round/bucket-band candidates changed order and routing across ranges;
this uses existing streaming feeders, including their collision queues.

The 29 October balanced-phi trial changed digit widths/top geometry; this retains
the exact accepted widths and top reduction. Its CPU-work reduction and failed
latency gate remain unchanged. The GLS4 ordinary/indexed/coalesced-top failures
also remain; no four-dimensional recipe is promoted. Existing weighted/reduction
kernels, scalar grouping, query readiness and native field code are unchanged.

This is a standard repartitioning experiment, not a novel mathematical MSM
algorithm or an assumed cure for the measured C7a scaling difference. Prospective
controls test it on Ryzen physical16/logical32 before any broader conclusion.
