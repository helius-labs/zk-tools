# Direct reduction of the signed Euclidean finish

The arithmetic checks pass, but the timing comparison is **unqualified**: the intended fresh OFF control actually linked the ON library. The candidate is not adopted, and the identical ON/ON comparison does not reject the arithmetic idea. Production and the accepted SDK are unchanged.

The post-capture identity audit found that Cargo's primary-target `target/release/libzopro_bn254.rlib` path was overwritten by the later candidate build before the probes linked. Both fresh and candidate executables have SHA-256 `79343e35d9ae23a0f841965b0fb90d50ba775434261e7f602f97472576c8f955`. Their selected library hash is also identical. The capture pins correctly describe those bytes, but the harness did not enforce the intended different identities. This is a harness selection error, not a mathematical failure. The [control audit](artifacts/control-identity-audit.json) recovers distinct immutable OFF/ON libraries through the hashed `.rmeta` companion paths and checks their Cargo flags. A corrected OFF probe links successfully with a different hash; it was not executed. No completed test or timing was repeated. A subsequent actual-caller study must enforce immutable library paths and distinct OFF/ON hashes before capture.

The current binary-GCD inverse ends with ordinary one-word Euclid. Its final signed linear form is reduced by one radix64 Montgomery cancellation and a full field multiplication to restore that scale. The candidate reduces that signed 320-bit form directly using a two-word constant reciprocal, up to three negative corrections and one existing canonical subtraction. It preserves the Euclidean loop, 31/62-bit matrices, coefficients and field representations. No variable division is used by the new reducer, while the unchanged Euclidean loop still uses division. There is no heap buffer, lookup table, key copy, new plan or assembly change. [Bounds and derivation](artifacts/bounds.md) state the exact supported modulus/input assumptions and fallback.

