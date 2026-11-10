# Bounded queue trees in one-plane ring phases

The full goal remains verified single-proof CPU AVX latency at least 10x the
supplied M5 CPU references for all eight shapes, reporting CPU and Metal.
Current accepted SDK is unchanged; primary32 is 7.783593581 ms and 0/8 meet
the goal. Canonical proof inventory remains 44,265. This is a new structural
experiment, not a timing retry of the closed affine-completion implementation.

The old one-plane phases spill to projective arithmetic. Mandatory affine
completion instead serializes colliding destinations, reaching 199 completion
rounds. Replace overflow and phase-end queue completion with balanced trees
of the queued points for each destination, followed by one batch attaching
each nonempty root to its existing bucket. Pending ordinary work executes
before the tree. Normal full-batch execution and its LIFO take are unchanged.
Every queued operand is consumed once; a destination with k operands costs
k-1 tree additions plus one attachment, not k serial attachments. With at
most 199 queued operands, the tree has at most eight levels and one attachment
stage. The existing ordinary batch can contain at most 200 operands.

Reuse exact prior ring recoding, unit2/phi/unit1/phi/unit0 phases, 2 partitions,
radix128, 19 positions, 2,731 ordinary slots, old indexed leading window,
representative axes and weighted trees. No width, queue-size, grouping,
worker, ordering, field or compiler sweep. Earlier guarded drain, one-by-one
completion and full-window indexed ring results remain closed. This proposal
changes the addition topology of bounded queued inputs; it is not their rerun.

Before model execution freeze sources/tools/dependencies/inputs and these
necessary checks on both retained primary and raw2x2 populations:

- all new algebra/conservation/termination tests pass;
- no generic projective spills or normalizations;
- every queue tree has <=8 levels and <=199 input operands;
- vector proxy strictly below accepted; inverse-root calls <= accepted;
- covered Fq products, fused2 events, scalar squares and inverses each no
  greater than the mandatory-affine phase recipe, with at least one strict
  improvement on each population.

There is no percentage gain floor. Counts are not timings. Failure closes this
fixed recipe without a favorable-size or threshold rescue. Passing permits
only resource/native correctness investigation, not adoption or a benchmark
claim. Read all old baseline observations rather than rerunning old tests or
models. Independent replay must use separate direct-MAP decoding and a
different queue/tree representation, and reconstruct the full work ledger.

The count model assumes generic noncancelling curve points. Formal additive
token tests include cancellations, but do not qualify native curve exception
dispatch. Raw2x2 is diagnostic: production may group it. No full-query speed,
SDK proof, native RAM/stack, concurrency or device behavior is established.

Native resource work must charge bounded grouping/routing, copied points,
pair records, tree levels and attachment buffers. Up to 199 G2 affine points
alone occupy 25,472 bytes per active queue tree; this cannot be called free or
included implicitly in the old forecast. No expanded key, retained point
table, solver plan, production pool or artificial architectural RAM cap.

Publish the result and inherited all-eight CPU/Metal table; commit and push
the completed experiment and verify exact remote HEAD. During any recording
or preflight only poll its exact live handle. This model creates none.
