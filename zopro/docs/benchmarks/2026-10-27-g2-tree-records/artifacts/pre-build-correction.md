The first setup helper stopped on its exact-match assertion: `s.groups.reserve_exact(full / LANES)` occurs in both the generic tree and the unchanged G2 streaming implementation. No candidate file had been written, no native build had started, and no SDK call had run. The original helper source is retained as `initial-setup.rs`.

The corrected edit includes the tree-specific `m`/`full` declaration and adjacent clear, so it matches exactly once. The source recovery audit still checks every edit. No arithmetic design, timing protocol or threshold changes.
