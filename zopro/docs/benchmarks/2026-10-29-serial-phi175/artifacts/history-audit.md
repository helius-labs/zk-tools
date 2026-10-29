# Prior implementation audit

Search retained benchmark Rust sources, patches and edit JSON for serial iter_mut/for-loops calling phi_split. No exact sequential implementation found. The relevant explicit task-boundary study is 2026-10-27-g2-scalar-grain: with_min_len(128), still parallel and retaining fork/join. Paired gain vs accepted0.267%, interval crosses parity, not adopted. This is not a repeat of that grain setting.

Joint MSM phases24Oct, G2 level tree26Oct and G2 prefix tree26Oct all retain parallel scalar decomposition in reviewed source. Grouping bypasses and whole-query scheduling policies are distinct and closed, as cited by the preceding G2 preparation report. No claim of algorithmic novelty or guaranteed speedup. This source audit precedes the candidate build and fixed measurement declaration.
