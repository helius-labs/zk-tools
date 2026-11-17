# Explicit pre-capture stack-bound correction

Before ANY220SDKpreflight/timing: nativeOFFandON142coretests eachPASS, including
actual1/4/16/32worker512KiBstacks+4concurrentcallers. StaticONprologue aligns
RSPdown4096 andreserves0x3000, withordinaryRuststackprobes. Sixsavedregisters
consume48B. SysVentryRSPmod16=8, so alignmentadjustmentmaximum4088B. Thus total
entryRSP-to-bottom maximum=48+4088+12288=16424B (16432Bincludingreturnaddress).
The original nominal16KiB bound overlooked savedregisters: strictbound is
exceeded40B beforethereturnslot. This is a disclosedengineeringexception made
BEFOREperformanceobservations, not all-original-preconditionsPASS. It remains
about3.14%ofrequested512KiBteststacks; nonewheap orstackreservationdefault.
No runtime/source/alignmentparameter changes are made to obtain thisexception.

ONkeyspillstore at0x234df4 isVMOVAPS32B to[rsp+0xf40]. RSPispagealignedthrough
loop, so spilloffset3904+32=3936<4096, also64B-aligned. Thisprovesdeclaration's
no4KiBcrossingmechanism. OFFretainsold[rsp+0xf0]store and16Bstackalignment.
REF/OFFfunctionaddress0x233bb0,size0x1dc8 same,loopinstructions identical;two
nonloopRIPreadonly-stringreferencesdiffer (diffretained). ONsymbol0x233b90,
size0x2103; prologueandregisterallocationchange. Do not attributeeverylater
cyclechangeexclusivelytopagesplitlatency. Noblindpad/offset/alignmentvariant.

If source220showsgenuinebenefit, a laterqualifiedengineeringfollowupmayreplace
thisdiagnosticpage-sizedmarkerwithminimal32Balignment,providedemittedcode
stillguarantees32Bkeyspillalignment. That is not implemented/tested/selected
here, and wouldneeditsownsource/tests/controls. Current220fixedreciperetained.
