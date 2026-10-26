# Borrowed pending operands in fused K/H

The 32-worker primary screen improves against both controls in all three blocks. The screen does not advance: one 16-worker OFF/accepted calibration ratio is 0.902882, outside the declared factor 1.05 bound. The positive candidate is retained; production is unchanged. No minimum improvement floor is imposed.

The private Rust candidate stores `(encoded bucket, &Affine)` in the fused K/H window's pending batch and collision queue. These records are 16 bytes instead of 72 on this host. It preserves operand order, sign encoding, exceptional additions, LIFO queue behavior, batch sizes, the original short-top algorithm and weighted reductions. Both borrowed slices outlive the whole window, including the segment boundary. Ordinary A and every G2 path remain owned. The existing G1 batch kernel is generic over `Borrow<Affine>`; its body matches accepted exactly after replacing operand access and removing whitespace. The retained owned K/H body is byte-identical. Three original files and one new private module change; 167 other originals and all 170 production files remain unchanged.

The specialized window also omits two legacy scalar-inversion vectors unused by the selected packed path. The used SIMD inversion scratch remains. Nominal capacity reduction is 39,200 bytes for pending records plus 22,400 bytes for unused scalar scratch per ordinary window (width 13, batch size 350), or 1,170,400 bytes across 19 ordinary windows. This is not simultaneous live peak, minimum RAM or a measured whole-process memory saving. The existing forecast remains conservative. No new key table, persistent cache, transformed points, solver plan, worker pool, image format or universal cap is added. Runtime dispatch requires the new option plus existing native IFMA, deferred-sign, packed and direct guards. Unsupported inputs retain the original path.

Host: Ryzen 9950X, 32 workers on CPUs 0–31 and 16 physical workers on CPUs 0–15. The exact accepted binary is compared with one freshly linked candidate OFF/ON, using the identical standard SDK driver and native/fat-LTO consumer settings. Three rotated/reversed blocks use fresh processes, each with 17 timed requests plus three primes. Timers include witness construction, fresh blinded proving, verification and serialization; key loading is excluded. Helpers, builds, hashes and archiving do not overlap SDK captures.

|Workers|Accepted ms|OFF ms|ON ms|Paired ON/accepted|Paired ON/OFF|Wins accepted/OFF|Paired saving vs OFF µs|ON CPU ms/request|ON peak RSS MiB|
|---|---:|---:|---:|---:|---:|---|---:|---:|---:|
|32|8.509716|8.599656|8.336420|0.980017841|0.973358908|3/3;3/3|305.987|192.642|97.00|
|16|12.080319|10.907097|10.198020|0.944861724|0.972008823|2/3;2/3|334.951|123.180|76.50|

Displayed medians are independent; ratios and savings pair process medians within blocks. At 32 workers, paired ON/accepted is 1.998% faster and ON/OFF 2.664% faster. The paired median saving is 159.230 µs versus accepted and 305.987 µs versus OFF. All primary improvement and both-case nonregression checks pass. At 16 workers, OFF/accepted block ratios are 1.005637, 1.011631 and 0.902882; the final block fails calibration. Candidate gains in that case therefore do not qualify adoption. These observations neither prove a 16-worker regression nor authorize excluding the control. No successful capture was repeated or excluded. The full matrix, cycle profiles and pressure qualification were conditional on advancement and were not run.

Validation passes 33 native field tests, 112 core tests, two raw tests and six cubic tests, the generator check, portable/no-IFMA consumers and 11/14 field tests respectively. New kernel checks cover 120,744 generator-based output comparisons across tails, signs, storage modes, inverse modes and repeated scratch use. New full-query tests check 72 borrowed and 72 owned sums against generator oracles across real primary dimensions, one/four workers, widths 12–14 and six scalar families. Collision/overflow reuse and 72/16-byte record sizes are checked separately.

All nine OFF/ON preflights produce 72 internally verified SDK requests and 18 distinct exports independently verified with Gnark. The short screen adds 360 requests and 18 exports. Total: 432 requests, 36 distinct Gnark exports and 72 proof/public members, bound to pinned public fixtures and fresh against 6,850 earlier identities; next ledger 6,886. Closing source/library/binary/input/tool pins and all 170 production sources are checked. The SDK is local at `/tmp/zopro-borrowed-pending/native/on-sdk`, SHA256 `1351b278e5474e5f5d36d516375db05e12c7ff36cf37e061132afe4c252a9738`. The source overlay and command/test/capture/proof receipts are archived. A helper's initial PathBuf borrow error was repaired before SDK capture; the repair note is retained.

The experimental primary median of 8.336420 ms at 32 workers corresponds to 4.438× the supplied 37 ms M5 CPU reference and 2.879× the 24 ms Metal reference. No other shape has a new latency measurement here. The table below retains the accepted controls from the [latest full matrix](../2026-10-26-pairing-x-chain-sdk/README.md), using supplied historical Apple references, not new Apple measurements. None meets 10× CPU.

|Shape|Previous full accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|
|---|---:|---:|---:|---:|---:|
|1x2|8.642453|37|4.281×|24|2.777×|
|2x2|11.756497|59|5.019×|30|2.552×|
|4x2|16.230727|109|6.716×|48|2.957×|
|8x2|24.740679|189|7.639×|74|2.991×|
|12x2|34.551946|284|8.220×|101|2.923×|
|16x2|40.814660|335|8.208×|124|3.038×|
|24x2|68.811062|528|7.673×|176|2.558×|
|merge-24x1|67.064555|507|7.560×|175|2.609×|
|merge-54x1|177.082271|—|—|—|—|

Merge 54 has no supplied reference. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM or throughput measurement is claimed. The all-eight CPU 10× goal remains active and unmet.

[Design](artifacts/design.md), [allocation clarification](artifacts/source-review-addendum.md), [analysis](artifacts/screen-analysis.json), [source audit](artifacts/closing-source-audit.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).
