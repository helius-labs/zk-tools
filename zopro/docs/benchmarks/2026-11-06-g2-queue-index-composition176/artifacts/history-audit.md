# History and source audit before implementation

2026-11-06. Read both component reports and their private setup/source-audit files. Directly read contained g2_phi/contained_indexed.rs and original g2_phi.rs top/ordinary dispatch, plus the finalize-drain source-audit and process selector. The ordinary dispatch calls the same g2_borrowed::try_window whose flush_queue is patched by draining; contained top packing does not replace the ordinary dispatch. This is source compatibility, not performance evidence.

README search with rg --no-ignore for packed.index/indexed.pack/indexed.top packing/contained.index finds the Oct29 packed-index and Nov6 contained-index reports. The new fixed composition was not found there. Absence of a search hit is not a novelty claim or proof that no equivalent implementation exists. Search actual source and reports again if an overlapping recipe is encountered before building.

The fixed-block odd experiment was a follow-up of the already closed Nov5 fixed-odd family. It was initially missed. Its current report a7356c23da explicitly corrects the history; actual Booth/indexed-top/grouped2x2 counts confirm a negative result, not a new algorithm. Model tests/counters/review all passed once; do not repeat them. Point-add call counts634486->662189 and654290->682672; no active-doubling reduction. No native recoding benchmark is warranted.

Ordinary G2 index gathering was already slower in 2026-10-26-g2-index-gather. Tree records/fused-top and the other stored alternatives remain closed. The memory-constrained MSM paper2026/1316 remains abstract-only in the local reading record: its inaccessible full PDF must not be presented as read or as an implementation recipe.

No source snapshot, native job, benchmark, proof or AWS resource has been created for this composition. Design is prospective and should be pinned before implementation/measurement.
