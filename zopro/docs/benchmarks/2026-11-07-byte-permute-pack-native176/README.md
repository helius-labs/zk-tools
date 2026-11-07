# Eight-lane byte-permutation packing: native qualification

The new constructor passes bit-mapping, field, core and linked-SDK checks. **Performance remains unmeasured; nothing is adopted.** Actual point consumers have more static instructions and stack operands, so a source operation-count saving is not a speed claim. The first complete-SDK trial is selected under a prospectively declared protocol.

## Construction

The accepted constructor transposes eight 256-bit field words into four limb columns, then joins shifted pieces into five radix-52 vectors. The private candidate uses the same four unaligned 512-bit input loads. Four 128-bit-lane shuffles group low/high halves; fixed byte permutations assemble the five digits. The middle digit joins three low-half bytes and five high-half bytes through disjoint zero masks. Two digits shift by four bits; the first four mask to 52 bits and the last retains 48.

There are six 64-byte selector constants, **384 logical bytes**, plus scalar mask/immediate constants. They are independent of key, witness and circuit. Source-level shuffle/permute/shift/Boolean work is 17 operations versus 22 before loads, constant setup, moves and allocation by the compiler. Constant loads and register pressure can offset that saving. No extra point/key load, heap buffer, key copy, solver plan, runtime preparation or pool is added.

Only `Ifma8::raw` changes under a private compile-time flag with AVX512VBMI. The original constructor remains the feature fallback. Four-lane, ARM, portable, field multiplication/reduction, Montgomery radix, point layout and query code remain unchanged. The official eight-lane generator runs once and passes its consistency check. This differs from earlier coordinate-reference staging, gather, word-export and native-bucket trials; their results remain unchanged.

## Qualification

The independent bit reader checks zero/all-ones, every one of the **2,048 input-bit basis vectors**, eight distinct-lane patterns and 4,096 random vectors. Each of four prime/radix tag contexts checks 6,154 vectors, totaling **196,928 raw output lanes per configuration**. Fixed permutations, shifts, masks and disjoint ORs are bit-linear; the complete basis and zero checks support the full mapping. They are not a formal ISA/compiler proof. Repeated tag/configuration populations are not additional distinct input vectors.

Public constructor tests add **65,536 roundtrip outputs and 32,768 Arkworks-checked products per configuration**, covering both primes and Montgomery radices. Arbitrary raw words are inspected as bits only; field operations use valid canonical inputs.

The first OFF suite reports 36 passed/one failed: all 35 original tests and the raw-bit test pass. The public test fails before its first constructor because its random-input oracle assumes little-endian bytes, while the existing `from_random_bytes` API uses big-endian. A separate snapshot changes only that oracle conversion. The failed OFF public test then passes with the other 36 filtered; their successes are inherited. The first complete ON suite passes **37 tests**. No successful OFF test is replayed for that repair.

Both unfiltered core suites pass **125 tests**, and ON passes two prover-library plus six cubic tests. Final source has 177 files: 173 exact accepted parents, three reversible parents (library module declaration, generator, generated constructor) and one new child. All 176 production hashes remain exact.

The native reader checks both 177-file snapshots, exact parent recovery, the test-only repair and six successful generator/field/core receipts. Its separate symbolic byte map checks **2,560 output bits: 2,048 data bits and 512 zero-padding bits**. These checks do not measure latency, stack peak or RAM requirements.

## Actual linked consumers

Coherent eight-library OFF/ON SDKs link with O/native/fat LTO/one codegen unit. Saved symbol tables select actual signed G1, borrowed G2, shared inverse and both stored FFT column directions. Each selected consumer is decoded once.

|Consumer|Instructions OFF → ON|IFMA sites, unchanged|Byte-permutation sites OFF → ON|Frame reservation OFF → ON, bytes|
|---|---:|---:|---:|---:|
|Signed G1|4,428 → 4,514|800|0 → 54|4,288 → 4,544|
|Borrowed G2|8,591 → 8,701|1,450|0 → 78|5,952 → 6,080|
|Shared inverse|1,139 → 1,152|500|0 → 6|1,080 → 1,080|
|FFT forward column|2,181 → 2,181|200|0 → 0|17,024 → 17,024|
|FFT inverse column|2,196 → 2,196|200|0 → 0|17,024 → 17,024|

No calls to the new packing wrapper remain. G1 ZMM stack operand/store rows grow 151/78→171/88; G2 grows 332/156→345/170. Shared-inverse counts remain 30/15. These static operands can include required data as well as spills. Frames exclude alignment, pushes and callees and are not a whole-program stack or RAM bound. Equal selected FFT metrics do not establish unchanged performance for every FFT path.

The initial optional FFT selector mistakes a function's direction parameter for a field-radix parameter and selects no FFT functions. A supplemental helper checks the actual `const FORWARD: bool` declaration and selects four additional consumers from the saved symbol tables. The earlier six successful decodes are retained exactly; no symbol listing or decode is repeated for this repair.

The final reader checks 356 source-pin occurrences, 18 SDK/symbol/decode receipts, actual optimization flags, eight libraries per SDK and all ten retained consumer counts/hashes. It is not a second disassembler, field oracle or speed estimate.

## Next measurement and unchanged baseline

The prospective first trial uses accepted/OFF/ON, fresh all-nine-shape Gnark preflight, then **31 fixed blocks** of primary32 with three primes and 151 complete requests per process. Ranks 10–22 give nominal **97.05506265% marginal median coverage**, conditional on independent, exchangeable blocks. Gain requires upper paired ratio below one and at least 21 wins against both controls; OFF/accepted must stay within factor 1.03. No gain floor, exclusions, extensions or favorable subset applies. Counters/profiles follow latency automatically; positive results still require all-eight plus merge54 and resource/portable qualification before adoption.

The larger block count is chosen before this candidate's first clock, informed by the resolution of a different closed trial. It does not extend or replace the previous uniform-prefix data. Whole-function size is descriptive rather than a performance gate; the current increases and register tradeoffs remain part of the evidence.

Accepted source176/SDK6d83 and canonical Gnark-export ledger **31,777** remain unchanged. Local 32-worker **1×2 remains 7.915095 ms** against supplied M5 CPU 37 ms and Metal 24 ms; its CPU10× limit is 3.7 ms. [All-eight CPU/Metal comparisons](../2026-11-07-uniform-prefix-pilot176/README.md#accepted-cpu-and-metal-comparisons-unchanged) remain inherited and all eight targets unmet. Merge54 remains 161.718956 ms without an M5 reference. No new AWS/Aeglos, mobile, giant-circuit, pressure/swap, minimum-RAM or twofold speed result is claimed.

[Artifacts](artifacts/) preserve changed sources, frozen designs, failed/fixed test sources, plans, pins, analyses, reviews and exact text/JSON receipts/disassembly, compared with originals. Unchanged source parents are identified by hashes. Actual keys/images, requests, witnesses, proof/public-input payloads, executables/libraries and raw perf data remain private.
