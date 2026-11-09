# Bounded fixed-factor FFT qualification

The compact fixed-factor multiply now passes complete FFT and product/Euler
consumer qualification in a private CPU AVX build. This stage establishes
correctness and bounded optional storage; it adds no latency result or adoption.

The implementation substitutes only twiddle products in the last five quad
layers and within-quad stages of the existing stored FFT leaves. Higher stages,
packing, task boundaries, weights and pointwise products retain their existing
implementation. Four compact scalar factors expand inside each consuming
butterfly, so the measurement must include those loads and expansion costs.
The qualified arithmetic uses 76 IFMA and five VPMULLQ sites per multiply versus
100 IFMA in the ordinary routine; static counts do not predict elapsed time.

Each direction retains at most 31 layer quads plus pair/single factors. Maximum
factor payload across three directions is 31,680 bytes. The actual retained
heap charge is **32,016 bytes**: three boxes of 752 bytes plus three vectors of
31 × 320 bytes. Three optional pointers add 24 bytes to the plan. Admission uses
the existing resource-aware policy before allocation. The original arithmetic
runs if admission fails, and table release drops the charges. These counts
exclude allocator overhead and transient stack storage; RSS and minimum RAM
were not measured.

Independent Arkworks oracles checked 3,947,126 scalar outputs across retained
successful runs. The 45 complete-transform cases check each inverse and forward
separately, both packed halves, ordinary/twisted roots, root powers 1/3, cosets
5/7 and domains up to 262,144. Consumer tests include sparse, zero and padded
inputs, release/rebuild, direct small-domain controls, cache clearing and forced
denial. Route counters prove the new products execute. Fifty-three worker
visits cover 1/4/16/32-thread pools with 512 KiB stacks. A separate process with
the actual preparation budget set to zero passes without fixed products or
retained charges. This tests configured denial, not operating-system pressure.
The generic x86-64 guard compiles; ARM/mobile execution is untested.

Two failures are retained: the initial test harness lacked an Arkworks trait
import, and its sparse product oracle initially omitted the existing twisted
mode's n² output scale. Only the oracle changed; the candidate core was built
once and remained unchanged. The padded test passed before the scale repair
and was not repeated. The three failed tests were rerun. An offline reader also
needed its test-name comparison made order-independent; no scientific command
was repeated for that repair. There are seven successful command receipts and
two failed receipts, rather than a claimed clean first run.

The private snapshot contains 178 entries, with 172 originals byte-identical to
production. The four changed parents and two children are archived. The field
implementation is identical to the [qualified native parent](../2026-11-09-fixed-factor-native176/README.md).
The production tree and accepted SDK remain unchanged.

Next: remove diagnostics from coherent release OFF/ON builds, qualify their
actual consumers and inspect emitted leaf code. Then freeze a matched complete
FFT/consumer trial with automatic profiles before any speed selection. Adoption
requires complete verified proofs, all supplied shapes and merge-54 validation.
The [review](artifacts/review.json), [source changes](artifacts/source-changes.txt)
and [continuation notes](artifacts/next.md) give the exact scope.

The following comparisons are inherited accepted-build results on the local
9950X with 32 workers, not measurements of this candidate. The supplied M5 CPU
and Metal timings are reference measurements from another machine. The CPU
10× objective remains unmet for all eight shapes.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

No new timing, profile, verified proof, SDK request, Gnark export or AWS/Aeglos comparison. The canonical ledger remains 40,154. Goal active, unmet 0/8. No owned job or cloud resource remains live.

[Source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
