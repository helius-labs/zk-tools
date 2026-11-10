# G2 instruction profile after incoming-Y fusion

**The existing profiles do not establish integer multiplication or increased stack spilling as the cause of the rejected candidate's G2 slowdown.** Exact instruction mapping shows fewer stack-access samples, an unchanged stack reservation and fewer than 0.1% of function samples at `vpmullq`. Most samples remain in IFMA and surrounding vector work. This is offline analysis of the [previous experiment](../2026-11-10-fused-incoming-dy176/README.md), with no new timing, recording, proof or production change.

OFF is the retained fixed-mask quarter candidate; ON adds fused incoming quartering and signed Y subtraction. Their existing 32-worker G2 counter panels report 19.341→19.157 million instructions but 10.121→10.300 million user cycles per complete Window. The previous four-arm performance screen remains closed **FAIL 7/48**, with no adoption. Accepted local 1×2 full-proof latency remains **7.783594 ms at 32 workers** and **9.016864 ms at 16 workers**.

The Rust analyzer derives relocation from the executable ELF64 `PT_LOAD` segment and recorded executable `MMAP2`, checks PID/TID/CPU and symbol bounds, and maps every target sample onto an exact instruction boundary. Subtracting the mapping's file offset alone would introduce a **4,096-byte error** for both binaries. `nm` and `readelf` agree with the parsed metadata; independent `perf annotate` output agrees with **every instruction's sample count**, including zero-count instructions. Input executables, libraries and existing profile payloads match their retained published hashes.

|G2Scratch::apply|OFF|ON|
|---|---:|---:|
|Function bytes|51,466|51,415|
|Static instructions|8,672|8,625|
|Prologue `sub rsp` reservation, bytes|7,872|7,872|
|Static instructions with explicit `rsp` memory operand|1,619|1,593|
|Function samples|30,112|30,575|
|Whole-profile samples|41,414|42,113|
|Samples at explicit `rsp` accesses|4,367|4,031|
|Samples at vector `rsp` accesses|4,245|3,879|
|Samples at IFMA instructions|12,969|13,640|
|Samples at other vector instructions|10,572|10,486|
|Samples at vector moves|5,003|4,787|
|Static `vpmullq` instructions|13|15|
|Samples at `vpmullq` instructions|14|25|

The hottest individual address in each function is a vector stack store: OFF has 470 samples at `vmovdqa64 [rsp+0x15c0],zmm21`; ON has 609 at `vmovaps [rsp+0x3c0],zmm11`. That local concentration does not establish increased total spilling: aggregate explicit-stack samples decrease. An `rsp` access can also be an intentional local array or call argument. Register-relative accesses are not assumed to be heap or key accesses.

These are cycle samples with possible skid, from two separate diagnostic panels on a shared host. Samples at an instruction do not measure its exclusive latency, retired frequency, cache misses, memory stalls or execution-port utilization. Counts at `vpmullq` therefore neither prove that it is cheap nor support claiming it is the dominant cause. IFMA sample growth also does not prove a particular dependency or port bottleneck. No new causal or statistical certainty is claimed.

Prior work constrains the follow-up: [whole-loop phase separation](../2026-11-08-g2-phase-callers176/README.md) was 5.27% slower in its G2 aggregate, while [fused-product outlining](../2026-10-29-g2-fused-outline175/README.md) slowed primary proof latency by 2.34% against its accepted control. Those were different builds and fixtures; their speed estimates are not transferred here. They are negative evidence against assuming a Rust call boundary will help.

A possible distinct follow-up is to shorten wide-value lifetimes *within* an eight-point reverse update, separating slope calculation from coordinate finishing. Final Y still requires converted bucket Y. Before implementation, a source/dataflow review must identify which packed values can be dropped and what input reloads would replace them. Extra call, reload, packing and stack costs must be charged; the profile does not establish this as the cause or promise a benefit. Neither prior boundary is repeated. Tiny constant selection in place of integer multiplication is deferred because the present evidence does not identify that operation as the dominant target.

All **10 offline workflow commands succeed**: eight symbol/segment/disassembly/annotation commands and two analyzer build/execution commands. Full commands and stdout/stderr hashes are retained. No successful scientific recording is replayed. The canonical proof inventory remains **44,265**. No new AWS/Aeglos, ARM/mobile, huge-circuit, pressure/swap or minimum-RAM result is added.

Accepted local 32-worker full-proof measurements and supplied M5 references below are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 / 4.754×|24 / 3.083×|3.7|
|2×2|10.932731|59 / 5.397×|30 / 2.744×|5.9|
|4×2|15.757088|109 / 6.918×|48 / 3.046×|10.9|
|8×2|23.973295|189 / 7.884×|74 / 3.087×|18.9|
|12×2|33.827269|284 / 8.396×|101 / 2.986×|28.4|
|16×2|39.900917|335 / 8.396×|124 / 3.108×|33.5|
|24×2|66.041083|528 / 7.995×|176 / 2.665×|52.8|
|merge-24×1|65.717994|507 / 7.715×|175 / 2.663×|50.7|

The verified CPU 10× objective remains active and unmet, **0/8**. Merge54 remains 162.170158 ms without a supplied M5 reference. A new candidate must pass complete-caller evaluation, then full-query/ALL8+54 SDK, fresh Gnark verification and resource checks before adoption.

[Analysis summary](artifacts/analysis.json), [protocol](artifacts/protocol.md), [Rust analyzer](artifacts/analyze.rs), [input identities](artifacts/inputs.json), [offline commands](artifacts/receipts.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Every mapped instruction row, tool output and full command remains in the indexed UTF-8 archive. Private binaries, raw perf payloads and unchanged inherited decoded samples/metadata are excluded; reproduction requires the pinned private inputs.
