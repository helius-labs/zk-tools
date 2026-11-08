# Signed linear-coordinate carry normalization

The new canonical-difference methods pass mathematical and native qualification.
**No speedup has been measured and production is unchanged.** This selects
complete G1/G2 caller integration, not adoption. The experiment targets carry
dependencies in coordinate inputs; multiplication and Montgomery reduction
remain byte-exact originals.

For B=2^52, split each signed independent coefficient into a low digit and a
floor carry. Transfer those carries to the next digit simultaneously. If
digits1–3 are all in[0,B), the result is already normalized; otherwise three
signed ripple steps repair every cascade. A single all-lane high-bit test
detects both negative digits and digits at least B. Top-digit masking retains
the exact serial result modulo B^5. Actual canonical differences lie in[0,2p],
so the represented nonnegative value is preserved too.

Canonical subtraction uses p+a-b. Signed dy forms p+by-ay or2p-by-ay directly,
using a masked 64-bit negation in negative lanes. This avoids the original
52-bit complement plus initial carry. The original strict<2p reducer remains,
including correction of negative zero2p to zero. Every output digit normalizes
before reaching unchanged IFMA. The [all-input argument](artifacts/design.md)
covers coefficients in[-2B,3B); finite checks below test its implementation.

The conditional-carry idea itself is inherited from the rejected
[CIOS final-normalization experiment](../2026-10-24-parallel-normalize-component/README.md).
This is a different input-operation consumer and a signed-domain extension,
not a new carry algorithm or a rerun of that multiplication candidate. The
[later CIOS mask-prefix trial](../2026-11-08-normalize-prefix176/README.md) and
[uncarried X-sum](../2026-11-08-linear-carry-fusion176/README.md) retain their
separate outcomes. No prior negative gate is rewritten.

## Mathematical and native evidence

The integer model exhausts14^5boundary tuples plus65,536random tuples at each
of radix8,256 and2^52: **1,810,080 tuples**. Serial output and independent
BigInt represented integers match, with both fast and repair cases and all
three cascading steps exercised. Forcing repair on otherwise-fast cases also
preserves output, covering the all-lane repair behavior. Canonical Fq/Fr and
two small-prime domains include all256mixed sign masks and negative zero.
The separate reader uses bounded unit adjustments instead of shifts/masks;
it checks10,368abstract repair states and all20,402signed canonical pairs over
F101. Its prime constants match the unchanged accepted FpConfig values.

The native snapshot has19field members:17byte-exact originals, one original
signed_delta parent with a reversible include suffix, and one new child.
Original generated ifma8, existing methods, scalar arithmetic, core routing,
key layout and all176production hashes remain unchanged. Generic no_asm/no_ifma
compilation passes with both new selectors present. No existing successful
field/core suite, model or decoder is replayed for bookkeeping.

|New native check|Count|
|---|---:|
|Exact raw coordinate-difference lanes versus original methods|258,592|
|Independent Arkworks export checks|258,592|
|Matching downstream unchanged-multiply outputs|258,592|
|Signed raw-normalization lanes|603,360|
|Raw-test vectors exercising repair|57,680|
|Actual-wrapper dependent, canonicalized-state lane outputs|24,576|

Subtraction covers both primes and R256/R260,41canonical edges including limb
boundaries, random pairs and unchanged multiply consumption. Signed dy covers
both primes at R256, all256sign masks, random inputs and128negative-zero outputs
per prime. The raw normalizer checks the full declared boundary alphabet and
random signed coefficients, including mixed lanes; its repair frequency is a
test construction, not a production workload estimate. Actual wrappers reuse
canonicalized outputs over512changing steps per operation/configuration.

## Actual generated code and limits

One symbol listing finds12operation names but8unique bodies: R256/R260
subtraction aliases within each prime. Each actual address is decoded once;
all names map to that evidence. The initial helper's unused decode-selection
stage is left unexecuted; the alias-aware inspector preserves already completed
builds/tests/listing. There is no failed scientific command or successful replay.

|Operation, same counts for Fq/Fr|Original sites|New sites|Original RSP/RBP operand sites|New RSP/RBP operand sites|
|---|---:|---:|---:|---:|
|Canonical subtraction, either radix|36|51|6|0|
|Signed canonical difference, R256|77|95|6|6|

