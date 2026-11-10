# Offline analysis correction

The first two objdump commands and three audit build/test commands succeeded.
The first analysis execution stopped before results: four lexical backward-branch
intervals, rather than one, contained the 1,200 reverse-loop IFMA sites. Compiler
cold/reentry blocks make lexical spans unsuitable for selecting this loop.
All command streams, failure exit and loop inventory are retained. The original
audit source is identified by its original pins; its overwritten bytes are not
claimed as a separately retained snapshot.

Corrected analysis builds the full direct-branch/fallthrough CFG, suppressing
fallthrough after RET, UD2 and direct panicking calls, and uses iterative
Kosaraju strongly connected components. Require one component containing exactly
1,200 IFMA sites, retaining every component member and every excluded row. No
performance capture, native code, input, timing or performance gate changes.
Add a disjoint-cycle/cold-reentry parser test. Rerun only the affected offline
audit build/test/analysis in a separate repair workflow; do not repeat objdump.

The design's original conservative-all-exit liveness wording was refined after
decode to loop-internal direct demand with outside uses excluded. The updated
analysis pins bind the exact method. Neither version establishes full stack
liveness, register spill provenance or causal costs. The old decode pins retain
the original design hash; this one documented design-only mismatch is exempted
when checking those historical pins, and current analysis pins bind the revision.
