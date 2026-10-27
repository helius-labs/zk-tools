# Research and prior-result check

The 2026 Waterloo [thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403), chapter5, reuses matching digit subsets by summing associated points and introducing auxiliary rows. Its operation-count results do not establish Zopro latency. The retained cross-row census and serialized global-window search already examine overlapping mechanisms; the latter's search alone exceeds its baseline MSM time. No thesis implementation is copied or rerun here.

The old future_designs.shared_point_window_simd entry says design-only, but authoritative completed receipts and reports show component, geometry and queue experiments. The isolated descriptor result improves; complete grouped filling loses overall in the geometry and queue trials. Those later results override the stale design entry. See2026-10-25-shared-point-window-{component,geometry,queue}; no unchanged repeat is justified.

The selected new mechanism is parallel exact scalar grouping. Its rationale comes from our retained grouping-cost census and source: primary serial maps allocate/build/drop before declining. Source/report search found no prior disjoint-map implementation. It introduces no curve-operation clustering, shifted-base table or prewarmed witness state. New timing is required; no paper speedup transfers to this prototype.
