# Verified full proofs with a mapped replacement B2 section

The experimental Euler image contains the same number of B2 curve points as the original. Each group sum replaces one member. The mapped index trailer adds eight bytes per point and a 16-byte header. The loader checks the permutation, group references and cycle markers with a temporary bit set. Ordinary images use the existing path. For the 12×2 and 24×2 plans all public wire offsets belong to ungrouped B2 members. Correction preparation uses their packed positions and checks the new B2 digest. All SDK outputs were verified separately against the original key and public witness.

The sparse path borrows the compact point prefix from the mapping and copies only nonzero tail corrections. At high exception density or when optional scratch is denied it permutes the already owned scalar buffer in place, subtracts each representative and runs a complete B2 MSM. Separate probes forced that path and set the optional memory limit to zero. The metadata validation and ordinary prover workspace still use memory. The trial covers native large-query B2 only; A and K use the ordinary query in every arm.

Four randomized blocks at 32 Ryzen workers compare the ordinary image, the ordinary image with a separately prepared compact B2 basis, the packed image, and the packed image forced through the dense path. Each process primed three proofs and timed eleven. RSS is the maximum whole-process high-water mark, including setup.

| Shape | Request | Ordinary / cached / packed / dense ms | Packed vs ordinary | Packed vs cached | Peak MiB ordinary / cached / packed |
|---|---:|---:|---:|---:|---:|
| 8x2 | Original | 23.766 / 23.158 / 23.388 / 26.623 | -1.58% (3/4) | +1.23% (1/4) | 419.5 / 469.5 / 469.0 |
| 8x2 | 153 | 24.569 / 23.093 / 23.085 / 26.612 | -6.11% (4/4) | -0.53% (2/4) | 440.0 / 492.5 / 472.2 |
| 12x2 | Original | 30.827 / 29.625 / 31.241 / 35.096 | +1.41% (2/4) | +5.16% (0/4) | 546.9 / 590.0 / 587.9 |
| 12x2 | 59 | 31.031 / 30.113 / 31.305 / 34.851 | +1.19% (0/4) | +3.50% (0/4) | 594.2 / 613.8 / 598.8 |
| 12x2 | 141 | 47.195 / 47.602 / 53.075 / 52.949 | +12.59% (0/4) | +11.02% (0/4) | 949.3 / 1010.4 / 991.9 |
| 24x2 | Original | 62.689 / 64.403 / 66.660 / 77.183 | +6.74% (0/4) | +3.48% (0/4) | 992.8 / 1034.1 / 1035.1 |
| 24x2 | 97 | 63.363 / 60.816 / 66.383 / 77.457 | +4.72% (0/4) | +8.82% (0/4) | 1067.9 / 1097.0 / 1016.6 |

[Process timings](artifacts/processes.json.gz) · [Image sizes and identities](artifacts/images.json) · [Execution routes](artifacts/probes.json) · [Diagnostic source](artifacts/incremental.patch) · [Independent verification](artifacts/verification.json)

The [offline packer](artifacts/pack_image.py) reproduced all three images byte for byte, including both offset files. The SDK rejected three independently corrupted 8×2 images before exporting a proof. [Validation results](artifacts/layout-checks.json)
