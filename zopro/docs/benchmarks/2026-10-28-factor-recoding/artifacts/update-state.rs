use serde_json::{json, Value};
use std::fs;
fn main() {
    let commit = std::env::args().nth(1).unwrap();
    assert_eq!(commit.len(),40);
    let root="/home/ubuntu/src/zopro-perf-work/factor-recoding";
    let path="/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let previous:Value=serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    let analysis:Value=serde_json::from_slice(&fs::read(format!("{root}/analysis.json")).unwrap()).unwrap();
    let status=json!({"status":"complete exact scalar-factor work screen; committed and pushed; no production adoption",
        "root":root,"report":"/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-factor-recoding/README.md",
        "commit":commit,"pushed":true,"remote_verified":true,"production_changed":false,"adopted":false,
        "accepted_source_files":172,"accepted_local_SDK_path":previous["accepted_local_SDK_path"],
        "accepted_local_SDK_sha256":previous["accepted_local_SDK_sha256"],
        "last_qualified_local_1x2_ms":8.324853,"accepted_primary_RSS_MiB":134.5,
        "goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,
        "last_goal_turn_classification":"PROGRESS","tests_passed":3,"census_rows":120,
        "SDK_requests":0,"new_independent_exports":0,"independent_proof_count":12580,
        "owned_live_jobs":[],"owned_cloud_resources":[],"component":analysis,
        "previous_completed_experiment":{"commit":previous["commit"],"report":previous["report"],"status":previous["status"]},
        "next_action":{"status":"unimplemented actual-scalar census and signed-centering source audit",
            "focus":"Factor2 saves no complete windows; synthetic width12 terms -0.743% full-width but6.267x small. Centered saves0.757% terms and preserves small scalars, butwidth15 insertion count worsens. No latency claim.",
            "candidate":"Audit existing magnitude/sign handling and count centered-vs-original digits on actual scalar feeds before choosing native integration; distinguish G1 full Fr from G2 Frobenius components. No duplicate point arrays or giant lookup tables.",
            "retired":"Do not revive original queue scanning/early-drain: older retained reports explicitly record negatives. Mixed triple SDK integration is also closed negative.",
            "requirements":"Keep all8 CPU/Metal goal active unmet0/8, preserve adopted172/SDK14da andledger12580; commit AND push each completed experiment; no questions."}});
    fs::write(path,serde_json::to_vec_pretty(&status).unwrap()).unwrap();
    let handoff="/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let older=fs::read_to_string(handoff).unwrap();
    let prefix=format!("CURRENT COMPLETE/PUSHED — FACTOR RECODING EXACT WORK SCREEN; NO NATIVE SPEEDUP\n\nHEAD/remote {commit}, branch sergei/zopro-cpu-perf. Root {root}; report /home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-factor-recoding/README.md. New primary-source selected chapter review (Waterloo Hrabar thesis chapter4), exact Rust model using copied native Booth,120 census rows/5 strategies/2syntheticsets/12widths. THREE independent exact tests PASS once:5116scalar identities, Booth allwidth4..15 integer/bit references,139input independent Ark G1 grouped MSM. All172production andSDK14da hashes exact; no production changes/newproofs/SDK latency/RSS/AWS. Ledger12580, acceptedprimary8.324853ms134.50MiB, acceptedall8inreport. GoalACTIVE UNMET0/8. No owned livejobs/cloud.\n\nAt width12,32768uniformfullwidth:original715288terms/672277idealinsertadds;centered709874/666864;factor2 709971/666961. Respectively -0.757%/-0.743% TERMS not runtime. Integers0..16383:original30716terms,centeredsame,factor2192507(6.267x). Allwidth4..15 ceil255/w==ceil254/w: no complete windowremoved byhalving. Factor3/2 and5/3/2add separate group/tables/weightedcorrections; idealinsertcounts alone misleading. Width15centeredfullwidthterms557040vs557045butidealinsert324313vs318837,so notuniversalworkimprovement. No nativegain/breakthrough/2xclaim;no sourceadoption. Preserve positivecounts honestly. No successfultest/modelrerun, no timedwork.\n\nIMPORTANTSUPERSEDESolderNEXTqueueidea: history audit foundexplicit preserved negative scanning/earlydrain reports in2026-10-22-universal-msm-diagnostics andretainedqueueaudit, plus2026-10-25-shared-point-window-queue negative. Exactoldsource notlocated;does NOTauthorizeunchangedreroll. Queueidea setasideWITHOUTimplementation. Serializedglobalwindowpairsearcholdnegativealreadyread; don'treroll.\n\nNEXT: actual-scalar centered-vs-original census plus source/historyaudit (no concrete nativeprotocol yet). Need preserve sign in top_window/reduce_indexed_buckets, borrowed/deferredY andfused paths; no duplicatepointarray orperkeytable. WholeFrcenteringcannotblindlyapplyto127bitG2 Frobeniuscomponents. Modelroot/binaries/resultsfrozen. Prior proofbasis remains mixed-sdk/prior-proof-sha256.json12527 +mixed-sdk/proof-inventory.json53=12580; no newexports. LatestAWS remains old171-source37261af77, notremeasured. Accepted172 FFT adoption engineeringexception/fullgateFALSE preserved. Userautonomousloop/noquestions/commitANDpush eachexperiment.\n\n--- PREVIOUS COMPLETED MIXED SDK STUDY BELOW ---\n\n");
    fs::write(handoff,format!("{prefix}{older}")).unwrap();
    println!("updated status/handoff for pushed {commit}; goal active, ledger12580");
}
