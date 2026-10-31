# Qualification wrapper repair

The first native qualification workflow stopped during parsing of the new
core child, before running tests. The generated BatchAdd impl lacked its final
closing brace. Initial candidate source, setup/audit/native helpers, source
maps, qualification plan/pins, Cargo log and structured failed receipts are
preserved unchanged.

`repair.rs` creates a separate candidate-fixed snapshot. Its sole source-byte
change is one `}` closing that impl. No arithmetic expression, test, field
operation, fallback, queue method or compiler flag changes. Native-v2 and
audit-v2 refer to the separate snapshot and have separate logs/plans/pins.
Already compiled dependencies may be reused; no successful test, performance
capture or proof is repeated. Initial source recovery audit did not catch the
syntax error: it recovered the inverse, stream and Window bodies, without
parsing the wrapper between them. Passing that audit was not build evidence.
