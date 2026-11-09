# Continue bounded FFT work toward the all-shape goal

Goal ACTIVE/UNMET: verified single-proof CPU AVX >=10x supplied M5 CPU ALL8;
report CPU and Metal. Previous goal turn was no progress (historical timing
clarification). This turn is progress: real compact-root/leaf integration and
complete independent consumer/resource/concurrency qualification. No timing.

Private root /home/ubuntu/src/zopro-perf-work/fixed-factor-fft176.
Read design.md, review.json, source-changes.txt, fixed-roots.rs, leaf-fixed.rs,
oracle.rs and fft-tests.rs. The 178-entry source snapshot is frozen. Four old
parents differ, 172 original exact. Parent native field/child are byte-identical.
Source changes are guarded by zopro_fixed_fft AND zopro_fixed_factor_trunc AND
the accepted native IFMA target/feature guard. Production176 unchanged.

Actual last-five-layer products now use compact fixed factors in
fft/pair/leaf.rs dif_serial/dit_serial; within-quad shuffle formulas are copied
exactly. All higher stages and other consumers preserve original operations.
Reservation::analysis admits Box+Vec before allocation after original Plan
tables finish preparing. Max10672B/direction=752Bbox+31*320Bvec, total32016B;
factor payload31680B, Option pointers24B/plan. No persistent expanded factors,
whole-domain quotients, new pool or artificial cap. These are allocation
charges, not RSS/stack/minimum RAM. Existing old tables remain as before.

Plan build+oracle linking initial run terminal1 (missing AdditiveGroup import,
no tests ran). Keep oracle-initial.rs/repair.json/pins.json/run. Core never
rebuilt. run-fixed terminal101: padded30cases/4212outputs PASSED, other3 failed
because sparse twisted product oracle missed existing n^2 scale. Keep
oracle-before-scale.rs/repair-scale.json/pins-fixed.json/run-fixed. Correct ONLY
that expected scale (accepted old packed_product_matches_quad_reference
documents it), no candidate source/population change. run-scale-fixed terminal0:
link,3previouslyfailedtests (1passedtestfiltered),genericguard,budgetlink,real0
budget oracle. 7 successful+2failed scientific receipts total; no old model or
native population replay. No aborted prefix counts included.

3,947,126 completed scalar comparisons total: complete45cases3713536,
worker53visits217088, fallback/rebuild/small9218, retainedpadded4212,
realzero-budget3072. Fullinverse/ordinaryforward/twistedforward checked
independently with Arkworks sparse coefficients, root1/3 coset5/7 n8..262144,
worker1/4/16/32 512KiBstacks. Actual modernstored_transform route counters
positive; original old Plan::quotient used ONLY additionalrelease/rebuild
control. Euler samebody production derivative path checked. Realenvbudget0
deniesallcache withoutdiagnosticDENY; fixedproducts0/livecharge0, noOSpressure
claim. Generic no_ifma/no_asm compile only, notARM.

Offline review initial101 onlybecausepassedtestsarrayorder differsfromdeclared;
sortsets repair retainedreview-initial.rs/review-repair.json. Finalreview PASS
all216pins/9receipts/production176/hash logs/test sets/actualsource. No oracle
rerun forreader. Qualified normalreleasefield hash
e180b452ba80d07c9af2c9319593dd0af39f04c9ac587f6867fca57dfae09d93.
Current core includes zopro_fixed_fft_diagnostics. DO NOT time this core.

NEXT coherent normalrelease OFF/ON (same178source andfield, onlyfixedFFTflag
differs) with diagnostics OMITTED. Qualify actual normalrelease consumer
output against NEW independent sparse cases via public euler/current study
methods, inspect SAMEELF dif_serial/dit_serial emittedcode. Do not replay
already-qualified field/million model/FFT populations. Then prospectively freeze
matchedacceptedREF/OFF/ON completeFFT/Eulerconsumer timings and profiles at
relevant domains and1/16/32workers; keep setup/memory/expansion costs explicit.
Current product study onTWISTED intentionallyreturns n^2 timesactualproduct;
prepare_euler_study removesTWISTED. Smaller currentproducts mayuse oldroute;
counterqualification directstoredtransform/Euler ensures actualnewleafcoverage.
No isolatedkernel extrapolation. FullSDKall8+54/Gnark/resource/portable/concurrency
beforeadoption. Don'tclaim speed benefit from76vs100IFMA or cachepayload.

Acceptedlocal9950X32commonfixed1x2=7.857170ms (<8), historical7.915095valid;
unadopted7.753327notproduction. ALL8CPUms
7.857170/11.052363/15.952018/24.080322/34.012417/40.128912/66.348981/65.829291;
M5CPU37/59/109/189/284/335/528/507;Metal24/30/48/74/101/124/176/175;
CPU10xtargets3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7. Goal0/8, no speedgain.
AcceptedSDK6d83 andcanonical40154proofledger unchanged. NoAWS/Aeglos/mobile/
ARM/huge/pressure/swap/minRAM/throughputclaim. No livejob/cloudresources.
Actual sessions68586exit1/42421exit1/47330exit0, function sessionkeycleared.
Rust1.98.1/serializedxtask/noPython/questions/subagents. CommitANDpush completed
experiments and polite factualreports. DuringANYfuturepreflight/recording ONLY
poll exactconfirmedlivehandle untilterminal; allrecordingsbeforeofflinehelpers.
Timeout/truncation isNOTterminal. Do not reread huge latest-status.json wholesale.
