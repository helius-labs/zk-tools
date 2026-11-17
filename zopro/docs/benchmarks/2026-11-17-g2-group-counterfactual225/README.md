# G2-only grouping bypass counterfactual225

Removing the G2 grouping attempt **does not reveal a primary proof-latency
gain**. Against accepted production, the paired candidate is1.38% slower at16
workers and0.99% slower at32; grouped2x2 is5.08% slower. All calibration and
RSS screens pass, as do all39 fresh Gnark exports. This fixed counterfactual
is closed without adopting a bypass, selecting circuit sizes or building a
grouping-decision cache from these results.

The preceding [exact-production stack study224](../2026-11-17-clean-stack224/README.md)
found page-safe grouping with about0.68ms/1.13ms inclusive G2 grouping wall
spans at16/32 workers. Retained212 input counts imply the primary full map
eventually declines the unchanged80% diversity cutoff. Those observations
motivated testing the complete caller before designing a cache. They did not
prove that removing the span would shorten the proof. The old global
dedup-off174 experiment also changed A/K/H grouping and forecasts; it is not
this isolated intervention.

|Context|Accepted REF ms|Retained fresh OFF ms|Bypass ON ms|Paired ON/REF|ON/REF wins|Paired ON/OFF|ON/OFF wins|
|:---|---:|---:|---:|---:|---:|---:|---:|
|1x2,16 workers|8.880708|8.814593|8.948206|1.013824|0/3|1.015360|0/3|
|1x2,32 workers|7.619736|7.650364|7.643793|1.009876|1/3|1.000813|1/3|
|2x2,32 workers|10.746605|10.680857|11.292093|1.050759|0/3|1.047325|0/3|
|merge54,32 workers|161.260924|164.102027|163.598415|0.994921|2/3|0.996114|2/3|

Times are medians of process means; paired ratios are medians of within-block
ratios and may differ in direction from quotients of displayed medians. These
are three-block descriptive screen estimates, without confidence/adoption
claims. Primary32 does not demonstrate either a reliable gain or the exact
size of a regression. Every observation/backing is retained. Merge54's full
paired ratios include one candidate/REF1.043507 and candidate/OFF1.034519;
its favorable medians do not establish a selective optimization.

|Context|ON/REF CPU ratio|ON/OFF CPU ratio|REF/OFF/ON peak RSS MiB|
|:---|---:|---:|:---|
|1x2,16 workers|0.998533|1.000808|79.75 /83.50 /58.75|
|1x2,32 workers|1.031004|1.026506|129.25 /130.00 /74.75|
|2x2,32 workers|1.104752|1.099698|295.50 /282.00 /188.00|
|merge54,32 workers|0.987304|0.980881|1593.48 /1603.89 /1566.22|

The peak-RSS reductions are real observations within this screen, including
loading and priming. They are not minimum RAM, live private-byte reductions or
pressure/swap qualification. The memory savings do not establish faster
proving; CPU is essentially flat at16 primary and higher at32 primary and2x2.
Keeping forecast source unchanged does not guarantee identical allocation
overlap, retained allocator state, admission outcomes or scheduling.

The private source copies all179 accepted files and changes only three cfg
lines in `g2_query_exclusive`. Under `zopro_probe_skip_g2_groups`, its local
`Option<ScalarGroups>` is `None`; the original expression remains verbatim
under the inverse cfg. Every other grouping entry, workspace forecast,
admission implementation, scalar split, key, field and dependency stays exact.
No runtime environment lookup is inserted before the grouping call. There is
no table, point/key copy, new pool or arbitrary RAM cap. Source reversal
recovers the original parent exactly, independently checked before SDK capture
and again in the raw review.

The ON native core and dependents are coherently rebuilt with the exact194
driver/support and flags plus this cfg. Both existing exclusive-G2 tests pass,
with132 other core tests filtered. They exercise generator-referenced complete
queries and exclusive conversion. Unchanged field and other qualification is
inherited; there is no new portable, ARM/phone or concurrent-SDK qualification.
No adoption is intended for this diagnostic branch. LTO removes the standalone
G2-exclusive symbol in ON, so changed inlining/call context accompanies the
removed work. This is a whole-consumer counterfactual, not a controlled
instruction-by-instruction arithmetic upper bound or a unique causal proof.

REF is exact accepted194:
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
OFF is retained unchanged-production222:
`fa101d27e74c6d7f9ca4b701da83f7e1a75effdecfc076fa4edf26de3b6e1565`.
Neither control is rebuilt here. ON is
`30bc31a81d612ab6ea00d10467f275eabc7b617b26bb6b4c9cc80b27ceafa0c3`.
Each arm uses three independent byte-identical backings, all participating.

Three4-worker functional smokes precede36 main processes: three fixed matched
blocks ×four contexts ×three arms, with rotated/reversed order. Each main
primary process contains251 timed requests,2x2 contains51 and merge54 seven,
all after three primes. The exact counts are39 exports,5,160 SDK calls and
5,043 timed records; these derived counts and the frozen jobs resolve the
earlier scratch arithmetic retained in the protocol. Every timed request
includes solving, fresh randomness, proving, SDK verification and serialization.
Loading, primes and export are outside timing. Accepted certified-output,
fused-preparation and indexed-bundle opt-ins remain enabled for all arms.

The predeclared primary signal requires a paired median below1 with at least
two wins against both controls at one of16/32 workers, without a gain floor.
It fails at both counts. OFF/REF medians stay inside factor1.03 everywhere;
all RSS screens pass1.10×control+8MiB. CPU is reported, not an undisclosed
veto. Grouped2x2 is an explicit tradeoff/correctness control rather than a
primary-signal veto; even a primary success would not permit universal bypass
adoption. No thresholds, samples or gates change after capture.

All actual build/test/link, SDK, verifier and two raw-reader steps pass first
execution. All main timing finishes before late Gnark verification and
analysis, with no owned CPU cojobs during recording. Separate readers agree
on all raw means, pairing, decisions, inverse source recovery and78 SDK/verifier
receipts with156 stream hashes. No new PMU/profile or saved-running-CPU claim
is added. The previous224 profile qualifies only its unchanged194 artifact.

This result does not support a grouping-decision cache for latency on the
tested inputs. The next diagnosis should determine which clean-production
query controls the final join, using sparse boundaries in the unchanged
binary. Prior profiles combining all G1 callers do not identify that deadline.
The changed call graph and memory overlap here remain alternative explanations
for why removing optional work did not improve the proof; no unique cause is
selected by this screen.

Canonical ledger is now **60,085**:
`/home/ubuntu/src/zopro-perf-work/g2-group-counterfactual225/proof-sha256-after-main.json`.
Its exact digest is in the independent review. Full raw evidence stays private
under that root; binaries, keys, witnesses, raw proofs and full ledgers are
excluded from the repository package. No AWS resources or GitHub push occur.
Production194/196 and its [local CPU/Metal comparisons](../2026-11-17-certified-all196/README.md)
remain accepted; [201 C7a results](../2026-11-17-accepted-c7a201/README.md)
remain the latest AWS evidence. The all-eight CPU10× objective stays active,
unmet0/8. Merge54 has no supplied M5 reference.
