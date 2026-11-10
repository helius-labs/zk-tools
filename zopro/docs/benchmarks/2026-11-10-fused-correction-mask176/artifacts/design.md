# Reuse fused correction classification

Fixed candidate: eight-lane reduce_fused only. Preserve needs=top>=top(2p)
and the original needs==0 return. In the correction branch, any equal top limb
takes the exact original reduce(t,2p). Otherwise needs is the exact per-lane
subtraction mask. Subtract masked 2p in one signed carry pass and return the
normalized digits directly, avoiding the original final five blends.

For normalized U in[0,4p), unequal high digits determine U>=2p exactly.
For selected lanes U-2p is nonnegative and below2p; unselected lanes retain U.
Each signed digit intermediate lies between -(B-1)-1 and B-1, B=2^52,
well inside i64. Carry is0or-1, final carry0. Equal-high fallback is exact even
for adversarial/modulus-boundary inputs. No probabilistic correctness argument.
MIXED/radix, CIOS products, borrowed-point formulas, scalar paths, records,
scratch, points, queues, widths, pools and full proof behavior are unchanged.

This differs from the closed general high-borrow subtraction: no guard is added
to its common path, and a previously computed correction classification is reused
only inside an existing branch. The new equality test/branch and masked constants
can still cost more; no gain is assumed. Four-lane code retains its old path.
No table/keyplan/input-sized buffer/new pool/architectural RAM cap/C/ASM/Python.

Qualification: exact raw output digits against original reduction and an
independent four-word conditional subtraction for both primes/radices, all256
top-tie masks, all directions, modulus/bounds/random vectors. All native field
and core tests with the actual candidate, generic field suite with selector
excluded. Then exact linked actual G1/G2 queries against independent full-MSM
oracles, 1/16/32 workers and512KiB stacks before any SDK adoption.

Freeze a complete-query protocol, all source/compiler/deps/binary/input pins,
analysis and separate reader before any preflight/recording. Preserve every
failure and positive result. During preflight/recording ONLYpoll exact livehandle
to terminal; no helper/build/hash/report/push overlap. No changed thresholds,
favorable shapes or successful reruns to rescue this fixed recipe. No gain floor.
Success still requires ALL8+54 SDK/Gnark/resource/portable/concurrency checks.
Goal remains all8 verified CPU10x and CPU/Metal reporting, currently0/8.
