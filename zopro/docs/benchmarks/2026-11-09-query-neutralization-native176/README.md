# Full-proof query neutralization: native qualification

The bounded diagnostic passes native correctness tests and coherent OFF/ON SDK builds. **No new full-SDK timing or proving optimization is established.** Its next use is to measure which query work actually controls full-proof latency. Production remains unchanged; the verified all-eight CPU 10× objective remains active and unmet, 0/8.

The diagnostic learns unblinded A, G2 and K/H query results during the first prime, running all original work. Later requests may substitute selected results for the exact same key generation, witness SHA256, omit-B mode and query path. Solving, quotient construction, corrections, fresh blinding, assembly, serialization and verification remain. Such shortened timings measure controlled work removal; they cannot count toward fresh-work prover speed or M5/Aeglos comparisons. Effects need not add because removing work changes Rayon helping and overlap.

Only four G1 results and one G2 result are retained: **576 bytes of coordinates**. The native controller occupies **944 bytes**, including embedded metadata/mutex/counters, before Arc and allocator overhead. These are object sizes, not RSS, minimum RAM or measured mobile footprints. No key, witness, scalar, plan or point-array copy, new production pool or large lookup table is introduced. The state is private to a diagnostic session.

## New qualification

|Check|Result|
|---|---|
|Full changed core suite|131 passed, zero ignored/filtered|
|Raw key/image suite|2 passed, including sparse >4 GiB cases|
|Cubic end-to-end suite|8 passed: six existing plus two diagnostic tests|
|All eight masks and first learning proof|9 distinct randomized proofs, native-verified|
|Changed witness, replacement key, mutated key|3 native-verified full-computation fallbacks|
|Actual worker indices|53 across pools of 1/4/16/32, requested 512 KiB stacks|
|Generic no-ASM/no-IFMA core with diagnostic selector|Compiled|
|Coherent native SDKs|Fresh OFF and diagnostic ON linked with O3/fat LTO/one codegen unit|

The six new core tests cover point preservation for fused and separate K/H topologies, every mask, binding/mode changes, incomplete learning, unwinding, duplicate callbacks, outstanding requests, nested/distinct owners, shared-owner rejection and explicit context transport through nested Rayon joins. Key epochs survive moves and change on preparation/invalidation/GPU release; unsupported requests compute normally. The actual cubic proof path observed separate K/H (path 2, topology 29). The all-shape SDK preflight must still establish coverage for other paths, particularly fused K/H and the small-core entry hop. Synthetic transport tests do not establish full-SDK coverage.

The exact accepted field library `1200…` is reused. Reversing declared core edits recovers all 176 production originals, with one private child module added. The new SDK driver includes scope/hash/lookup overhead inside each full-request interval and reports priming and timed coverage separately. Its three edits and suffix recover the original driver exactly. A separately written same-agent reader verifies 389 merged source/library pins, recovery, stream hashes and the qualification results; its execution workflow binds 470 inputs. This is not an external audit.

Seven linked functions were decoded once: three query/solve roots in each SDK plus the ON path checker. OFF has zero diagnostic-module symbols; ON has eleven. The roots show diagnostic path dispatch, while complete Rayon callback-body inspection and full-shape runtime coverage remain capture-stage work. Static instruction sites are not cycles or proof latency. The 12 new diagnostic proofs use the existing native verifier; no new independent Gnark export or full-SDK request is claimed. Canonical SDK proof inventory stays 37,023.

## Preserved failures and limits

The original core workflow stopped after its successful 131-test command because copied required-test metadata still named three old tree-policy tests. Its exit-0 test receipt and workflow failure are preserved; successful tests were not rerun. The pending generic check ran once in the subsequent 13-command SDK workflow. Two initial code-inspection commands failed before decoding any instruction body: inherent-method demangled names include angle brackets, and GNU objdump's demangled selector did not match the raw symbol. The corrected seven-body inspection and qualification reader pass. Together there are 21 command receipts: 19 successes and those two inspector failures; the core metadata rejection is a separate workflow failure.

Two SDK-plan helper attempts also failed before that workflow: the benchmark driver/support files were absent from the 176-file manifest, then getrandom had two frozen versions. Explicit tracked driver/support paths and the accepted core's exact getrandom binding resolved them. Initial sources are retained. Helper compiles/preparation/publication have tool-transcript evidence, not additional workflow receipts. No successful native suite or decoder was replayed.

No latency, PMU/profile, RSS, minimum-RAM, concurrent full-proof, giant-circuit, AWS, ARM/phone or TEE claim follows from this qualification. The next stage must freeze calibration/observer limits, inputs, all-arm ALL8+merge54 coverage/Gnark preflight, primary 16/32-worker and 4×2/32-worker block captures, resources and automatic profiles before any recording. Diagnostic timings must remain labeled as work removal. See [next steps](artifacts/next.md) and [design](artifacts/design.md).

## Accepted full-proof comparison, inherited

The unchanged accepted executable's latest local 32-worker observations come from the [standard-SDK campaign](../2026-11-08-tree-policy-sdk176/README.md). Supplied M5 references use different hardware; nothing in this diagnostic is included in these times.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1×2|8.040649|37 / 4.602×|24 / 2.985×|3.7|
|2×2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4×2|16.069415|109 / 6.783×|48 / 2.987×|10.9|
|8×2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12×2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16×2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24×2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge24×1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge54 remains 160.890372 ms without an M5 reference. Accepted local 1×2 previously measured **7.915095 ms** on the same executable; private 7.859756/7.869400 ms candidates were unadopted. The latest 8.040649 ms does not establish an adopted regression or its cause. Describe accepted local 1×2 as roughly 8 ms while preserving the individual observations.

[Qualification and identities](artifacts/qualification.json), [core recovery](artifacts/core-integration.json), [linked code](artifacts/codegen.json), [diagnostic module](artifacts/query_neutralization.rs), [SDK driver](artifacts/driver.rs), [new proof tests](artifacts/cubic-extra.rs), [source/receipts/disassembly archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS). Archive members are UTF8 with individual SHA256 hashes and verified gzip roundtrip. Binaries/libraries, unchanged production source, keys/witnesses/proofs and full third-party source are excluded. Reproduction requires the pinned accepted source/dependencies and adapting absolute paths.
