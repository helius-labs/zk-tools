The initial workflow plan was rejected before compilation because the helper
inserted a null argv field into link-sdk. Plan-v2 deletes only that field.
The first native compile then found a missed test-only Plan initializer;
plan-v3 adds its empty private cache and completes the new native build.
During source review before any SDK call, the readiness parser's index mask
was found to retain the existing Poseidon TASK bit. It would reject tagged
leaves and fall back. Plan-v4 corrects the mask to the existing 27-bit encoding
and adds actual tagged Poseidon solver-vector comparisons for all12 widths.
These are qualification corrections, not responses to performance results.
No SDK calls or exported proofs occurred in these attempts. Original plans,
receipts and affected pre-repair sources remain retained.
