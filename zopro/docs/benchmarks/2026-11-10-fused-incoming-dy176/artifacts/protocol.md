# Fused incoming quarter and signed Y difference

Prospective10November2026. Previous goal turn PROGRESS: distinct deferred-tag
format qualified and closed FAIL9/48, pushed37e85d3548. No live owned inherited
job/cloud. Goal unchanged: verified CPUAVX single-proof >=10x M5CPU all8 shapes,
report CPU+Metal; active/unmet0/8. Acceptedlocal32primary7.783593581ms unchanged.

Fresh source inspection rejects the proposed tagged raw-difference fusion as
a conversion elimination: finish_g1/finish still consume converted ay, so that
conversion survives. No code, timing or success claim for that proposal.

Distinct mechanism: return to fixed-mask UNTAGGED quarter format (snapshot
scalar-quarter-codec176), retaining the bucket value needed for final Y. Fuse
only incoming canonical Y quartering and signed subtraction. For b=4a+r<p,
p=3mod4 and T=(p+1)/4, quarter(b)=a+r*T<p. Compute positive
2p+a+r*T-ay or negative3p-a-r*T-ay with one signed radix52 carry chain,
instead of first normalizing quarter(b) then subtracting ay in another chain.
ay<2p; outputs (0,3p) or (0,3p], respectively; negative b=ay=0 gives3p.
Select sign with mask-subtract of each unnormalized incoming-quarter digit.
Every such digit <4*2^52, signed intermediate magnitude <2^55, within i64.
Final carry0. Fused helper Wide3 bound inclusive. No tagged/lazy words in Fq.
This changes the full G1/G2 caller, without new tables/keycopy/expanded solver,
bitmap/allocations/larger buckets/new pools. No scalar parameter/worker rescue.

Before clocks: native independent BigUint exact and OLD-path comparisons for
all256 sign masks, 256 boundary/residue/lazy populations and64changing groups
per mask (131072 or more lane comparisons); canonical b0/p-1 and residues,
ay0/p/2p-1, inclusive3p, reused/changing output-fed canonical inputs. Both
helper output exactwords and modulo bounds, untouched inputs. Four existing
prefix/completeWindow/every-worker/shared-pool/layout tests on coherent new
field/core; 24conversion allocation cases + fullWindow requested-live panel.
All source changes reverse exactly to181-member parent; portable guards and
inactive fallback unchanged. Emitted old/new combined incoming+dy probes show
static instructions/branches but do not predict complete-caller speed.

If qualified, one frozen four-arm actualWindow campaign. REF retainedaccepted
API-only; BASE newcoherent core originalWindow; OFF retained fixed-maskquarter
candidate (scalar-quarter-codec176); ON fusedincomingquarter+dy. Parent/control
choice is prospective: this tests fusion without the closed deferredtag path.
Same exact independentArkworks fixtures,15045points/w12/column0/batch200,
G1/G2 workers1/16/32,CPU i/solo2,512KiBstacks. 3primes17timedwaves,
15rotated/reversed context/arm blocks. Construction/feeds/scalarpaths/queues/
normalization/fulloutput equality/checksum/drop included; input load/hash/pool
excluded. One Window per worker perwave, not wholeMSM/proof/serverthroughput.

48prospective checks unchanged:6BASE/REF factor1.03 calibrations;
18ONnonreg upper<=1.02 against BASE/OFF/REF;3primaryG232 and3equal-log
G1/G232 gains upper<1 and>=10/15wins against all3;6RSS maxcontrols*1.05+1MiB;
12PMU/profile quality. Ranks4..12 of15 nominal96.484375% individual coverage,
exchangeability assumption, nofamilywise guarantee. No gainfloor/exclusion/
extension/unchangedrerun/favorablegroup-worker rescue after capture.
24preflights360latency then6PMU+6cycleprofiles (2048waves,FIFOwarmgate,
period16M/512pages,>=1000samples/unknown<=5%/zeroLOST/PIDTIDCPU/MMAPchecks,
PMUcoverage>=99.9%) then30offline steps=426steps/396callers/906752checkedWindows.
Freeze source/deps/tools/binaries/inputs/order/thresholds/producer/rawreader
before preflight. During ANY preflight/recording ONLYpoll exact live handle
until terminal; no helper/build/hash/analyze/report overlap. Timeouts and
truncation not terminal. Preserve failures; reuse successful scientific steps.

Commit AND push completed experiment; polite factual report CPU+Metal ALL8
inherited explicitly. Passing component requires fullquery/ALL8+54SDK/fresh
Gnark verification and resources beforeadoption. Canonical44265 unchanged;
seed allnew proofexports frontend-overlap176/proof-sha256-after-capture.json.
Rust1.98.1+serializedxtask; no questions/subagents/Python/CASM/giantLUT/newpool/
expandedkeys/solverplans/arbitraryRAMcap. No newAWS needed for this mechanism.
