# History and scope

- Prior G1 endpoint peel and conditional-loop trials are both rejected: 2026-10-25-batch-endpoints-sdk and endpoint-branches-sdk. They change G1's stored numerators. This candidate changes G2's later chain/norm-only prefix implementation, which was not modified by those trials.
- Previous G2 norm-heap/phased-inverse/packed-X changes alter chains, passes, scratch or packing; they remain closed. This proposal preserves all eight chains and both record slots and only removes unit/dead endpoint products.
- Bounded-delta caller aggregate passed and its SDK lost. Real production replay shows borrowed200 accounts for95.469% of primary prefix contributions, while that context's earlier caller interval spans parity. This new gate requires the dominant primary context independently; old gates and measurements remain unchanged.
- Previous real-replay library includes a recorder Mutex at G2 entries even when disabled. Timing must remove those hooks and omit recorder implementation; an unchanged capture library is not a fair baseline. No prior capture, Arkworks oracle or Gnark verification is repeated.
- No new external arithmetic algorithm is copied. Existing field operations remain byte-exact. The candidate and recurrence proof are local changes.
