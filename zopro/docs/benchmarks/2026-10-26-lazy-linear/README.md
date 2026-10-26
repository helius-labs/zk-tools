# Lazy linear SIMD field operations

Rejected by the arithmetic component gate. The intended Fq/R256 expression slows by **14.52% against fresh ordinary arithmetic**, with zero wins in seven paired blocks. The four intended patterns regress by 27.1%, 95.8%, 32.0% and 14.5%; their aggregate regresses 39.3%. Production remains unchanged, and no SDK trial follows.

The earlier R260 bounded-subtraction probe produced canonical outputs and failed its historical 3% gain floor despite a small measured gain. That result remains unchanged. This new Rust primitive returns normalized lazy values below 2p: it omits the old final canonical correction, adds a matching addition operation, and selects quotient-times-modulus coefficients through four fixed choices per digit. All old field-operation bodies remain identical. Small positive gains have no minimum acceptance floor here; this candidate regresses materially.

For X=a+b or X=2p+a-b, a downward leading-digit quotient gives 0≤X−qp<2p. One signed carry pass therefore suffices. The quotient is 0..3. Five small constant vectors replace five low-64-bit multiplications; at most 320 bytes per eight-lane table instance, independent of key/input size. There is no point table, input cache, heap buffer, key conversion, scheduler or production dispatch change. The per-instance constant size is not a whole-binary or minimum-RAM bound. [Frozen design and bounds](artifacts/design.md).

## Component measurements

AMD Ryzen 9 9950X, one thread on CPU 2, seven rotated/reversed fresh-process blocks. The consumer explicitly links both the exact accepted native field library and candidate library. Modes are accepted, fresh ordinary and lazy. Each process uses three 512-iteration primes and 1,000,000 timed iterations, with matched changing input vectors. Inputs/packing and frozen-reference setup/checks are outside timing; the arithmetic kernel and its returned vector state are inside. Canonical conversion and output checks follow the timer. The four-stream case divides per-lane figures by four. The curve expression uses existing multiply plus add/sub operations; it is not a complete elliptic-curve addition or proof. No owned build/profile/hash/archive overlaps latency.

|Field/radix/pattern|Accepted ns/lane iteration|Fresh ns|Lazy ns|Lazy/fresh paired|Wins|
|---|---:|---:|---:|---:|---:|
|fq-256-p0|0.774797|0.777990|0.987481|1.270971782|0/7|
|fq-256-p1|0.735389|0.736972|1.442747|1.957672923|0/7|
|fq-256-p2|0.290211|0.290619|0.383450|1.319947064|0/7|
|fq-256-p3|9.098554|9.095423|10.418420|1.145237390|0/7|
|fq-260-p0|0.775693|0.776403|0.987379|1.271875439|0/7|
|fq-260-p1|0.736627|0.736792|1.440246|1.958237442|0/7|
|fq-260-p2|0.290626|0.290997|0.383756|1.321457270|0/7|
|fq-260-p3|8.503264|8.503899|9.751412|1.147246611|0/7|
|fr-256-p0|0.775075|0.776087|0.985506|1.269835993|0/7|
|fr-256-p1|0.736212|0.737736|1.444198|1.957347393|0/7|
|fr-256-p2|0.291032|0.290946|0.384319|1.321028469|0/7|
|fr-256-p3|9.092569|9.093855|10.413211|1.145135094|0/7|
|fr-260-p0|0.776839|0.775485|0.987432|1.274025883|0/7|
|fr-260-p1|0.737062|0.738182|1.441938|1.956537811|0/7|
|fr-260-p2|0.290892|0.290806|0.384838|1.339635835|0/7|
|fr-260-p3|8.497349|8.498284|9.748723|1.147418085|0/7|

p0 = dependent add; p1 = dependent subtract; p2 = four independent alternating add/sub streams; p3 = dependent affine-expression arithmetic. Displayed times are medians of process times; ratios are geometric means of paired block ratios. Reference outputs and all 128 final 64-bit words per process agree.

The intended Fq/R256 curve expression and four-pattern aggregate must improve against both controls with 4/7 winning blocks. Every Fq/R256 pattern must remain within 3% of each control. Fresh/accepted calibration must stay within factor 1.03 each block and 1.02 paired median across all 16 cases. Other field/radix timing is descriptive; its correctness is mandatory. All intended-case calibration checks pass, but one Fr control block fails the complete calibration. That failure remains part of the failed full component gate. The intended-case regressions independently reject the candidate. No minimum gain floor, exclusions or unchanged retry.

calibration_failures:

