# Drain suitable collision queues through the existing affine batch

Declared before builds/capture, 29 October 2026. Previous goal turn is progress:
component partitioning was tested and rejected; it remains closed. Accepted175
and SDK20fde stay unchanged; all8 CPU10x targets remain active/unmet.

Existing borrowed G1/G2 Window::flush_queue sends queued contributions to scalar
projective accumulators, including at the end of input after execute cleared the
pending busy bits. The new native cfg zopro_queue_drain may first execute/drain
through the ORIGINAL affine batch when the last SIMD-width queued destinations
are distinct and the current pending batch is empty or at least SIMD width.
It repeatedly uses the existing take_from_queue/execute, then retains original
projective fallback for a short or colliding prefix. Identity/equal/opposite
points keep complete handling. A distinct destination is not a guarantee of an
ordinary pair; exceptions may leave a partial arithmetic batch. No field formula,
queue insertion, scalar width/decomposition, query readiness or Rayon change.

No sorting, allocation, extra buffer, field representation, key table, copied
points, plan or pool is added. Queue decreases by at least SIMD width on each
successful drain. Existing queue bound is below original batch capacity, so
pending batch/scratch remain within original maximum geometry. Window fields
and workspace forecasts remain byte-exact. Actual retained scratch/projective
capacity and RSS can differ with execution; no minimum-RAM claim follows.

Private176files:3parents(msm_affine module, borrowed G1, borrowed G2) plus one
queue predicate child. OFF must preserve original code, including source line
positions in old functions. Full native OFF/ON core/prover qualification, with
new destination/sign/threshold predicates and independent generator bucket
oracles for pending/queued/exception/reuse/activation cases. All17field files
exact; inherit33field tests. Freeze coherent libraries and original SDK driver.

New fixed pilot: all9w32 shapes xaccepted/OFF/ON plus primary16 x3 each1+3,
independent Gnark BEFORElatency.15blocks primary32 CPUs0-31 and physical16
CPUs0-15 x3variants151+3. Rotate/reverse variants; topology order fixed32then16.
All90latencies before6four-eventPMU31+3 and6cycles:u fixedperiod16M251+3.
Allremaining102Gnark+owner/decode/metadata/loss after ALLcaptures. Allhelpers
inclanalyzer,publisher,summary compiled BEFOREcapture. No ownedcompiler/hash/
archive/analysis/helper/secondCPUworkload overlaps capture; hostshared.

Primary32 gain BOTHcontrols upper<1 and>=10/15wins. Physical16 nonregression
BOTHcontrols upper<=1.02. OFF/accepted calibration intervals BOTHcases inside
[1/1.03,1.03]. Max lifetimeRSSON/control<=1.05 BOTHcases/controls. These new
prospective requirements do not reinterpret component-partition's failed gate.
Ranks4..12of15 nominal96.484375%individual conditional coverage under independent
exchangeable blocks, no familywise. No gain floor/exclusion/extension/unchanged
rerun/threshold revision/subset adoption. Success requires separate all9
performance/resource/portable qualification before any adoption.

Exactly294workflowsteps/15708completedSDKrequests/132new164BdistinctGnarkexports,
prior22366->22498. Six groupedcounterpanels require>=99.9%coverage, sixprofiles
CPU/TIME/PERIOD/MMAP+onePIDzeroLost checks. Known period undercoverage means
profiles qualitative only, not localCPI/exclusivecost/removablewallfractions.
Keepall8CPU/Metal accepted comparisons. Publish/commit/push either outcome.
No newAWS,phone/ARM/TEE,hugecircuit,pressure/swap/minRAM,concurrency/throughput.
