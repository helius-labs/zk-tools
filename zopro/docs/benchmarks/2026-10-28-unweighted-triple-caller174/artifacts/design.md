# Complete unweighted G1 ternary caller

Declared before capture. Test a private source174 derivative, not a production
change. Earlier small triple callers measured two streaming additions, not a
complete reduction. The source174 first-layer census isolates unweighted indexed
tiles and affine-group jobs; this experiment covers their real size range with
modeled point inputs, not captured proving-key points.

G1 only. Replace eligible unweighted binary layers by a three-way layer when at
least eight triples exist across the caller. Record eight FqBatch values per
eight triples. Ordinary two-point remainders share the same inversion and retain
five words per eight pairs. Pad incomplete lane groups with nonzero dummy
denominators and suppress padded writes. Any input identity or zero denominator
rejects the whole layer before writes; the unchanged complete binary layer runs.
Small layers remain binary. No weighted tree, streaming, field, solver, key or
production resource forecast changes. No lookup table.

The caller gathers permuted prepared affine bases into limb points, initializes
counts, performs ALL reduction layers, compacts, and writes affine outputs on
every iteration. Indexed cases create/drop point and both scratch structures on
every call, matching the lifetime of indexed input tiles. Group cases reuse
worker-local buffers, matching group_affine_impl. This single-thread component
does not simulate query/task dispatch or establish proof latency.

Fixed cases (count arrays, fresh buffers, rounds):
- tile257: [257], true, 1024
- tile384: [384], true, 1024
- tile511: [511], true, 1024
- tile2048: [2048], true, 256
- group35973: [35973], false, 16
- groups31x256: [31;256], false, 64
- tails: [0,1,2,3,7,8,9,23,24,25,26,47,48,49], false, 1024

Every process primes three times with eight calls, verifies after each prime,
then times its fixed round count and verifies all final outputs. One executable
contains both algorithms. Selection is outside the loop; both use the same
allocator and inputs. Count allocations during timing; fresh cases are expected
to allocate. Report actual retained vector capacities for common, old binary,
new ternary, and fixed Work storage separately; old and new scratch coexist and
must be added, not called replacement storage. This is not RSS/minimum RAM.

31 rotated/reversed process blocks, seven cases and both variants, pinned CPU2,
one Rayon worker. All latency captures finish before four whole-process cycles:u
profiles (199Hz), tile384 and tile2048, both variants. Profiles include fixture
setup and checks; no warm-gating or causal wall-time fraction claim. No owned
build/helper/hash/archive/second benchmark overlaps capture.

Per-case component condition: paired median and ranks10..22 of31 direct/binary
ratios, upper bound below one and at least21 wins. No gain floor. SDK integration
is permitted only if ALL main cases tile384/tile511/tile2048/group35973 pass.
Other cases are retained diagnostics and cannot override main failures. This is
not an all-eight proof gate. No unchanged reroll or post-result threshold change.

Tests: unfiltered native core suite with real-curve tails/guard/no-write tests;
external Arkworks full-width scalar oracle for permutations, multiple groups,
all tail lengths and boundary/cancellation inputs. Field source is unchanged,
so do not claim a fresh field suite. Full proof/Gnark, AWS, ARM/mobile, pressure,
giant-circuit and concurrent-proof qualification are outside this component.
Keep the all-eight 10x goal active until verified at its actual scope.
