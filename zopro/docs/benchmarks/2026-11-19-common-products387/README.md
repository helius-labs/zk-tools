# Repeated products in G2-query constraints

Each constraint is examined for factors of the form a·x + b and c·y + d. The nonconstant part of their product is proportional to x·y. Two rows with the same unordered wire pair can therefore give a linear equation after cross multiplication and subtraction. When x equals y and is an output of a Bits hint, x² equals x. The scan expands stored slots in the resulting affine equations, removes identities, and deduplicates equations modulo a nonzero scalar. Compound sum expressions with more than four terms are excluded as candidate factors. Slots in a factor are compared by exact slot identity. The circuit and key remain unchanged.

| Shape | Simple-factor rows | Distinct wire pairs | Distinct derived equations | Already present as direct rows | Derived equations touching B wires outside bit outputs | Direct pivot upper bound | Total B-query terms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 30764 | 30750 | 14 | 0 | 14 | 14 | 30089 |
| 2x2 | 53649 | 53631 | 18 | 0 | 18 | 18 | 52274 |
| 4x2 | 98293 | 98267 | 26 | 0 | 26 | 26 | 95537 |
| 8x2 | 188853 | 188811 | 42 | 0 | 42 | 42 | 183347 |
| 12x2 | 278997 | 278939 | 58 | 0 | 58 | 58 | 270757 |
| 16x2 | 369301 | 369227 | 74 | 0 | 74 | 74 | 358343 |
| 24x2 | 550965 | 550859 | 106 | 0 | 106 | 106 | 534619 |
| merge-24x1 | 525222 | 525090 | 132 | 0 | 132 | 132 | 507895 |

The primary circuit had 30,764 simple-factor rows and 30,750 distinct unordered wire pairs. Eliminating repeated products yielded 14 new equations. Its 1,753 matched Boolean-square constraints simplified to identities. Even combining the direct-row bound of 73 with these 14 gives at most 87 positions out of 30,089, or 0.29%, before rank and substitution costs. The scan does not justify building transformed G2 bases for these relations.

The upper bound counts possible direct eliminations within the derived equations. It excludes identities and allows for dependence among equations and additional replacement scalars. No speed measurement or proving-key transformation was made. All direct-row counts match the earlier independent scan.

[Image hashes and equation counts](artifacts/counts.json) · [Reader](artifacts/reader.py) · [Constant-factor rows](../2026-11-19-linear-rows386/README.md)
