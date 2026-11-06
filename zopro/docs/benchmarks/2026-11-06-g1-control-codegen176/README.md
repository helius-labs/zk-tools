# G1 control code audit

The G1 stream function and its three arithmetic callees match across the original reference, two-chain control and descriptor-group control after relocation normalization. **No arithmetic-body difference was found in this scope.** This does not explain the G1 calibration shift, rescue the [descriptor trial](../2026-11-06-g2-descriptor-chunks176/README.md), or establish an optimization.

The previous trial's unchanged-source G1 OFF/FROZEN estimates were 1.043611, 1.041298 and 1.031897 at 1/16/32 workers. Those results remain unchanged. This audit reads existing binaries and disassemblies; it launches no new timing, profile, proof or native test.

| Compared function | Bytes per build | Instruction rows per build |
|---|---:|---:|
| G1 stream | 26,198 | 4,425 |
| FqBatch inverse_nonzero | 6,979 | 1,139 |
| invert_lanes | 4,008 | 706 |
| Scalar Fq inverse | 7,723 | 1,739 |
| Total instruction rows | — | 8,009 |

Both new controls match the reference in every normalized row of all four functions. There are zero unresolved RIP-relative operands. The comparison preserves instruction offsets, mnemonics, registers, masks, literal immediates, stack operands and local branch offsets. It resolves 110 constant-load sites per build through ELF file-backed segments and compares the exact bytes read, rather than masking all relative addresses. External call signatures and the alias graph of 29 address-only references also match.

This is a scoped disassembly comparison, **not bit-identical ELF code or a proof of identical runtime behavior**. Address-only references are not decoded into semantic objects. Allocator and panic callees are outside scope; the scalar inverse has only panic calls beyond its inspected arithmetic body. The helpers and raw saved inputs are retained for review; no separate semantic reviewer is claimed.

The original G1 entry is at offset 48 modulo 64, while both later entries are at offset zero. That placement observation does not establish an alignment cause. The two later builds share that entry alignment despite their different historical calibration outcomes. Cache placement, runtime data, frequency, OS activity and other causes were not isolated. No alignment, attribute, compiler or allocation-placement sweep follows from these observations. Earlier [large-buffer alignment](../2026-10-24-memory-alignment/README.md) and [prefix-placement](../2026-10-28-g2-runtime-layout/README.md) results remain closed.

## Evidence and decision

Four source-pinned Rust/xtask steps complete once: the original symbol/disassembly extraction, main-function comparison, two hot-callee comparisons and the remaining scalar inverse comparison. They check 6/10/17/21 pin occurrences respectively. The nine additional callee disassemblies supplement the three original main-function disassemblies. No successful extraction, test, capture or comparison is repeated. A pre-execution review corrected the normalizer to retain operands preceding a RIP-relative memory operand; its corrected source is the one used by every comparison.

The exact existing binary hashes are:

- Original caller: `f07229ce665cf61c972037a27752bb53cce8c5dd58be0c71d8776434c961064b`.
- Two-chain control: `19a37c35830e0300a3040fc9d82a3b16d384ed0560a218c779dd375aed399c91`.
- Descriptor control: `740708e5ab03491c26b5db4e843b0af95c5a6bdf2c9634652b514d6471dfd99c`.

The closed descriptor trial still fails its declared screen. This audit selects no runtime change and supplies no basis for repeating it unchanged. It rules out differences in the inspected arithmetic instruction bodies and loaded constants as the explanation to pursue next. Remaining placement/runtime explanations are unproven; they are not a new prover speed claim.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references. No new measurement of either machine is made.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Accepted primary RSS132.75 MiB is inherited, not remeasured. Source176/SDK6d83 and the 30,846-proof ledger remain unchanged. The complete all-eight CPU10× objective remains active/unmet, **0/8**. No AWS/Aeglos, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.

[Summary](artifacts/summary.json), [extractor](artifacts/extract.rs), [normalizer](artifacts/compare.rs), [callee builders](artifacts/prepare-helpers.rs), [scalar-leaf builder](artifacts/prepare-scalar.rs), [exact source/disassembly/command receipts](artifacts/source-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Executables/libraries and runtime key/witness arrays are excluded.
