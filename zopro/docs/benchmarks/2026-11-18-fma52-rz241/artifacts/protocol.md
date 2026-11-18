# Exact 52-bit FMA/RZ product prerequisite

Previous240 PROGRESS: distinctlatephiKHquerynegative, closed; source188/d523
adopted233unchanged. FullALL8verifiedCPU10x/M5Metal goalACTIVE/unmet0/8.
Canonical62191/c2407f7fd7d43e32a685db92664fc94facac7c2608ba7843419be07629700cf3.
Actual241AWSSTSdevfailed255expiredSSO/refreshfailed(thirdconsecutivegoalturn);
239frozenpackage/source188/tool/payload/fourproofbytepinsrecheckedallPASS,
no AWSresources. Independentnewprerequisitework remains; no fullgoalblocked.

Assumption challenged: exactnativeFMA fallback necessarilyusesold51bitRNEformat.
Newfixedrecipe:52bitlimbs, AVX512embeddedroundtowardzero+SAE usingGPUstylebiased
high/lowproduct. Preserve5limbs52/52/52/52/48(mixedR256) or5x52(uniformR260).
OldOct23exact-fma-rne negative remainsclosed, no RNE/width/thresholdreroll.
Primarysource indexedexcerpt inspected:
https://www.ecs.umass.edu/arith-2018/pdf/arith25_17.pdf describesGPU52productbias;
fullPDFfetch timedout. HereownRustintrinsics, no unseenpapersectionclaim,
codecopied orspeeduptransferred. Integerproofandnativeoracle standindependently.

For0<=a,b<B=2^52, P=ab<B^2. H=RZ_fma(a,b,B^2)=B^2+floor(P/B)*B.
BecauseH in[B^2,2B^2), itsfractionbitsareq=floor(P/B). D=(B^2+B)-H
isexactbySterbenz, equals(1-q)B. L=RZ_fma(a,b,D)=B+(P modB), anexactinteger
in[B,2B). Itsfractionbitsarer. Exponentsfixed104+1023and52+1023.
In5x5schoolbookconvolution sumrawIEEEbitpatterns asintegeru64, subtract known
columncounts*high/lowexponentbias modulo2^64 oncepercolumn. Truecoefficients
<10B<2^56, so this recovers exactnativeIFMAlo/hiaccumulators. Carries thennormalize.
No prime-specificmodularreduction/fieldAPI/Montgomeryclaimfromrawproduct alone.

Predict: same2productoperations perlimbpair, plusFPsub/integeraccumulation and
10inputconversions perfullproduct whenstartingintegerstorage. Couldlosebadly;
onlyactualnativecomponentmeasurementselectsfieldfollowup, notISAcounts alone.
Kernelusesfixed5operand/10coefficientvectors, no heap/table/persistentkey/pool.
Packing/integerbiasedaccumulation maychangecompilerregistertraffic. No memory
orCPUcriticalpath/2xforecast. EarliernativeIFMAcalibration isalreadyretained.

Cheapesttest: bothnativeIFMAandnewFPfull5x5product produceall10normalized52bit
limbs. Independentu128schoolbookoracle coversboundary/randomvalues andraw
coefficients, bothradices, repeatedoutputs, all16round/DAZ/FTZmodes withrestoration.
ActualemittedFMA mustincludeRZ/SAE. NoCPUcojobsduringANYtiming; onlypolllivehandle.

FreezeonecoherentRust1.98.1/znver4consumer,3independentbyteidenticalbackings,
3matchedrotated/reversedblocks,2radices x2envelopes x2arms=24captureprocesses.
Threeexcludedprimesof10000loops,1000000timedproducts/process, pinCPU2/oneOSthread.
Bothrawandpreparedenvelopesmaterializeall10coefficients, consumeoutputlowlimb
into nextinput andblack_boxalloutputs toavoiddeadcode/hoisting. PreparedFP
excludesinitial10conversions butincludesupdatedlowlimbconversioneachiteration;
rawFPconvertsall5+5inputsinsidekernel. IFMAalreadyusesintegerarrays. Allcall,
outputmaterialization, normalizationandchecksums costsinside timedloop, setup
andfinaloracles outside. No freepreparedformat/fieldconsumerclaim.

Botharms4functionalpreflights(2radices/rawenvelope), semantic+separate reader
mustPASSbeforeguardedmain. 24mainprocesses first, thenofflineanalysis; no exclusions,
extensions, nativecompile/layout/rounding/width rescue. Gateforfieldprototype:
mixedR256rawpairedmedian<1 and>=2/3wins; allothercellmedians<=1.02. No minimum
gainfloor/noCIorprecisepenaltyfrom3blocks. Failingonlydeclines this fixedbackend
recipe, notallFMA/field algorithms orproofperformance. NoSDKfollowupadoptionfrom
componentgain; survivingfieldcandidate needsactualG1/G2caller/profile/fullSDK.
Recordfullprepandfirstdiscriminantclock;allow1200sinitialprep/120sactivecapture.
No proof/solver/Gnark/SDK/newledger/productionchange. No pushwithoutfreshapproval.
