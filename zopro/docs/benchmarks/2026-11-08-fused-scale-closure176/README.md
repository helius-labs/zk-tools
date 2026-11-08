# Wide fused numerator followed by narrow scaling

**The field arithmetic passes exact and native qualification.** Removing the intermediate correction cuts22/26 static instruction sites from sum/difference wrappers while retaining250 IFMA sites each. This selects complete G2 point-caller qualification; **no latency improvement, SDK trial or adoption is established**. Production176 remains unchanged.

## Range argument and implementation

With normalized inputs a,b,c,d,f≤2p and R=2²⁵⁶, an uncorrected fused result U=REDC(ab+cd) is below `p+8p²/R`. The following ordinary multiplication V=REDC(Uf) is below `p+2p²/R+16p³/R²`. Both actual BN254 primes satisfy `16p²+2pR<R²`, so V remains strictly below2p. Difference uses the existing2p−c complement, including negative zero. The compile-time sufficient prime bound is checked separately in [design.md](artifacts/design.md).

The new helpers keep U private until the following multiplication. They return the original normalized output range. Existing Ifma8/get/compact APIs and runtime bodies retain their contracts and source; no general wide field type escapes. R260 is mathematically checked as a representation control; native tests and intended G2 integration target R256.

The private field source has19members:17 originals exact, one append-only module declaration and one new child. The child copies original CIOS with renamed/private helper references and removes its final fused correction only. The original generated IFMA files and generator remain byte-exact. The following ordinary CIOS arithmetic is unchanged and its wider-input safety is justified inside this coupled helper. No products are removed.

This differs from signed late-centering, later affine-offset subtraction, wider complex-square operands and matched scaled norm/numerator trials. Those outcomes stay unchanged. It keeps R256 throughout and changes only the bound across this specific fused→ordinary dependency. Montgomery deferred correction is established arithmetic; no novelty claim is made. The current correction already has a fast compare/branch path, so this is a modest optimization hypothesis, not a forecast of a twofold proof improvement.

## Qualification

|Check|New evidence|
|---|---:|
|Exact mathematical scaled outputs, both primes/radices/signs|49,552|
|Literal52/48-bit row identities against whole integers|495,520|
|Private mixed-radix native oracle records|24,776|
|Executed native oracle lanes, including24 padded repeats|24,800|
|Native output-fed chain comparisons|32,768|
|Additional variable-wrapper field outputs|64|

The mathematical population crosses five raw edges in all five operands, adds radix boundaries and2048 deterministic full-width random tuples, then256 output-fed tuples per field/radix. Full-radix BigUint REDC independently checks each literal CIOS stage and final canonical field interpretation. R256 first-stage values reach or exceed2p in334Fq/394Fr outputs. Final raw values differ from the original sequence in266/279cases, while canonical values agree. A reader must not require raw-representative equality between the two valid algorithms.

Two new unfiltered native tests check exact raw outputs, canonical outputs and unchanged original sequences. Their input constructors are diagnostic-only and do not construct invalid canonical Fp values. Native and generic/no-IFMA/no-assembly field compiles pass. Unchanged field/core/prover suites are inherited rather than replayed; no native point, exception, threaded/small-stack, SDK or independent Gnark proof qualification is claimed yet.

The initial model run failed because its canonical expectation omitted the third Montgomery inverse: two REDCs plus conversion out of Montgomery require three. A separate repair changes that one expression; initial source/pins/failed receipt remain. Corrected model compilation/execution pass. Native preparation separately caught a reverse-audit omission for the `.map(splat)` function-value spelling. Only the reverse audit was corrected, preserving the partial copy and initial helper. No native build had begun. All four native commands, three codegen commands and eight unique decodes then pass first execution. Total19workflow receipts include one failed mathematical run; helper commands are tool-transcript observations, not invented workflow receipts.

## Actual emitted wrappers

NativeO3/fatLTO/one-CGU variable-input wrappers instantiate both primes. Counts below are identical for Fq and Fr. All eight bodies are decoded once after one symbol listing.

