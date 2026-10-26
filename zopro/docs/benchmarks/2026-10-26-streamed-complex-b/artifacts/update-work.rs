use serde_json::{json, Value};
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn write(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    let work = json!({"experiment":"streamed-complex-b","root":root,"phase":"codegen pass; starting fixed496process native component latency, then4profiles","last_pushed_commit":"cb574b5ba7fde1fd6df702fe252c1143e5418934","initial_build_session":69245,"corrected_build_session":97715,"builds_terminal":true,"native_field_tests":33,"corrected_oracle_tests":4,"exact_outputs":91136,"ark_chain_lanes":729088,"timing_pins":214,"standalone_ifma":250,"standalone_frame_bytes":1144,"old_shared_frame_bytes":6008,"repeat_frame_bytes":22016,"old_shared_repeat_frame_bytes":21696,"codegen_not_causal_speed_claim":true,"accepted_unchanged":true,"adopted":false,"goal_achieved":false,"new_aws_resources":[]});
    write(root.join("work-state.json"), &work);
    let p = Path::new("/tmp/zopro-latest-optimization-status.json");
    let mut s = read(p);
    s["current_work"] = work.clone();
    s["status"]=json!("New streamed-B field component: correctness/codegen passed; fixed timing starting. Last diagnostic pushedcb574b5b; production unchanged.");
    s["owned_live_jobs"] =
        json!([{"phase":"next owned xtask timing session; inspect tool handle"}]);
    write(p, &s);
    let p = Path::new("/tmp/zopro-validation-checkpoint/state.json");
    let mut s = read(p);
    s["current_experiment"] = work.clone();
    s["current_component_work"] = work;
    s["validation_in_progress"] = json!(true);
    write(p, &s);
    let handoff = Path::new("/home/ubuntu/src/zopro-cpu-perf-handoff.md");
    let old = fs::read(handoff).unwrap();
    let prefix="# CURRENT — 2026-10-26: STREAMED-B COMPONENT CODEGEN/ORACLES PASS; FIXED TIMING STARTING; LAST PUSH cb574b5b\n\nGoal active/unmet no budget; previous diagnostic fully pushedcb574b5ba7fde1fd6df702fe252c1143e5418934(177files roottool+report). New private root/tmp/zopro-streamed-complex-b; protocol experiment.json frozen. Preserve old3product2CIOSsignedaccumulatorsand250IFMA; stream exactb0+b1oneB52limbperrowwithcarry insteadprecompute5sumBvectors. Keep5summedA, all reduction/product expressions unchanged. No heap/table/buffer/barrier/assembly/worker/coredispatch. Only2fieldmethodadditions;removebothblocksrecoverswholeoriginalbytes,168otherfilesmatchaccepted170. Production unchanged.\n\nBuildexec69245terminalPASS33unfilteredfield/gen and4initialoracles, BUT initialcopiedprobecontainedinlinepreviousCombacoefficientmodel (still actualnewmethodalgebra/BigIntpassed); newrowmodel fileunused. CaughtBEFOREtiming; harness-repair.json/probe-initial.rs/initialbinaries preserved. ReplaceONLYcfgtestsectionwithrelativeinclude newprobe-tests. Original185pinsverifiedwithoneexactoriginalprobeSHArelocation. No field/runtimeprefix/gatechange. V2exec97715terminalPASS4correctunfilterednewtests+rebuildbothprobes197pins; independentrowCIOScoefficientmodel/exactBsumcarry/directoldsharedrawdigits+BigInt91136exactoutputs+Ark729088chainlanes allpass,4/8laneFqFrR256R260;maxcoef41058117838107261<32B144115188075855872,postmixed<2^62.\n\nActualcodegen250IFMAnew andoldshared vsordinary300; standaloneoldshared6717B969instr6008literalstack210RSP ->new5570B870instr1144stack86RSP. Actualrepeatoldshared40092B6136instr21696stack1071RSP ->new40224B6171instr22016stack1030RSP. No dynamicspill/speedclaim. Bothnormalizedroutinesdistinct; declaredcodegenscreenPASS. Newtiming.rs/analyze.rs freeze496processes31rotated/reversedblocks4patterns×4variants acceptedordinary/freshordinary/shared/streamed,CPU2x1,100kiter+3×2kprimes,214pins;21/31aggregategainandallpattern<=3%,cal<=3%against3controls, nofloor/exclusion/reroll. AlllatencyTHEN4affine20miteration999Hzcyclesprofiles. Need inspect actualnexttoolhandle; no SDK/Gnark/AWS. Commit/push completedresult before anotherexperiment; componentpassalonecannotadopt, separatelyfrozenactualG2caller/fullSDKqualificationrequired. Acceptedmanifest63374de... SDK03e126... historicalprimary8.594267ms0/8CPU10x.\n\nOlderCURRENT/RUNNINGheadersbelowhistoricalsuperseded.\n\n";
    let mut data = prefix.as_bytes().to_vec();
    data.extend(old);
    fs::write(handoff, data).unwrap();
}
