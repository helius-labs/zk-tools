# Table-free Boolean G2 reduction207

Frozen2026-11-17 17:04:15UTC before new model/native observations. Previousgoal
turnPROGRESS:204-206complete, notadopted; production179/194unchanged. Canonical58915.

1. Challenge: when existing exact grouping declines, solver-known HintBits values
   still cause a concentrated stream of 0/1 contributions. Rather than send all
   true bits to one ordinary bucket, reduce selected Boolean bases with the
   existing indexed affine tree and add its unweighted result to ordinary G2.
   Historical Boolean-LUTpartition used20.5GBtables/projectedvectors and lost;
   do not repeat that recipe. Existing generic tree/grouping negatives remain.
2. Prediction: fewer concentrated low-window arrivals/projective spills; extra
   bit guards/index collection/tree+onepointadd. No arithmetic/radix/width change.
   Keep original grouping decision: grouped calls take their original path and
   do not peel. Only ungrouped calls may peel; no second hash/grouping pass.
3. Cheapest discriminator: extract HintBits positions from the current pinned
   solver, bind query point order to the retained real fixtures, count fresh0/1
   values, then test the complete G2query with original real Arkworks oracles.
   Native tests cover identities/opposites/repeats, zero/one, invalid Boolean
   guards/indices and grouping bypass before a short caller timing screen.
4. Memory: a static u32positionlist (~7KBprimary expected), bounded selected
   indices and existing one-bucket tree temporaries. Drop these before ordinary
   MSM, retain just one projective sum. Reuse mutable proof-local scalar words;
   zero chosen words only AFTER every guard succeeds. Never copy key points,
   build a point lookup table/newpool or cache witness values/proof outputs.
   Prototype resources are not yet full SDK admission/portability qualification.
5. Decision: correctness failure stops timing. Complete-query screen:1x2/1/16/32
   and2x2/32, both modes in one binary,3byteidenticalindependentbackings,3fixed
   rotated/reversed blocks;3primes+17timed+onepostcheck/process.8one-round
   preflights before24mainprocesses. AllqueriesmatchretainedArkworkshashes,
   basesimmutable; scalarcopy/grouping/guards/tree/zeros/ordinaryMSM/finalnormalize
   alltimed. Hashchecking outsidewalltimer. Pin1workerCPU2,otherwiseCPUs0..N-1,
   512KiBdiagnosticstacks. NoownedCPUcojobsduringcapture;onlypolllivehandle.

Select further full-SDK work ifprimary32pairedmedian<1 with>=2/3wins and allfour
contextmedians<=1.02. No minimumgainfloor. Grouped2x2 must assert original bypass;
primaryON must assert peel route. ResourceRSSdiagnosticretained,notminRAM.
No sampleextensions, favorable-subset promotion or unchangedtimingretry.
Preparation/first-resultallowance15min from17:04:15UTC;recordoverrunandsimplify.
Positive selects missing integration/resource/correctness/full-proof qualification,
not adoption. Negative closes fixed recipe without a table/width/threshold rescue.
No SDK/Gnark/newAWSplanned by this component screen; no publicpush.
