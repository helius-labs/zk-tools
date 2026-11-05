use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/parallel-readiness176";
fn main(){let r=Path::new(R);let prior="/home/ubuntu/src/zopro-perf-work/absolute-norm176";let s=fs::read_to_string(format!("{prior}/native.rs")).unwrap().replace(prior,R).replace("zopro_absolute_norm","zopro_parallel_readiness");fs::write(r.join("native.rs"),s).unwrap();
 let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/wire-readiness175/model.rs").unwrap();
 s=s.replace("    snapshots: Vec<Value>,","    snapshots: Vec<Value>,\n    edges: Vec<(usize,usize)>,").replace("        self.last_read[wire] = op;","        self.last_read[wire] = op;\n        self.edges.push((wire,op));").replace("        snapshots: vec![],","        snapshots: vec![],\n        edges:vec![],");
 let old="pub fn census(data: &[u8]) -> Result<Value, String> {";assert_eq!(s.matches(old).count(),1);s=s.replace(old,"pub fn census(data:&[u8])->Result<Value,String>{Ok(program(data)?.0)}\npub fn program(data: &[u8]) -> Result<(Value,Vec<usize>,Vec<(usize,usize)>,[Vec<usize>;2]), String> {");
 let old="    Ok(\n        json!({\"complete\":true";assert_eq!(s.matches(old).count(),1);s=s.replace(old,"    let metadata=\n        json!({\"complete\":true");let old="\"new_timing_samples\":0}),\n    )\n}";assert_eq!(s.matches(old).count(),1);s=s.replace(old,"\"new_timing_samples\":0});\n    Ok((metadata,state.producer,state.edges,state.queries))\n}");fs::write(r.join("model.rs"),s).unwrap();
}
