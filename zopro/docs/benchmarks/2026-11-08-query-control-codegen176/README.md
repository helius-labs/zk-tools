# Query-control kernel comparison

**No arithmetic instruction or constant-load difference was found in the inspected stream kernels.** Across the retained query control and the new indexed-tile binary,17,526 normalized instruction rows match in three point-stream specializations and one work-stealing primitive. This narrows the explanation for the calibration gap; it does not explain that gap, establish equal runtime behavior or qualify an optimization. Production and accepted timings remain unchanged.

The [preceding indexed-tile trial](../2026-11-08-indexed-tile-bundles176/README.md) remains closed. Its threaded fresh-OFF/FROZEN ratios fell outside calibration, while same-binary ON/OFF showed no established primary32-worker gain. This audit does not retime either artifact, revise a failed gate or promote the single-worker positive.

## Compared scope

FROZEN is the exact optimized query executable `0ba4cd4ebd2747db943c484de7b4784ee32c53c946e0a45ec30ea2f94d15420b`; NEW is the previous experiment's runtime OFF/ON binary `52e9014ab509563e61cfc5dc3bc67deb376866678c65c2e8330adc8db57bf039`. Their compiler identity, core features and release-profile fingerprints match. The new experiment adds its cfg hook and diagnostic API; this is not a claim of whole-binary identity.

Four exact symbol specializations are fixed from the earlier [link-profile audit](../2026-10-31-sdk-link-flags176/README.md). Its saved FROZEN disassembly and source-bound symbol metadata are reused. Only the new binary is decoded: one saved symbol listing and four function disassemblies, each once. No successful previous decode, build, model, test, profile, timing or verifier is replayed.

|Function specialization|Bytes per build|Instruction rows per build|IFMA sites per build|Constant loads per build|
|---|---:|---:|---:|---:|
|Canonical signed G1, owned operands|26,282|4,423|800|79|
|Canonical signed G1, borrowed operands|26,230|4,428|800|81|
|Canonical borrowed G2|51,862|8,591|1,450|169|
|Crossbeam Stealer::steal|356|84|0|0|
|Total|104,730|17,526|3,050|329|

The comparison preserves instruction offsets, mnemonics, registers, masks, literal immediates, stack operands and local branch offsets. RIP-relative literal loads are resolved through ELF file-backed segments and compared by the exact bytes read:658 load sites across both builds. It also preserves external call signatures and address-reference alias graphs. The three arithmetic kernels have no unresolved RIP-relative operands.

Two scalar MOV sites in each stealing function read the writable `COLLECTOR` global. Their symbol, field offset, load width and writable ELF segment match. These are matching static bindings, not proofs of equal dynamic collector/TLS state. External callee bodies, pointed-to object semantics, allocator behavior, invocation counts, runtime data, placement, OS activity and critical-path scheduling remain outside this audit. Matching all scoped rows is not a semantic proof of the full executable.

The result removes instruction-body or loaded-literal changes in these functions as a supported explanation for this particular control discrepancy. It does not identify a backing/page/cache/alignment cause. Earlier [G1 control-code audit](../2026-11-06-g1-control-codegen176/README.md) and [executable-backing diagnostics](../2026-10-28-sdk-control-copies/README.md) retain their original scopes and outcomes; no alignment, compiler, copy-placement or allocator sweep follows here.

## Retained tooling limitations and validation

The first observer failed while asserting that every demangled symbol in the executable was globally unique. Unrelated duplicate names made that assertion invalid; no function disassembly had yet executed. Its source, successful compile, failed receipt and saved symbol listing remain. The corrected observer filters only the four declared specializations and reuses that listing; it does not rerun `nm`.

The inherited normalizer initially reported five apparent arithmetic differences, all external allocator-call labels. The saved old disassembly is already demangled, while the new one contains raw mangled labels; trimming every trailing angle bracket removed a meaningful generic delimiter only from the old form. The stealing primitive also had three such label differences and two unresolved scalar global loads per build. A separate refinement reads the saved outputs, removes only the outer symbol delimiter and resolves the writable symbol bindings. All original comparisons remain intact. No disassembler or native code is rerun or changed for the refinement.

Publication review passes on its first execution:195 initial,202 corrected and223 refinement pins; five successful command receipts and one preserved failed receipt; all four function pairs and exact normalized-file hashes; all658 constant-load records and four writable-global binding records. It checks the stored comparison output without independently reimplementing the normalizer or proving instruction semantics. [Review and summary](artifacts/review.json), [design](artifacts/design.md), [corrected plan](artifacts/plan-fixed.json), [refinement plan](artifacts/refine-plan.json), [pins](artifacts/refine-pins.json).

## Research disposition

History review located existing negative GLV/KH, symmetric-norm, centered-Montgomery, R260, exact-FMA, Bos–Coster, fixed-factor FFT and scalar-reuse studies. They are not new optimization proposals or unchanged retries. Rechecking [ePrint2026/1902](https://eprint.iacr.org/2026/1902) confirms the bucket-set paper already used locally. The newly inspected [PMNS truncation paper](https://arxiv.org/html/2610.09767v1) discusses large multiprecision coefficients/moduli; its reported gains do not establish a BN254 five-limb improvement. No paper speedup is transferred and no new arithmetic candidate is selected.

The next source-level lead is configuration lookup ownership: most hot-path options are already cached in scratch, but some dispatcher calls still read environment variables per batch/job. Their actual complete-request frequency and allocation cost need evidence before a caching change. No global cache, environment-semantics change or performance claim is introduced here. Work removal on ordinary MSM windows remains the main target; this control audit supplies no twofold path by itself.

## Accepted CPU and Metal comparisons, inherited

There are zero new SDK requests, query invocations, timings, profiles or proofs. Production176, accepted SDK6d83 and the35,171-proof inventory are unchanged. The same accepted binary previously measured7.915/7.929/7.996ms locally for1×2; its latest observation is8.003ms. The cause of inter-run variation remains unconfirmed, with no adopted regression established.

|Shape|Accepted local ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 /4.623×|24 /2.999×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|merge-24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Source: [preceding full comparison](../2026-11-08-indexed-tile-bundles176/README.md). Seven nonprimary rows remain from the earlier full campaign. Merge54 remains162.240109ms without a supplied M5 reference. The all-eight verified CPU10× objective remains active and unmet, **0/8**. No AWS/Aeglos, huge-circuit, mobile/ARM/TEE, minimum-RAM, pressure/swap or concurrent-throughput result is added.

[Exact observer/disassembly/comparison/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) retain the failures and refinements. All UTF-8 member hashes and the gzip roundtrip are checked. Executables/libraries, private point/scalar/key/witness/proof arrays, raw performance recordings, unchanged full production source and full papers are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/query-control-codegen176`.
