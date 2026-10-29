# Packed indexed G2 reduction on the accepted two-component path

Declared before implementation. Previous goal turn PROGRESS: coalesced GLS4 top,
new skew oracle/collective memory bound,7902SDK/78Gnark and profiles, published
ccac9ce1ca. Rejected8.796095ms vs8.032045ms accepted,0/15wins. All GLS4 variants
remain closed. Accepted175/SDK20fde qualified1x2 8.022165ms/max134MiB; all8CPU10x
goal active/unmet0/8. No budget limit or minimum positive gain threshold.

New algorithm applies ONLY to the existing native G2 indexed reducer, preserving
accepted phi2 and all other query algorithms. Keep its1024-point tile size, but
cut the globally ordered index stream at point-tile boundaries instead of making
one tile per nonempty bucket. A tile can span adjacent buckets. Each nonempty
intersection is one segment; reduce all of that tile's segments together through
the existing complete reduce_layer. Thus many small buckets share inversion work
while N/1024 independent tasks retain point-sized parallelism. Final segment
sums remain ordered by original bucket and use original group_affine. Existing
adaptive prefix selection can fire differently; this is a coupled algorithm,
not an isolated inversion experiment. No tile/group/width sweep or new field.

Flat segment(first,end) metadata preserves input order and bucket identity.
partial_starts records segment ranges per bucket; tile_starts records disjoint
segment ranges per global point tile. Segments <=ceil(N/tile)+bucketcount.
Safe recursive Rayon joins split MaybeUninit output slices at known tile segment
boundaries, with one nonforking leaf per tile. A leaf copies <=1024 limb points,
creates <=min(1024,buckets) local starts/counts, runs original reduce_layer and
writes each segment result exactly once. Publish initialized length only after
all joins. No new pool, key/point lookup table or retained setup.

Private176-source snapshot:174 originals exact, cfg hook+module declaration only
in msm.rs, new msm/packed_index.rs. Inverse edits recover175. Field17, original
indexed body/group_affine/reduce_layer, Window, phi split/maps, solver/FFT/key
exact. New full optional admission charges all metadata/output arrays, tile
starts, partial indices/counts, final collective grouping scratch and workers
times first-stage typed grouping+local metadata. All checked, all metadata and
partial-output reserves fallible before executing. Reservation::analysis sits
on top of original forecasts; denial/overflow/unsupported/empty input takes
original indexed implementation without input mutation. G2 native TREE_PREFIX
guard preserves other curves/targets. Runtime trace after successful admission.

Three new tests: independent geometry ownership/tile/bucket coverage at tails
and empty/skewed boundaries; per-bucket generator-sum oracle with repeated,
opposite and identity points, varied/skewed buckets and1/4/32 Rayon pools;
checked geometry/overflow and admission denial preserving inputs. OFF124/ON127
unfiltered core plus8prover each. Inherit33 field tests from17 exact files.
Existing complete query/proof tests exercise the integrated callsite. Official
generator check; freeze/inspect actual SDK. Primary ON preflight must contain
16packed leaf-entry traces (four top component/partition calls per proof times
four SDK requests), buckets128/tile1024, before Gnark and all latency. No trace
in timed/profile runs. Helpers and PMU feasibility ready before capture.

Fixed pilot inherited unchanged:27all9correctness preflights;15balanced primary
blocks accepted/OFF/ON,151observed+3prime each;3stat/3profiles.7902SDK78freshGnark
156members; prior17708->17786 only after all binding/freshness/pins close. Gain
upper paired ratio<1 and>=10/15wins vs both accepted and OFF; calibration bounds
within[1/1.03,1.03]. No exclusions/extensions/successful reruns/changed gates.
No builds/helpers/hash/archive/owned competing workload during capture. Passing
primary still needs separate all8/physical16/resource qualification; no adoption
or original goal completion from primary alone. Report full inherited CPU/Metal
references plus fresh primary. Negative closes exact packing implementation.
No AWS/Aeglos/giant/phone/pressure/minRAM claim. Commit/push completed result.