|Operation|Instruction sites, original→new|Bytes, original→new|IFMA sites|Vector compare sites|Explicit RSP/RBP sites|
|---|---:|---:|---:|---:|---:|
|Fused sum then multiply|581→559|3813→3633|250/250|2→0|96→92|
|Fused difference then multiply|610→584|4006→3806|250/250|2→0|104→98|

Conditional jumps fall3→1; the retained branch belongs to the diagnostic input-packing loop. Each wrapper retains one common1600-byte fixture memcpy call. These fixture costs are part of the decoded consumers, not newly introduced production operations. The sum wrapper's literal stack subtraction grows3456→3520bytes; difference stays3520bytes. Static stack-operand counts are not executed spills, peak stack, heap resources or proof latency. No cache, RAM-bandwidth or critical-path cause is inferred.

A separate reader checks five pin sets, all19 command/stream receipts, exact original176 recovery, both source-only repairs, whole-prime range inequalities, native test markers, all eight saved disassemblies and unchanged37,023-proof inventory. It independently reconstructs static counts and checks copied-source recovery evidence; it does not replay finite arithmetic/native tests or implement a second field backend. [Qualification](artifacts/qualification.json).

## Next gate and resources

Next, route only the native8 G2 scaled-numerator consumer through the two qualified field helpers. Keep generic/four-lane paths, norm/prefix records, inverse, point formulas, tails, signs, queues, storage and solver unchanged. Qualify complete point outputs, exceptions and actual small-stack threaded callers before declaring matched complete-caller timing with automatic counters/profiles. Reuse the exact qualified field/oracles/decodes; do not rebuild them or repeat successful observations for bookkeeping. A caller gain would only select full verified SDK/all-eight/resource qualification.

The field helpers add no heap buffer, point/key table or copy, retained solver plan, production pool or architectural RAM cap. Compiler stack behavior can change. No whole-prover peak, minimum RAM, pressure, mobile or concurrent-throughput claim is made. There are zero new timing/profile captures, SDK requests or proofs.

## Inherited CPU and Metal comparisons

Accepted176SDK6d83 remains unchanged. The following latest local32 full-proof observations are inherited from the [tree-policy SDK comparison](../2026-11-08-tree-policy-sdk176/README.md), against supplied historical M5 references. Same accepted primary previously measured7.915ms; private7.860–7.869ms candidates were not adopted. No new accepted regression or absolute record follows from separate-run variation.

|Shape|Local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.040649|37 /4.602×|24 /2.985×|3.7|
|2×2|11.149097|59 /5.292×|30 /2.691×|5.9|
|4×2|16.069415|109 /6.783×|48 /2.987×|10.9|
|8×2|24.131178|189 /7.832×|74 /3.067×|18.9|
|12×2|33.940628|284 /8.368×|101 /2.976×|28.4|
|16×2|40.164083|335 /8.341×|124 /3.087×|33.5|
|24×2|66.187704|528 /7.977×|176 /2.659×|52.8|
|merge24×1|65.361571|507 /7.757×|175 /2.677×|50.7|

Merge54 remains160.890372ms without a supplied M5 reference. Verified CPU single-proof≥10×M5 CPU for all eight shapes remains **active/unmet,0/8**. No fresh AWS/Aeglos, huge-circuit, mobile, memory-pressure/minimum-RAM or concurrent-proof result is added.

[Source](artifacts/fused_scale.rs), [model](artifacts/model.rs), [native tests](artifacts/native-tests.rs), [source audit](artifacts/native-source-audit.json), [math results](artifacts/analysis.json), [qualification](artifacts/qualification.json), [exact source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) preserve the stage, including both preparation/model corrections. All UTF-8 member hashes and gzip roundtrip are checked. Oracle arrays, binaries/libraries, key/witness/proof payloads and unchanged full production source are excluded. Private root:`/home/ubuntu/src/zopro-perf-work/fused-scale-closure176`.
