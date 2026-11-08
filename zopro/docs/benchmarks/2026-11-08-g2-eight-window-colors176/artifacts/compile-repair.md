# Obstruction audit compile correction

The first obstruction workflow stopped at compilation, before executing its audit. rustc E0689 required an explicit integer type for `row_count.div_ceil(200)`. Keep original source/plan/pins and compiler stderr/exit unchanged. `obstruction-fixed.rs` changes only `(0,0)` to `(0usize,0usize)` in the row/pair counters. No schedule/cap/parts/window/fixture choice changes. The fixed plan names the distinct fixed source and output receipt directory. Its reviewer changes only the new workflow and pin paths. Initial parent model/tests are not rerun.
