# Low-word Montgomery quotient multiplication: native qualification

The new eight-lane field implementation passes native correctness and actual-code checks. **No timing or prover speedup is established.** Five quotient IFMAs become five VPMULLQs per field product, with unchanged masked quotients and output digits. Ordinary/fused IFMA sites are95/145 rather than100/150; total multiply instructions do not decrease. This selects a complete G1/G2 caller trial, not adoption.

For k≤52, masking a full64-bit product to k bits gives the same result as masking both inputs to52 bits first. The change replaces only the CIOS quotient product m=t0·INV modulo2^k: four52-bit rows and a final52- or48-bit row. Data/modulus products, cancellation, mixed packing, normalization and lazy bounds remain. Two textual sites change in an isolated18-member field copy; reversing them and the qualification-only diagnostic suffix recovers the accepted file exactly. The other17 field members and all176 production originals match. Four-lane field/core code and production are unchanged.

The hypothesis is shorter quotient dependency latency. The primary [uops.info Zen5 measurements](https://uops.info/html-instr/VPMULLQ_ZMM_K_ZMM_ZMM.html) list2-cycle register-source latency and0.50 reciprocal throughput for the displayed masked form, while its linked AMD documentation lists3cycles. The [existing local IFMA calibration](../2026-11-06-instruction-calibration176/README.md) measured about4cycles. These are not matched local VPMULLQ calibration or a full-kernel prediction. Intel entries show15cycles: an unconditional universal x86 default would be inappropriate. Production target policy, its actual cost and architecture qualification remain unresolved. The direct AMD guide fetch was unavailable.

## New native evidence

One five-command Rust1.98.1/xtask workflow builds ON and fresh OFF with original native optimized flags, links and runs two new tests, and checks generic gating with both selectors present. All first executions pass. Native tests cover both Fq/Fr and R256/R260,52 loose boundaries including2p, full-width random values and1,024 dependent rows per panel. A byte-exact original CIOS copy supplies raw-digit comparisons; independent Arkworks arithmetic supplies residue/export checks.

|Check|New comparisons|
|---|---:|
|Exact raw output lanes against original CIOS|554,496|
|Canonical exports against independent field expectations|554,496|
|Full-width masked quotient lanes, native IFMA/VPMULLQ/wrapping integer|328,400|

The quotient cases include five inverse/factor words, full64-bit inputs and masks48/52. The algebra applies to all words; finite native cases do not alone prove all-input arithmetic. Existing unchanged field/core suites, prior mathematical models and proof verifiers are inherited, not replayed or counted as fresh.

Twelve actual variable-input wrappers, linked native O3/fatLTO/oneCGU, check outputs once and are decoded once each after one symbol listing. Both primes/radices show the intended opcodes and no calls. These are static sites, not executed instruction counts, runtime spills or complete-stack bounds.

|Operation|Original IFMA premise|Actual new IFMAs|Actual new VPMULLQs|Calls|
|---|---:|---:|---:|---:|
|Product|100|95|5|0|
|Fused sum/difference|150|145|5|0|

The fresh ON/OFF compiler argv differ only by the quotient selector and output path. A separately written same-agent token parser reconstructs all twelve opcode counts; the review also checks208/217/220 pin populations, all20 successful command/stream receipts, source recovery and artifact identities. No native test, wrapper execution, build or decoder was replayed. Helper compiles/preparation/review/publication have tool-transcript evidence rather than complete workflow receipts. The first wrapper-helper compilation failed because two `extern "C" fn` type aliases lacked a space; only those spaces were fixed before any wrapper execution, with initial source retained. No field arithmetic or successful qualification changed.

There is no new production heap, table, key/point copy, record, retained plan or pool. Native test/compiler allocations and static stack operands are not prover RSS, minimum RAM or mobile/pressure evidence. The qualification-only raw/reference APIs are privately cfg-gated. Generated-source integration remains pending: production ifma8 generation must reproduce the eventual change before adoption. Existing rowpair/preadd/carry/split-plane/complex proposals remain closed; this is a quotient-opcode substitution.

## Next stage and inherited comparisons

Reuse these qualified ON/OFF libraries without rebuilding or replaying their tests. Build coherent complete G1/G2 callers from original176 core, without the closed phase-boundary/tree-policy/offset edits. Qualify the changed field in complete point kernels, then freeze all200/201/352/353 sizes and1/16/32workers, frozen accepted-source/freshOFF/ON controls, output/resources/calibration/nonregression/gain and automatic PMU/profiles before any preflight. Only a full-caller benefit selects actual SDK ALL8+merge54/primary16/Gnark/generator/portable/resources/concurrency qualification. No isolated favorable multiply benchmark substitutes for this.

The following latest accepted local32-worker medians are inherited from the [full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md). M5 CPU/Metal timings were supplied and use different hardware. The same accepted primary build previously measured7.915095ms; latest8.040649ms does not establish an adopted regression.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|8.040649|37|24|4.602×|2.985×|3.7|
|2×2|11.149097|59|30|5.292×|2.691×|5.9|
|4×2|16.069415|109|48|6.783×|2.987×|10.9|
|8×2|24.131178|189|74|7.832×|3.067×|18.9|
|12×2|33.940628|284|101|8.368×|2.976×|28.4|
|16×2|40.164083|335|124|8.341×|3.087×|33.5|
|24×2|66.187704|528|176|7.977×|2.659×|52.8|
|merge24×1|65.361571|507|175|7.757×|2.677×|50.7|

Merge54×1 remains160.890372ms with no supplied M5 reference. The verified single-proof CPU10× goal remains active/unmet on all8shapes,0/8. Accepted176SDK6d83 and canonical37,023-proof ledger are unchanged. No SDK/Gnark proof, cloud resource, AWS/Aeglos/huge-circuit/mobile/throughput result is added.

[Design](artifacts/design.md), [qualification and actual code counts](artifacts/qualification.json), [source recovery](artifacts/source-audit.json), [separate review](artifacts/publication-review.json), [full source/receipt/decode evidence](artifacts/evidence.json.gz), [member hashes](artifacts/archive-index.json), [publisher](artifacts/publish.rs) and [checksums](SHA256SUMS). Every archive member and gzip roundtrip is verified. Executables/libraries, unchanged full source copies, keys, witnesses, private fixtures/proofs and raw perf data are excluded.
