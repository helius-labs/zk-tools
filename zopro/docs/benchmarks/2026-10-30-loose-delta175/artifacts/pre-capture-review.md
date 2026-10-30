# Harness and helper review before capture

The design's phrase "three harness tests each variant" was a mistaken inventory. The unchanged shared perf control module has two FIFO tests, and the common caller adapters have four point/dispatch tests already included in each128-test core suite. The two shared FIFO tests run once on the exact common perf.rs; no fictitious third test or duplicate per-variant claim is made. These checks supplement the full35field/128core/8prover suites and90real-caller correctness preflights. No performance gate, context, block, round, variant or output requirement changes.

Capture publication generator initially failed to compile because a raw-string delimiter collided with the generated Markdown title string. Increased delimiter depth before generator execution. Original prepare-capture-initial.rs remains. No native test, capture or verifier repeated. Original compiler diagnostic was returned in the tool transcript; no separate initial stderr file was saved.

The standalone shared perf-test compile initially omitted its serde_json extern. Added that existing dependency without changing perf.rs; initial compiler log is retained. Both real FIFO tests then pass on their first execution. No successful test is repeated.

Pre-capture review strengthens metadata validation within existing loss/decode steps: retain and check executable MMAP/FORK records, CPU affinity and timestamps. This adds no capture process and keeps930steps. Sample count/unknown-period flags are descriptive attribution qualification, retaining known absolute-period undercoverage. No resampling or timing exclusions follow a weak profile.

Scope review: this private prototype is qualified only with the existing native ifma8 feature. Its new x86 target guards must also explicitly require feature="ifma8" before any four-lane/portable integration; no portable compatibility is claimed from this build. Production is unchanged. A passing caller stage would require that source adjustment and portable/resource/SDK qualification rather than direct adoption.
