# Fixed backing diagnostic for retained SDKs

The preceding full-proof gather screen remains failed and closed. This tests
whether its OFF/ON latency gap persists across predetermined file backings,
following the earlier component-level backing finding. It is not adoption
qualification and no fastest-copy selection is permitted.

Use six equal-length paths in one new directory: base-a0/base-b0 hardlink the
exact retained OFF/ON SDKs; copy-a0/copy-b0 and copy-a1/copy-b1 are independently
written complete byte-exact copies (ordinary write, no reflink/hardlink). Verify
SHA, device/inode identity and executable mode before and after. Copies change
backing, not virtual function offsets or ELF bytes. Physical mappings are not
measured. No ASLR, global kernel setting or native code changes.

Primary1x2 at16 fixed workers on0-15, original driver and accepted environment.
15 fixed blocks, rotate six cases by block modulo6 and reverse odd blocks.
151 measured requests after3primes. 90processes/13860SDKcalls/13590samples,
90fresh independently verified exports; canonical51910→52000 if all verified.
All recordings precede independent verification/analysis, with no owned overlap.

Report all case medians, paired ON/OFF for original backing and each copy pair,
and copy/original for each arm. All within-block ratios use sorted15 ranks3/11
and median7; nominal96.484375%marginal coverage, no simultaneous guarantee.
Two integer binomial constructions cross-check coverage before execution.
Report latency, whole-loop CPU and RSS. No hardware-mechanism or proof-speed
optimization claim follows from file-copy sensitivity. Do not reopen the old
screen, rerun successful captures, extend samples or choose favorable copies.

Freshness, proof/image/public bindings, command/stream receipts, worker masks
and all input/executable hashes must pass. This diagnostic adds no AWS/Aeglos,
ALL8 performance, portable, pressure or memory-minimum result. NO PUSH without
explicit user approval. Full ALL8 CPU10x objective remains active and unmet.
