# Compact radix-index scalar grouping on accepted175

Goal remains verified CPU single-proof latency >=10x supplied M5 CPU for all
eight shapes, with both CPU and Metal comparisons. Accepted source175/SDK20fde
is 8.022165 ms /134 MiB primary; required primary latency is 3.7 ms, 0/8 achieved.
Previous goal turn completed and pushed the failed direct-finish SDK pilot,
901bfe191e; ledger18154. No source adoption or running job is inherited.

New algorithm: replace only the temporary grouping HashMap with four stable
byte-radix passes over packed (32-bit fingerprint,32-bit original index) u64s.
Actual equality always compares all four scalar words. Equal-hash runs with
different full keys use in-place full-key/index sorting; all-equal runs scan
linearly. Each exact class writes its first original index into the existing
classes buffer. A linear input-order pass changes roots to class IDs, preserving
original first-occurrence values and stable membership order without sorting
the class descriptors. Output values/counts/starts/order are identical to the
original representation; their growth/packing and ownership remain ordinary.

The current sample, zero handling,80/95 percent cutoff, process-wide reservation
and canonicalization remain. Before any new buffer allocation the checked new
workspace must fit the existing reservation forecast; otherwise use the exact
old map. A cardinality excess after exact run grouping returns the same None
as the original. No probabilistic decline or changed cutoff. No point copy,
key table, witness cache, duplicate solver plan, new dependency/unsafe code,
thread pool, runtime flag, ISA change or grouping parallelism is introduced.
The native experimental cfg is zopro_radix_groups. Production remains untouched.

Private source: one original grouping.rs edit and one new radix.rs child,176
files,174 exact originals/reverse175 exact. All17 field source files remain
byte-identical; inherit33-field qualification rather than rerun it. Native
OFF/ON each run128 core tests (124 inherited +4 new),8 prover tests. Four new
tests cover stable radix conservation, exact independent ordered-map metadata
under full and partial hash collisions, admission-forecast bounds across
1/4/16/32 worker contexts and overflow, and compiled OFF/ON policy dispatch.
The dispatch counter is test-only and thread-local; it does not enter SDKs.
All helpers and exact cfg/library/code identities qualify before timing.

First complete SDK matrix: all9shapes at32workers accepted/OFF/ON preflight,
then15 fixed rotated/reversed blocks over all9contexts and3variants. Primary
has151 measured requests after3primes; other shapes31 after3primes. Loading
excluded, fresh solving/randomized proving/SDK verification/serialization timed.
All405 latency captures precede their405 independent Gnark checks. Every sample
and block retained. All9OFF/accepted rank intervals must lie within factor1.03.
All9ON/control upper nonregression ratios must be<=1.03 vsBOTH controls. Primary
and original-eight log-geometric aggregate must each have upper<1 and>=10/15
wins vsBOTH. Sorted ranks4..12/15 give nominal96.484375% individual conditional
coverage under independent exchangeable blocks, not familywise. No gain floor,
exclusions/extensions/successful reruns, threshold/width/hash/flag/layout sweep,
subset rescue or post-result admission change. Failing closes this exact matrix.

After all latency and verification,3separate FIFO four-event stat runs31req
and3separate199Hzcycles:u profiles251req,after3primes each. All6diagnosticcaptures
precede their verification and decoding. No owned build/helper/hash/archive or
other CPU work overlaps captures; prepare measurement helpers before capture.
Poll the exact confirmed live handle; observation timeout is not termination.

Expected totals20142SDK requests/438fresh unique Gnark exports/876proof-public
members;885workflowsteps,ledger18154->18592ONLYonactual verification. Nine fresh
32-worker latency rows include the8supplied M5CPU/Metal comparisons;54->1 lacks
a supplied reference. No16worker/portable/pressure/mobile/250/4999/minRAM/AWS/
Aeglos/concurrent-throughput qualification from this matrix. Passing permits
separate compatibility/resource/final-artifact qualification; no adoption or
goal completion follows from a matrix that remains below the10x targets.
Accepted full-performance-gate failure/physical16 engineering exception and
final-artifact true remain recorded. Commit/push completed outcomes including
negative results, dry factual report.
