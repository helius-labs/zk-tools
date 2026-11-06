# Source and research scope

Read cpu/msm.hpp, cpu/msm.cpp and cpu/pool.hpp at Aeglos commit0290613e8eeca17a4ce88f6f2d726e8e06a5385d, plus the scalar-recoding/Job/run/Sorted/Windowed/Task and Lane2 arithmetic portions of cpu/msm_impl.hpp. Pool header only: cpu/pool.cpp was not fetched, so this audit does not establish its internal waiting costs. Four exact source SHA256 pins are in source-pins.sha256. GitHub connector404 fallback to authenticated read-only gh API was successful. Full Aeglos source stays private in this audit; public results link exact upstream files.

Review existing source176 g2_phi.rs, g2_borrowed.rs, msm/g2_borrowed.rs and msm.rs weighted reduction, and the pinned history reports. Source formulas are not observations of actual Aeglos per-query plan selection or allocation peaks. Published raw query lengths30089/52274 are illustrative geometry inputs; grouped queries and different query bases need different dimensions. Histories retain all failed gates; a source refresh is not a new implementation.

Primary searches revisited SRE arXiv2610.27452v1 sections4–6 and Noel–Plantard ePrint2026/1316 abstract. Both were previously reviewed; no new paper discovery or transferred runtime gain is claimed. SRE compares methods in a single-core Python implementation; its cross-window matching overlaps prior rejected partial-window reuse. The ePrint PDF returned403. Springer exposes Algorithm3/4 images publicly; Algorithm4 was downloaded at full resolution and visually inspected, not inferred from OCR or its abstract. Its displayed loops visit descending intervals of at most d buckets per window, rescan all n terms for each interval, retain the running total T across intervals, and perform the weighted updates. This is a memory/work tradeoff, not a shift-table construction or a new low-memory twofold CPU speed claim. The full article prose was not accessible. Public report uses only brief paraphrases/links, without redistributing the paper images.

Primary URLs:
- https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm_impl.hpp
- https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm.hpp
- https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/msm.cpp
- https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/pool.hpp
- https://arxiv.org/html/2610.27452v1
- https://eprint.iacr.org/2026/1316
- https://link.springer.com/article/10.1007/s13389-026-00393-z
- https://media.springernature.com/full/springer-static/image/art%3A10.1007%2Fs13389-026-00393-z/MediaObjects/13389_2026_393_Figg_HTML.png
