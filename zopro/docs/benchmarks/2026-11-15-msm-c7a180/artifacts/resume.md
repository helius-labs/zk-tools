# Completed C7a diagnostic — historical launch instructions

The target capture and all220 host proof verifications are complete. All owned
cloud resources are cleaned. Do not execute the old launch sequence again.
Current canonical55664 is `/home/ubuntu/src/zopro-perf-work/msm-c7a180/transport/proof-sha256.json`,
SHA256`5aa3b76fe67d3cc541636a7c94fa8eef7459b91a1cc7840d5330c901ddef0a4a`.
See the current README and target-review.json for62/64 calibration/backing checks
and10/10 environment checks. The instructions below preserve the original plan.

Private root `/home/ubuntu/src/zopro-perf-work/msm-c7a180`.
No successful build, smoke proof or host verification needs repeating.
Current canonical55444 is `smoke-transport/proof-sha256.json`, SHA256
`dbcd55c1e554260a2f26e8e5f3b4d25e1eb22b31578a6be3520cd733be9b8ec3`.
Never overwrite a later canonical branch; pass the actual current ledger below.

1. Run `./seal check` in the private root. This checks frozen inputs and the
   exact local final image, without AWS or SDK execution. Check current AWS dev
   authentication. The last observation was expired SSO; no resources exist.
2. With valid authentication, `./cloud publish`, then `./cloud launch`.
   The helper requires role lifetime exceeding110minutes, verifies no existing
   temporary benchmark and no ingress, and launches at most one owned c7a.24xlarge
   with90minute terminationTTL. Immediately persist its returned instance ID.
3. `./cloud boot`, then poll only `./cloud poll boot` after submission until
   Success. While SSM is not registered, repeat the boot observation; the helper
   doesn't submit a duplicate command. Then `./cloud pull`, poll its command to
   Success, and `./cloud measure` exactly once.
4. During capture only poll `./cloud poll measure`. Do not build, hash, analyze,
   write reports or submit helper jobs. A timeout is not terminal. Preserve the
   confirmed SSM command and instance handles. The helper is idempotent for an
   already submitted command; never restart completed SDK work.
5. After authoritative terminal capture, `./cloud package`, poll package to
   Success, then `./transfer create`, `./transfer test`, `./transfer upload`,
   `./transfer poll` until Success and `./transfer download`. These operations
   scope the private temporary bucket to this run. Preserve secret presigned-URL
   files privately; never include them in reports. Download checks remote size
   and SHA256. Preserve partial outcomes if capture failed and resume only
   missing work; do not run the complete matrix again.
6. After retrieval, `./cloud cleanup` and `./cloud cleanup-status` until the
   owned instance is terminated, volume absent and temporary ECR tag removed.
   Run `./transfer cleanup` after retrieval to remove the owned object/bucket and
   secret URL file. Record actual observations, not just cleanup requests.
7. Extract the verified archive into a new `remote-results` directory. For a
   complete capture, prepare host verification:

   ```sh
   ./host prepare \
     /home/ubuntu/src/zopro-perf-work/msm-c7a180/remote-results/output/msm-c7a180 \
     CURRENT_CANONICAL_PATH CURRENT_CANONICAL_SHA256 \
     /home/ubuntu/src/zopro-perf-work/msm-c7a180/transport
   /home/ubuntu/src/zopro-perf-work/tool-target/release/xtask workflow \
     /home/ubuntu/src/zopro-perf-work/msm-c7a180/transport/plan.json \
     /home/ubuntu/src/zopro-perf-work/msm-c7a180/transport/run
   ```

   Use real current canonical values in place of the two placeholders. Host
   verification rechecks all220 transferred exports, source/stream receipts,
   raw sample medians and target-ledger ancestry, then writes a disjoint union
   at `transport/proof-sha256.json`. This is verification, not an SDK replay.
8. Inspect every target calibration, per-backing and environmental result in
   `remote-results/output/msm-c7a180/review.json`. A valid proof ledger does not
   imply representative profiling or a speedup. Compare all block phase changes
   and retain overlapping-span/residual limitations. Write factual results,
   commit locally, update the global canonical/handoff, and continue the full
   CPU10x goal. Do not push to GitHub without approval.

Owned lifecycle sources are frozen; the target has not been run. If operational
repair is necessary, preserve failed receipts and original scientific data and
change only the missing operational step. No new algorithm is selected by
preparation alone.
