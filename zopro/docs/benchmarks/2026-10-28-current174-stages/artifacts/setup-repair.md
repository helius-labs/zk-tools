# Registration repair before SDK capture

The initial xtask install command failed with zero completed steps and zero SDK
requests/exports. A first atomic-pipe-payload change did not fix it. strace of
tee versus shell printf establishes the actual difference: GNU tee -a calls
lseek(SEEK_END) on the tracefs control file, receives EINVAL, and never writes
the definition. Shell printf writes directly and succeeds. Those logs and both
failed helper sources are retained. Temporary manual wire3/sdk_enter probes
were removed after the check; they ran no SDK.

The helper now uses a sudo direct writer opening the fixed tracefs path without
append/truncate/seek. Its argument must equal one of the14 predeclared owned
definitions or a deletion of one of those exact names. One manual install
succeeded, registering all14. Do not repeat that successful installation.
The repaired workflow omits install, pins its completed receipt, and retains
all remaining91 original steps without a protocol/timing/source change. Move
the zero-completed failed workflow aside; no successful capture is repeated.

Parser ns/parse/observe/trace and their three passing unit tests are unchanged
by registration and pin-path repairs. Inherit those successful tests after
checking the exact source slices, rather than repeat them. No prover rebuild
or production source change. Initial helper retains the expected unused_mut
compiler warning; it has no bearing on diagnostic event semantics.
