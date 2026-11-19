# Count a finer scalar-diversity bitmap with the same scratch allocation

On the ordinary 1×2 reference, exact counting found 17,234 distinct nonzero scalars, above the 16,804 grouping cutoff. The release bitmap certified 16,631, so the SDK built and then discarded the full dictionary. Read one additional hash bit by visiting two disjoint input partitions in sequence and reusing the same bitmap allocation. The count remains a deterministic lower bound. Equal inputs always choose the same pass and slot. The trial adds a second linear input scan and preserves the original reservation and fallback.

Before interpreting latency, run the original SDK and candidate with admission tracing on the reference. Check whether four warm and measured calls eliminate all four 21,006-scalar dictionary constructions. Keep a failed mechanism observation if the bound remains insufficient. Run the existing native correctness suite and a forced old-bucket collision that separates only on the extra bit.

Measure reference 1×2, two independent 1×2 witnesses, and original 2×2 and 4×2 with the same affinity, server settings and unchanged verified SDK entry. Use eight paired blocks with fifteen calls per process plus one CPU profile per arm. Report paired latency and peak RSS on every input and both supplied M5 references where available. Test this change only after experiment 635 has finished using the host.

[Original admission census](../2026-11-19-canonical-index629/README.md) · [Ordinary release](../2026-11-19-checked-release630/README.md) · [Source](artifacts/source.patch)
