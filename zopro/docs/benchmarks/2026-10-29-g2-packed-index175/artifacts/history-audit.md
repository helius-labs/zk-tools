# Prior boundary

Coalesced GLS4 top ccac9ce1ca reduced targeted sampled shares but added total
work and lost every block. Rejected GLS4 decomposition/maps/ordinary buckets
are not used here. The accepted two-component algorithms stay unchanged.
26Oct ordinary G2 level/prefix/index-gather trees are closed; they preserved
indexed top behavior.24Oct SIMD bucket phi and transformed-X caches are closed.
22Oct GLS4 narrow tops used the original per-bucket indexed tile reducer.

Current reduce_indexed_buckets makes a separate task for each bucket's <=1024
points. New tasks span adjacent buckets at fixed global point-tile boundaries,
reducing multiple segments per leaf. Final original-bucket sums use the original
kernel. A README/design search found no prior packed-across-bucket indexed leaf;
the current callsite/source audit must still close before capture. This does
not retry a group-width/field/flag layout. No speed prediction from profiles.
