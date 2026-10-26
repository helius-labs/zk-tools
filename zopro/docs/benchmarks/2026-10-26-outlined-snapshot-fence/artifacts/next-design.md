# Move from local queue tuning to arithmetic and the dependency path

Close and PUSH this experiment before another. No next experiment is declared
or implemented. Do not sweep more tile sizes, inline attributes, backoff, victim
ordering or GC intervals after this result. Do not compose prior rejected modes.

The one-attribute source identity and static gate pass: parent1661accepted /
5757inlined /1654outlined bytes, separate scan4119bytes. Outlining improves the
original8 paired geometric aggregate against old-inlined ON by about0.77%
(ratio.992318, all7blocks). It remains about0.80% slower than accepted and0.87%
slower than new OFF. Primary ratios1.001884/1.001586/1.004400 versus accepted/OFF/
inlined all fail improvement. One physical16 OFF calibration factor1.031438
also fails, but the32worker checks already fail independently. Preserve13failed
checks and every row; no unchanged reroll or claim that fixing calibration alone
would qualify it. Current production remains unchanged.

Primary unprofiled CPU falls199.588 ->193.517ms but switches411.94 ->587.03.
Profile OFFepoch9.604% ->ON3.385%, scheduler4.397 ->6.039%; CPU199.888 ->193.304ms,
system5.253 ->8.076ms, switches406.82 ->580.58. This supports a real reduction in
some CPU work without a consistent single-proof latency gain. Static code size
does not establish cache misses or the cause of any difference.

Next concrete read/audit targets from the new actual primary ON profile:

- `msm.rs::stream_g1_impl<G1Config,true,true>` and
  `msm/g2_prefix.rs::apply<G2Config,true>` are the two largest single symbols.
  Other major symbols include fused_kh::feed, Fq inverse, g2_phi::hb_raw and
  G1/G2 reduce_layer. Inspect actual operation/representation work and the
  existing phase-span/CPU evidence rather than treating symbol shares as span.
- The current G2 prefix source first packs coordinates and computes denominator
  norms/prefixes; reverse traversal packs coordinates again, then computes slopes
  and output coordinates and unpacks. Audit the actual FqBatch new/get and
  fq2_pack/unpack code/assembly before claiming conversion overhead, because
  inlining/CSE may already remove work. Quantify the work before proposing any
  replacement. Avoid duplicating loaded key points or creating a key table.
- An operation/packing/occupancy census of complete verified SDK requests could
  distinguish field/curve operation count, SIMD occupancy, conversion and
  inversion costs. It must be an explicitly declared instrumented diagnostic
  with correctness checks and counts, not a speed claim from perturbed timing.
  Reuse Rust xtask, source guards and full supplied-shape context. Audit prior
  arithmetic/collision/phase diagnostics first; do not duplicate old counters.

Do not rediscover four-way G2 GLS decomposition: current g2_phi uses a cube-root
two-component split with signed components below2^126, but archived10/22 GLS4
sparse/parallel and10/23 four-parts experiments already exist. Read those reports
and exact source before proposing a related change. Likewise prior rank-basis/
direct linear-wire census showed little eligibility, and scalar/table candidates
must not silently become large persistent caches.

Any new representation or arithmetic design must preserve independent curve/
field oracles, tails/identity/cancellation, full SDK verification, and memory
accounting/fallbacks. If investigating packing, keep key storage borrowed and
bound/charge proof-local scratch; do not trade the universal/phone/very-large
memory requirement for a benchmark-specific point cache. A major jump requires
fewer useful field/curve operations or a shorter measured dependency span;
another runtime-category reduction alone is not evidence of that outcome.

C7a remains an optional separately declared architecture diagnostic with tested
znver4 binaries, credentials and cleanup; no new AWS/Aeglos result exists here.
Full all-eight supplied CPU10x objective with CPU/Metal reporting stays active
and unmet. No new question/subagent/C-ASM rewrite/large lookup table. Commit and
PUSH every completed experiment, including negative or diagnostic outcomes.
