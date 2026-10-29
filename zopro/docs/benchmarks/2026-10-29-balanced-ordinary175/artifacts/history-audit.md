# Source and history audit

Accepted source175/a1585b86 is authoritative. The real fused caller in
lib.rs invokes fused_kh::fused(...,13), and its guard limits k+h<65536.
Thus simply extending the fused implementation to12/14 would not change
these proofs. That initially proposed scope is rejected before any build.

The accepted manifest explicitly sets ZOPRO_CPU_DEFER_SIGN_WIDTH=12 and
ZOPRO_CPU_DEFER_SIGN_PARTS=1. msm_raw applies these settings to native
small deferred-sign queries; the Fq borrowed_msm trait selects the borrowed
ordinary implementation only for supported native targets, packed/direct
mode,1024..65535 points and1..2parts. The earlier center-native route recorded
primary A at21006/width12/parts1. Since then the A solver/scalar path and
images remain unchanged; new CFG dispatch is inspected and full real proof
preflights/profile namespace remain the practical evidence for this study.
No unobserved larger-case route count is asserted.

This changes ordinary variable-width arithmetic topology while keeping
partition count, fields, original key/scalar views, weighted merge and queue
kernels. It differs from old G2 coupled width/partition (2026-10-26), packed
radix52 bucket storage (2026-10-22), signed centering (2026-10-28), fused top
combination (2026-10-27), and G1 Horner/shifted-table trials. Their negative
results are not rerun or relabeled. Accepted fused13 stays byte-exact.

The bound35840 versus43008 uniform ordinary bucket slots is theoretical.
It neither bounds wall time nor establishes a whole-process RAM reduction.
The old memory envelope reserves22full2048 parents plus indexed-top scratch;
new windows fit that envelope. Compiler stack frames remain additional to
352bytes of span metadata. No permanent key storage or device RAM cap.

Source references: current crates/zopro-core/src/lib.rs:2204;
msm_affine/fused_kh.rs:eligible; msm_affine.rs:msm_raw;
msm.rs:BatchInverse for Fq::borrowed_msm;
msm_affine/borrowed_pending.rs:msm;
msm_affine/workspace.rs:shape/envelope.
Evidence: accepted175 production manifest and its closing input pins;
2026-10-28-center-native/README.md;2026-10-22-universal-native-bucket52/README.md;
2026-10-26-g2-balanced-buckets/README.md;2026-10-27-fused-top/README.md.
No fresh paper or external code is imported for this extension.
