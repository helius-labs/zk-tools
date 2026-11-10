# G2 packed low-digit exception filter: native qualification

Original goal remains verified single-proof CPU AVX >=10x supplied M5 CPU for
ALL8, reporting CPU and Metal. Accepted primary7.772288ms32workers;targets0/8.
No token budget. Previous G1 record and block-gap recipes remain closed.

Current borrowed G2 Window.add checks initial identity then complete X equality
for each nonempty arrival before batching. Candidate keeps identity handling,
busy bitmap, borrowed queue and geometry, but defers X-equality classification
to the existing packed prefix pass. Compare the already packed X.c0 low radix52
digit across eight lanes in one vector comparison. For matching digits perform
the exact original scalar Fq2 X comparison; unequal digits cannot be equal X
under the canonical Fp coordinate invariant. This is a filter, not approximate
equality. Deliberate low-digit collisions must remain ordinary when full X differs.

For real equal-X pairs compute complete signed point addition at classification,
substitute one for that lane's norm, and skip its dummy result scatter. This
keeps the shared inverse nonzero and counts all dummy arithmetic; it can change
exceptional-path work/order versus the old separate projective accumulator.
Scalar tails perform exact scalar classification, original ordinary tail formula,
unit-norm dummy exceptional records and real-only scatter. No first-arrival
arithmetic is added: empty buckets retain the original immediate signed write.
All ordinary prefixes, norms, inversion and point formulas remain original.

One status byte per full group plus one tail byte uses a new reusable Vec<u8>
inside Fq2 scratch. reserve_exact precedes resize, without a fixed batch/RAM cap.
Checked workspace charges its logical payload/header and24-byte header for G1
whose BatchAdd also embeds Fq2 scratch. Record type remains640bytes. Actual
capacity, resource guards, allocations/stack/RSS and full caller remain toqualify.
No key/point expansion, retained plan, LUT, new pool or architectureRAMcap.

Add only two field helper APIs in one append-only native child: raw low-limb
equality mask and masked replacement with one. Existing arithmetic, generated
runtime bodies and generator stay byte-exact. Two tiny core wrappers expose them.
Native guards exclude the new path without required ISA/ifma8/no_ifma settings.
Fallback Window path and arbitrary original field methods retain existing code.

The pinned Aeglos0290613 PairBatch handles exceptional lanes after packing;
that is a design lead, not a transplanted algorithm, current remote source or
speed claim. Its arrays/inversion/scheduling differ. Both GitHub connector and
public browser could not fetch current alex/cpu-latency,so no freshness claim.
This implementation is independently written Rust with accepted Zopro formulas.

Qualify field masks against exact scalar raw-bit expectations across all256
masks and deliberate low-limb collisions; preserve unselected lanes and inputs.
Qualify coupled native batches against independent Arkworks full additions,
identities/opposites/doubling, allsigns/tails/reuse, original ordinary results,
complete Windows, actual1/4/16/32workers and concurrent callers. Compare outputs,
memory forecasts, retainedmask capacities and generic exclusion. No speed or
adoption follows from native correctness. Only then freeze full-query consumers
and both readers before preflight,including whole setup/diagnostics/resources;
fullSDKALL8+54/freshGnark still mandatory for any adoption. No subsetrescue,
thresholdchange or unchanged timing reroll. Keep all failures.

Canonical45335 unchanged;seed every future proof export from
retained-tail-sdk176/proof-sha256-after-capture.json. During ANYpreflight/recording
onlypoll exactconfirmed livehandleuntilterminal;noowned helper/build/hash/analyze/
report overlaps. Rust1.98.1+serializedxtask;commitANDpush every completedexperiment
and updatehandoff/latest-status. Full goal remainsACTIVE/UNMET0/8.
