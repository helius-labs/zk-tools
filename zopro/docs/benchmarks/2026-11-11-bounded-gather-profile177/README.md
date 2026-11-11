# Retained gather regression: CPU diagnostic

The additional CPU cost in the failed gather candidate is concentrated in the
borrowed-G2 arithmetic kernel. Its normalized assembly matches the disabled
control. This evidence narrows the investigation beyond the changed gather
loop, but does not establish whether code placement, data layout or another
execution effect causes the regression. The candidate remains disabled.

This separate diagnostic preserves the [failed latency screen](../2026-11-11-bounded-gather-sdk177/README.md).
It uses the same OFF `51f2fa06…` and ON `cbd87b66…` SDKs without rebuilding.
Ryzen 9950X, 1x2, 16 fixed workers on CPUs 0–15. Three blocks each contain
plain, hardware-counter and sampled-cycle runs for both binaries, with rotating
panel order and alternating consumer order. Each process measures 251 requests
after three primes. These diagnostic runs cannot qualify adoption.

| Observation | OFF | ON |
| --- | ---: | ---: |
| Median user cycles/proof | 564.672 million | 676.047 million |
| Median retired instructions/proof | 1,332.239 million | 1,333.743 million |
| G2 apply samples across three profiles | 4,577 | 9,910 |
| Total samples across three profiles | 26,590 | 31,826 |
| G2 apply function size | 51,862 bytes | 51,862 bytes |
| G2 apply normalized instructions | 8,591 | 8,591 |

Cycles increased about 19.7%, while instructions increased about 0.11%.
The G2 symbol's sample increase accounts for approximately the overall sample
increase; other symbols vary as well. Samples identify an investigation target,
not exclusive CPU attribution or removable wall time. Cache-miss and branch-miss
counters remain in the raw result; they do not establish a particular memory
or frontend mechanism.

The kernel starts at ELF address `0x480f50` in OFF and `0x480810` in ON,
a difference of 1,856 bytes. The comparison removed instruction addresses,
RIP-relative displacements and numerical branch targets while retaining
instruction text, registers, immediates and symbolic targets. All 8,591 lines
matched. This is not byte equality or proof that referenced constant values,
runtime addresses and microarchitectural behavior are identical. Both functions
have the same alignment modulo 64 bytes. Sample IP inspection accounts for
each process's executable MMAP and ELF load-segment address/offset difference.

All observer checks passed their predeclared coarse factor-1.05 guard. Median
profile/plain latency ratios were 1.0085 OFF and 1.0061 ON; counter/plain ratios
were 1.0065 OFF and 1.0048 ON. All six profiles passed sample count, ownership,
affinity, unknown-symbol and loss checks: 58,416 total samples, maximum unknown
share 1.5463%, zero LOST. Counter groups passed 99.9% running-coverage checks.
These checks do not establish statistical equivalence or unbiased sampling.

All 18 exported proofs passed independent Gnark verification and input/image
bindings. There were 4,572 SDK calls including primes and 4,518 timed samples.
The canonical ledger advanced from 51,892 to 51,910. No AWS resources, new
eight-shape performance comparison or speedup claim were added.

Local evidence root:
`/home/ubuntu/src/zopro-perf-work/bounded-gather-profile177/`.
It retains the frozen protocol and source pins, process/verification receipts,
all six raw profiles, decoded ownership streams, counter CSVs, `result.json`,
original/normalized assembly, and comparison helpers. Runtime source and
retained binaries were unchanged. The result supports investigating binary
placement and memory access behavior with a distinct controlled experiment;
it does not justify promoting the gather policy or altering the closed screen.

The report is committed locally. No push or publication is approved.
