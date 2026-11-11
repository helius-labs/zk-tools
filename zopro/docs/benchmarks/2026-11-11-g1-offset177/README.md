# G1-only affine offsets: prepared candidate

The isolated G1-only candidate builds and passes native and generic correctness
checks. **Full-proof performance is unmeasured; production is unchanged.**

The earlier [combined G1/G2 implementation](../2026-11-08-affine-redc-offset-mask-callers176/README.md)
failed its declared screen. Its G1 caller gains at one and sixteen workers
motivate this separate candidate, but do not qualify it. The old failure and
all its observations remain unchanged. No favorable subset of that campaign
is presented as an adoption result.

This candidate folds canonical coordinate offsets into G1 multiplication's
final normalization. G1 scratch stores bx in place of xsum, retaining five
fields, 1,600 bytes per record and the original allocation behavior. G2,
inversions, FFT, reduction helpers and scheduling retain accepted source.
There are no additional key tables, retained plans, production pools or RAM caps.
The private `zopro_g1_affine_offset` selector defaults off.

Of the accepted baseline's 176 files, 173 remain byte-exact and three change:
the field module declaration, two SIMD wrappers, and G1 streaming endpoints.
Two files are added: the previously qualified field arithmetic module and a
test-only caller adapter. The pending gather experiment is not enabled or
included in this isolated baseline. The source review reverses all four G1
edits to the accepted hash and checks 19 field inputs against the prior native
qualification's pins. Field arithmetic and its bounds are inherited exactly;
the new core integration is tested freshly.

Validation completed:

- Native: 130 core tests, two prover unit tests and six cubic proof tests.
- Generic x86-64 with no IFMA or assembly: 97 core tests, unfiltered. The new
  candidate cfg is present, while native feature guards exclude its code.
- Integration tests exercise changing state, signs, scalar tails, both scratch
  storage modes, reuse, and 1/4/16/32-worker pools with requested 512 KiB stacks.
- Coherent OFF/ON SDKs are linked from eight exact Cargo libraries with Rust
  1.98.1, znver4, fat LTO and the accepted native flags.

An initial report checker expected 102 portable tests from an earlier
configuration. The actual command completed all 97 tests in its generic
configuration with zero failures or filtered tests. The corrected reader checks
that output; its original source and binary are retained. No test or capture
was repeated. Both versions of the reader are included in the artifacts.

Next, freeze a separate SDK protocol before its first preflight. Compare accepted,
fresh OFF and ON using predetermined independent executable backings, retaining
all copies and outcomes. Include all eight supplied shapes, merge54, primary
16/32-worker behavior, fresh independent Gnark verification, CPU/RSS and automatic
profiles. Old component gains do not replace these checks. Generator integration
and applicable resource/concurrency qualification remain adoption requirements.

This preparation makes no SDK requests, independent Gnark exports, latency or
memory-peak measurements, AWS calls or Aeglos comparisons. The canonical ledger
stays at 52,000. The full all-eight CPU 10× goal remains unmet; the accepted
[CPU and Metal comparisons](../2026-11-11-phi-grain-parent176/README.md) are unchanged.
This is a possible incremental gain, not a predicted twofold improvement.

[Build review](artifacts/build-review.json), [source review](artifacts/source-review.json),
[source changes](artifacts/source-audit.json) and [design](artifacts/design.md)
retain the evidence. Private artifacts and SDKs are under
`/home/ubuntu/src/zopro-perf-work/g1-offset177/`. This work is local only;
pushes and publication require the user's approval.
