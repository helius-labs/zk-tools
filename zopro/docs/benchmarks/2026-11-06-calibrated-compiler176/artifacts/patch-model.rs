use std::{fs, path::Path};
use serde_json::json;
use sha2::{Digest,Sha256};
const R:&str="/home/ubuntu/src/zopro-perf-work/calibrated-compiler176";
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn replace_one(s:&str,from:&str,to:&str)->String{assert_eq!(s.matches(from).count(),1,"replacement count: {from}");s.replacen(from,to,1)}
fn main(){
 let dir=format!("{R}/source/llvm/lib/Target/X86");
 let cpu=fs::read_to_string(format!("{dir}/X86.td")).unwrap();
 let original=fs::read_to_string(format!("{dir}/X86ScheduleZnver4.td")).unwrap();
 assert_eq!(cpu,fs::read_to_string(format!("{R}/research/X86.td")).unwrap());
 assert_eq!(original,fs::read_to_string(format!("{R}/research/X86ScheduleZnver4.td")).unwrap());
 let mut candidate=original.replace("Znver4","ZoproZnver5").replace("Zn4","ZoproZn5");
 let old="\"VPMADD52(H|L)UQ(Z|Z128|Z256)(r|rk|rkz)\"";
 let new="\"VPMADD52(H|L)UQ(Z128|Z256)(r|rk|rkz)\",\n        \"VPMADD52(H|L)UQZ(rk|rkz)\"";
 candidate=replace_one(&candidate,old,new);
 let old_sub="(instrs VPSUBBZrr, VPSUBWZrr, VPSUBDZrr, VPSUBQZrr,";
 let new_sub="(instrs VPSUBBZrr, VPSUBWZrr, VPSUBDZrr,";
 candidate=replace_one(&candidate,old_sub,new_sub);
 let overrides="\n// Private partial Zen5 calibration. Only the four unmasked register opcodes\n// below change. Memory/masks/shifts and all other entries remain inherited.\n// Throughput calibration does not independently identify physical ports.\ndef ZoproZn5CalibratedIFMA52Reg : SchedWriteRes<[ZoproZn5FPFMisc01]> {\n  let Latency = 4;\n  let ReleaseAtCycles = [1];\n  let NumMicroOps = 1;\n}\ndef : InstRW<[ZoproZn5CalibratedIFMA52Reg],\n             (instrs VPMADD52LUQZr, VPMADD52HUQZr)>;\ndef ZoproZn5CalibratedQwordALUReg : SchedWriteRes<[ZoproZn5FPVAdd0123]> {\n  let Latency = 2;\n  let ReleaseAtCycles = [1];\n  let NumMicroOps = 1;\n}\ndef : InstRW<[ZoproZn5CalibratedQwordALUReg], (instrs VPADDQZrr)>;\ndef ZoproZn5CalibratedSubQwordReg : SchedWriteVariant<[\n  SchedVar<MCSchedPredicate<ZeroIdiomPredicate>, [ZoproZn5WriteZeroLatency]>,\n  SchedVar<NoSchedPred, [ZoproZn5CalibratedQwordALUReg]>\n]>;\ndef : InstRW<[ZoproZn5CalibratedSubQwordReg], (instrs VPSUBQZrr)>;\n";
 // Definitions must remain inside the inherited SchedModel scope.
 let close=candidate.rfind("} // SchedModel").or_else(||candidate.rfind("} // let SchedModel")).unwrap_or_else(||panic!("inspect scope closing before patching"));
 candidate.insert_str(close,overrides);
 let target=format!("{dir}/X86ScheduleZoproZnver5.td");assert!(!Path::new(&target).exists());
 let new_cpu=replace_one(&replace_one(&cpu,"include \"X86ScheduleZnver4.td\"","include \"X86ScheduleZnver4.td\"\ninclude \"X86ScheduleZoproZnver5.td\""),"ProcModel<\"znver5\", Znver4Model,","ProcModel<\"znver5\", ZoproZnver5Model,");
 // Independently reverse every edit, including the namespace transformation.
 let inverse=replace_one(&candidate,overrides,"");let inverse=replace_one(&replace_one(&inverse,new_sub,old_sub),new,old).replace("ZoproZnver5","Znver4").replace("ZoproZn5","Zn4");assert_eq!(inverse,original);
 let inverse_cpu=replace_one(&replace_one(&new_cpu,"include \"X86ScheduleZnver4.td\"\ninclude \"X86ScheduleZoproZnver5.td\"","include \"X86ScheduleZnver4.td\""),"ProcModel<\"znver5\", ZoproZnver5Model,","ProcModel<\"znver5\", Znver4Model,");assert_eq!(inverse_cpu,cpu);
 fs::write(&target,&candidate).unwrap();fs::write(format!("{dir}/X86.td"),&new_cpu).unwrap();
 fs::write(format!("{R}/patch-result.json"),serde_json::to_vec_pretty(&json!({"source_llvm_commit":"52ed14fcd56afc30f9cccd8ca8ce237c2eef7e04","cpu_before":sha(cpu.as_bytes()),"cpu_after":sha(new_cpu.as_bytes()),"inherited_model":sha(original.as_bytes()),"private_model":sha(candidate.as_bytes()),"original_model_unchanged":true,"inverse_exact":true,"opcodes":["VPMADD52LUQZr","VPMADD52HUQZr","VPADDQZrr","VPSUBQZrr"],"all_shifts_unchanged":true,"production_changed":false})).unwrap()).unwrap();
 println!("applied exact four-opcode scheduling correction; inverse source audit passed");
}
