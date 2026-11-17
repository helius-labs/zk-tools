# Reader correction and sequencing deviation

Original smoke reader incorrectly requires canonical solver TID=SDK main PID. Raw smoke events show SDK on main and canonical/A on an owned Rayon worker, with valid containment/entry-return pairs. Repaired source replaces only that equality with membership in the already recorded SDK-owned thread set; all PID/CPU/stack/span-count/containment/timing guards remain.

The main capture was launched after the failed smoke-reader command because orchestration did not gate on its exit. This is a real sequencing deviation from the announced smoke-first flow. All twelve main captures already completed, once; no rerun, dropped observation, source or threshold change is made. Original failure and missing-output independent attempt remain in the tool transcript. Both the retained smoke and main data are read after capture. Results are exploratory, and full protocol qualified=false even if the declared observer/boundary checks pass.

The new manager refuses any future main launch unless both smoke producer quality and independent reader success exist. This prevents the same orchestration error in a reused helper; it does not retroactively fix this run. No prover or SDK was rebuilt. Original source/pins remain immutable. All24 owned probes were removed by the original wrapper after capture, and tracefs was independently checked empty.
