# Auxiliary-prime Euler derivative: algebra prototype

**The exact operator prototype passes correctness checks, including every output of a 32,768-point case.** No speedup, production adoption or complete-proof result is claimed. This qualifies the algebra for a separate reconstruction/caller experiment; it does not qualify the BigInt reference implementation as a faster prover.

The accepted Euler product uses two Fr transforms for each of A and B. This experiment replaces that differentiation operator with an auxiliary-prime convolution. It converts at the boundary of a whole convolution, unlike the previously rejected per-field RNS arithmetic experiments. No proving-key point table or duplicate solver plan is introduced.

For roots `x_i=omega^i`, differentiating the Lagrange cardinal polynomials gives

```text
E(A)(x_i) = sum_j K[(i-j) mod n] * A(x_j)
K[0] = (n-1)/2; K[j] = 1/(omega^j-1), j != 0
E(f) = X*f'
```

With `rho` of order `2n`, twisting both inputs by `rho^-j` turns this cyclic convolution into a negacyclic convolution over Fr. Restoring output coefficient `j` by `rho^j` changes the negative wrap back to the required positive wrap. The implementation includes the accepted unnormalized factor `n` and returns `n*(E(A)*B+A*E(B))`, truncated to the original requested output length. The twist occurs in Fr; reducing Fr roots into unrelated auxiliary fields would not establish this identity. [Design](artifacts/design.md), [implementation](artifacts/src/lib.rs).

Twisted canonical coefficients lie in `[0,r)`. Every signed integer negacyclic coefficient is bounded in magnitude by `C=n*(r-1)^2`. The fixed descending prime selection finds 17 distinct primes below `2^31`, all congruent to1 modulo65536. Their product has527 bits and exceeds16C for n32768; C has523 bits. This is enough to reconstruct signed coefficients exactly before reducing modulo Fr. The reference uses BigUint Garner reconstruction. [Algebra result](artifacts/algebra.json).

The auxiliary transforms use the published pure-Rust [tfhe-ntt0.7.1 Plan](https://github.com/zama-ai/tfhe-rs/blob/49c390edeffdc211811ff96b98d34aa0278dca14/tfhe-ntt/src/prime32.rs), with its default AVX512 feature enabled. The dependency and transitive source hashes are pinned; no upstream runtime code is copied or changed. The feature selection does not constitute a dynamic instruction trace. [Dependency lock](artifacts/Cargo.lock), [identity and license](artifacts/upstream.json). FLINT's [small-prime transform documentation](https://flintlib.org/doc/algorithms/fft/small-prime-fft.html) supplied background on using auxiliary transforms; its floating-point backend is not used here and none of its timing claims are transferred.

Three new tests pass once:

- The circulant identity and field twist agree with independently differentiated polynomial coefficients on domains2 through32.
- Each of the17 auxiliary transforms agrees with exact signed integer negacyclic coefficients at length32, including negative wrap terms.
- The complete CRT product agrees with direct Fr interpolation and derivative evaluation on domains32/64/128, with zero, unit, near-modulus, full-width, partial and empty inputs. Insufficient CRT range is rejected.

A separate full-domain call checks all32,768 outputs against low-degree coefficients differentiated and evaluated independently. It covers the complete operator at the primary domain size, with synthetic inputs; it is not a proof-witness benchmark or full-width random large-domain qualification. The independent reviewer verifies primality by trial division, all1,421 pins, the exact coefficient/product bounds, three successful workflow commands and hashes of their streams, and all176 unchanged production files. [Review](artifacts/review.json), [review source](artifacts/review.rs), [workflow](artifacts/plan-fixed.json).

Known public domain-context array payload is **12.625MiB**:

|Arrays|Bytes|
|---|---:|
|Four auxiliary root arrays per prime|8,912,896|
|Transformed kernel arrays|2,228,224|
|Fr twist and restore arrays|2,097,152|
|Sum|13,238,272|

This excludes ownership headers, alignment/allocator overhead, BigUint constants and all query scratch. It is not peak RSS, minimum RAM or a pressure qualification. Future use must account for complete live workspace and context ownership, preserve the original FFT under admission denial or unsupported domains, and avoid constructing large domain tables. No low-RAM/mobile or huge-circuit benefit follows from this prototype.

The next possible reconstruction computes only `c mod r` as a fixed modular dot product. For `M=product(q_i)`, `M_i=M/q_i`, `t_i=residue_i/M_i mod q_i`, the signed coefficient is `sum(t_i*M_i)-round(sum(t_i/q_i))*M` under the proven coefficient bound. A separate exact BigInt check verifies this formula on1,035 fixed signed/bounded examples, using extended Euclid for the inverse constants. Those examples are not a universal floating-point rounding-error proof. A native implementation still needs that proof, import/output scaling checks, range/overflow qualification and complete caller timing. [Unimplemented reconstruction notes](artifacts/next-math.md).

The first fetch ran before a Cargo target existed and failed before dependency work. The first build then stopped before tests because a Rustup shim attempted a toolchain update and failed. The repair pins explicit installed Rust1.98.1 cargo/rustc/rustdoc paths and selects that toolchain in the environment. The reviewer confirms that only those paths/environment changed, all original content pins still match, and source, dependencies and mathematical inputs remained unchanged. Original failures and corrected receipts are retained; no successful test or full-domain call was repeated. Existing field/core/prover qualification is inherited, not rerun.

There are no new SDK requests, proof exports, Gnark verifications, actual prover latency, CPU profiles, AWS/Aeglos comparisons, memory-pressure or concurrent-throughput measurements. The unique-proof ledger remains29,953. Root preparation and BigInt costs have not been timed against the accepted FFT. Inputs are synthetic public algebra fixtures; no private keys/images/witnesses/point arrays or executable/library blobs are published.

These are unchanged accepted local32-worker observations against the supplied historical M5 references, not measurements from this algebra study. Hardware differs. The [latest full-SDK experiment](../2026-11-05-finalize-drain-bundle176/README.md) retains its measured primary gain and failed adoption gate.

|Shape|Accepted CPU ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|4.667×|24|3.027×|3.7|
|2x2|11.151932|59|5.291×|30|2.690×|5.9|
|4x2|16.086386|109|6.776×|48|2.984×|10.9|
|8x2|24.179453|189|7.817×|74|3.060×|18.9|
|12x2|33.884455|284|8.381×|101|2.981×|28.4|
|16x2|40.077381|335|8.359×|124|3.094×|33.5|
|24x2|65.850777|528|8.018×|176|2.673×|52.8|
|merge24x1|65.060937|507|7.793×|175|2.690×|50.7|

Accepted merge54x1 remains161.883899ms without a supplied M5 reference. The verified all-eight CPU10× goal remains active and unmet,0/8. A correct new representation is a qualification step, not an achieved speed target.

The initial compiler stderr contains trailing whitespace. Its public receipt is stored as `stderr.log.json`, whose UTF-8 text and original SHA256 reconstruct the raw stream byte-for-byte. This is a publication encoding change; original logs and mathematical sources remain unchanged.
