# Fixed native follow-up

The six frozen algebra/work gates passed. Keep their result and all helpers
unchanged. This follow-up implements that exact block-256 identity using the
existing affine G2 streaming kernel, including its prefix-record path, ordinary
exceptional addition fallback and original field. Both signed components prepare
concurrently in the existing Rayon pool; each component has up to 255 dependent
stages. Corrected coefficients and suffix points then enter the original Phi
bucket-Horner window/fill/merge/weighted path. Layout concatenates the low and
high component arrays with the unused component words zero. Fixed width12,
parts2 and eleven windows are preserved. Sorting, arrays, zero-word walks,
exception handling, decomposition, grouping and all allocation/drop are charged
to the complete-query caller. No block/worker/window/scan variation is selected.

This is private experimental source, not production integration or adoption.
Checked workspace admission/fallback is not implemented yet. The earlier 216B
per-row logical payload is only a model, not a native capacity bound; actual
temporary vectors and concatenation capacity must be inspected before any
integration. Do not claim mobile/large-circuit pressure qualification.

Build fresh OFF/ON against exact accepted 176-file source and original field,
with dense-busy-reset retained. OFF excludes the candidate by compile-time cfg.
Expose the same private complete-query API in both. Native tests must cover
block boundaries, zero/identity/equal/opposite points and all existing core tests.
Only after they pass, freeze a separate complete-caller timing protocol and its
reader before functional preflight. Retained actual-query independent Arkworks
oracle hashes may be reused; no old oracle/fixture production or proof replay.
No full-proof claim follows from native-only qualification. Canonical45335.
