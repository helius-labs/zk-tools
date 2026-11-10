# Stable unfinished-class descriptor scans

The new mechanism changes group_affine_impl's repeated descriptor traversal,
not point storage, point arithmetic or scalar grouping. The old reducer visits
every class during pair discovery and compaction at every level, even after
its count is zero or one. A stable u16 list retains only classes whose count is
greater than one. Filter after each level. Pair order and every arithmetic
expression remain identical; completed coordinates stay in their original slots.

Use one fixed [u16;256] stack array:512 bytes per live nonforking group task.
Charge this stack payload in the existing typed grouping-workspace helpers;
no extra heap allocation, memory-policy snapshot, lookup table, point copy,
key expansion, production pool or architectural RAM cap. Compile-time native
selection only. Disabled and generic release paths retain the original loop.
Do not combine this with rejected singleton input-copy elision, packed/compact
point trees, indexed-wide geometry, class splitting or threshold tuning.

Implementation qualification must compare complete old/new layer trajectories,
raw point coordinates, counts and independent Arkworks class sums; preserve
identity/equal/opposite/cancellation, odd tails and point immutability. Exercise
ordinary and prepared getters, partial256-class chunks, empty/singleton/hot
classes, real512KiB workers and concurrent callers. Verify checked workspace
growth/overflow and generic fallback. Reuse the original field1200.

Before performance selection, quantify removed descriptor visits on retained
actual grouped populations and include initial list construction and filtering.
This is a structural screen, not a cycle model. Primary main grouping can decline,
so this is not a standalone route to a twofold primary improvement. If useful,
freeze complete grouped/whole-MSM or SDK ALL8+54 controls and automatic profiles
before preflight; keep all contexts and adverse results. No positive subset or
unchanged reroll. Full verified CPU10× ALL8 goal remains active and unmet.

Targeted history ruled out repeats of G1 GLV, fixed/sliding odd recoding, indexed
wide bands, key-column factorization, compact point trees, singleton copy elision,
kernel fission and field-only count proxies. Aeglos branch alex/cpu-latency still
points to0290613e8eeca17a4ce88f6f2d726e8e06a5385d; its pool.cpp was newly read,
but no external scheduler or speed claim is imported. Source refreshed through
authenticated read-only gh after GitHub connector404; no upstream code copied.
