# A single plurality reference for the merge-24×1 prover

A fixed public plurality reference produced fewer nonzero residual H scalars than the modal reference on twelve independent requests. This trial measures whether changing the reference improves complete verified proofs. The offline converter builds one flat image. Each timed process loads one key and uses the same SDK executable and 32 pinned Ryzen workers.

Seven randomized blocks compare the ordinary key, the existing flat modal image and the new flat plurality image on the original request and four additional inputs. Inputs 121 and 124 had respectively large and small reductions in the diagnostic. Inputs 125 and 128 were absent from the training and the diagnostic. Each process performs three warmups, measures fifteen requests and exports one proof for independent verification with the original key. Report the paired median on each input, absolute medians, peak resident memory and both M5 ratios on the original. Report offline conversion separately.

[Process order](artifacts/schedule.json)
