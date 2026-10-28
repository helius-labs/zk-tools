# Interpretation

The fused symmetric norm is not adopted. The declared pilot gate fails three
conditions: primary32 ON/accepted gain, merge54 ON/accepted nonregression and
merge54 ON/OFF nonregression. The other four required checks pass. No gate is
waived, and no capture is extended, excluded or repeated.

Primary32 ON/OFF improves by a paired 0.53305%, ratio 0.994669471452, bounds
[0.972928174848, 0.999106395701], seven wins out of seven, with a 0.043803 ms
paired saving. This measured improvement is retained. ON/accepted is
1.000607796124 [0.985819409315, 1.005104052038], three wins and a -0.004949 ms
paired saving; it does not establish a gain against the accepted control.
Displayed block-median latencies are accepted8.142533, OFF8.217384 and
ON8.126523 ms. Ratios of displayed medians differ from medians of paired ratios.

Fresh OFF and accepted SDKs have exactly the same SHA25681033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20.
They are different executable paths and processes, with the same arithmetic,
libraries and machine-code bytes. Their primary32 paired ratio is
1.006903035002 [0.996475253305, 1.013249934374], passing calibration. The
remaining process/path/address-layout/host/time effects are not isolated.
The fresh-control gain must not silently replace the accepted comparison.
The identical bytes rule out a compiler-code difference between these two
controls; they do not prove stable latency or a specific cause of variation.

The primary32 CPU point estimate decreases187.765 ->186.562 ms/request versus
OFF and187.642 ->186.562 versus accepted. Primary peak RSS median/max is
OFF131.25/135.25 MiB, ON130.75/133.00 and accepted131.00/133.75. This is a
small local observation, not a minimum-RAM or general memory-pressure result.

At16x2 both paired estimates improve: ON/OFF0.996811180 and ON/accepted
0.997946151, but both intervals include parity. Its nonregression checks pass;
OFF/accepted descriptive calibration fails at its lower bound0.963888894.
Merge54 retains an ON/OFF improvement point estimate0.997012883, four wins,
0.485145 ms saving, with bounds[0.974186083,1.036043323]. ON/accepted is
1.003227876 [0.984734958,1.034816547], one win. Both upper bounds exceed the
declared1.03 threshold. Merge54 observed ON peak1905.09 MiB exceeds
OFF1892.81 and accepted1832.01; no universal memory saving is claimed.

Physical16 ON/OFF has a large positive point estimate0.899353503 and five wins,
with wide bounds[0.721255051,1.083933449]. Its identical-byte OFF/accepted
control is1.111401815 [0.978069952,1.393956016], failing descriptive calibration.
ON/accepted is0.998636845 [0.961051722,1.109286174], five wins. Neither the
physical16 point estimate nor displayed OFF median justifies a10% gain claim.

Separate primary32 PMU captures show cycles/request accepted856.087million,
OFF857.046million and ON847.862million. Instructions are respectively
1453.699,1453.581 and1453.087million. All24 event measurements run100% of
enabled time. This is one counter capture per context, with no seven-block
confidence statement. The modest instruction change is consistent with the
additional symmetry bookkeeping; reducing product IFMA count is not the same
as reducing total instructions by40%. Counter changes do not identify ports,
cache levels, exclusive kernel cost or a removable proof-time fraction.

All six199Hz cycles recordings have one sampled PID and zero lost events,
44084samples total. Physical16 profiles have17sampled TIDs, logical32 have33:
the main thread can participate alongside configured Rayon workers. CPU shares
include arithmetic and runtime work and are not critical-path measurements.
Profiling occurs after latency and does not replace it.

Correctness: OFF33 and ON35 native field tests,122native core and8prover tests
per mode pass. The new norm tests compare36distinct lazy boundary pairs and
32768full-width random pairs against Arkworks (edge pairs repeat across SIMD
lanes),512dependent eight-lane chains, and normalized output bounds. The
random generator's raw top word is masked to62bits; it is not a uniform sampler
over the entire[0,2p) interval. Separate boundary pairs include2p-1. Existing
core G2 point/MSM/tree oracles and fresh complete SDK proofs cover integration.
Optional unavailable-fixture scenarios are not claimed. No new portable,
overflow-checking build or forced-pressure qualification is claimed.

The new six-coefficient schedule adds both square diagonals before each
Montgomery cancellation. Thirty product IFMA source lines are duplicated for
the second operand, giving60product IFMA instructions instead of100 in the
general fused product. Cross terms are doubled, diagonal terms are not, and
previous reduction contributions are never doubled. With normalized operands
below2p and4p<R=2^256, T<8p²<2pR and REDC<3p; the accepted reduce_fused
correction restores below2p. This removes real arithmetic but affects only the
G2 norm portion, not all curve additions or all proof work. There is no claim
that this limited transformation alone could reach the3.7ms primary target.

All234 workflow steps finish. The9672 SDK requests verify internally;108fresh
164-byte exports independently pass Gnark with public binding and freshness,
ledger15325 ->15433. All source175/library/executable/helper/input pins pass
after capture. Native92599 and capture37729 are terminal/reaped0. No owned
build/helper/hash/archive/verifier or second benchmark overlaps proof capture.
Only the six owned root0600 perf files are chowned after completion for local
hash/archive access. No global kernel setting or probe is installed.

Close this fixed symmetric-norm schedule without adoption or unchanged reroll.
The instruction saving is real, and the measured44µs primary OFF comparison is
not zero. The full observed gate does not establish an accepted-prover gain or
large-shape qualification. Future work must supply a distinct mechanism,
preserving this result and the earlier G1 square/storage/scheduling negatives.
Accepted174/SDK81033 and its inherited all-eight CPU/Metal table remain.
The original all-eight10x CPU goal is active/unmet0/8. No new AWS/Aeglos,
giant, mobile/ARM/TEE, minimum-RAM, swap, concurrent-proof or pressure result.
