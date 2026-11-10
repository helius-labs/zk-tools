# Fixed signed seed in the G2 subgroup check

**The private signed-seed implementation passes native and generic correctness qualification. Performance is unmeasured; production is unchanged.** It replaces only the fixed integer multiplication inside G2 subgroup validation. It removes no validation condition and adds no point table, heap buffer, key cache or execution plan. This is a small scalar-work opportunity, not a demonstrated solution to the remaining multi-millisecond proof-latency gap.

The accepted check uses the relation `[x+1]P + ψ([x]P) + ψ²([x]P) = ψ³([2x]P)`, with `x=4965661367192848881`. Its binary loop invokes 64 doublings and 28 mixed additions, including initial identity work. For ordinary nonexceptional trajectories that is 62 nonidentity doublings and 27 mixed additions. The candidate initializes from the leading +1 and uses **62 doublings and 23 mixed additions**, plus one affine negation. These are source operations, not cycle estimates; exceptional points can take different branches.

Two public integer masks encode the standard non-adjacent form:

```text
positive = 0x450a14044a890a01
negative = 0x0020815000200010
positive - negative = 4965661367192848881
```

There are 24 nonzero signed digits, with no adjacent nonzero digits and a leading +1 at bit62. The equality holds over integers. It is not a scalar-field decomposition that assumes P is already in the subgroup. The original infinity handling, subgroup relation, Frobenius maps, curve arithmetic and on-curve precondition remain intact. The OFF branch retains the exact original loop. Selection is a private compile-time cfg, with no per-call environment lookup.

NAF is standard signed-digit arithmetic; no novelty is asserted. [Primary NAF background](https://arxiv.org/abs/1110.0966) describes the representation. The existing subgroup relation is also used by [Gnark Crypto v0.21.0](https://github.com/Consensys/gnark-crypto/blob/v0.21.0/ecc/bn254/g2.go). Its seed multiplier uses a different fixed projective addition/subtraction chain; its 60-doubling/17-addition count is not directly comparable with mixed additions, and no implementation or speedup is imported here. The earlier Fq12 exponent-chain trial concerns a different group and remains separate.

|Qualification|Observed result|
|---|---:|
|Native enabled field/curve/pairing suite|37 pass, unfiltered|
|Generic no-ASM/no-IFMA enabled suite|13 pass, unfiltered|
|Native disabled subgroup tests|4 pass, 33 unrelated tests filtered|
|Exact seed/curve case family|442 point entries, including 375 outside the subgroup|
|Arbitrary twist population|125 points, plus their negatives and cofactor components|
|Small-stack worker family|53 standard threads across 1/4/16/32-worker configurations|
|Per worker-family execution|636 subgroup checks and 636 exact-seed checks|

All suites have zero failures or ignored tests. The same deterministic case families execute under native OFF, native ON and generic ON; repeated configurations are not counted as additional unique mathematical cases. The 442 entries include identity, generator/negative, 64 subgroup points, arbitrary twist points, negatives and full-order multiples retaining cofactor components.

Independent Arkworks integer multiplication checks every new seed result. Full multiplication by r and Arkworks' subgroup predicate check validation results; the original local full-limb multiplication supplies another seed comparison. Existing subgroup tests retain independent full-order and original short-relation checks. Workers share immutable fixtures and request 512 KiB stacks. These are finite host correctness checks, not a formal compiler proof, full SDK concurrency qualification or execution on ARM/mobile hardware.

Only one of the 176 accepted parents changes, reversibly, and one child is added; 175 originals remain byte-identical. Source recovery checks every original against current production. Field primitives, point formulas, Cargo.lock and dependencies remain exact. The new function uses ordinary result/negative-point locals and fixed masks, with no new allocation site. Actual compiler stack peak, whole-process RSS and minimum RAM remain unmeasured.

All six serialized Rust1.98.1/xtask qualification/build commands pass on first execution. Native builds retain the accepted flags; generic uses no_asm/no_ifma. Every argv, managed environment, exit and stdout/stderr hash is checked. The separate same-agent reader reverses the parent edits, verifies all source pins, reconstructs the masks through an independent signed Euclidean recurrence, checks test markers and freezes the three release-library identities. It does not independently rerun Arkworks or constitute an external audit. Direct preparation/review/publication compilation is outside the six workflow-command count.

Next, freeze a complete subgroup-check comparison against the actual accepted field library and fresh OFF/ON. The measured call must include seed multiplication, all ψ maps, final group additions/equality and dispatch. Valid subgroup inputs represent the SDK parsing path; mixed/cofactor inputs preserve general validation coverage. Include solo latency, threaded resource/nonregression checks, exact outputs, native fat-LTO linkage and automatic profiles, with both readers frozen before preflight. No gain floor, favorable subset, extension or unchanged reroll is selected. A component gain would only justify full verified SDK, all-eight-plus54, portable/concurrent/resource qualification before adoption.

There are **zero new SDK requests, proof exports, latency observations or hardware profiles** in this stage. The canonical verified inventory remains **47,437**. The full verified CPU AVX single-proof ≥10× supplied M5 CPU objective remains **active and unmet,0/8**. Below are inherited accepted local results from the preceding complete-SDK campaign; M5 references are supplied historical timings on different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.863367|37|24|3.7|
|2×2|11.051833|59|30|5.9|
|4×2|15.850652|109|48|10.9|
|8×2|24.050327|189|74|18.9|
|12×2|33.877870|284|101|28.4|
|16×2|39.777696|335|124|33.5|
|24×2|66.208517|528|176|52.8|
|24→1|65.473144|507|175|50.7|

Inherited 54→1 is162.652338ms, without a supplied M5 reference. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or proof-throughput evidence is added. No cloud resource is launched.

[Implementation and tests](artifacts/seed_naf.rs), [source recovery](artifacts/source-audit.json), [review and library identities](artifacts/review.json), [design](artifacts/design.md), [authored source, frozen plan/pins and exact streams](artifacts/evidence.json.gz), [publication checks](artifacts/quality.json), [checksums](SHA256SUMS), [preceding full comparison](../2026-11-10-compiler199-sdk176/README.md), [accepted AWS comparison](../2026-11-10-accepted-current-c7a176/README.md). Keys, requests, point/witness/proof payloads, full private ledgers, executable/library blobs and unchanged full source/third-party trees are excluded. Absolute paths require adaptation to reproduce. Private root: `/home/ubuntu/src/zopro-perf-work/seed-naf176`.