- fr-256-p2/b0:fresh/frozen=0.7639285577434248

failures:

- fq-256-p0:lazy/fresh=1.270971781558776>1.03
- fq-256-p0:lazy/frozen=1.2737985119274915>1.03
- fq-256-p1:lazy/fresh=1.9576729225805871>1.03
- fq-256-p1:lazy/frozen=1.9603670878876958>1.03
- fq-256-p2:lazy/fresh=1.3199470644391655>1.03
- fq-256-p2:lazy/frozen=1.3218752571955923>1.03
- fq-256-p3:lazy/fresh=1.1452373904366409>1.03
- fq-256-p3:lazy/fresh=1.1452373904366409,wins0/7
- fq-256-p3:lazy/frozen=1.1446970393959577>1.03
- fq-256-p3:lazy/frozen=1.1446970393959577,wins0/7
- FqR256aggregate lazy/fresh=1.3926180468419471,wins0/7
- FqR256aggregate lazy/frozen=1.394215135972321,wins0/7

## Correctness and profiles

All 37 native field tests and generator consistency pass. New tests check 75,616 exact raw result lanes against independent Arkworks big integers and 393,216 chained result lanes against Arkworks field arithmetic. They cover inclusive 2p, limb/modulus boundaries, random loose values, eight-step add/sub/multiply/fused-product chains, both primes, both Montgomery radices and four/eight lanes. The independent reduced-radix model checks 1,600,784 operation cases, quotient bounds, signed carries, exact remainders and closure below 2p. The range argument and finite tests are not a formal machine-code proof. The first generator run stopped before writing output because a test helper name matched its insertion marker; only test helper names changed before compilation. A pipeline-helper delimiter was repaired before consumer compilation or timing.

All 336 latency processes finish before three 999 Hz cycles:u profiles of the intended expression, 20 million iterations plus three primes each. Warm FIFO gating excludes reference setup and primes. The first decode command requested a CPU display field absent from these process-scoped captures; a separate workflow removed that field and decoded the retained files. No latency or profile is rerun. The failed receipt and [repair note](artifacts/retained-decode-repair.md) remain archived. Binaries and root-owned raw captures stay local with checked hashes; --no-buildid-cache avoids executable metadata changes. Profiles are descriptive cycle samples, not exclusive opcode costs or removable proof wall time.

|Mode|Samples|Lost|PIDs|
|---|---:|---:|---:|
|frozen|1447|0|1|
|fresh|1447|0|1|
|lazy|1657|0|1|

The timed Fq/R256 expression has the following static code observations. LLVM merges the accepted and fresh ordinary expression into the same linked symbol; both controls still run separately and use their respective library types. The one failed Fr calibration block is retained.

|Actual timed expression|Symbol bytes|Static instructions|IFMA sites|Vector permutations|RSP operand lines|Literal stack subtraction bytes|
|---|---:|---:|---:|---:|---:|---:|
|Accepted/fresh ordinary|4867|750|300|0|78|1016|
|Lazy linear|4773|728|304|20|54|1016|

The new operation adds a quotient dependency before lower-limb corrections and uses additional IFMA/permutation operations. The smaller function and fewer static instructions did not produce a speed gain. These counts do not isolate a causal share, count retired instructions, measure spills or bound the complete stack. [Exact linked code observations](artifacts/codegen.json) and both disassemblies are retained.

The study has 396,000,000 timed kernel iterations, 520,704 priming iterations and 396,173,568 separate frozen-reference iterations. All 43,392 final 64-bit word comparisons agree. These are arithmetic iterations, not SDK requests or proofs. The three profiles contain 4,551 samples with zero lost records and one PID each.

Zero new SDK or Gnark proofs; prior **6,850** unchanged. All 170 production files and exact source/library/consumer/tool pins are checked unchanged. A component pass would require separately declared actual-caller/full-SDK, consumer, portable and resource qualification; this method does not advance. No AWS/Aeglos, phone, TEE, giant, concurrent, pressure, minimum-RAM or full-proof gain is claimed. The [latest complete SDK CPU/Metal comparison](../2026-10-26-pairing-x-chain-sdk/README.md) retains all eight supplied references; primitive nanoseconds do not replace proof timings. The all-eight CPU 10× goal remains active and unmet.

[Analysis](artifacts/analysis.json),[source audit](artifacts/source-audit.json),[native tests](artifacts/native/field/stdout.log),[model](artifacts/native/model-tests/stdout.log),[consumer](artifacts/probe.rs),[jobs](artifacts/jobs.json),[artifact hashes](artifact-index.json).
