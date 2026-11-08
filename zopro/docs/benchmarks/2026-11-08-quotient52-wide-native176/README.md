# Overwide radix-52 quotient: native qualification

The new field variant passes exact raw-output and independent arithmetic checks.
All twelve decoded native operations remove four quotient AND instructions for
R256 or five for R260 compared with the masked parent. **No latency improvement
has been measured.** Production and the accepted prover remain unchanged.

The [previous masked variant](../2026-11-08-quotient-mullo-callers176/README.md)
slowed complete G1/G2 callers and remains rejected. Its generated code added
quotient masks after replacing IFMA with VPMULLQ. This variant keeps the normal
52-bit quotient as a full 64-bit product: every actual consumer discards its top
12 bits. This changes the representation at those consumers without changing
any accumulator or output digit. It is a distinct implementation, not a rerun
or reinterpretation of the previous negative result.

For any 64-bit x and v, the low 52 bits of their wrapping product equal the
low 52 bits of the product of their low 52 bits. Both IFMA halves truncate
each multiplicand before multiplication. The equality therefore applies to
low and high products, either operand position, and arbitrary accumulator wrap.
The source audit enumerates every use of the normal-row quotient; none reads
its upper bits. Induction through unchanged carries and row shifts proves exact
raw-state equality. The final mixed-radix 48-bit mask stays: its unshifted IFMA
consumers can observe four additional bits. A native counterexample confirms
that dropping that mask would change results.

The semantics agree with the primary Rust documentation for the
[512-bit low](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_madd52lo_epu64.html)
and [high](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_madd52hi_epu64.html)
intrinsics. [Intel 319433-049](https://cdrdv2-public.intel.com/782879/architecture-instruction-set-extensions-programming-reference.pdf),
pp. 2-47/2-48, supplies matching per-lane operand-truncation formulas for VEX.
The all-input equality above is a derivation from those semantics; finite tests
are implementation evidence, not a replacement for that argument.

Four new Rust 1.98.1 / serialized xtask commands build the changed library,
link/run its native tests, and compile generic no_asm/no_ifma gating with all
three selectors present. Existing original/OFF and masked-parent libraries
are reused without rebuilding. The private field snapshot has 18 members;
17 are byte-exact originals, and one expression reverses to the qualified
parent. Reversing its prior audited changes recovers the accepted file. All
176 production source hashes remain exact. The original CIOS reference and
scalar arithmetic are unchanged.

|New check|Count|
|---|---:|
|Exact raw output lanes against original CIOS|554,496|
|Canonical exports checked with independent Arkworks arithmetic|554,496|
|Full-width quotient-consumer lanes|164,280|
|Consumer assertions, including both IFMA halves and operand positions|1,478,520|
|Observed accumulator wraps in those consumer cases|49|
|Native mixed-48 mask counterexamples|8|

Field tests cover Fq/Fr, R256/R260, multiplication/fused sum/difference, 52
boundaries including 2p, random inputs and 1,024 dependent rows per panel.
Both tests pass on their first execution. One preparation audit initially
counted a comment containing `m` as a use and stopped before copying source
or building. The scanner was corrected to skip comment lines; the initial
helper is retained. No arithmetic or successful native test was changed.

Three more commands link/check twelve actual variable-input O3/native/fatLTO
wrappers and list their symbols once. Twelve commands decode each actual
function once. Every operation has five VPMULLQs, 95 IFMAs for multiplication
or 145 for fused operations, and zero calls. A separate same-agent token parser
reads both new and saved parent disassemblies and confirms the mask reductions.

|Radix (same counts for Fq and Fr)|Operation|Masked parent ANDs|New ANDs|
|---|---|---:|---:|
|R256|multiply|13|9|
|R256|fused sum|18|14|
|R256|fused difference|23|19|
|R260|multiply|9|4|
|R260|fused sum|9|4|
|R260|fused difference|14|9|

These are static instruction sites, not executed counts, spill time or proof
latency. Total multiply count remains unchanged. There is no new table,
allocation, key/point copy, retained plan or production pool. Compiler/test
memory and static stack operands are not prover RSS or minimum-RAM evidence.
Generic compilation does not establish performance on other architectures;
target policy and generated-source integration remain pending.

The separate publication review checks all 19 successful command receipts and
stream hashes, all three pin populations, reversible source, library identities,
unchanged flags plus the new selector, reused oracle source, and actual opcodes.
Helper compilation/preparation/publication have tool-transcript evidence rather
than complete workflow receipts. The tests, builds and decoders were not replayed.

The next stage is new coherent complete G1/G2 caller integration, then a fixed
prospective comparison covering both curves, all 200/201/352/353 sizes and
1/16/32 workers, original frozen/fresh OFF/ON controls, resources, calibration,
nonregression/gain and automatic PMU/profiles. Only complete-caller benefit
selects a full SDK trial. A field-level instruction reduction is insufficient
for adoption or a whole-proof speed claim.

Latest accepted local 32-worker single-proof medians below are **inherited**
from the [full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md).
Supplied M5 timings use different hardware. The same accepted executable
previously measured 7.915095 ms for 1x2; the latest 8.040649 ms observation does
not establish an adopted regression.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10x target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.040649|37|24|4.602x|2.985x|3.7|
|2x2|11.149097|59|30|5.292x|2.691x|5.9|
|4x2|16.069415|109|48|6.783x|2.987x|10.9|
|8x2|24.131178|189|74|7.832x|3.067x|18.9|
|12x2|33.940628|284|101|8.368x|2.976x|28.4|
|16x2|40.164083|335|124|8.341x|3.087x|33.5|
|24x2|66.187704|528|176|7.977x|2.659x|52.8|
|merge24x1|65.361571|507|175|7.757x|2.677x|50.7|

Merge54x1 remains 160.890372 ms with no supplied M5 reference. The full verified
single-proof CPU 10x objective remains active and unmet on all eight shapes,
0/8. Accepted source 176, SDK 6d83 and the canonical 37,023-proof ledger are
unchanged. No new proof, SDK timing, cloud resource, AWS/Aeglos, huge-circuit,
mobile, memory-pressure or concurrent-proof measurement is claimed.

[Design and proof](artifacts/design.md), [qualification and code counts](artifacts/qualification.json),
[source audit](artifacts/source-audit.json), [separate review](artifacts/publication-review.json),
[source/receipts/decodes](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json),
[publisher](artifacts/publish.rs), [checksums](SHA256SUMS).
All archive-member hashes and the gzip roundtrip are checked. Executables,
libraries, unchanged source-tree copies, keys, private fixtures, witnesses,
proofs and raw perf data are excluded.
