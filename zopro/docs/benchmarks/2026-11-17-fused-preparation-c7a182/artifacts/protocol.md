# Exact fused preparation C7a comparison

One owned c7a.24xlarge,96physicalcores/noSMT,32/96workers fixedplacement.
Compare primary1x2 complete fresh verified requests: previous accepted Zopro REF
b646c7b8..., exact locallyqualified c285cbc8... candidate OFF/ON, retained Aeglos
Sorted REF eab9fbfd... . Same key/request/publicbindings and accepted policies.
Do not rebuild SDKs. Accepted Zopro means previous production baseline in this
comparison; fused native opt-in was locally integrated after33/33releasechecks.
No timing evidence for default-on, othercircuits or physicalmobile from thisrun.

Three byte-identical independent executablebackings perbinary;9distinctfiles.
Nine rotated/reversed blocks,3blocks/backing,alternate32/96andengine/armorder.
Eachmainprocess51timed+3prime,72mainprocesses. Eightfunctionalpreflights across
allfourarms/32/96,1timed+3prime.80exports,3920SDKcalls. Alltimingbeforelatechecks;
preflightsverifiedbeforemain. No per-tasktrace orphaseobserver. Setupoutside
requesttiming but included inwholeprocessRSS. Aeglos warmed keycache preserved.
No new tables,plans,workspace capor pool. Container30GiBceiling is infrastructure
headroom,not algorithmworkspace policy; swapoff,environmentguardsrequired.

Localcontainercompatibilitysmoke: ZoproOFF/ON,AeglosREF at4workers,3primes+1timed,
3exports/12calls. Currentseed56502SHA2721a5c49351ae5dea2fdf91c3889d99a3f48619ee22fc520b5e36adfdeb3b23.
Smoke56505; targetisolatedbranch56582; hostverifiedunion56585. Do notoverwrite
smokehistory. Eachproof independentlyGnarkverifiedontargetandhost,freshnessledger
andinputbindingchecks. Allrequestsselfverify. Preservepartialoutputs,resumeonly
missingwork;neverreplay successfulSDK callsforbookkeeping.

Primaryquestion: doesfusionimproveC7a32/96withoutCPU/RSSregression, especially
removingpartofthehighcorepreparationpenalty? Report9pairedmedianratiosplus
sortedzero-basedranks1..7,96.09375%marginalmedianboundsassumingindependent
continuousblocks;nojointguarantee. CompareON/REF,ON/OFFandSortedREF/ZoproON at
bothcounts,allbackingmedians,and96/32scaling. Improvementevidence requiresupper<1;
nonregressionagainstbothZoprocontrolsupper<=1.02latency/CPU,maxRSS<=1.05control+1MiB.
No minimumgainfloor; donotextendrun/dropcopies/relaxlimits. OFF/REFdescriptive,
notadoptionveto. Localadoptionstandsunlessnewcorrectness/resourceproblem;target
failurelimitsC7aclaimsandguidesnextwork. Aegloscomparisonisdescriptive,noreleaseveto.

Captureonlyaftermatchingimageverifiedandpinned,adequatecredentiallifetime,
90minterminateTTL,originalrestrictednetwork/SSMrole. Oneownedinstance only;
retrieveresults,terminateinstance,confirmvolumeabsent,removetemporaryECRtag
andtransferbucket. NoGitHubpush. Fullall8CPU10xgoalstillunmet.
