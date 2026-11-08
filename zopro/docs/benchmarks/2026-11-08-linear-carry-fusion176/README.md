# Deferred normalization of the X-coordinate sum

**New arithmetic qualification passes; performance is unmeasured and production is unchanged.** The candidate combines carry propagation for `square(slope) - (Ax + Bx)` into one subtraction boundary. It selects complete G1/G2 kernel qualification and matched caller measurements, not adoption or a prover speedup.

The accepted local 1×2 observations remain approximately7.9–8.0ms. Earlier7.915095/7.928660/7.995682ms and latest8.010122ms use the same accepted SDK. The inter-run cause remains unconfirmed. This arithmetic study adds no proof timing.

## Arithmetic change

Let B=2^52, normalized numerical representatives0≤A<2p and canonical anchors0≤X,Y<p. Store the five independent sums `S[i]=X[i]+Y[i]` without propagating carries. Each digit can reach2B−2, although their exact numerical total X+Y remains below2p. An owning `UncarriedSum8` type has no multiplication or ordinary field export. Its constructor requires canonical anchors; that numeric precondition still has to be established at integration sites.

The only arithmetic consumer calculates `v[i]=A[i]−S[i]+carry`, emits `v[i] mod B`, and passes `floor(v[i]/B)` to the next digit. Internal carries may be**−2**, so a one-bit borrow replacement would be incorrect. Since −2p<A−X−Y<2p and2p<2^260, the final carry is−1 or0. Negative results receive the original2p correction, producing exactly the original lazy representative in[0,2p).

The unnormalized digits never reach IFMA. A separate counterexample shows why that boundary matters: `(B−1)+(B−1)` exceeds52bits, and feeding its low52bits to IFMA loses B, which is nonzero modulo BN254. Ordinary field inputs, product bounds, Montgomery representation and output normalization retain their existing contracts.

G1 could retain this type in its existing X-sum record; G2 could create it immediately before subtraction. Neither integration is implemented here. The constructor uses five vector adds, with no carry propagation, table, heap, key/point copy, retained plan or new pool. Payload size follows the existing five-vector shape; no process-memory or stack saving is measured.

This differs from the[canonical-X-sum experiment](../2026-10-28-g2-canonical-xsum/README.md), which still normalized the sum; the[normalized lazy-linear quotient recipe](../2026-10-26-lazy-linear/README.md); and[late-centered arithmetic](../2026-11-08-late-centered-native176/README.md), which changed intermediates through the multiplication circuit. Their outcomes remain unchanged. The[Pornin2026/780 abstract](https://eprint.iacr.org/2026/780) prompted checking redundant limbs at a whole-primitive boundary; its P256/RISC-V implementation and performance are not transferred. The actual identity and bounds above are independently derived from accepted BN254 add/sub code.

## Executed qualification

One private177-file snapshot has175 exact accepted originals, one append-only IFMA parent and one child. Removing the suffix recovers all176 accepted source hashes. Original generated arithmetic, core and prover bodies remain exact; the private extension is not a new generator-consistency qualification.

The four-step native workflow passes once: native field build, new test/actual-wrapper link, two new tests and generic no-ASM/no-IFMA field compile. Rust1.98.1, native O3/oneCGU/bitcode field build, fat-LTO test consumer with overflow/debug checks. The generic compile is a field-module guard check on x86, not ARM/mobile or complete-prover execution. Unchanged field/core/prover suites are inherited, not rerun.

|Check|Coverage|
|---|---:|
|Native raw outputs against exact BigInt subtraction modulo2p and both original operations|3,637,248|
|Native canonical exports against independent Arkworks arithmetic|3,637,248|
|Actual unnormalized sum digits at or above2^52|2,112,406|
|Native-input internal−2 carry occurrences checked by the integer model|1,392,394|
|Independent exhaustive small-radix cases|4,676,960|
|Small-radix internal−2 carry occurrences|1,196,800|
|Invalid IFMA-operand witness|1|

Fq/Fr and R256/R260 each check48³ boundary triples,2,048 deterministic changing random vector triples and1,024 returned-state vector steps:113,664 rows and909,312 output lanes per configuration. Every raw result is normalized, below2p and bit-identical to original canonical-add/sub and generic-add/sub. Exports are canonical. Four additional wrapper comparisons retain the actual inspectable Fq/R256 functions. Counts include related cases, not distinct cryptographic proofs.

## Actual emitted code

One symbol listing and one full disassembly inspect the linked test consumer. The offline reader independently reconstructs opcode counts and separates `int3` padding. These are standalone expression wrappers, not full point kernels or dynamic retirement counts.

|Wrapper|Non-padding instruction rows|Padding rows|
|---|---:|---:|
|Original canonical add, then subtract|83|3|
|Original generic add, then subtract|104|5|
|Combined sum/subtraction|71|15|
|Deferred sum constructor alone|18|3|
|Deferred sum consumer alone|61|15|

The combined wrapper removes four right-shift, four add and four mask sites versus the canonical expression. The generic expression has additional correction work. Separate constructor/consumer ABI and memory operations remain costs; they must not be treated as a complete inlined-kernel cost. No multiplication is removed, and the ordinary G1/G2 multiplication counts remain unchanged. Smaller static expression code does not establish a lower cycle count or a twofold path.

The independent reader checks six successful native/codegen command receipts and stream hashes, all original-source recovery, pins, coverage arithmetic, exact saved wrapper bodies and opcode counts. It inherits the executed per-output BigInt/Arkworks oracles rather than rerunning them. Two helper compiles needed type corrections; the first compiled reader missed libtest's first prefixed row and was repaired. Original helper sources and[repair notes](artifacts/preparation-notes.md) are retained. Diagnostics for these helper failures are tool-transcript observations, not separately archived raw failure streams. No successful build, test or decoder was repeated.

[Qualification](artifacts/qualification.json),[independent review](artifacts/review.json),[codegen](artifacts/codegen.json),[new field source](artifacts/linear_carry.rs),[new tests/model](artifacts/native-tests.rs) and[design](artifacts/design.md) accompany the evidence archive. The gzip JSON map stores UTF8 payloads, lengths and hashes. It includes complete native streams and codegen command/exit metadata; full disassembly stdout stays private with a hash, while all five selected bodies are public. Executables/libraries, full unchanged snapshots, external full text, keys/images/witnesses/proofs/public arrays and private proof ledgers are excluded.

## Next and unchanged full-proof comparison

Qualify this exact type in actual G1/G2 full/tail/signed/reused kernels, preserve unsupported and exceptional fallback, establish the anchor preconditions and check scratch/resource behavior. Inspect actual final callers, then freeze complete matched1/16/32-worker latency and automatic PMU/profile rules before capturing anything. Only a credible complete-caller gain selects verified SDK primary/all8+merge54/primary16, Gnark, portable and resource qualification before adoption. There is no gain floor or authorization to reroll earlier failed recipes.

All proof timings below are inherited: the fresh primary accepted control from the[preceding pilot](../2026-11-08-tree-owner-sdk176/README.md) and seven older full-campaign rows. Supplied M5 CPU/Metal references use different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.010122|37 /4.619×|24 /2.996×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|Merge24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Merge54 remains inherited162.240109ms, without a supplied M5 reference. Production176/SDK6d83 and canonical35,314 verified exports stay unchanged; zero new SDK requests or proofs. The original all-eight CPU10× goal remains active and unmet,0/8. No new timing/profile, allocation/RSS/minimum-RAM, phone/ARM/TEE, pressure/swap, huge-circuit, service concurrency or AWS/Aeglos comparison is claimed.
