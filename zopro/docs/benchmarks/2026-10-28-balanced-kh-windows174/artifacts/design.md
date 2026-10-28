# Balanced fused K/H windows: primary SDK screen

Declared before builds/captures. The existing signed width13 fused K/H computes
20windows:19full13-bit windows and a7-bit positive top handled by two indexed
reductions. Cover the same255bits with20windows of widths12/13, distributing
boundaries by floor(255*i/20):five12-bit and fifteen13-bit windows. The final
13-bit span covers bits242..254, so canonical Fr inputs have a nonnegative
top digit<=4096. A general offset Booth reader telescopes to the exact input.
Use the existing borrowed pending/collision Window and current200/352-entry
batch sizes, followed by the unchanged weighted tree. Horner shifts use each
span's width. No digit vector, point/key table, copied key, execution plan or
worker pool is added. The only new query-local geometry is20stack spans.

The balanced full-window capacity is71680 buckets. The original19full windows
have77824, plus two independent128-bucket top reductions. This capacity count
is not a measured addition/inversion/cycle saving: five smaller batches can
cost more inversions, bucket occupancies change, and streaming the top may
cost more than indexed reduction. Mathematical radix span drops260->255, with
the initial identity doublings irrelevant to the represented scalar.

Apply only at existing borrowed fused bits==13 entry, preserving its native
dispatch, input length/normalization guards and unsupported fallback. Other
fused widths, ordinary A, G2, field arithmetic, solver and scheduler retain
their paths. Existing workspace forecast already charges20full4096-bucket
parents, max352-entry leaves, and both top-index parents in addition. Candidate
uses no larger parent/leaf or extra heap geometry. Existing charge remains
conservative; no hardRSS/minimumRAM/resource qualification is inferred.

Validate exact320-bit integer digit reconstructions with an independent
bit-loop reader, complete weighted-generator sums under1/4workers, tails,
identity/equal/opposite/zero/one/minus-one/random/boundary inputs and preserved
width12/14 alternatives. Both native core suites and8prover tests must pass.
Field source exact; inherit33field-test qualification, do not rerun that suite.

Use fixed primary32 SDK protocol: accepted/freshOFF/ON,15rotated/reversed blocks,
151measured complete requests after3primes,CPUs0–31/32workers.3preflight exports
independently Gnark before45latency processes; all45latency exports verify after
ALLlatency. Then3FIFO four-event counters31requests and3separate199Hzprofiles
251requests,after3primes,all6captures before theirGnark/decode. No ownedbuild,
helper,hash,archive,verifier or second workload overlaps captures except plan
verification stages. ONpreflight alone enables geometry trace: exactly4route
records inclprimes. Latency/diagnostic processes have no trace flag.

Gate: ON/OFF andON/accepted upper paired ratio<1 and>=10/15wins;OFF/accepted
interval within[1/1.03,1.03]. Sorted ranks4..12of15,96.484375% nominal coverage
per comparison, no familywise guarantee. No minimum gain floor, exclusion,
sample extension, changed threshold or successful repeat/unchanged reroll.
Primaryfailure closes implementation; pass permits separately declared larger,
physical16/all8/portable/resource qualification, not automatic adoption.
117steps/7806SDK/54freshGnark/108members;prospectiveledger15865->15919.
Original all8CPU10x objective, CPU+Metal references and acceptance scope remain.
