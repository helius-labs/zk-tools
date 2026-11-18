# Snapshot audit repair before any build

The first setup exits101 after writing all intended192 source files. Its inverse
audit calls read_to_string on every accepted source member, including the binary
Poseidon constants blob, and fails InvalidData:stream did not contain validUTF8.
No native/generic build, test, SDK capture or plan execution follows the failure.
The original setup.rs and executable remain unchanged. Native plans/pins were
not created, so no frozen successful result is modified.

finalize.rs consumes the existing snapshot without recopying/regenerating it,
checks binary members as bytes and only decodes the four edited Rust parents.
It reconstructs the intended suffix/module-selection inverses, checks all186
accepted hashes and exact192-file membership, then writes audit/pins/plans.
Arithmetic, Window, wrappers, geometry and protocol remain unchanged. The
failure is a preparer bookkeeping error, not a failed arithmetic/correctness run.