The [previous coprime-finish candidate](../2026-10-25-coprime-finish-sdk/README.md) changed termination and failed SDK qualification; that change is absent. Prior signed62 divsteps, fused coefficient updates and first-limb reuse are also separate experiments. The existing inverse follows [Pornin's binary GCD](https://eprint.iacr.org/2020/972); the author's [practical inversion article](https://purplesyringa.moe/blog/faster-practical-modular-inversion/) was inspected for background. Its timing figures are not used as BN254 predictions. The direct finish reduction and its bounds are local derivations.

Both fresh OFF and selected ON pass all 34 native field tests. The initial probe link failed because its libc argument was missing and the product-tree helper belongs to the core wrapper rather than the field vector. Preserved logs show a harness compile failure before any probe ran. Linking libc and copying the exact retained helper repaired the harness; no field test or measurement was repeated. One new test checks 68,672 distinct signed full-width residues across Fq/Fr against Arkworks, with zero/near-modulus values and extreme opposite-sign coefficients. The same test runs in both builds; repeated execution is not counted as new distinct inputs. Existing exponentiation and full inversion tests pass. The probe copies the accepted core eight-lane product-tree body exactly into a generic field wrapper; every output is independently checked against Arkworks. The original finish is recovered byte-for-byte by removing the dispatch/include; 171 other original private files and all 172 production files match accepted hashes. Core and prover sources are unchanged.

The fixed screen uses 31 rotated/reversed blocks, CPU2, and accepted-library, intended-fresh (actually ON), and selected-ON probes. Each process runs three same-size primes then 16 complete passes over 4096 independent nonzero inputs. Loading, canonical input decoding and reference setup are outside timing; the scalar inverse or existing eight-lane product-tree inverse, batch packing, extraction and output writes are inside. Every process checks all outputs against an independent Arkworks inverse fixture and reports zero timed allocations. Entries are medians of 31 process means in ns per output element.

| Field | Caller | Accepted ns/element | Intended fresh, actually ON ns/element | Candidate ns/element |
|---|---|---:|---:|---:|
| fq | scalar | 543.693 | 522.285 | 528.657 |
| fq | batch8 | 78.847 | 103.384 | 89.339 |
| fr | scalar | 748.512 | 735.813 | 736.140 |
| fr | batch8 | 79.660 | 77.485 | 78.563 |

| Field | Caller | Archived label ratio | Median | Conditional interval | Wins | Check |
|---|---|---|---:|---:|---:|---|
| fq | scalar | fresh/accepted | 0.985443078 | [0.979102987, 0.993693482] | 31/31 | pass |
| fq | scalar | candidate/accepted | 0.989355743 | [0.979081058, 0.994229113] | 31/31 | pass |
| fq | scalar | candidate/fresh | 0.999804061 | [0.999321263, 1.000983425] | 17/31 | fail |
| fq | batch8 | fresh/accepted | 0.987075834 | [0.980349103, 0.990026604] | 30/31 | pass |
| fq | batch8 | candidate/accepted | 0.988243191 | [0.980402668, 0.989741865] | 29/31 | pass |
| fq | batch8 | candidate/fresh | 0.999941201 | [0.998991751, 1.000642508] | 16/31 | fail |
| fr | scalar | fresh/accepted | 0.980594216 | [0.978444763, 0.983714735] | 31/31 | pass |
| fr | scalar | candidate/accepted | 0.980641747 | [0.979239317, 0.983440581] | 31/31 | pass |
| fr | scalar | candidate/fresh | 1.000425248 | [0.998968769, 1.001064658] | 14/31 | pass |
| fr | batch8 | fresh/accepted | 0.971563253 | [0.964150531, 0.973369005] | 31/31 | fail |
| fr | batch8 | candidate/accepted | 0.971305082 | [0.965141638, 0.972363524] | 31/31 | pass |
| fr | batch8 | candidate/fresh | 1.000125059 | [0.998901306, 1.001295939] | 14/31 | pass |

Labels in the tables preserve the original process names: every `fresh` row is a second ON measurement. Candidate/fresh ratios therefore cannot estimate an ON/OFF effect. Candidate/accepted Fq paired median ratios are 0.989356 scalar and 0.988243 batch, but the missing OFF arm prevents attributing those differences to this arithmetic change. No gain is qualified. Fq process times also change sharply around block 15; that bimodal distribution makes ratios of independently displayed medians misleading. All process means and paired observations are retained, without assigning a cause or excluding blocks. The original statistical checks below remain as declared, not as valid ON/OFF conclusions.

The 10th–22nd ordered ratios have nominal 97.055063% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. Fresh/accepted calibration must fit [1/1.03,1.03] in all four contexts. Both Fq callers require candidate upper ratio below one and at least 21 wins against both controls. Fr upper ratios must be at most 1.02. There is no gain floor, exclusion, adaptive extension or unchanged reroll.

All 372 latency processes precede four separate FIFO-gated 199 Hz cycles:u profiles. Scalar profiles have 1024 passes and batch profiles 8192 passes, each after three same-size primes. All profiles precede decode, hash and archive work. No owned helper, build or other CPU benchmark overlaps capture. Profiles bracket the measured loop/accounting, exclude setup/primes/oracles, and have one sampled PID and zero lost records. Their symbol shares are not removable full-proof wall-time fractions.

| Caller | Variant | Samples | Lost |
|---|---|---:|---:|
| scalar | fresh | 433 | 0 |
| scalar | candidate | 429 | 0 |
| batch8 | fresh | 523 | 0 |
| batch8 | candidate | 498 | 0 |

Detailed means, CPU usage, process RSS, allocation counts, symbols and pins are in [analysis](artifacts/analysis.json). Component RSS includes decoded fixtures and output buffers; it is not prover RAM or minimum memory. No new SDK/Gnark proof was generated; ledger remains 12,785. No new actual MSM caller, all-eight/54, portable/mobile/TEE, pressure/concurrency, large-circuit or AWS/Aeglos result is claimed. The missing OFF arm prevents timing qualification. The next useful check is a separately declared actual-caller study with audited immutable OFF/ON libraries; this completed primitive schedule will not be rerun.

The original all-eight 10× M5 CPU objective remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. These local proof timings and supplied references are inherited.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted primary historical peak RSS is 134.50 MiB. Merge 54→1 remains 161.341835 ms / 1,910.1875 MiB historical peak with no supplied M5 reference. Current-source AWS/Aeglos has not been remeasured here.
