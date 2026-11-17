# Existing clean-production profile review223

The exact accepted production binary's existing profiles **do not show the
same hot-load fingerprint** as the slow diagnostic build. Grouping accounts
for 0.73–0.79% of sampled whole-process cycles, and none of its 195 samples
lands at the suspect load, function offset `+0x108a`. This supports questioning
the diagnostic context before another stack-alignment experiment. It does not
prove production never suffers that stall.

|Existing recording|All samples|Grouping samples|Grouping share|Samples at +0x108a|
|:---|---:|---:|---:|---:|
|195 cycles-ON-0|8,436|66|0.782%|0|
|195 cycles-ON-1|8,432|67|0.795%|0|
|195 cycles-ON-2|8,446|62|0.734%|0|

These are the three previously qualified certified-ON recordings from195:
16 fixed workers, three independent backings, 251 timed fresh SDK requests
per recording. The executable is the exact accepted194 consumer,
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
There is no new source build, SDK call, proof export or measurement campaign.
The [clean full-proof comparison222](../2026-11-17-store-frame-clean222/README.md)
already failed to establish a gain from the compact aligned frame.

The accepted function is at `0x22d600`, size `0x1dc8`. Its suspect load is
`0x22e68a`, immediately after the 32-byte key spill to `[rsp+0xf0]`, matching
the relative instruction investigated in the diagnostic build. The review
reads the binary's ELF program headers and each recording's executable MMAP
to translate raw sampled IPs into function offsets. A separate `perf script`
decode with symbol offsets agrees on every grouping histogram. It also
rechecks195's input pins, raw perf-data hashes, decoded-stream receipts and
the previously passed observer/quality checks.

The largest sampled symbols in the clean recordings are:

|Symbol family|Samples|Share of 25,314 samples|
|:---|---:|---:|
|G1 streaming MSM|6,549|25.87%|
|G2 borrowed apply|4,570|18.05%|
|Fq inverse|1,941|7.67%|
|G1 reduction layer|1,274|5.03%|
|G1 balanced feed|1,257|4.97%|
|G2 reduction layer|1,168|4.61%|
|Crossbeam epoch pin helper|925|3.65%|
|Scalar grouping|195|0.77%|

These shares describe sampled running cycles across all threads. They are
neither removable wall-time fractions nor guaranteed speedup opportunities;
functions overlap, and their own callers and inline work matter. Earlier epoch
and scheduling experiments remain closed despite visible profile weight.

The recordings are earlier than222 and use hardware-cycle sampling at a
16-million-cycle period. The diagnostic219 investigation used software
CPU-clock samples scoped to the G2 grouping interval. Their sample counts and
shares are not directly comparable. The clean whole-process profiles include
every grouping call, have no live RSP observation, and do not provide a validated
absolute grouping-cycle total or CPI. A zero count at one IP is not a proof of
absence, and this review does not establish an instrumentation-induced cause.

The next optimization should address work visible in clean production and use
an unchanged production consumer for diagnosis before qualification. The large
diagnostic group-cycle reduction should not be carried forward as an assumed
production opportunity. No unchanged frame retry or new source adoption follows
from this review. Production194/196 remains accepted; the eight CPU10× targets
remain unmet.

Evidence remains private at
`/home/ubuntu/src/zopro-perf-work/clean-profile-reuse223` and the original195
recording root. The canonical ledger remains60,025 from222, SHA256
`ac50e8c1c48858afcc1d7438cc88443aab322e88408a6cf3d07fed1bb2e63a83`.
No cloud resources were used. Committed locally; not pushed.
