# Arithmetic audit of the disabled scheduler build

The arithmetic comparison finds no changed instructions in the selected hot kernels after accounting for relocated references. **16 function instances contain 62,887 decoded instructions and 747 explicit readonly literal loads.** Instruction positions, sizes, mnemonics, registers, stack offsets, immediates and local branch targets agree; all checked literal payloads agree. Three remaining raw target differences are calls to `_Unwind_Resume@plt`, confirmed by separately decoded PLT entries.

This rules out changed lowering of those selected arithmetic bodies as an explanation. It does **not explain the 8.11% disabled-path slowdown**, prove whole-program equivalence, or resolve the failed control calibration. The preceding [batch-transfer screen](../2026-10-27-batch-local-steal/README.md) remains rejected: primary32 ON/OFF is 2.18% slower with 0/7 wins. Production and accepted latency remain unchanged. The original all-eight CPU 10× goal remains unmet, 0/8.

## Scope

The exact accepted control is `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`; the candidate used OFF/ON is `fde4038c7106968518666299f422911abaf5f1408e84c9c006d2de28201e5ab3`. Both executables, actual direct libraries and recorded features are checked. Their linked arithmetic has equal native feature sets. The 172 production source hashes remain unchanged. No prover build, request, performance capture or Gnark verification is repeated in this audit; the proof ledger remains 12,518.

Symbols are matched by demangled name and address-order occurrence for duplicates. Every instantiated symbol in the declared G1 streaming, G2 prefix, G1/G2 tree, Fq inverse, vector inverse and borrowed-window method families is checked. Direct control targets normalize to same-function byte offsets or named symbols with occurrence indices. RIP-relative displacements normalize separately from checking explicit-width `.rodata` payloads. Non-RIP register and memory operands remain significant. This is not a comparison of every function, indirect pointer binding, runtime relocation, input distribution, allocation address, cache behavior, frequency or scheduling. Static agreement cannot identify removable time.

|Function family|Instances|Decoded instructions|
|---|---:|---:|
|Borrowed-window methods|4|1606|
|G1 streaming|5|22349|
|G1/G2 affine trees|2|17510|
|G2 prefix|2|16805|
|Scalar Fq inverse|2|3478|
|Vector inverse|1|1139|

[Per-function comparison](artifacts/analysis.json), [RIP references](artifacts/RIP-references.json), [PLT resolutions](artifacts/unwind-PLT-resolutions.json), and [direct library hashes](artifacts/direct-library-comparison.json) retain the evidence. Raw disassemblies are local with hashes. They were decoded from the existing binaries without executing them.

Five new observer tests pass across two invocations. They cover duplicate names and names with spaces, relative branches and named calls, preservation of register/stack/immediate changes, RIP widths/offsets and selecting borrowed-window methods. The unchanged ELF section reader and its existing test are inherited, not rerun. The first selection audit stopped because the borrowed-window matcher omitted its leading `<`; original source/log are retained. That selector alone was repaired and one targeted test added; four already successful normalization tests were not repeated. The audit then completed.

## Next diagnostic

Do not infer a new arithmetic rewrite, layout sweep or unchanged batch-steal retry from this result. A separately declared IBS demand-load diagnostic comparing these exact control and OFF executables could test whether load-source/latency behavior changed in the G2 prefix kernel. The [older IBS study](../2026-10-23-local-ibs-memory/README.md) used different source/binaries and does not answer this comparison. No such new capture occurs here. Memory metadata would still not prove a causal speedup, bandwidth saturation or insufficient RAM. A new optimization needs a distinct mechanism supported by measured whole-proof work.

## Inherited CPU and Metal comparison

These are the accepted local observations against supplied historical M5 figures, not new timings from this audit. The [canonical all-shape report](../2026-10-27-clean-fft-adoption/README.md) retains its full-performance-gate failure and engineering exception.

|Shape|Accepted ms|CPU reference ms|CPU/accepted|Metal reference ms|Metal/accepted|
|---|---:|---:|---:|---:|---:|
|1x2|8.324853|37|4.445×|24|2.883×|
|2x2|11.460675|59|5.148×|30|2.618×|
|4x2|15.854365|109|6.875×|48|3.028×|
|8x2|24.084626|189|7.847×|74|3.072×|
|12x2|33.693397|284|8.429×|101|2.998×|
|16x2|39.981636|335|8.379×|124|3.101×|
|24x2|66.010670|528|7.999×|176|2.666×|
|merge-24x1|65.086624|507|7.790×|175|2.689×|

Accepted merge54 remains 161.341835 ms with no supplied M5 reference. The latest fresh primary control observation remains 8.358001 ms from the preceding screen. No AWS, portable, pressure, concurrency, phone, TEE, minimum-RAM or large-circuit qualification is added.

[Design](artifacts/design.md), [observer tests](artifacts/tests.log), [additional selector test](artifacts/selector-test.log), [artifact hashes](artifact-index.json).
