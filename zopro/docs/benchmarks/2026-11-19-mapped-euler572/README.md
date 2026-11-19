# Mapping a preconverted Euler key on merge-24×1

The offline converter replaces the original private and public G1 arrays in place in a new image, preserves the slot list, uses a distinct file format identifier, and embeds the lossless public origin. The runtime maps the transformed coefficients and decodes the row dictionary once. Runtime processes are required to emit the mapped-origin event and zero key-conversion events. Verification uses the original, unmodified image and separately hashed public inputs.

Five randomized pairs compare complete ordinary and transformed SDK runs on the original and four held-out requests. Both processes in each pair use the same resident key for all five inputs, priming each before 21 verified requests. Three additional pairs on the original request run under each of two enforced Linux memory limits with swap disabled. Peak RSS includes setup and earlier requests in the process. The filesystem cache was warm; cgroup measurements do not establish minimum physical RAM.

| Request | Limit MiB | Completed ordinary / mapped | Ordinary / mapped ms | Paired mapped vs ordinary | Median running peak MiB ordinary / mapped |
|---|---:|---:|---:|---:|---:|
| Original | Uncapped | 5 / 5 | 63.078 / 61.007 | -3.60% (5/5) | 1086.250 / 1040.000 |
| 112 | Uncapped | 5 / 5 | 67.542 / 65.310 | -3.09% (5/5) | 1286.859 / 1217.406 |
| 124 | Uncapped | 5 / 5 | 68.123 / 67.756 | +0.05% (2/5) | 1358.262 / 1313.406 |
| 122 | Uncapped | 5 / 5 | 66.639 / 65.938 | -0.69% (4/5) | 1400.512 / 1354.906 |
| 109 | Uncapped | 5 / 5 | 66.115 / 63.114 | -3.96% (5/5) | 1434.262 / 1391.133 |
| Original | 950 | 3 / 3 | 141.048 / 131.761 | -6.97% (3/3) | 631.465 / 624.254 |
| Original | 1536 | 3 / 3 | 129.265 / 120.981 | -7.52% (3/3) | 825.684 / 815.891 |

63 exported proofs verified independently. The converter's full-domain group FFT scratch is charged to the offline process. Generic arbitrary-parts requests are outside the experimental image's supported interface. [Image identities](artifacts/images.json) · [Samples and limit events](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
