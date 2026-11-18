use std::{fs,path::Path};use serde_json::{json,Value};use sha2::{Digest,Sha256};
fn main(){let r=Path::new("/home/ubuntu/src/zopro-perf-work/counted-ready247");let p=r.join("source/crates/zopro-core/src/solver/ready.rs");let old=fs::read_to_string(&p).unwrap();let extra=r#"
#[cfg(test)]
impl Ready {
    pub(super) fn check_edges(&self,p:&Plan,s:&Schedule){
        use std::collections::{BTreeMap,BTreeSet};
        let mask=if p.ops.len()<=0x07ff_ffff{0x07ff_ffff}else{u32::MAX};
        let mut owners=BTreeMap::new();
        for(i,leaf)in self.leaves.iter().enumerate(){for &op in leaf.get(s){for wire in p.writes(&p.ops[(op&mask)as usize]){assert!(owners.insert(wire,i).is_none());}}}
        let mut reference=BTreeSet::new();let mut reads=Vec::new();
        for(i,leaf)in self.leaves.iter().enumerate(){for &op in leaf.get(s){p.reads(&p.ops[(op&mask)as usize],&mut reads);for wire in &reads{if let Some(&parent)=owners.get(wire){if parent!=i{reference.insert((parent,i));}}}}}
        let mut actual=BTreeSet::new();for i in 0..self.leaves.len(){for &child in &self.graph.children[self.graph.offsets[i]..self.graph.offsets[i+1]]{assert!(actual.insert((i,child as usize)));}}
        assert_eq!(actual,reference);assert_eq!(actual.len(),self.graph.children.len());
        let mut parents=vec![0;self.leaves.len()];for &(_,child)in &reference{parents[child]+=1;}assert_eq!(parents,self.graph.parents);let roots=parents.iter().enumerate().filter_map(|(i,&n)|(n==0).then_some(i as u32)).collect::<Vec<_>>();assert_eq!(roots,self.graph.roots);
    }
}
"#;fs::write(&p,old+extra).unwrap();fs::copy(&p,r.join("ready.rs")).unwrap();let solver=r.join("source/crates/zopro-core/src/solver.rs");let old=fs::read_to_string(&solver).unwrap();let anchor="let graph=ready::build(plan,schedule).expect(\"admitted complete synthetic plan\");";assert_eq!(old.matches(anchor).count(),1);fs::write(&solver,old.replace(anchor,&format!("{anchor}\n                graph.check_edges(plan,schedule);"))).unwrap();let poseidon=r.join("source/crates/zopro-core/src/solver/sparse_poseidon/tests.rs");let old=fs::read_to_string(&poseidon).unwrap();let anchor="let graph=super::super::ready::build(p,&schedule).expect(\"tagged leaf admitted\");";assert_eq!(old.matches(anchor).count(),1);fs::write(&poseidon,old.replace(anchor,&format!("{anchor}\n        graph.check_edges(p,&schedule);"))).unwrap();let mut pins:Value=serde_json::from_slice(&fs::read(r.join("build-pins.json")).unwrap()).unwrap();for p in[&p,&solver,&poseidon,&r.join("qualify-prepare.rs")]{pins[p.display().to_string()]=json!(format!("{:x}",Sha256::digest(fs::read(p).unwrap())));}fs::write(r.join("build-pins-v2.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();let mut plan:Value=serde_json::from_slice(&fs::read(r.join("build-plan.json")).unwrap()).unwrap();plan["source_pins"]=json!(r.join("build-pins-v2.json"));fs::write(r.join("build-plan-v2.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();println!("Independent BTreeMap/BTreeSet edge/reference checks added across actual tagged and refined solver oracles");}
