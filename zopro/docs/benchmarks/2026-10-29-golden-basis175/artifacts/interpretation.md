# Offline interpretation

This text was written after the first successful capture and analysis. All
pinned source, tests, binaries, capture helpers, analyzers and publishers remain
unchanged. No successful runtime work was repeated. The precompiled publisher's
original report text remains reproducible from its archived source.

The trace-one polynomial allows a cheaper multiplication than ordinary complex
arithmetic in the measured chains, but the chosen exact-divide-by-three square
has a much longer dependency and correction path. Its additional residue
selection is one IFMA-high operation; digit division contributes five IFMA-low
operations, so the complete square has 206 sites rather than 200. Describing
only the five exact-division products would omit the residue-selection cost.

The cost measured outside multiplication is decisive: the aggregate regresses
15.02% and the norm-affine expression 24.10% versus fresh. This result rules out
this fixed recipe under the declared component gate. It does not prove that
every extension basis or every possible square algorithm is slower. A materially
different recipe would require its own prospective design, model and gate; these
failed checks must remain in the record.

Read-only `objdump -Cd --no-show-raw-insn` dumps were created before capture and
are pinned in pins.json. Static counts below include each whole named function,
including loop/control code and cold branches. The four-stream loop retains one
shared body; these counts are not multiplied by iteration count or streams.
Counts of stack mentions are textual operands referencing rsp, not measured
spills or memory stalls.

|Native body|Ordinary instructions|Candidate instructions|Ordinary IFMA sites|Candidate IFMA sites|Ordinary rsp operands|Candidate rsp operands|
|---|---:|---:|---:|---:|---:|---:|
|probe::streams|723|903|300|250|87|184|
|probe::squares|645|921|200|206|62|117|
|probe::norms|725|820|350|350|71|94|
|probe::affine|3178|3990|1250|1156|407|755|

The exact frozen and fresh ordinary versions have the same counts in every
listed body. Counts do not prove byte identity or exclude layout effects. Fewer
IFMA sites do not imply fewer total instructions, and these static observations
do not isolate a hardware cause. The six sampled profiles contain 6–49 samples
each, so detailed symbol attribution is not supported. No resampling follows.

There is no key-format, point-caller, SDK or production implementation of this
basis. Excluding conversion made the arithmetic screen optimistic; the failed
result supplies no reason to spend memory on prepared-key expansion. All prior
cached-hash SDK memory and 54-to-1 latency gains remain valid within their own
failed universal speed gate.
