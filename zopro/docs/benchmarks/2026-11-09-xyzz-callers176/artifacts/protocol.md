# Fixed complete MSM caller comparison

Declared before any caller preflight or performance capture. The implementation
is the exact qualified sparse XYZZ runtime from xyzz-window176. Four immutable,
synthetic valid-point/full-width-scalar populations: balanced G1 K/H with
30529+32768 points and20 mixed12/13-bit windows; uniform G1 K/H with the same
points at width12; ordinary G1 with21006 points, width12, one part; G2 Phi
with15044 points at width12/two parts. Dimensions come from existing qualification
fixtures and the saved ordinary G2 census, not new private proof inputs. An
untimed Arkworks helper generates all points/scalars and full MSM results.
Each benchmark process regenerates native inputs and verifies their complete
canonical point/scalar SHA against that oracle before primes. No posthoc fixtures.

Each complete MSM uses1/16/32 Rayon workers,512KiB stacks and fixed worker affinity
(CPU2 solo,0–15 or0–31). One operation per loop, using that pool internally;
this is single-caller latency, not N independent MSMs per worker. Three arms:
REF uses unchanged accepted176source plus identical benchmark-only shims;
OFF uses the qualified generic sparse abstraction with Jacobian/feature off;
ON enables its admitted XYZZ path. Three coherent originalfield1200/native/O3/
fatLTO/oneCGU final consumers; no field rebuild. Driver, shims and allocator
wrappers identical. The accepted SDK itself is not replaced or remeasured here.

The unchanged required proof forecast is held through the operation. ON's exact
qualified owner additionally admits its checked sparse-vector charge. No new
runtime flag suppresses admission cost. Each process validates three primes.
Time15 complete operations: construction/admission, ordinary affine batches,
sparse doubling/exceptions, queues, Phi, weighted tree, final affine result,
intermediate output destruction. G2's fresh mutable scalar copy is included
because Phi consumes its descriptors. Fixture/pool/oracle creation, primes,
final equality/hash and final result destruction are excluded. Shared allocator
wrappers perform only a false audit-flag check during timing. This is a complete
MSM screen, not proof latency or minimum device memory.

Before latency,36 allocation-audited one-operation preflights (12contexts×3arms),
then explicit denied ON G1-balanced and G2-Phi preflights at32workers. These
clocks are NOT latency data. Cross-arm outputs must equal independent Arkworks.
Requested allocation/reallocation counts and cumulative bytes are not live peak.
FullprocessRSS includes fixtures, native point construction, pool and primes.

Fixed15 paired blocks. Contexts rotate/reverse by block; arm order rotates and
reverses.540 latency processes,15operations each. Ratios use ordered ranks4..12
of15blocks (nominal96.484375% marginal median coverage under independent,
exchangeable blocks; no simultaneous guarantee). ALL12 OFF/REF intervals must
fit[1/1.03,1.03]. ALL24 ON/control nonregression upper bounds must be<=1.03.
ALL24 processRSS comparisons must be<=1.10×control+2MiB. This gate is a caller
regression screen, not an architectural RAM cap. Equal-log four-case32worker
ON/control aggregate upper must be<1 and>=10wins/15 against BOTH controls.
No positive gain floor; retain all small positives and all adverse results.
No subset promotion, extension, reroll or threshold/worker/curve/layout rescue.

After ALLlatency,16 four-event PMU panels (4cases×16/32workers×OFF/ON),256 MSMs
each, and16 separate fixed-period cycles profiles,1024 MSMs each. Warm FIFO control
records only loops. PMU cycles:u/instructions:u/cache-misses:u/branch-misses:u
must each run>=99.9%. Profiles cycles:u/fixed16Mperiod/512pages/sampleCPU+time+
period:>=1000samples,unknown<=5%,zeroLOST,owned binary/PID/TIDs,declared workerCPU,
MMAP/task/header/event bindings. Preserve weak results without extension. Flat
symbol shares do not identify wall-critical fractions or prove a unique cause.
All recording finishes before80 offline ownership/decode/metadata/event/loss
commands. No independent owned build/helper/hash/report/push overlaps any capture.

Total prospective workflow691commands:38preflights+preflightcheck+540latencies+
32diagnostics+80offline. Actual caller processes610. Frozen plans, source/library/
binary pins, analysis and profile readers precede preflight. Inspect every result
and retain failures. Only passing this complete caller comparison justifies a
separate ALL8+54 SDK/freshGnark study. Full CPU10× goal stays active until every
supplied shape is verified; current inherited accepted targets met0/8.