All bodies have zero IFMA and call sites, as expected for linear operations.
The new subtraction's address prefix through its first return has41sites,
versus36original; this is not a dynamic fast-path count. Its repair block
follows that return and branches back. The actual common code contains
independent arithmetic shifts followed by the high-bit predicate; original
subtraction chains each carry into the next coefficient. The compiler also
materializes constants differently and removes standalone subtraction's stack
operands. These observations do not isolate cause, measure spills/stack bounds
or establish a cycle reduction inside an inlined point kernel.

The tradeoff is a shorter dependency chain with additional predicate/branch
work and more live values. Adversarial carries pay repair. Main-kernel register
allocation can outweigh the apparent standalone benefit; the recent quotient
trial already showed that instruction changes need complete-caller evidence.
There is no2x projection, timing result, RSS/minimum-RAM or mobile claim.

All17workflow commands pass first execution:2mathematical,4native qualification,
3actual-wrapper build/check/listing and8unique decodes. Receipts retain exact
argv/exit/streams; helper compiler/preparation/publication invocations have tool
transcript evidence rather than complete workflow compiler receipts. The model
was extended with mixed-mask coverage before its first execution. No arithmetic
source was repaired after a failed check.

No heap allocation, table, point/key copy, retained execution plan or production
pool is added. Temporary SIMD values and the repair branch need actual caller
resource checks. The [SQIsign paper](https://arxiv.org/html/2608.13948v1) was read
for representation/dependency ideas; its prime, protocol and reported gains
are not transferred here. Prior GLV/KH, scheduler pinning, inversion sharing,
phased inverse, endpoint and divider proposals were checked and not repeated.
The accepted FFT already avoids C's forward transform.

Next, select these exact methods through native FqBatch's canonical-sub and
signed-canonical-difference wrappers in a private coherent core. Leave G1's
signed scaled delta and all multiply/reduction paths original. Qualify complete
G1/G2 signs, tails, reuse, actual small-stack workers and final generated code.
Freeze both curves, all200/201/352/353sizes and1/16/32workers with both controls,
resource checks, calibration/nonregression/gain and automatic PMU/profiles before
any recording. Only credible complete-caller gain selects full SDK ALL8+54,
primary16, independent Gnark proofs, extension consistency, architecture,
portable, resource and concurrency checks.

## Accepted CPU and Metal comparison, inherited

These latest accepted local32-worker single-proof medians come from the
[full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md), not this study.
Supplied M5 references use different hardware.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10x target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.040649|37|24|4.602x|2.985x|3.7|
|2x2|11.149097|59|30|5.292x|2.691x|5.9|
|4x2|16.069415|109|48|6.783x|2.987x|10.9|
|8x2|24.131178|189|74|7.832x|3.067x|18.9|
|12x2|33.940628|284|101|8.368x|2.976x|28.4|
|16x2|40.164083|335|124|8.341x|3.087x|33.5|
|24x2|66.187704|528|176|7.977x|2.659x|52.8|
|merge24x1|65.361571|507|175|7.757x|2.677x|50.7|

Merge54x1 remains160.890372ms without an M5 reference. Historical same accepted
binary7.915095ms remains valid; later8.04ms is not an adopted regression. The
full verified single-proof CPUAVX10x goal remains active/unmet0of8. Production176,
accepted SDK6d83 and the canonical37,023-proof ledger remain unchanged. No new
SDK request, Gnark proof, AWS/Aeglos, giant-circuit, phone/ARM/TEE, swap/pressure
or concurrent-throughput result is added.

[Design](artifacts/design.md), [native scope](artifacts/native-design.md),
[new source](artifacts/parallel_delta.rs), [model](artifacts/result.json),
[separate mathematical review](artifacts/review.json), [native qualification/code](artifacts/qualification.json),
[source audit](artifacts/native-source-audit.json), [full source/receipts/decodes](artifacts/evidence.json.gz),
[member hashes](artifacts/archive-index.json), [publisher](artifacts/publish.rs),
[checksums](SHA256SUMS). Every archive-member hash and the gzip roundtrip are
checked. Binaries/libraries, unchanged source copies, proving material and
raw perf data are excluded.
