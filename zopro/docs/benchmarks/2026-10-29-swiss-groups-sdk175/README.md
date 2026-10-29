# Index only grouped scalar dictionary in full SDK proofs

**The full-proof pilot fails its gate; production remains unchanged.** ON measures **8.011907 ms** versus **8.031194 ms accepted** and **8.822366 ms fresh OFF**. The paired saving versus accepted is **22.313 µs**, with bounds spanning parity and 20/31 wins. OFF is 10.04% slower by its paired estimate and fails calibration. The clear gain over fresh OFF does not establish a speed gain over accepted production.

Observed maximum lifetime RSS is **102.50 MiB ON versus 133.50 MiB accepted**, 31 MiB or 23.22% lower in this primary capture. This positive memory observation is retained; it does not establish minimum RAM, pressure behavior or a universal reduction. The exact SDK recipe closes without adoption or an unchanged reroll.

## Candidate and parent evidence

The [parent component](../2026-10-29-swiss-groups175/README.md) passes its declared constructor gate on real A/B scalar inputs. Its 55.93% aggregate gain remains valid in that compiled context, alongside the already recorded change in original-control timing. It is not transferred into a full-proof percentage.

The private snapshot uses pinned hashbrown 0.16.1 `HashTable<u32>`, with grouped control-byte probing, full four-word equality and rehashing through the existing owned values. The exact parent runtime is retained, plus one native forced-collision/growth test. No new persistent key table, point copy, witness reuse, pool, arithmetic primitive or execution plan is added. The existing grouping sample, admission, cutoffs, stable class order, zeros and allocation forecast remain.

Three original files change reversibly: grouping.rs, core Cargo.toml and Cargo.lock. One child is new: 176 private files, 172 of 175 originals exact. All 17 field files remain exact and the new cfg is not a field selector; the 33-test accepted field qualification is inherited without claiming arithmetic binary identity. Fresh OFF passes all 124 core and eight prover tests; ON passes 125 core and eight prover tests. The standard SDK driver and accepted native flags remain, with only the private cfg added for ON.

The initial setup's inverse audit removes matching dependency lines globally and fails before creating any build plan or running tests. Its audit is corrected to invert only the zopro-core dependency stanza. Initial private sources are retained. All ten subsequent native build/test/freeze/link steps pass on their first execution; no successful step is repeated. Source flags, frozen actual dependencies and linked grouping disassembly are pinned. New dictionary symbols appear only in ON, while both controls retain their original grouping route. [Source audit](artifacts/source-audit.json), [linked audit](artifacts/map-code-audit.json).

## Verified primary latency

Ryzen 9950X, 16 physical/32 logical CPUs, 32 workers pinned to CPUs 0–31. Thirty-one fixed rotated/reversed blocks compare accepted SDK 20fde with fresh OFF/ON; each process measures 151 requests after three primes. Timed work includes fresh witness solving, randomized proving, SDK verification and serialization. Loading/primes are excluded from latency and included in lifetime process RSS. CPU sums user+system time across threads.

|Variant|Median latency ms|CPU ms per request|Median / maximum RSS MiB|
|---|---:|---:|---:|
|accepted|8.031194|182.108|130.50 / 133.50|
|OFF|8.822366|196.167|129.75 / 134.50|
|ON|8.011907|182.187|97.00 / 102.50|

|Comparison|Paired ratio|Bounds|Wins/31|Paired saving µs|
|---|---:|---|---:|---:|
|1x2-w32/OFF/accepted|1.100402956|[1.095713809, 1.101948962]|0|-800.830|
|1x2-w32/ON/OFF|0.907667496|[0.903037230, 0.909997208]|31|810.519|
|1x2-w32/ON/accepted|0.997222755|[0.995722920, 1.000449909]|20|22.313|

Displayed cells are medians of process medians. Paired ratios/savings use matched blocks and need not equal differences/ratios of the marginal medians. The prospective gate requires upper ratio<1 and at least 21/31 wins against both controls; fresh OFF/accepted calibration bounds must fit [1/1.03,1.03]. ON/OFF passes, ON/accepted fails, and calibration fails. All failures and the small positive accepted-relative estimate remain. There is no gain floor, exclusion, extension, threshold waiver, layout/flag selection or unchanged reroll.

Sorted paired ranks 10–22 of 31 give nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. The accepted build's historical physical-16 calibration exception is not transferred to this candidate.

The fresh OFF control's regression and unchanged constructor source expose sensitivity to compiled context. These results do not isolate a compiler/cache/allocator cause, nor show that RAM limits explain a latency difference. No extra source/flag/layout trial is selected to repair the failed control after observing results.

## Independent verification and automatic CPU profiles

All nine shapes × three variants pass 27 preflights, each one request after three primes and an independent Gnark verification, before latency. All 93 latency captures precede six separate FIFO-gated 199 Hz cycles:u profiles: primary 16/32 × three variants, 251 requests after three primes. All 99 latency/profile captures finish before their remaining 99 Gnark checks and before owner-user decode/loss checks. Physical 16 profiles are diagnostic and do not qualify 16-worker latency. No owned helper/build/hash/archive or second CPU workload overlaps capture.

All **15,954 SDK requests** verify internally. All **126 fresh 164-byte exports** pass independent Gnark, exact image/public binding, hashes and freshness against 20,881 earlier exports. Their 252 proof/public members are archived losslessly. The distinct-proof ledger becomes **21,007** only after those checks. All 276 capture/offline steps complete.

|Profile|Samples|Lost records|
|---|---:|---:|
|cycles-1x2-w16-OFF|6173|0|
|cycles-1x2-w16-ON|5238|0|
|cycles-1x2-w16-accepted|5205|0|
|cycles-1x2-w32-OFF|9731|0|
|cycles-1x2-w32-ON|9162|0|
|cycles-1x2-w32-accepted|8941|0|

Each profile has one benchmark PID and zero lost records; full symbol periods remain in the analysis. Flat residence and function sizes do not establish exclusive wall time or a cause of control drift. Raw recordings, executables and native libraries remain local with hashes.

## Supplied M5 comparisons

Only primary 32 latency is newly measured. The following uses supplied historical M5 references of 37 ms CPU and 24 ms Metal, rather than new Apple measurements.

|Variant|Primary ms|M5 CPU speedup|M5 Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|accepted|8.031194|4.607×|2.988×|3.7|
|OFF|8.822366|4.194×|2.720×|3.7|
|ON|8.011907|4.618×|2.996×|3.7|

All eight complete accepted-production comparisons remain inherited below, with the accepted physical 16 exception and true final-artifact gate retained. They are not candidate full-shape measurements.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The original all-eight CPU 10× objective remains active/unmet 0/8. There is no new AWS/Aeglos, ARM/mobile/TEE, forester 250/4999, swap, minimum-RAM, active-pressure or concurrent-throughput qualification. The candidate's lower observed primary RSS and positive standalone constructor result remain evidence within their measured scopes; neither changes this failed full-proof gate.

[Design](artifacts/design.md), [all blocks and profile weights](artifacts/analysis.json), [build/measurement plans](artifacts/run-plan.json), [verification inventory](artifacts/proof-inventory.json), [source archive](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proof/public archive](artifacts/proofs.json.gz), [accepted local/AWS summary](../2026-10-29-current-cpu-summary/README.md).
