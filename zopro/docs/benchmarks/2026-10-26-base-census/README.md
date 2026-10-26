# Equal and opposite proving-key bases

This read-only census tests whether repeated bases could remove online point work by adding their scalars before MSM. It uses all nine current accepted Euler images and the unchanged accepted loader/libraries. No prover source, proof latency or memory policy changes.

The counted opportunity is small: primary A removes27/21,006 rows (0.129%) and B77/30,089 (0.256%). Folded K, product H and combined K/H have no repeated or opposite points in any of the nine images. The largest separate-query reduction is0.416% for24×2 A. No class contains both signs, so all observed reductions are exact duplicates. This does not justify implementing a new index plan for the current speed goal; it does not prove such a plan could never help another key.

Every scanned point is checked on curve; compressed negation must flip exactly its sign bit. A normalized compressed class records which signs occurred. Exact classes equal signed classes plus classes seen with both signs. Identity points are excluded from potential new arithmetic savings because the existing stream already skips them. The combinedK/H row is a count model for a shared linear sum; only the existing small fused query geometry is marked, and geometry alone does not establish runtime eligibility.

|Shape|Query|Rows|Identities|Exact classes|Signed classes|Potential signed row removals|Fraction of nonidentity rows|
|---|---|---:|---:|---:|---:|---:|---:|
|1x2|A_G1|21006|0|20979|20979|27|0.128535%|
|1x2|B_G2|30089|0|30012|30012|77|0.255907%|
|1x2|folded_private_G1|30529|0|30529|30529|0|0.000000%|
|1x2|product_G1|32768|0|32768|32768|0|0.000000%|
|1x2|combined_KH_G1|63297|0|63297|63297|0|0.000000%|
|2x2|A_G1|36637|0|36587|36587|50|0.136474%|
|2x2|B_G2|52274|0|52119|52119|155|0.296515%|
|2x2|folded_private_G1|53079|0|53079|53079|0|0.000000%|
|2x2|product_G1|65536|0|65536|65536|0|0.000000%|
|2x2|combined_KH_G1|118615|0|118615|118615|0|0.000000%|
|4x2|A_G1|67154|0|67046|67046|108|0.160824%|
|4x2|B_G2|95537|0|95226|95226|311|0.325528%|
|4x2|folded_private_G1|97075|0|97075|97075|0|0.000000%|
|4x2|product_G1|131072|0|131072|131072|0|0.000000%|
|4x2|combined_KH_G1|228147|0|228147|228147|0|0.000000%|
|8x2|A_G1|129064|0|128792|128792|272|0.210748%|
|8x2|B_G2|183347|0|182724|182724|623|0.339793%|
|8x2|folded_private_G1|186411|0|186411|186411|0|0.000000%|
|8x2|product_G1|262144|0|262144|262144|0|0.000000%|
|8x2|combined_KH_G1|448555|0|448555|448555|0|0.000000%|
|12x2|A_G1|190734|0|190234|190234|500|0.262145%|
|12x2|B_G2|270757|0|269822|269822|935|0.345328%|
|12x2|folded_private_G1|275427|0|275427|275427|0|0.000000%|
|12x2|product_G1|524288|0|524288|524288|0|0.000000%|
|12x2|combined_KH_G1|799715|0|799715|799715|0|0.000000%|
|16x2|A_G1|252548|0|251756|251756|792|0.313604%|
|16x2|B_G2|358343|0|357096|357096|1247|0.347991%|
|16x2|folded_private_G1|364699|0|364699|364699|0|0.000000%|
|16x2|product_G1|524288|0|524288|524288|0|0.000000%|
|16x2|combined_KH_G1|888987|0|888987|888987|0|0.000000%|
|24x2|A_G1|376992|0|375424|375424|1568|0.415924%|
|24x2|B_G2|534619|0|532748|532748|1871|0.349969%|
|24x2|folded_private_G1|544587|0|544587|544587|0|0.000000%|
|24x2|product_G1|1048576|0|1048576|1048576|0|0.000000%|
|24x2|combined_KH_G1|1593163|0|1593163|1593163|0|0.000000%|
|merge-24x1|A_G1|359220|0|358932|358932|288|0.080174%|
|merge-24x1|B_G2|507895|0|506048|506048|1847|0.363658%|
|merge-24x1|folded_private_G1|516297|0|516297|516297|0|0.000000%|
|merge-24x1|product_G1|1048576|0|1048576|1048576|0|0.000000%|
|merge-24x1|combined_KH_G1|1564873|0|1564873|1564873|0|0.000000%|
|merge-54x1|A_G1|804765|0|804117|804117|648|0.080520%|
|merge-54x1|B_G2|1137190|0|1133033|1133033|4157|0.365550%|
|merge-54x1|folded_private_G1|1156887|0|1156887|1156887|0|0.000000%|
|merge-54x1|product_G1|2097152|0|2097152|2097152|0|0.000000%|
|merge-54x1|combined_KH_G1|3254039|0|3254039|3254039|0|0.000000%|

Maximum counted potential removal among separate queries and size-eligible fusedK/H rows is0.415924%. Counts are an optimistic arithmetic opportunity before scalar aggregation, indexing, startup and memory costs. They do not establish a speedup or a universal circuit property. Cross-query removals and map capacities are retained in [results](artifacts/results.json).

Four tests pass: signed/exact counters against independently sorted point encodings; G1/G2 signed-scalar combination against generator oracles; identity-only inputs; checked range/overflow/truncation handling. The accepted loader validates each whole image before scans following the pinned current format source. Points are decoded in4096-point tiles. Temporary hash maps grow with distinct points and are diagnostic-only. Process resource receipts include loading, mapped pages, point checks and maps, not minimum proving RAM.

The whole census takes5.97s with maximum process RSS989.57MiB. These figures include loading and checking the largest key and are not proof latency, incremental index memory or minimum proof RAM. An initial driver link failed on a byte-string cast spelling before any scan; the [build repair](artifacts/build-repair.md), original source and stderr are retained. No scan was repeated.

The next arithmetic design must address useful parallel depth without assuming scalar or base repetition. An audit of prior results distinguishes the rejected G1 indexed-tree layout and G2 bucket-round routing from a possible G2 tree that batches additions across many buckets at each level. The latter remains unimplemented and unmeasured here; any prototype needs a bounded workspace, independent oracles and a declared comparison against the current stream. This census supplies no evidence that it will be faster.

No witness solve, SDK proof, Gnark export, CPU latency/profile or AWS run is added. Prior verified proof count5958 stays unchanged. No M5 speedup is newly measured; [last all-nine full SDK comparison](../2026-10-26-prefix-pool/README.md) retains CPU/Metal reference tables, and [latest stage diagnostic](../2026-10-26-prefix-stages/README.md) explains the query critical path. Neither establishes the original all-eight CPU10× goal. No source is adopted.

[Protocol](artifacts/design.md), [driver](artifacts/probe.rs), [input hashes](artifacts/input-pins.json), [raw output](artifacts/run/census/stdout.log), [resource receipt](artifacts/run/census/stderr.log), [tests](artifacts/run/tests/stdout.log) and [artifact index](artifacts/artifact-index.json) retain evidence. Keys/libraries/executables remain local with hashes.
