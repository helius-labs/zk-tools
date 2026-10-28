# Same-code executable identity diagnostic

This compares unchanged frozen/OFF G2 MSM executables while varying mode label and pathname. It does not rerun the borrowed-queue optimization gate or qualify a candidate. Production and accepted SDK remain unchanged; the original component gate remains failed.

Seven fixed rotated/reversed blocks cover six cases,42FIFOgrouped counter processes. Each uses30,089points,onepoolworker pinnedCPU2,seed17,3checkedprimes and7measuredMSMs. Every output matches the generator oracle and all outputs agree across cases. No candidate ON is included.

Original frozen modes0/1 and original OFF modes0/1 isolate output label changes with the same path/inode. Equal-length control-a/control-b aliases are hardlinks to those executables, preserving each original inode and byte identity while changing its path. Alias comparison still has different backing inodes and diagnostic line/build-ID data. Paths/inodes/content hashes are recorded. These are controlled interventions, not an exclusive cause of any observed difference.

| Case | ms/MSM | CPU ms/MSM | Cycles/MSM | Instructions/MSM |
|---|---:|---:|---:|---:|
| alias_frozen0 | 42.201525 | 42.200714 | 228703226.857 | 574553477.143 |
| alias_off0 | 61.561337 | 61.559714 | 336536091.143 | 574553506.714 |
| frozen0 | 42.211888 | 42.211143 | 228638201.714 | 574553477.143 |
| frozen1 | 42.247819 | 42.246143 | 228537233.286 | 574553557.429 |
| off0 | 61.549289 | 61.547714 | 336436130.714 | 574557708.429 |
| off1 | 61.541919 | 61.539714 | 336497278.429 | 574557935.429 |

| Comparison | Paired median | Wins |
|---|---:|---:|
| alias_frozen0/frozen0 | 1.000445069 | 2/7 |
| alias_off0/alias_frozen0 | 1.456665499 | 0/7 |
| alias_off0/off0 | 0.999418806 | 4/7 |
| frozen1/frozen0 | 1.000496563 | 2/7 |
| off0/frozen0 | 1.458161291 | 0/7 |
| off1/frozen1 | 1.457386519 | 0/7 |
| off1/off0 | 0.999905539 | 4/7 |

All event groups100%running: **true**. All7ratios and observations are retained; no confidence or adoption claim, exclusion, extension or unchanged reroll. Event windows include their common control boundary; genericcache events are not DRAM bytes/bandwidth. CPU/cycle differences alone do not isolate clocks, cachecoloring, aliasing, allocator layout or scheduling.

There are294countedMSMs+126primes, not proofs. No native rebuild/test/SDK/Gnark calls occur. Both source-verified parent binaries and754parent pins are rechecked, along with new compiler/helper/alias identities. No build/helper/hash/archive/otherowned benchmark overlaps counters; host snapshots do not prove exclusive use. No ASLR/globalkernel settings change. No new pressure/minimumRAM/mobile/TEE/giant/AWS/Aeglos claim. Proofledger13,181; all172production/SDK14da unchanged.

## Inherited CPU and Metal comparison

The originalall8CPU10xobjective remains active and unmet,0/8. AcceptedFFTengineeringexception/falsefullgates remain. These are inherited accepted local timings/suppliedM5references, not measurements of this diagnostic.

| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37.0 | 24.0 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59.0 | 30.0 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109.0 | 48.0 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189.0 | 74.0 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284.0 | 101.0 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335.0 | 124.0 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528.0 | 176.0 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507.0 | 175.0 | 7.790× | 2.689× |

Historical primary134.50MiB; merge54 161.341835ms/1910.1875MiB without suppliedM5reference. No proving-time improvement is established.

[Design](artifacts/design.md), [identities](artifacts/identities.json), [all results](artifacts/analysis.json), [workflow](artifacts/runs/result.json), [parent borrowed-queue study](../2026-10-28-g2-borrowed-queue/README.md). Captured logs retain bytes, including trailing blank lines; authored sources/report whitespace checked separately.

## Interpretation and limits

The original frozen control measures42.212ms/MSM and OFF61.549ms atmode0; equal-length hardlink aliases measure42.202ms and61.561ms. OFF/frozen paired median is1.45816 in original paths and1.45667 through aliases. Label-only ratios are1.00050 for frozen and0.99991 for OFF. Alias/original ratios are1.00045 and0.99942. Thus the large gap persists across the tested relabeling and path changes; those interventions do not recover it.

Retired instructions remain approximately574.55million/MSM across all six cases. Frozen/alias-frozen take about228.6–228.7million cycles; OFF/alias-OFF about336.4–336.5million. The cycle/CPU-time quotient is similar (roughly5.41–5.47GHz), so a simple lower-frequency account is inconsistent with these observations. Genericcache misses and branch misses are also similar, but they cannot rule out instruction/frontend, cache-level, address-aliasing or other dependency effects.

The six changed Rust panic-line fields and buildID remain associated with each executable, as do its backing inode and mapped code pages. Program headers also match by read-only inspection. This study does not isolate inode identity, physical placement, virtual layout or diagnostic metadata as the cause. FIFO control paths vary with case labels and are part of the process environment, so equal executable-path lengths do not establish equal complete process layout. No unsupported causal or proof-speed inference is made.

The next distinct intervention is fresh byte-exact copies of both controls, with equal-length names and one shared FIFO pathname, followed by relevant frontend counters if available. This can test whether the discrepancy follows original backing or original ELF contents. It is a control diagnostic, not an optimization gate rerun or adoption. Keep the borrowed-queue study's failed gate, its6.81% physical16 improvement over original frozen and its requested-memory saving intact.
