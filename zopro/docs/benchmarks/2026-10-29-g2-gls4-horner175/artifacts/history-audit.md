# Distinction and provenance

Compared current g2_phi.rs, g2_borrowed.rs, workspace.rs and the archived
22Oct g2_gls4.rs. Historical four-component code makes component_buckets four
times, normalizes each, applies its own map, then merges three times. The new
Horner recurrence shares the current borrowed Window across all four passes,
preserving its existing affine/projective exception handling and kernel.
It does not repeat that old bucket-plane architecture or parameter sweep.

Only the previous local lattice constants/quotient/split/digit bodies are reused.
Their original GnarkCrypto/GLV source attribution and exact arithmetic bound are
retained via the historical source and integer-oracle receipt. The old100767
integer cases are inherited evidence, not newly executed. New tests recheck
reconstruction, signed digits and subgroup maps before any performance claim.

Fresh search rediscovered Le/Sica2026/1902, already tried in the22Oct hexagonal
experiments; those remain rejected. Expanded bucket representation, direct point
packing, Gauss products, RNS, worker fencing and previous joint-query task graphs
were also audited and remain closed. No external implementation is ported and
no paper or hardware speedup is transferred to this CPU design.
