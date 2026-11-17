# Fixed chunk pair-cache count screen231

Goal remains verified CPU AVX latency <= supplied M5 CPU/10 for all eight
shapes, with CPU and Metal comparisons. Accepted227/source186 is unchanged;
canonical inventory61143 remains unchanged. No production candidate is created.
Previous goal turn PROGRESS:230 report/negative evidence committed91a6793ca9.

Source: Grigaitis, Optimizing Multi-Scalar Multiplication Over Fixed Bases,
Preprints202604.0045v1, posted2026-04-02, sections3.1/4.1. Read primary HTML.
It caches all pair sums in chunks of a BGMW shifted-point array; experiments
target small MSMs,2..64inputs. Do not transfer its reported speedups to our20k+
queries. No external implementation copied. This local signed generalization
uses the already closed183/188 all-window prepared geometry: width12,22windows,
point-major shifted order. All shifts are computed modulo the curve by a future
builder; the model uses exact integer weights. Unlike current Phi2/balanced
streaming, it has one unsigned-magnitude global bucket set after sign routing.

Measure exact logical additions that a complete immutable pair cache could
remove from this prepared fill, and retained raw point bytes required. For each
original nonidentity point, retain22shifted points. In each fixed chunk of
2,4,8,16,32,64,128,256 consecutive shifted entries, cache every unordered pair.
Plus-only needs one extra point per pair and matches equal SIGNED nonzero digits;
plus/minus needs two points per pair and matches equal absolute nonzero digits.
Arbitrary signs negate the selected precomputed sum/difference as required.
Pair within each chunk/bucket, use each shifted entry at most once; saving is
floor(k/2). No cross-chunk pairing, cache pruning, deduplication, cancellation,
data-dependent reorder, cached witness or specialized circuit assumed.

All key-nonidentity entries participate in table sizing even when the captured
witness digit is zero; scalar-dependent table pruning would not be universal.
Known fixed identities can be omitted. Base shifted payload is22*n*pointbytes,
including identity slots as in183/188. Additional bytes use each chunk's actual
nonidentity count:choose(k,2)*pointbytes*(1or2). Count but NEVER allocate caches.
Counts exclude headers/indexes/setup/intermediate/reducer scratch; not peakRAM,
minimumRAM, a fixed architectural cap or a measured latency prediction.

Use exact retained1x2/2x2 A and B2 fixtures/pins/oracles from lfg-tiled-count176.
Do not reexport/solve/regenerate. First verify hashes. Both2x2queries already
admit original scalar grouping: their raw rows are hypothetical, not a bypass
proposal. Current G1 balanced255/20 and G2 Phi2 component/query algorithms are
NOT replaced or equated to this22-window model. Do not price projective/affine
adds as interchangeable cycles or reuse old fill counts as current227 counts.

Fresh two tests: (1) exact signed digit reconstruction for zero, boundary,
changing full-width four-word integers and every bit; (2) exhaustive small
signed populations/identity masks, checking explicit disjoint paired formal
coefficients and count-vs-independent matching. Main checks whole-query integer
dot products modulo65537and1000000007 after routing, plus every cached-pair
output by independent algebra. A separate reader independently extracts bits,
recodes using carry, finds pairs by sorted runs, computes payload from chunks,
and compares all64rows, every digest/receipt/source/ledger pin. No native field,
curve, SDK or Gnark execution; no correctness claim beyond this group/count model.

No timing gate or arbitrary minimum gain floor. This is a cheap structural
screen before considering any memory-heavy native implementation. Retain all
table-size/savings rows, including multiGiB theoretical payloads; these are
calculations, never architecture proposals. No native trial is selected merely
because savings are positive: the complete caller must improve over current
streaming and preserve grouping, and183/188/197 negatives stay closed. A future
sparse/deduplicated cache would be a distinct mechanism requiring new evidence.
No table/width/order/host rescue after counting. All8 CPU10x remains active.

Prepare both independent implementations and fixed xtask plan/pins before model
execution. Run tests then main then independent review with checked dependencies.
Capture allowance5min, preparation30min; record actuals via xtask receipts and
filesystem timestamps. Commit complete evidence locally; no push/AWS/subagents.
