# Explicit workspace bound

Let n be input length and u=floor(n*percent/100). All sizes use existing checked
grouping_cost helpers, with the existing per-allocation64-byte margin. Let
E=vector<u64>(n,false), C=vector<u32>(n,false), V=vector<[u64;4]>(u,true),
K=vector<usize>(u,true), S=vector<usize>(u+1,false), P=vector<usize>(u,false),
O=vector<u32>(n,false). The still-live original sampling set has the unchanged
hash_table<&[u64;4]>(min(n,2048)) envelope H.

Four phases are mutually exclusive: sort E+E; roots E+C after dropping scatter;
reconstruction C+V+K after dropping entries; lists V+K+C+S+P+O. Therefore
H+max(2E,E+C,C+V+K,V+K+C+S+P+O)+256*sizeof(usize) is a conservative explicit
payload forecast. The last term conservatively includes the fixed histogram.
This is not a compiler-frame, allocator-retention, process-RSS or minimum-RAM
bound. The actual classes buffer is the same n*u32 shape as the original.

Check the bound against the untouched existing grouping_cost::peak(n,percent)
after original admission and before allocating either sort buffer. On overflow
or a failed comparison, retain the untouched map path with the unconsumed
reservation. No process/global/device budget is changed. Returned buffers and
their consumers retain the original reservation until all ownership drops.
Value/count growing-vector bounds are unchanged; starts/positions/order match
the original layout. Hash collisions can add comparison work but no buffers:
sort_unstable_by is in-place, and every original index is preserved.

The test checks all n=256..65536 and representative larger endpoints under
1/4/16/32 Rayon contexts at both existing cutoffs, plus invalid sizes/overflow.
The bound is based on requested capacities and inherited growth assumptions;
actual proof RSS, device pressure and concurrent consumers need later measured
qualification if performance passes. No new large key-derived lookup table.
