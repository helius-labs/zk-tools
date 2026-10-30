# Before build

The first offline lock resolution failed because the requested libc0.2.180 is not in the cached registry. Original Cargo manifest is retained as initial-Cargo.toml. Replaced only that dependency pin with cached0.2.191, the version used by earlier native components. Resolution then passed. No compiler, test, native program or performance observation had run before this repair.

No arithmetic/protocol change. The reused perf control helper is byte-identical to interleaved-complex175/src helper and its two existing FIFO tests are inherited, not rerun.
