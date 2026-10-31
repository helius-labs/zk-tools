# In-place inverse-hint aliases

Previous goal turn PROGRESS: semantic-cse176 completed, independently checked
and published as f727df6428. The accepted prover and timings remain unchanged;
the original all-eight verified CPU 10x objective is active and unmet.

Implement the exact raw-input inverse aliases established by that census in a
private source snapshot. Keep original image encoding and original input ranges.
An alias hint reads an earlier inverse output and multiplies by its certified
constant scale. Zero semantics hold. Require the original definite-assignment
check, so raw wire IDs denote stable values and invalid legacy images retain
the original path. Preserve every constraint and validation step.

Store source u32 and eight u32 factor words inline in HintKind. Check the native
Op size and alignment against the accepted layout: no expansion across millions
of operations is allowed. No retained sidecar, DAG, map, key/plan copy or table.

Discover with two temporary flat arrays: normalized coefficient vectors for
original inverse-hint terms and their sorted descriptors. Coalesce repeated
wires and zero coefficients; skip identically zero inputs. Compare full exact
vectors, choose earliest operation in each class. Temporary storage is checked,
fallibly allocated, charged to the current resource policy, and dropped before
schedule construction. No arbitrary 64 MiB or shape cutoff. Allocation/admission
failure preserves the original plan. Definite-assignment preparation uses its
existing admission and fallback. CPU/allocator overhead and added dependency
edges remain real costs.

Schedule reads include the original hint inputs plus the alias source. Update
the bounded scheduler's read-capacity forecast. Preserve original image kind
zero on serialization; aliases are transient optimization metadata. Equality
grouping keys distinguish source and factor instead of assuming all inverses
have identical behavior. Plan preparation must happen before schedule caching.

Qualify exact layout, coefficient cancellation/scaling/order, zero behavior,
serialization, forced no-budget fallback, real synthetic scheduled execution,
and dependencies in 1/4/16/32-worker pools with small stacks. Run the complete
native core/prover suites for changed solver code; reuse unchanged field code
with exact hashes. Portable solver qualification follows before adoption.

Then measure the actual full solver caller on retained 1x2 and2x2 fixtures,
including allocation/execution/drop. Use original accepted-library control and
same candidate executable OFF/ON, fixed paired blocks and no gain floor. Include
priming and result checks, resource counters and automatic CPU profiles. If the
caller supports a useful gain, measure full verified SDK proofs on all supplied
shapes, with current controls and Gnark verification before adoption. A census
or microbenchmark cannot establish a proof speedup or completion of the goal.

Do not reroll closed G1/G2 tree, layout or queue experiments. Publish results
factually, exclude private inputs and aliases, commit and push each completed
experiment. No subagents or Python runners.
