# Derived counter-rate JSON comparison

The original frozen analyzer completed once. Its original frozen independent
reader failed offline at review.rs:9:867: 233.02130889892575 versus
233.02130889892578. It used exact serde_json::Value equality between a parsed
serialized derived `per_wave` counter rate and a fresh f64 division.

The separate `review-fixed.rs` changes only that derived counter-rate
comparison to the existing reader `same` tolerance, 1e-11*(1+abs(value)),
with explicit null handling. Raw counter counts/coverage, source pins,
receipts, sample counts, output digests, allocation/capacity checks, every
timing ratio/calibration/nonregression/gain gate and profile-quality gate
remain unchanged. No new tolerance is applied to a selection threshold.

Original pinned review.rs and executable remain exact. The fixed reader
uses a separate source/binary/stdout/stderr path; its source difference and
identities are retained. No recording, successful analyzer, native build,
test, oracle or decoder is repeated. All failures remain visible.
