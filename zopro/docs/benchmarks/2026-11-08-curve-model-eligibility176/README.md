# Alternative curve-model eligibility on BN254

**Twisted Hessian, Montgomery and twisted Edwards conversions over the current coordinate fields are unavailable for both prover groups.** Exact character certificates show that G1 over Fp and the actual G2 twist over Fp2 have no rational points of order two or three. No conversion code, native optimization, timing or proof is added; production remains unchanged. This closes this specific coordinate-model route, not all arithmetic or MSM optimizations.

This checks a change of algebraic model before investing in addition kernels. Montgomery curves contain rational2-torsion at (0,0); twisted Edwards curves contain it at (0,-1). Their equations and group laws are in the [Explicit-Formulas Database](https://www.hyperelliptic.org/EFD/g1p/auto-montgom.html), [twisted Edwards entry](https://www.hyperelliptic.org/EFD/g1p/auto-twisted.html). Section5 of [Twisted Hessian Curves](https://eprint.iacr.org/2015/781) establishes rational3-torsion for twisted Hessian curves over finite fields. Its Section5 text was available in search; the author PDF fetch timed out, so full-paper reading is not claimed. A field-defined group isomorphism preserves rational torsion. No external code or performance estimate is imported.

For y²=x³+B, finite2-torsion requires x³=-B. The certificate (-B)^((q-1)/3)≠1 rules out that root when q≡1 modulo3. Finite3-torsion satisfies ψ3=3x(x³+4B)=0. At x=0 its y-coordinate would square to B; at x³=-4B it would square to -3B. Both are quadratic nonresidues, excluding all finite3-torsion. These checks apply to the full coordinate-field curve, not just its chosen prime-order subgroup. They do not use an asserted full group order.

|Actual group|Coordinate field|Quadratic character B / -3B|Cubic character of -B|Rational2-torsion|Rational3-torsion|
|---|---|---|---|---|---|
|G1|Fp|-1 / -1|β ≠ 1, β³=1|Absent|Absent|
|G2 sextic twist|Fp2|-1 / -1|β ≠ 1, β³=1|Absent|Absent|

Here p=21888242871839275222246405745257275088696311157297823662689037894645226208583 and β=21888242871839275220042445260109153167277707414472061641714758635765020556616. G1 uses B=3; G2 uses B=3/(9+u), u²=-1. The independent integer implementation reconstructs this twist coefficient and matches the actual production constants. It does not substitute the untwisted Fp curve into the Fp2 test. Full public inputs, exponents and residues are in the [certificates](artifacts/analysis.json).

The original native field library and a separately implemented BigUint Fp/Fp2 arithmetic agree on all six character values. Native square-root rejection supplies a secondary same-backend check. Complete point enumeration over96 prime-field curves and12 extension-field curves checks5,585 finite points: direct [2]P/[3]P group addition agrees with the torsion-polynomial criteria. All108 small-curve character cases pass, including71 certificates excluding2-torsion and53 excluding3-torsion. These toy cases test the criterion implementation; they are not an exhaustive BN254 field proof.

One new Rust1.98.1/xtask screen and one separately written certificate/receipt review pass on their first executions. All176 accepted source hashes match, with182 screen pins. The reviewer checks exponents, certificate inputs/results, implications, counts and exact receipts without rerunning the native/toy screen. It is by the same agent, not an external audit. An unused Field import warning remains; no cleanup rebuild was performed. Helper compilation/preparation/publication have tool-transcript evidence rather than complete workflow receipts.

The result concerns isomorphic models over unchanged coordinate fields. Extension fields, other curves/protocols, nonisomorphic mappings and different addition formulas are not evaluated or ruled out. There is no latency, memory-peak, minimum-RAM, concurrency, mobile, giant-circuit or AWS/Aeglos result. No key copies, lookup tables or retained solver plans are introduced.

## Accepted CPU and Metal comparisons, inherited

The [latest full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md) supplies these local32-worker accepted-build medians. Supplied M5 CPU/Metal references were not remeasured; ratios compare different machines. Historical same-build primary7.915095ms remains valid; latest8.040649ms does not establish an adopted regression.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|8.040649|37|24|4.602×|2.985×|3.7|
|2×2|11.149097|59|30|5.292×|2.691×|5.9|
|4×2|16.069415|109|48|6.783×|2.987×|10.9|
|8×2|24.131178|189|74|7.832×|3.067×|18.9|
|12×2|33.940628|284|101|8.368×|2.976×|28.4|
|16×2|40.164083|335|124|8.341×|3.087×|33.5|
|24×2|66.187704|528|176|7.977×|2.659×|52.8|
|merge24×1|65.361571|507|175|7.757×|2.677×|50.7|

Merge54×1 remains160.890372ms without a supplied M5 reference. All eight CPU10× targets remain active and unmet,0/8. Accepted SDK6d83 and canonical37,023-proof ledger are unchanged.

[Design](artifacts/design.md), [screen](artifacts/screen.rs), [separate review/publisher](artifacts/close.rs), [review result](artifacts/review.json), [source pins](artifacts/pins.json), workflow/command/exit/stream receipts and [checksums](SHA256SUMS) preserve the study. All numerical parameters are public curve constants; no keys, witnesses, point/scalar fixtures, proofs, executables or libraries are published.
