# Paired weighted trees: native selection and adoption

The qualified paired-tree reducer is **adopted locally** in
`b164b9776ba67cd2fdb56ba80ebee59b3fd329ee`. It shares batch inversions between
two independent original256 weighted trees. Primary full-proof latency improves
**0.89% at16workers and0.44% at32** versus accepted227, with all three backing
medians positive against both controls. These small gains are retained.

The [all-shape run234](../2026-11-18-paired-weighted-all234/README.md) passes
70/70 performance/resource checks, including both latency/CPU nonregression
controls at every context. [Current diagnostics235](../2026-11-18-paired-weighted-profile235/README.md)
pass all quality/observer checks: approximately2% fewer retired instructions,
and2.4%/2.1% fewer user cycles at16/32workers. Three PMU ratios are descriptive,
not confidence intervals or causal cache/DRAM attribution.

The measured32-worker primary median is7.460937ms versus7.500146ms for REF,
with peakRSS128.75MiB versus128.25MiB. Marginal times and paired-ratio medians
are different estimators. Native16 paired latency ratio0.991083278 has rank
interval[0.988653631,0.992568685]; native32 ratio0.995564248 has
[0.992863251,0.998344444]. Complete all-shape/RSS and supplied CPU/Metal
comparisons remain in234. This is a small improvement, not a twofold jump.

## Native build selection

Every reducer implementation and test byte matches qualified233. The only
additional source edit is two lines in core build.rs: register the cfg and
emit it inside the existing x86_64/ifma8/AVX512IFMA+VL+DQ/notno_ifma guard.
The reducer retains the existing resource-schedule guard, checked eligible
length/budget fallback and ordinary generic path. The normal native
resource-schedule build no longer requires forcing the experimental cfg.

A new isolated native core test build omits explicit paired cfg/check-cfg
RUSTFLAGS and passes141tests unfiltered, including all three new paired tests.
Generic noASM/noIFMA complete-core compilation also passes. Eight selector
cases check native enablement, missing Cargo feature, no_ifma, each missing
CPU feature, aarch64 and wasm32. Removing the two permitted output lines
recovers every old build-selector output exactly in all cases.

This stage validates native selection and integrates source. It does not
relink or time a new normal-selection SDK. The reported latency belongs to
exact233's forced-cfg SDK:

```text
d523f49e8548620c4373a1ab4be2712d705709fdd7a590c5ed73e9aff97424e3
```

Implementation-file identity is established; complete188-file snapshot
identity to233 is not asserted because build.rs differs. No claim is made
that every Cargo profile/compiler/layout reproduces those exact timings.
The native selected-test configuration is new; unchanged field, SDK, Gnark,
all-shape and profile qualifications are inherited without timing replays.

## Resource and source review

The pure checked requested-capacity comparison against the existing weighted
workspace forecast happens before allocation. It covers all paired jobs and
partials, with fallback on ineligible lengths, overflow or incompatibility.
Existing proof admission remains conservative; no new OS queries or locks
are added per reduction. Each leaf owns two original trees, reuses scratch
and creates no retained plan, lookup table, key copy or worker pool.

All234 RSS guards pass10%+8MiB. Increases remain reported: merge54 ON peaks
at1945.46MiB versus1864.73REF and1848.43OFF. These process-lifetime peaks
include loading/priming and do not establish minimumRAM, allocation savings
or physical-pressure/swap/phone/concurrent-throughput behavior.

Before production writes, the adopter verifies all current186 production
hashes,188candidate hashes, source inverse edits,234/235 independent reviews,
all build/stream receipts and new-file destination absence. Four files are
copied:msm.rs,build.rs and two new module/test children. All188 final hashes
match the manifest;184original files remain exact. All field code and other
resource policy stay unchanged. Native188/generic122 tests pass or are inherited
by source/configuration identity, plus141new native-selection test executions.

The233-235 program has1005freshGnark exports and56544SDK calls. This selection/
adoption stage adds no SDK calls or proofs. The full all-eight CPU10x goal
remains active and unmet,0/8. There is no new AWS or Aeglos measurement.

Canonical inventory stays62187; SHA-256 `38314450cd7201e88e527845db52c4de60479688cc2c7b8d0201c4d472596ff3`.

Next work can seek a larger main-path improvement or measure the adopted
implementation on C7a when that comparison is scheduled. These completed
checks and old failed candidates remain preserved. No cloud resources were
created. Source and reports are committed locally; no push was performed.

[Protocol](artifacts/protocol.md), [adoption review](artifacts/adoption-source-review.json),
[selector matrix](artifacts/selector-review.json),
[production manifest](artifacts/production-source-manifest.json),
[sources/plans/receipts](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
Private inputs, proofs, complete inventories, binaries and raw profiles remain
excluded. Publication awaits approval.
