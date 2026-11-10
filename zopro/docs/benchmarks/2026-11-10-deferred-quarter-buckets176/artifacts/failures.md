# Retained setup failures

Before any native build, proof or timing, the first source-snapshot preparer
exited101: it attempted to read every manifest member as UTF-8, including
the binary verification-key fixture. Rust reported InvalidData, "stream did
not contain valid UTF-8", at prepare.rs:8:47. No native plan was written.
The preparer now copies all unchanged members as bytes and reads only the
three edited Rust children as text. This changes setup bookkeeping only;
no successful scientific step was repeated. The tool transcript records
the original failure and its patch.

The native workflow completed its first16commands. Its last allocation
execution exited101 at borrowed_pending.rs:564 with assertion enabled():
the generated execution step omitted the required production environment.
The allocation helper and both libraries had already built successfully.
A separate one-command plan supplies the same environment as the native
release build. Only the failed allocation execution is retried; no source,
binary, arithmetic test or successful recording is rerun.
