# Fixed-limb unreduced GCD coefficients

**The standalone fixed-limb arithmetic checks pass. Speed and production integration remain unqualified.** This implements the [positive coefficient-count model](../2026-11-08-unreduced-gcd-coefficients176/README.md), preserving the original approximate integer GCD, 31-bit matrices, paired composition and one-word Euclid finish. Production source and accepted proof timings remain unchanged.

The implementation keeps two exact sign-magnitude coefficients in fixed 17-word arrays with active lengths. It cancels their accumulated dyadic denominator once at the end using an 18-word local buffer, restores the sign and multiplies by a compile-time Montgomery scale. There is no heap buffer, lookup table, key copy, retained plan or pool. Fixed array payloads are not a complete stack, process-memory or minimum-RAM bound.

The widened final Euclid row can have equal-sign product terms whose sum exceeds signed i128 even though each term fits. The implementation uses unsigned u128 for equal-sign sums, signed i128 for opposite-sign differences, and explicit whole-row sign normalization. This preserves the model's full range instead of reusing the old signed two-product helper outside its contract. Original integer helpers and packed matrix routines are byte-exact fragments from accepted source176; the complete coefficient propagation and late correction are new. [Implementation](artifacts/native.rs), [design](artifacts/design.md).

## Arithmetic qualification

The standalone artifact uses O3, native CPU targeting, one codegen unit, overflow checks enabled and **LTO off**. The original design requested fat LTO. Its first compile fails on a field/method typo; the exact initial source and receipt are retained. The only source fix is `b.len()` to `b.len`. The second compile reaches LTO and fails because a reused dependency has no embedded LLVM bitcode. No arithmetic check runs in either failed workflow. A third workflow builds the unchanged corrected source without LTO and runs the new qualification once; both steps pass. Warnings remain preserved. [Scope correction](artifacts/qualification-scope.md).

|Check|Observed count|
|---|---:|
|Widened signed linear rows, including full carry/sign envelopes|3,927|
|Complete inverse case rows, both BN254 fields|17,912|
|Additional full inverses on 1/4/16/32-worker pools|71,648|
|Complete late-correction cases, both fields|144|
|Raw-zero checks returning None|2|

Each field checks 8,956 complete inverse case rows, including raw modulus/scale boundaries, power-of-two neighbors and 8,192 newly generated full-width random values. Case rows include repeated boundary values and are not claimed as distinct coverage. Results agree with independent BigUint Fermat inversion and the unchanged accepted field library, remain canonical, preserve inputs and satisfy the field inverse identity. The four additional threaded passes compare with those already computed expected outputs; they do not rerun the BigUint oracle. Each Rayon pool uses 512 KiB worker stacks. That establishes observed execution, not a whole-stack bound or portable/device qualification.

Correction tests cover both signs, zero, broad magnitudes and shifts through the retained 992-bit cap. Exact BigInt row checks cover lengths through 16 input words and coefficients up to signed-i64 magnitude limits. The conservative capacity envelope is conditional on the unchanged 32-round cap. This work does not newly prove all-input GCD termination or every early-exit branch's dynamic coverage.

The offline reader passes on its first execution. It checks the 189/191/194-pin populations, the initial source reconstruction, exact accepted helper fragments, four command receipts including two failed builds, stream hashes, actual non-LTO argv and qualification count/schema bindings. It does not rerun arithmetic checks or independently prove the fixed-limb implementation. All 176 production hashes remain unchanged. No old test suite, mathematical model, successful build, recording or decoder is replayed. [Qualification](artifacts/qualification.json), [reader](artifacts/review.rs), [review](artifacts/review.json). The full staged whitespace check flags one blank line at the end of the byte-exact accepted integer-helper fragment; it is preserved for source binding. The authored-file check excluding only that fragment passes.

**Next: create one coherent fat-LTO field/consumer artifact, inspect its actual optimized full-inverse bodies and freeze a controlled timing protocol.** The non-LTO correctness artifact is not a timing control or a substitute for that stage. Complete G1/G2 callers and verified full-SDK/all-shape/resource checks remain necessary before adoption. The parent model's roughly 53% reduction concerns coefficient/finish product expressions; it is not an inverse or proof-speed measurement.

## Inherited CPU and Metal comparison

No time is measured in this stage. Accepted local 1×2 remains **roughly 7.9–8.0 ms** with the same binary: older 7.915/7.929/7.996 ms, latest 8.003 ms. The variation remains unexplained; no adopted regression is established.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 / 4.623×|24 / 2.999×|3.7|
|2×2|11.241463|59 / 5.248×|30 / 2.669×|5.9|
|4×2|15.933816|109 / 6.841×|48 / 3.012×|10.9|
|8×2|24.234311|189 / 7.799×|74 / 3.054×|18.9|
|12×2|33.855565|284 / 8.389×|101 / 2.983×|28.4|
|16×2|40.071563|335 / 8.360×|124 / 3.094×|33.5|
|24×2|65.917462|528 / 8.010×|176 / 2.670×|52.8|
|merge-24×1|65.305023|507 / 7.764×|175 / 2.680×|50.7|

Source: [preceding accepted comparison](../2026-11-08-unreduced-gcd-coefficients176/README.md). Seven nonprimary rows remain from the earlier full campaign. Merge54 remains 162.240109 ms without a supplied M5 reference. The original all-eight verified CPU 10× goal remains active and unmet, **0/8**. No proof, SDK request, profile, RSS, AWS/Aeglos, huge-circuit, phone/ARM/TEE, pressure/swap, minimum-RAM or concurrent-throughput result is added. Accepted production176/SDK6d83 and the canonical 35,181-proof inventory are unchanged.

[Evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). The archive preserves exact initial/fixed sources, plans/pins, both failed builds and the successful standalone qualification streams. Executables, libraries, unchanged full production source, private key/image/request/witness/proof/public arrays and complete proof inventories are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/unreduced-gcd-native176`.
