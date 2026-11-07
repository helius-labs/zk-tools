# Uniform scaled prefixes: native qualification and consumer screen

The construction passes native correctness checks but **fails the prospective consumer screen**: G1 has fewer packing shifts and slightly more total static instructions. G2 has fewer instructions and a larger frame. The exact construction closes without adoption or a full-proof performance pilot. **Whether it changes proving latency is unmeasured.**

## Implementation and qualification

The [mathematical report](../2026-11-07-uniform-prefix176/README.md) derives uniform scaled multiplication throughout G1/G2 prefixes with one inverse-bridge correction. The private implementation changes G1 streaming and owned/borrowed G2 prefix paths behind a compile-time flag. Final coordinate arithmetic, scalar tails and ordinary inverse-tree multiplies retain their original formulas. Partial batches receive the correction after tail reversal; complete batches fold it into the scalar inverse root. Generic inverse and other MSM paths stay unchanged.

The raw multiply wrapper uses the existing R260 kernel directly on R256 digits. Scaled sum/difference wrappers are inherited byte-for-byte from the preceding experiment. The signed wrapper preserves the existing bias/difference code and changes only its final multiply. Four-bit scalar-root cancellation uses the existing inverse. No lookup table, point codec, key copy, new pool, additional scratch buffer or record layout is added. This does not establish mobile, ARM, minimum-RAM or memory-pressure behavior.

Three fresh field tests pass: **500,256 native output lanes** across both primes and four/eight-lane SIMD, plus **8,206 scalar-root cases**. The population covers raw inclusive 0..2p edges, radix neighbors, random operands, changing chains and every sign mask. Returned digits/ranges and logical /16 results are checked against Arkworks. The unchanged official eight-lane generator runs once and passes its check.

Unfiltered core suites pass **127 OFF and 127 ON tests**, comprising all 125 accepted tests and two new tests. New coverage includes signed G1 and borrowed G2 across mixed tails, both inverse modes, canonical modes and repeated scratch reuse; G1 also covers both record-write modes. Existing tests cover all sign masks and 1/4/16/32-worker pools with 512 KiB stacks. These core curve oracles share accepted field/curve arithmetic; the native field oracle is independent. Two prover-library and six cubic tests pass on ON.

Final core source has 179 files: 174 exact field-qualified parents, four reversible core parents and one new child. The field snapshot has 178 files: 173 exact accepted parents, three changed field parents and two children. The source reader verifies these relations, the unchanged generic inverse and the scalar-root-only difference in the new inverse tree. All 176 accepted production hashes remain exact. Production and the accepted SDK are unchanged.

## Actual optimized consumers

Coherent eight-library OFF/ON SDKs link with O/native/fat LTO/one codegen unit. Saved symbol tables select the actual signed G1 and FAST borrowed G2 monomorphizations; each consumer is decoded once. The prospective screen requires fewer total static instructions and mixed-packing shifts in **both** consumers, unchanged IFMA sites and no new arithmetic-wrapper calls.

|Consumer|Instructions OFF → ON|IFMA sites OFF → ON|Left-4 / right-48 shifts OFF → ON|Frame reservation OFF → ON, bytes|
|---|---:|---:|---:|---:|
|Signed G1|4,428 → 4,458|800 → 800|48 / 40 → 12 / 10|4,288 → 4,288|
|Borrowed G2|8,591 → 8,541|1,450 → 1,450|72 / 60 → 24 / 20|5,952 → 6,720|

No calls to the new arithmetic wrappers remain. G1 static ZMM stack operand/store rows change 151/78→149/77; G2 changes 332/156→344/168. The stack parser retains raw prologues and handles probe loops; these four consumers have no probe-loop reservation. Frames exclude alignment, pushes and callees. Static operands are not dynamic spills, and individual frames are not a whole-program stack bound.

G1 instructions increase 0.68%; G2 instructions decrease 0.58%. These are code-size/count changes, **not measured speed changes**. Packing shifts fall substantially, but multiplication counts stay unchanged. G1 fails the declared instruction criterion, so the combined screen fails. G2's passing sub-result does not promote a post-result subset. No SDK timing, counter/profile panel, fresh Gnark preflight or worker/compiler/layout rescue follows this failed screen.

The final reader checks 538 input-pin occurrences, 12 SDK/symbol-table/decode receipts, eight libraries per SDK, actual optimization flags, binary/input hashes, test summaries and retained assembly counts/gates. It reuses saved decodes; it is not a second disassembler, field/curve oracle or formal proof. The earlier native reader checks 357 snapshot pins and five generator/field/core receipts.

## Preparation repairs

The first field build fails before tests because the new test macro conflates the SIMD word type with its field-wrapper type. A separate snapshot changes only the macro type parameter and references; arithmetic and generated kernels remain exact. The corrected build runs the three new tests once. Original failed sources and receipts are preserved.

The first core preparation stops before any core build when a formatting space prevents substitution of a cfg placeholder. The corrected preparer changes the placeholder match and output snapshot path. Its native suites run once per arm. A codegen-helper workflow launch is rejected while the SDK build holds the host lock; no command runs in that attempt. The helper builds after the SDK completes. Successful models, field/core tests, proof verifiers, captures and decoders are not replayed for bookkeeping.

## Accepted comparisons, unchanged

Inherited local 32-worker complete-SDK observations and supplied M5 references follow. None measures this construction. All eight CPU 10× targets remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.915095|37 / 4.675×|24 / 3.032×|3.7|
|2x2|11.169812|59 / 5.282×|30 / 2.686×|5.9|
|4x2|16.085882|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.517520|189 / 7.709×|74 / 3.018×|18.9|
|12x2|33.838626|284 / 8.393×|101 / 2.985×|28.4|
|16x2|40.083672|335 / 8.358×|124 / 3.094×|33.5|
|24x2|65.506198|528 / 8.060×|176 / 2.687×|52.8|
|merge-24x1|65.200982|507 / 7.776×|175 / 2.684×|50.7|

Merge54 remains 161.718956 ms without a supplied M5 reference. No new AWS/Aeglos comparison is added. Canonical independently verified Gnark-export count stays 31,699. Goal remains active and unmet, 0/8.

[Artifacts](artifacts/) retain changed source versions, accepted-parent hashes, helpers, designs, plans, analyses, disassembly and raw text/JSON receipts, compared with originals. Unchanged parents are available in the repository and identified by hashes; compiled shaders/static data are omitted. Keys, requests, witnesses, actual proof/public-input payloads, CPU executables/libraries and raw perf data remain private.
