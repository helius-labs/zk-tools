# Shared proof local witness classification SDK pilot

The nine-fixture count/semantic screen passed in wire-union-census175. Build
an actual native caller implementation, rather than another isolated count.
The current canonical-wire entry covers 1x2 and 2x2; other paths stay unchanged
and get all-nine correctness preflights. This pilot does not qualify all shapes.

Merge validated sorted A/B indices without materializing their union. A
HashTable<u32> stores representative wire indices, hashing and comparing the
existing four-word witness values. Dense representative labels plus one reused
local remap reconstruct stable original groups. Outputs keep the existing ABI.
No persistent plan/lookup table/point array, scalar copy for the global map,
witness reuse, custom pool, arithmetic change or unsafe operation is added.

Classify concurrently with the existing product FFT using one Rayon join.
This front-loads classification before MSMs and can delay B despite reducing
total hashing. Measure the complete SDK cost, including remaps and allocations.
Outer None selects original per-query grouping. Attempted/declined grouping is
represented separately and never causes a second original grouping pass.

Scratch admission checks the global hash table including growth, two dense
u32 vectors and both sample tables. It is an additional Reservation::analysis,
held through classification/remapping. Each unchanged group/consumer forecast
is separately reserved and held through its consumers. Existing required
workspace forecasts retain FFT scratch; none is borrowed as optional credit.
If any checked cost/admission fails, drop prior tokens and use the original
path. These are resource-aware checks, not a new fixed cap or RAM minimum.

Private cfg zopro_shared_wire_groups; five originals change reversibly and one
child is added. Hashbrown0.16.1 native-only/default-features=false/inline-more.
All17 field source files remain exact, with accepted qualification inherited.
Unfiltered OFF/ON core/prover suites; forced-collision/stable-metadata oracle,
zero/empty/overlap/disjoint/fullword cases and checked-cost overflow coverage.
Freeze actual native libraries and link the unchanged standard SDK driver.

Before timing: all9 shapes x accepted/OFF/ON, 1 request after3 primes, with each
export independently Gnark verified. Then31 fixed rotated/reversed primary32
blocks x3variants,151 requests after3 primes. Then6 FIFO-gated199Hz cycles:u
profiles, primary16/32 x3variants,251 requests after3 primes. Record CPU IDs
and timestamps upfront. All captures finish before remaining Gnark checks,
owner-user decode/loss checks, hashes/archives/helper compilation. No owned
CPU workloads overlap; no AWS resources are created.

Prospective speed pilot gate unchanged: ON/control upper paired ratio<1 and
>=21/31wins against BOTH accepted and fresh OFF. OFF/accepted interval must
fit[1/1.03,1.03]. Ranks10..22 give nominal97.055% individual median coverage
under independent/exchangeable blocks. No minimum gain floor, extension,
exclusion, unchanged reroll, threshold rescue or selected subset adoption.
Observed RSS and CPU time retained separately; speed gate is not a permanent
ban on a future declared memory/nonregression qualification.

Expected15954 internally verified SDK requests and126 fresh Gnark exports.
Prior ledger21007; expected21133 only after verification and freshness checks.
No production adoption from a pilot. A positive result requires larger paths,
full shape/portable/resource/final-artifact qualification. The original all8
CPU10x goal remains active and is not replaced by this pilot gate.
Commit and push the completed experiment before another optimization.
