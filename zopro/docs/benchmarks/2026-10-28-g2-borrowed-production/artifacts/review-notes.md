# Source and evidence review

Written while the fixed final-artifact capture is running; no partial timings
are used here to choose another executable, protocol, threshold or topology.

The two new private modules retain the experimental implementation exactly.
The clean integration changes only three originals: private module declarations
in msm.rs and msm_affine.rs, and an ordinary-window dispatch in g2_phi.rs after
the unchanged top-window return. lib.rs and field arithmetic are unchanged.
The existing supported-native guard replaces the private experiment flag.
No public diagnostic exports are copied into the clean source.

Borrowed records reference immutable base slices for the synchronous window
operation. Their lifetimes use ordinary Rust references, with no unsafe lifetime
extension or shared mutation. Signs are encoded in the upper index bit and
materialized for Y packing, initial insertion and projective exceptional cases.
Affine, busy and projective bucket ownership remains window-local. A disabled
packed/direct/canonical/vector-inverse/prefix option returns to the retained
owned path before allocating the new Window. Capacity forecasts remain the
accepted conservative estimates; smaller pending records do not require a new
fixed RAM cap, key table, plan or preparation allocation.

The new guard is why native122core and raw2/cubic6 tests were rerun for this
clean source. The earlier portable11/99/8 tests and raw/denied/opt-out,
small-stack and concurrent SDK cases support the unchanged excluded path and
identical module bodies, within their finite scopes. They are not newly run
clean-SDK cases. Final exports in this experiment add all9 default32 shapes.

Earlier primary allocation traffic fell by40requests/1472000bytes; actual RSS
did not consistently fall. Do not transfer that traffic figure into a resident
memory or minimum-RAM guarantee. The forced-high test failed on the accepted
baseline alone: held anonymous ballast exceeded memory.high even without the
prover, swap was disabled, and neither fresh request completed within300s.
That is a retained qualification limitation, not a measured candidate failure,
candidate success, OOM, or reason to retry larger quotas until a pass appears.

Read-only research audit during capture found the empty-victim precheck already
tested in2026-10-22-universal-empty-steal-probe. It removed much epoch CPU share
but increased switches/system CPU and failed primary latency. Bounded guard
reuse in2026-10-25-batched-steal and batch transfer in2026-10-27-batch-local-steal
also have closed SDK screens. Do not treat any of these as an untried new idea
or convert sampled CPU share into removable wall time. No new scheduler patch
or arithmetic candidate is selected in this cleanup study.

Archival review: final.rs's generic text-extension filter omits Cargo manifests,
lockfiles, licenses and shader inputs. Before publishing, supplement the archive
with every exact source file in the172-file baseline manifest plus both new
modules for each applicable source snapshot, then rebuild and strictly verify
the archive index. Preserve the pinned helper bytes; this affects archival
coverage only, and must not rerun timing or successful verification.
