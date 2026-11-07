# Final traversal, before any model execution

Supersedes the output-slot traversal in recipe.md, which remains preserved as
unimplemented design history. Reviewing accepted reduce_layer exposed a better
alias-safe traversal: write each completed pair to its original left input slot
and compact only after all pair outputs exist. No algebra, work count, timing,
native qualification or selected gate has run before this amendment.

Use pair slots only for the eight prefix chains. First pass stores compact
prefixes and multiplies denominator norms. Butterfly gives D and chain
cofactors. Compute D²/D³, then reverse pair blocks: reload original coordinates,
recompute the norm, form the cofactor, compute the new coordinates and write to
that pair's original left slot. Pair inputs are disjoint. This fuses cofactor
completion with coordinate output; no final-cofactor or carry-copy buffer.

After every pair is complete, scale odd carried points at their original slots,
in separate padded eight-carry blocks. Pair writes never touch those slots.
Then compact pair results/carries forward within all group segments. Every
write precedes the next unread original result. Group/count metadata supplies
forward/reverse pair positions without an O(n) index array. A zero denominator
at a late level still declines the private whole-tree result; input callers
retain their original frontier and use complete fallback.

Both real and generic-complex constructions use this traversal. Prefix storage
is one compact canonical eight-value record per pair block, allocated for the
first level and retained. Padding of pair and carry blocks is charged. Final
normalization of all surviving groups is charged; no singleton-level exception
or worker/size policy is added. All other conditions in recipe.md/design.md
remain unchanged. Keep elementary/fused/vector/scalar units separate.
