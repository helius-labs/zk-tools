# Active-chain inversion in short addition trees

Component advancement: **false**. Production remains unchanged. This measures serial arithmetic layers, not proof latency; the all-eight CPU10× objective remains active and unmet. Small reliable gains have no minimum cutoff.

One-pair layers improve by about 17% for G1 and 10–11% for G2 across both seed families, winning all seven blocks against both controls. The short-case aggregate also improves by 0.43–0.44%, with seven of seven winning block aggregates. These positives are retained. However, 13 of 40 cases exceed the 2% nonregression limit against both controls, with regressions up to about 9.5%; control calibration passes throughout. The implementation therefore closes without a full SDK trial. Its average gain does not erase those measured regressions.

The candidate starts from accepted source and changes only the ordinary affine-tree inverse when no full SIMD group exists and one to seven lanes are active. A scalar Montgomery batch inverse processes the active values using3(m−1)field multiplications and one existing field inverse. Inactive chains have value one and inverse one. Empty layers, full groups, scalar point formulas, exceptional compaction, streaming kernels, reduction topology and scheduling remain unchanged. Only fixed stack arrays are used; no table, cache, new execution plan or heap storage is added.

## Actual-layer measurement

Ryzen9950X, CPU2, one serial thread and no Rayon pool. Both G1/G2; requested pairs1/2/3/4/5/6/7/8/9/16 and two seed families. Every fresh process cycles16 distinct dense curve-valid fixtures so inversion inputs vary across calls. Identical controlsA/B call an exact accepted layer copy; candidate calls the changed real layer. Seven rotated/reversed paired blocks, three primes per stage and4096calls per stage. Reset is outside each kernel timer; caller time includes reset, clock and loop overhead. All modes share prewarmed fixture addresses and scratch. Per-call timing overhead is retained. All40cases/840stages are included, with no successful rerun or posthoc count selection.

|Curve|Pairs|Seed|A µs/layer|B µs/layer|Short µs/layer|Short/A|Short/B|Wins/A|Wins/B|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|g1|1|0|0.711|0.710|0.590|0.829526|0.829857|7|7|
|g1|2|0|0.812|0.811|0.726|0.894246|0.894882|7|7|
|g1|3|0|0.911|0.911|0.856|0.940735|0.941112|7|7|
|g1|4|0|0.994|0.994|0.984|0.991198|0.991308|7|7|
|g1|5|0|1.100|1.100|1.122|1.019176|1.019600|0|0|
|g1|6|0|1.188|1.187|1.244|1.046949|1.047890|0|0|
|g1|7|0|1.268|1.268|1.377|1.086056|1.085845|0|0|
|g1|8|0|0.754|0.756|0.755|1.002060|1.000468|1|3|
|g1|9|0|0.877|0.877|0.878|1.001120|1.001912|1|0|
|g1|16|0|0.933|0.933|0.921|0.987741|0.987549|7|7|
|g2|1|0|0.926|0.926|0.826|0.893086|0.892849|7|7|
|g2|2|0|1.194|1.194|1.179|0.988239|0.987800|7|7|
|g2|3|0|1.461|1.462|1.485|1.016020|1.015829|0|0|
|g2|4|0|1.714|1.714|1.800|1.051216|1.051140|0|0|
|g2|5|0|1.972|1.973|2.104|1.066665|1.066883|0|0|
|g2|6|0|2.239|2.238|2.420|1.080663|1.081121|0|0|
|g2|7|0|2.515|2.514|2.745|1.091506|1.091716|0|0|
|g2|8|0|0.933|0.934|0.930|0.997020|0.996540|7|7|
|g2|9|0|1.285|1.286|1.279|0.995162|0.994858|7|7|
|g2|16|0|1.254|1.254|1.250|0.996828|0.997310|7|6|
|g1|1|1|0.717|0.717|0.596|0.830881|0.830874|7|7|
|g1|2|1|0.804|0.804|0.720|0.896126|0.895295|7|7|
|g1|3|1|0.903|0.902|0.854|0.946007|0.946974|7|7|
|g1|4|1|0.996|0.997|0.979|0.983134|0.982623|7|7|
|g1|5|1|1.081|1.081|1.113|1.029989|1.029463|0|0|
|g1|6|1|1.169|1.169|1.242|1.061767|1.062270|0|0|
|g1|7|1|1.259|1.258|1.377|1.093659|1.094590|0|0|
|g1|8|1|0.759|0.759|0.758|0.999367|0.999266|5|5|
|g1|9|1|0.879|0.879|0.880|1.000997|1.001592|2|0|
|g1|16|1|0.921|0.921|0.922|1.000436|1.000502|1|1|
|g2|1|1|0.919|0.920|0.826|0.898251|0.898056|7|7|
|g2|2|1|1.203|1.202|1.173|0.975452|0.974857|7|7|
|g2|3|1|1.466|1.467|1.477|1.007152|1.007246|0|0|
|g2|4|1|1.727|1.726|1.806|1.044420|1.044909|0|0|
|g2|5|1|1.988|1.988|2.100|1.056898|1.057201|0|0|
|g2|6|1|2.250|2.251|2.408|1.069985|1.069954|0|0|
|g2|7|1|2.531|2.532|2.731|1.077995|1.078134|0|0|
|g2|8|1|0.930|0.932|0.930|0.999689|0.998468|6|7|
|g2|9|1|1.279|1.279|1.274|0.996525|0.996857|7|7|
|g2|16|1|1.265|1.265|1.261|0.996918|0.996766|7|7|

