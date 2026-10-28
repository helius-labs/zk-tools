# Quadratic BN254 base-field arithmetic screen

The two exact representations are algebraically valid, but the specified coupled reduction offers no operation-count advantage over current ordinary CIOS. The closed R131 box uses **111 IFMA sites versus100**, with six native coordinate vectors instead of five. No native kernel, SDK trial or production change follows. This is a mathematical screen, not a measured performance regression.

The [accepted checkpoint](../2026-10-27-clean-fft-adoption/README.md) remains 8.324853 ms /134.50 MiB for local1×2. The original all-eight CPU10× goal remains active and unmet,0/8.

## Construction and bounds

For u=4965661367192848881, the base-field prime has the locally derived identity p=(6u²+3u+1)²+3u². Test both a basis with tau²=-3 and pi=(6u²+3u+1)+u*tau, and an Eisenstein basis omega²+omega+1=0 with pi=(6u²+4u+1)+2u*omega. Both norms equal the actual Fq prime. Evaluating the basis element at -A/B modulo p supplies the field isomorphism.

The seed and prime match primary [Arkworks configuration](https://github.com/arkworks-rs/curves/blob/master/bn254/src/curves/mod.rs) and [Fq source](https://github.com/arkworks-rs/curves/blob/master/bn254/src/fields/fq.rs), and pinned local production constants. [Safieh and Freudenberger2021](https://www.mdpi.com/2410-387X/5/1/6) supplies general Gaussian-ring Montgomery background; it supplies no performance claim for this Fq proposal. Direct article access this turn returned HTTP429; the indexed abstract and prior Gaussian studies were available. The present identities, bounds and work model are local derivations.

Centered ring Montgomery cancellation computes m=-T/pi modulo R and z=(T+m*pi)/R with exact coefficient divisibility. [Bounds and derivation](artifacts/bounds.md) give sufficient rectangular-box closure for |coordinates|≤H=2^126+3B, plus the exact conversion oracle.

| Basis | Radix bits | Sufficient box closure | Template IFMA sites | Terminal high coefficient survives |
|---|---:|---|---:|---|
| sqrt-minus-three | 128 | false | 106 | false |
| sqrt-minus-three | 130 | false | 106 | false |
| sqrt-minus-three | 131 | true | 111 | true |
| sqrt-minus-three | 132 | true | 111 | true |
| eisenstein | 128 | false | 106 | false |
| eisenstein | 130 | false | 106 | false |
| eisenstein | 131 | true | 111 | true |
| eisenstein | 132 | true | 111 | true |

The sqrt(-3) R130 form has18 actual out-of-box results in the fixed set, including an archived counterexample. The Eisenstein R130 form has none in that set; its sufficient inequality remains inconclusive, and finite samples do not prove closure. R131 sufficient inequalities pass for both forms. The tested box is an implementation invariant proposal, not a universal constraint on every representation.

Increasing R130 to R131 changes the final divisor from2^26 to2^27. The previously discarded coefficient at2^156 contributes2^129 after division and survives modulo2^131. Retaining those five high IFMA halves changes the coupled template from106 to111 sites. Porting the old truncation unchanged would be incorrect.

## Work assessment

The coupled template forms each quadratic product from three ordinary coefficient products, skips the modulus's zero third imaginary digit and counts every required low/high IFMA. It needs three coupled cancellation rounds. The current ordinary five-limb cancellation-carry template has50 raw,5 cancellation and45 modulus IFMAs, total100. Quadratic R131 has54 raw,9 cancellation and48 modulus IFMAs, total111. R130 would still have106 sites, before any range repair.

These counts exclude shifts/additions for small coefficient factors, carries, sign corrections, normalization, conversion, memory and stack effects. They are counts for one explicit construction, not machine-code measurements, cycle estimates or a lower bound on every possible algorithm. Fewer serial rounds alone are not a new advantage: the [rejected Gaussian Fr CIOS study](../2026-10-27-gaussian-fr-cios/README.md) already used three rounds and a small second modulus coordinate. No distinct dependency improvement over that negative construction is identified here.

Stored signed coordinates fit a pair ofi128 values,32bytes, but the working representation expands to six radix52 vectors versus five in current arithmetic. That fact is not a proof-RAM or register-allocation measurement. Native import/export, addition normalization, complete curve formulas and pressure behavior have not been implemented. The exact conversion oracle uses large-integer division and is not proposed as a production converter.

## Validation

Three new Rust tests pass on the first invocation: exhaustive arithmetic in two small prime rings, signed centered cancellation, and the independent terminal-coefficient divisibility condition. The exact BN254 model then checks both bases. Per basis, it checks2,401 corner pairs and4,096 full-width box pairs at each of R130/R131, plus2,048 changing R131 multiplication/addition steps and2,048 exact imports. Three-product expressions match direct quadratic multiplication; field results match independent prime-field arithmetic; all R131 outputs fit the proven box bounds. The fixed samples are not extended to seek an R130 counterexample.

All172 production source hashes, the observer/helper inputs and actual BigInt/serialization/hash libraries are pinned before model execution and rechecked afterward and at closure. No inherited prover, field, Gaussian or verifier suite is repeated. This study uses exact integer arithmetic only: zero new SDK requests, proofs, exports, verifier calls, captures or timing measurements. Ledger remains **12,524**. No source is adopted.

No new AWS, Aeglos, phone, TEE, concurrency, memory-pressure, minimum-RAM or giant-circuit qualification is claimed. The result rejects proceeding with this specified template; it does not reject every quadratic representation. A next distinct question is whether a residue-basis reduction can remove serial carry dependencies after paying for basis extension, conversion and live registers. That direction needs its own exact cost analysis before a native trial.

## Inherited CPU and Metal comparison

The accepted timings below and supplied historical M5 references are inherited; none are measured by this mathematical screen. The canonical report retains its full-performance-gate failure and engineering exception.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445× | 24 | 2.883× |
| 2x2 | 11.460675 | 59 | 5.148× | 30 | 2.618× |
| 4x2 | 15.854365 | 109 | 6.875× | 48 | 3.028× |
| 8x2 | 24.084626 | 189 | 7.847× | 74 | 3.072× |
| 12x2 | 33.693397 | 284 | 8.429× | 101 | 2.998× |
| 16x2 | 39.981636 | 335 | 8.379× | 124 | 3.101× |
| 24x2 | 66.010670 | 528 | 7.999× | 176 | 2.666× |
| merge24x1 | 65.086624 | 507 | 7.790× | 175 | 2.689× |

Accepted merge54 remains161.341835ms without a supplied M5 reference.

[Design](artifacts/design.md), [model](artifacts/model.rs), [all exact results](artifacts/analysis.json), [bounds](artifacts/bounds.md), [tests](artifacts/tests.log), [source pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
