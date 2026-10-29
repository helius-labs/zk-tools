# Coalesced GLS4 top-window affine reduction

Declared before implementation. Previous goal turn was PROGRESS: new hybrid
algorithm, two extra oracle/thread tests, full7902SDK/78Gnark pilot and profiles,
published64778d8ad6. It remains rejected:10.325850ms vs8.036072ms accepted,
0/15wins,257.335ms CPU vs182.053ms. Source175/SDK20fde remains accepted;
all8CPU10x objective active/unmet0/8. No minimum gain threshold or token budget.

Evidence: hybrid lowers sampled G2 projective add share relative to historical
ordinary-top GLS4, but fresh Fq inverse and G2 reduce_layer shares rise sharply.
Its indexed reducer builds a separate tree for each nonempty bucket tile. The
new algorithm removes that per-bucket first pass ONLY in the GLS4 highest window:
call the existing group_affine directly on the signed bucket index ranges.
At each level it batches ordinary pairs across256 adjacent buckets using the
unchanged complete-curve/prefix kernels, sharing inversion work. No new kernel,
formula, group width, parameter sweep or scheduling/annotation rescue. Changed
pair grouping alters inversion count, input copying and task graph. Lowerfive
Horner windows, fourparts, foursequentialcomponents,1024magnitudebins/2048sign
classes, phi/compound/phi maps and component/part merges are exact parent.

History audit:26Oct full ordinary-window G2 level/prefix/index-gather trees lost
against streaming and are closed; all explicitly preserved the existing narrow
top.22Oct GLS4 sparse/parallel used tiled indexed tops. This new callsite is
top-only coalescing, not retrying those full-window algorithms. Prior negatives
remain visible. Crowded/skewed buckets may reduce parallel depth or increase
scratch and lose; no speed forecast from operation counts/profile shares.

Private177-source snapshot, only gls4.rs and its resource estimate differ from
the frozen parent. Production175 untouched;172 originals exact/inverse175.
Original field17/MSM methods/Window/lattice/solver/FFT/key exact. No key/point
lookup table, retained setup, runtime pool, new field or synthetic RAM cap.
Worker-local limb point copies are temporary and charged. Keep conservative
six-output shape and four indexed-parent metadata/outputs/three retained affine
vectors each. Add workers*original nonforking leaf PLUS workspace_grouping(N,32)
for all coalesced groups collectively and32*512usize local stack arrays. Across
four parts and eight signed256-bucket groups each, input ranges are disjoint;
sum of retained per-init maximum point capacities <=N. Existing checked typed
grouping bound covers pair growth, generic/prefix records and allocator margins.
No workers*N charge for the same disjoint inputs. Existing dynamic analysis
admission includes40N short scalars and all new memory on top of old forecasts;
overflow/admission/try_reserve failure preserves original words and falls back.

Fresh OFF124/ON131 core and eight prover tests each. Parent six GLS4 tests rerun
(full-width digit/maps/MSM/admission, signed-top extrema, custom1/4/16/32 pools).
One added skew/complete-curve top test covers empty/odd/many equal/opposite/
identity points and two distant sign-class bins under1/4/32 pools. Exact17field
files inherit33 field tests; old100767 lattice integer cases inherited. Official
generator check. Freeze/coherent-link actual SDK and inspect; primary ON must
emit four COALESCED trace markers before Gnark and any timing. Build all helpers
and PMU feasibility before capture; no owned build/helper/hash/archive/competing
workload during capture.

Unchanged fixed full-SDK pilot:27all9correctness preflights;15balanced blocks of
accepted/OFF/ON with151observed+3prime requests;3stat/3profiles.7902SDK/78fresh
Gnark/156members, ledger17630->17708 only after identity/image/public/freshness
and pins close. Paired gain upper<1 and>=10/15wins vs BOTH accepted and freshOFF;
calibration bounds in[1/1.03,1.03]. No exclusions/extensions/successfulreruns.
Negative closes exact coalesced-top candidate. Passing pilot needs separate
all8/physical16/resource qualification before adoption. Full all8CPU/Metal
accepted comparisons retained with fresh primary. No AWS/Aeglos/mobile/giant/
pressure/minimumRAM or all8 candidate performance claim. Commit/push result.