Times are medians of stage averages, ratios are geometric means of paired block ratios and can differ from ratios of displayed times. [Rows](artifacts/rows.json) retain kernel/caller time, CPU, faults, context switches, RSS and scratch capacities. Component RSS is not whole-prover memory or minimum RAM. Timed allocations are zero.

## Frozen decision

Every identical-control B/A case must have median within factor1.03 and geometric ratio within factor1.05. Candidate geometric ratio must be≤1.02 against both controls in every case, including unchanged full cases. The changed short-case within-block aggregate must improve against both controls with at least4/7 winning block aggregates. There is no minimum gain floor, per-block veto, exclusion or threshold reselection. A component pass supports a separate standard SDK study; it cannot establish end-to-end speed. A failure closes this implementation.

Control failures: `[]`.

Nonregression failures: `["g1-p6-s0 short/control-a geometric1.0469486175870886","g1-p6-s0 short/control-b geometric1.0478895980060496","g1-p7-s0 short/control-a geometric1.0860559406481447","g1-p7-s0 short/control-b geometric1.085845483561889","g2-p4-s0 short/control-a geometric1.0512161745117392","g2-p4-s0 short/control-b geometric1.0511404644292606","g2-p5-s0 short/control-a geometric1.0666652936798426","g2-p5-s0 short/control-b geometric1.0668828495297151","g2-p6-s0 short/control-a geometric1.0806631691129847","g2-p6-s0 short/control-b geometric1.0811211396805758","g2-p7-s0 short/control-a geometric1.0915055943664957","g2-p7-s0 short/control-b geometric1.0917164280633358","g1-p5-s1 short/control-a geometric1.029988949626866","g1-p5-s1 short/control-b geometric1.0294629098222394","g1-p6-s1 short/control-a geometric1.0617665855742437","g1-p6-s1 short/control-b geometric1.0622701624967559","g1-p7-s1 short/control-a geometric1.0936588311692363","g1-p7-s1 short/control-b geometric1.0945903417707519","g2-p4-s1 short/control-a geometric1.0444200917939204","g2-p4-s1 short/control-b geometric1.0449087028111355","g2-p5-s1 short/control-a geometric1.0568983808711145","g2-p5-s1 short/control-b geometric1.057201125698256","g2-p6-s1 short/control-a geometric1.069984626927189","g2-p6-s1 short/control-b geometric1.069954393110974","g2-p7-s1 short/control-a geometric1.0779949483121","g2-p7-s1 short/control-b geometric1.0781344719825459"]`.

Aggregate failures: `[]`.

Changed-short aggregates: `{"control-a":{"block_ratios":[0.9955934543050958,0.9962276328265224,0.9954351720166458,0.9958539654729526,0.995255398892498,0.9956773514157514,0.9953975927843108],"geometric_ratio":0.9956343206884046,"median_ratio":0.9955934543050958,"wins":7},"control-b":{"block_ratios":[0.9964308353694892,0.9960097243697072,0.9953486883913596,0.9955886131403872,0.9954845365263996,0.9955158799570384,0.9958328245957014],"geometric_ratio":0.995744382229758,"median_ratio":0.9955886131403872,"wins":7}}`.

## Correctness and profiling

All112 native core tests pass, including three new tests. Inverse checks use independent per-element inversions across1024boundary/random samples for every active count. Complete-curve checks cover G1/G2, both inverse modes, dense and ragged exceptional cases, empty/short/full boundaries and scratch resets. Every backing point and count matches the accepted reference; live outputs also match independent scalar complete-curve additions. Raw2/cubic6, generator and portable/no-IFMA consumer checks pass. Field source and its accepted qualification are unchanged. The reference copy and public fixture are diagnostic-cfg-only. Initial overlay verification caught one stale copied source, reconciled against the current170-file manifest before candidate edits and native testing; no capture used that stale copy.

After all latency, four separate warmFIFO999Hz cycles:u profiles capture candidate G1/G2 at one and seven pairs,65536calls each. They cover the full caller, not isolated timer brackets. Each capture samples one own PID/TID with zero lost records. [Profiles](artifacts/profile-summary.json) retain every symbol period and sample count. No sampled fraction is a removable proof walltime or causal cache measurement. No helper, build, hash, archive or verifier overlaps a capture. Closing checks rehash all pins and accepted production sources.

Only 312 samples were collected across these short captures (26/78/41/167 for G1 one/seven and G2 one/seven pairs). Their statistical resolution is limited; no symbol-share comparison or causal explanation is established from them. Raw captures stay local with hashes in [the raw-profile inventory](artifacts/raw-local-only-sha256.json).

No new SDK/Gnark proof is produced: the independent proof ledger remains8310. No new AWS/Aeglos, Apple, phone, TEE, giant-circuit, swap, pressure, minimum-RAM, concurrency or throughput measurement is claimed. The [latest full SDK comparison](../2026-10-27-tree-kernel-split/README.md) contains all supplied M5 CPU/Metal shapes and remains0/8at CPU10×; it measures a different candidate. Its earlier adaptive31.530µs9/9positive is retained with its unresolved qualification.

[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [analysis](artifacts/analysis.json), [native tests](artifacts/native/core/tests.json), [artifact hashes](artifact-index.json).
