# Four-component G2 with one bucket state

Declared before new implementation or timing. Previous goal turn progressed:
sum3 dense Poseidon tested/rejected/pushed8e23863d. Current production175/20fde
remains1x2=8.022165ms/max134MiB. All8CPU10x objective active/unmet0/8, CPU/Metal
comparisons and physical16 calibration exception retained. No budget limit.

The 22Oct four-component GLS trial used four separate component bucket fills,
endomorphism transforms and three component merges per window/partition. It
failed and remains closed. New mechanism: reuse the accepted borrowed Window
and one bucket state through four component passes. No original method changes.
For component sums S0..S3 and commuting maps phi^3=1 and psi, execute:
feed S3; phi; feed S2; phi^2*psi; feed S1; phi; feed S0.
Result S0+phi*S1+psi*S2+phi*psi*S3. Map both affine and deferred-projective
accumulators after each feed has drained its batch/queue. Thus three component
merges and separate component bucket arrays disappear. Original scalar lattice,
quotient and signed65-bit packing are reused with explicit old-source pins.

One fixed geometry, selected before measurements: six signed11-bit windows
(65 magnitude bits plus Booth carry fit66), four point partitions.24 fill jobs
are comparable to accepted11windows*2parts=22. Each new bucket set has1024slots
versus2048; no narrow indexed final window is needed. These counts are not a
timing prediction. Four input traversals, more scalar scratch, transform work,
changed collision rates and scheduling may lose.

No copied/transformed key, point LUT, new field primitive, pool or duplicate
solver plan. A new40-byte-per-scalar vector is proof-local. Before allocation,
checked typed costs charge the entire new scalar vector and candidate geometry
in addition to existing proof forecasts through Reservation::analysis. This
uses dynamic headroom and explicit caller limits, not a new global cap. Failed
admission/allocation retains original phi2 path without mutating input words.
Native eligibility matches the existing1024..65535 exclusive G2 path; all other
sizes/targets keep original behavior. No mobile/giant feasibility claim follows.

Qualification: unchanged17field files inherit33 native field tests; fresh
124OFF/128ON core and8prover tests each. Four added tests cover decomposition
and digit reconstruction, affine/projective map sequence, complete MSM against
independent generator sums with tails/exceptions, and denied admission/checked
costs/input preservation. Actual linked dispatch and a trace-only ON primary
preflight confirm activation; no trace in timed/profiler loops.

If qualified, fixed165-step primary SDK pilot:7902SDK requests/78freshGnark
exports, ledger17473->17551; all9shape preflight;15rotated/reversed151-request
blocks+3primes;3stat and3profiles. All helpers compiled and all pins frozen
before captures, no other owned work during capture. Both paired upper gain
bounds<1 with>=10/15wins and OFF/A calibration within1.03. No gain floor,
excluded/extended/repeated successful captures or changed thresholds. Failure
closes exact candidate. A pass permits separate all8+54/physical16/portable/
resource/clean qualification, not automatic adoption or goal completion.

Publish completed experiment, including negatives, with polite factual report.
