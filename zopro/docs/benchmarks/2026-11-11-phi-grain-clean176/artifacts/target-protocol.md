# Clean default source: all-shape C7a qualification

Freeze before ANY clean-stage SDK preflight. Goal remains verified single-proof
CPUAVX>=10x EACH supplied M5CPU reference; report CPU+Metal ALL8. This is an
intermediate adoption qualification, not a smaller completion definition.

Three frozen consumers under the original accepted driver/allocator:
REF=d5e1b2c981fb9bcab2beb69e3b2af5608d298d792c7e3814e87273b04509662e accepted;
PARENT=c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe qualified
experimental min128; CLEAN=actual new coherent8lib default source, no phi cfg.
PARENT differs algorithmically from REF; no false parity calibration is imposed
between them. CLEAN must preserve or improve both frozen controls, including
resource guards, and reproduce a96-worker gain versusaccepted. Gains vsPARENT
are allowed; no lower-bound calibration rejects an additional positive result.

Local functional preflight: all9CLEAN +primaryREF/PARENT at4workers, each1request
after3primes:11exports/44calls. Canonical50351->50362 ifcomplete. This is not a
local timing campaign. Existing native/generic and freshproof validation required.

Target contexts, fixed beforeclock: original9shapes at prior accepted best
counts (1x2/2x2 at32, remaining7 at64); extra1x2 at16/64/96 and2x2 at96.
Thirteen distinct contexts, all declared/gated. Primary1x2 contexts151requests,
allothers31, after3primes/process. Target preflight all13x3arms,1request+3primes
=39exports/156calls. Main9rotated/reversed blocks x13contexts x3arms=351processes.
Eachblock rotates contextlist left byblock%13, reverses onoddblocks; arms
REF/PARENT/CLEAN rotateleft byblock%3 and reverse onoddblocks. All main timing
processes before their Gnarkverifiers. Main23841timedsamples/24894calls;
cloud390exports/25050calls, combined401exports/25094calls. Finalledger50752
ifcomplete. Cloudtimed samplecount23880 including39preflights.

SDK timer includes fresh assignments, solving, blinding, proving, internal
verification and serialization. Load/primes/export/reader outside SDK timer.
Process arithmetic means; summarize median9blockmeans. Pair exactcontext/block.
Sorted pairedratio ranks1/7 (zerobased), nominal96.09375%marginal coverage under
independent exchangeable blocks; nofamilywise/stationarity guarantee. All66
checks required before performancequalification:
* For ALL13contexts, CLEAN/REF andCLEAN/PARENT latency upper<=1.02 (26checks).
* For ALL13contexts, CLEAN/REF andCLEAN/PARENT CPU/request upper<=1.02 (26).
* For ALL13contexts, maxCLEANlifetimeRSS <=max(REFmax,PARENTmax)*1.05+1MiB (13).
* Primary1x2_96 CLEAN/REF latency upper<1 (1gaincheck), no minimumgainfloor.
CPUuser+system sums include ordinaryloopwork between SDKcalls, not exclusively
SDK CPU; lifetimeRSS includes load/primes/preparation, not minimumRAM. Report
allratios/bounds/wins/absolute times/CPU/RSS/faults, originalALL8 CPU+Metal
comparisons,54noM5reference. Primarybestcountselection descriptive acrossall4
declaredcounts; don'tpretend it is a new matched Aegloscomparison. NoAeglosrun.

Allnative/generic/defaultsource semantics, prooffreshness andbindings required;
source/release resource/concurrency review remains required before actual
adoption. New source is175exact+1reversible original; pure in-place iteration
granularity only, no new sharedmutable state/unsafe/heap/table/keyplan/pool/RAMcap.
Do not claim physicalARM/phone/concurrentthroughput from these measurements.

No sampleexclusions/extensions/contextselection/thresholdchange/unchangedrerun
afterresults. Freeze producer, independentreader, models, inputs/order/gates
before firstcontainerSDK request. Only poll exactlivehandleduringANYpreflight/
recording; noownedhelper/build/hash/analysis/report/push overlap. Fresh164B
exports independentGnarkverified; transportedexports reverified once locally
without doublecount. Partialfailures retained; ledgerupdated onlyforverified
freshproofs, no successfulscience replayforbookkeeping.

Exactlyoneownedc7a24xl dev/eu-north-1,96physical/noSMT checks; currentnetwork
noingress verified and noownedlivebenchmarks beforelaunch. UniqueRunID/clienttoken,
encrypteddeletevolume,90minautomatictermination with>=110mincredentiallife.
Local package/preflight complete beforepublication/launch. Container96CPUquota/
30GiB/noSwap/networknone/PID2048/capdropALL/no-new-privileges, harnessconfiguration
notarchitecturalRAMcap. Physicalhostexclusiveuseunverified. Retrieve SHA/size
boundarchive; removeandverify ownVM/EBS/ECRtag/S3. Nootherresources touched.
CommitANDpush eachfinishedexperiment; fullALL8goal remains unmetuntilproven.
