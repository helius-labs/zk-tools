# Matched local scorecard for eight M5 references and the larger merge

Five fresh randomized blocks compare the accepted Zopro release, one frozen experimental Zopro executable, and the prepared CPU Aeglos executable. The same three configurations run the same original request within each circuit. All use three warmups and 35 timed calls on 1×2, 2×2 and 12×2, or eleven calls elsewhere. Each setting uses 32 Ryzen logical CPUs except on 12×2, where all three use the same rotating 30-CPU mask. The small-shape Zopro frontend placements are identical across the release and candidate. Each process exports a proof for the original independent verifier. The Aeglos process must report a prepared-key hit on every timed call.

The candidate uses the blocked origin key on 1×2, 4×2, 12×2 and merge-24×1; grouped A/K/B2 on 4×2, 8×2, 12×2, 16×2 and 24×2; early centering and nested B2 only on 12×2. The 2×2 and merge-54×1 cases use ordinary keys. The schedule fixes these choices before timing. Per-shape medians cover all five process means. The report includes CPU and Metal ratios for all eight supplied references and keeps merge-54×1 as an additional same-host comparison.

[Fixed configurations and process order](artifacts/schedule.json)
