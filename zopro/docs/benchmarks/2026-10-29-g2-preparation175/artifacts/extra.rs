use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-preparation175";
type E=[u64;10];
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn med(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);assert!(!v.is_empty());let n=v.len();if n%2==1{v[n/2]}else{(v[n/2-1]+v[n/2])/2.}}
fn clip(a:u64,b:u64,x:u64,y:u64)->u64{b.min(y).saturating_sub(a.max(x))}
fn coverage(events:&[E],worker:u64,x:u64,y:u64)->Value{
    assert!(y>=x);let mut span=0;let mut cpu=0;let mut partial=0;let mut ids=vec![];
    for e in events.iter().filter(|e|e[7]==worker&&e[8]>0){
        let n=clip(e[5],e[6],x,y);if n>0{span+=n;ids.push(e[0]);if e[5]>=x&&e[6]<=y{cpu+=e[9]-e[8];}else{partial+=1;}}
    }
    assert!(span<=y-x);
    json!({"start_ns":x,"end_ns":y,"owner":worker,"elapsed_ms":(y-x)as f64/1e6,"covered_serial_span_ms":span as f64/1e6,"fully_contained_CPU_ms":cpu as f64/1e6,"partially_clipped_leaves_CPU_unknown":partial,"covered_span_fraction":if y>x{Some(span as f64/(y-x)as f64)}else{None},"event_ids":ids})
}
fn concurrency(events:&[E])->(u64,usize){
    let mut edges=vec![];for e in events{assert!(e[6]>=e[5]);if e[6]>e[5]{edges.push((e[5],1i32));edges.push((e[6],-1));}}
    edges.sort();let mut live=0i32;let mut max=0;let mut union=0;let mut prev=0;
    for(t,d)in edges{if live>0{union+=t-prev;}live+=d;assert!(live>=0);max=max.max(live as usize);prev=t;}assert_eq!(live,0);(union,max)
}
fn ancestors(id:u64,events:&BTreeMap<u64,E>)->Vec<u64>{let mut out=vec![];let mut id=id;while id!=0{assert!(!out.contains(&id));out.push(id);id=events.get(&id).unwrap()[1];}out}
fn main(){
    if std::env::args().nth(1).as_deref()!=Some("analyze"){panic!("analyze");}
    let graphs=read(r("graphs.json"));let mut decoded=BTreeMap::new();let mut rows=vec![];let mut bytes_total=0;let mut records_total=0;
    for g in graphs.as_array().unwrap().iter().filter(|g|g["job"]["mode"]=="on"){
        let p=g["graph"]["file"].as_str().unwrap();let bytes=fs::read(p).unwrap();assert_eq!(hash(&bytes),g["graph"]["sha256"]);assert_eq!(bytes.len()%80,0);
        let events:Vec<E>=bytes.chunks_exact(80).map(|s|core::array::from_fn(|j|u64::from_le_bytes(s[j*8..j*8+8].try_into().unwrap()))).collect();
        let reconstructed:Vec<u8>=events.iter().flat_map(|e|e.iter().flat_map(|n|n.to_le_bytes())).collect();assert_eq!(bytes,reconstructed);records_total+=events.len();bytes_total+=bytes.len();
        let map:BTreeMap<_,_>=events.iter().map(|e|(e[0],*e)).collect();assert_eq!(events.len(),map.len());
        decoded.insert(p,json!(events));
        if g["job"]["phase"]!="main"{continue;}
        let one=|k|{let v:Vec<_>=events.iter().filter(|e|e[2]==k).collect();assert_eq!(v.len(),1);*v[0]};
        let q=one(5);let group=one(30);let stage=one(32);let ready=one(4);let kernel=one(3);
        let folds:Vec<_>=events.iter().filter(|e|e[2]==33).copied().collect();assert_eq!(folds.iter().map(|e|e[3]).sum::<u64>(),stage[3]);
        let last=folds.iter().map(|e|e[6]).max().unwrap();let first=folds.iter().map(|e|e[5]).min().unwrap();let cpu:u64=folds.iter().map(|e|e[9]-e[8]).sum();let maxcpu=folds.iter().map(|e|e[9]-e[8]).max().unwrap();
        let (union,maxlive)=concurrency(&folds);let mut workers=BTreeMap::<u64,Value>::new();
        for w in 0..33{let es:Vec<_>=folds.iter().filter(|e|e[7]==w).collect();if !es.is_empty(){workers.insert(w,json!({"folds":es.len(),"items":es.iter().map(|e|e[3]).sum::<u64>(),"CPU_ms":es.iter().map(|e|e[9]-e[8]).sum::<u64>()as f64/1e6}));}}
        let otherquery:Vec<_>=events.iter().filter(|e|e[8]>0&&!ancestors(e[0],&map).contains(&q[0])).copied().collect();
        let group_points:Vec<_>=events.iter().filter(|e|e[2]==31).collect();assert!(group_points.len()<=1);
        let row=json!({"job":g["job"]["name"],"round":g["graph"]["round"],"query_pre_input_ms":(ready[6]-q[5])as f64/1e6,"query_owner":q[7],"grouping_ms":(group[6]-group[5])as f64/1e6,"grouping_CPU_ms":(group[9]-group[8])as f64/1e6,"grouped_point_stage":group_points,"query_entry_to_grouping_ms":(group[5]-q[5])as f64/1e6,"grouping_end_to_kernel_entry_ms":(kernel[5]-group[6])as f64/1e6,"kernel_to_decomposition_ms":(stage[5]-kernel[5])as f64/1e6,"decomposition_ms":(stage[6]-stage[5])as f64/1e6,"decomposition_owner":stage[7],"stage_entry_to_first_fold_ms":(first-stage[5])as f64/1e6,"fold_CPU_ms":cpu as f64/1e6,"fold_CPU_ns_per_item":cpu as f64/stage[3]as f64,"fold_max_CPU_ms":maxcpu as f64/1e6,"folds":folds.len(),"empty_folds":folds.iter().filter(|e|e[3]==0).count(),"fold_items":folds.iter().map(|e|e[3]).collect::<Vec<_>>(),"fold_span_union_ms":union as f64/1e6,"max_live_fold_spans":maxlive,"fold_CPU_over_stage_elapsed":cpu as f64/(stage[6]-stage[5])as f64,"workers":workers,"last_fold_to_stage_return":coverage(&events,stage[7],last,stage[6]),"last_fold_to_stage_return_other_query":coverage(&otherquery,stage[7],last,stage[6]),"stage_other_query_owner_service":coverage(&otherquery,stage[7],stage[5],stage[6]),"stage_end_to_ready_ms":(ready[6]-stage[6])as f64/1e6,"query_owner_preinput_foreign_service":coverage(&otherquery,q[7],q[5],ready[6])});
        rows.push(row);
    }
    assert_eq!(rows.len(),9);let mut metrics=BTreeMap::new();
    for key in ["query_pre_input_ms","grouping_ms","grouping_CPU_ms","query_entry_to_grouping_ms","grouping_end_to_kernel_entry_ms","kernel_to_decomposition_ms","decomposition_ms","stage_entry_to_first_fold_ms","fold_CPU_ms","fold_CPU_ns_per_item","fold_max_CPU_ms","folds","empty_folds","fold_span_union_ms","max_live_fold_spans","fold_CPU_over_stage_elapsed","stage_end_to_ready_ms"]{
        let values:Vec<f64>=rows.iter().map(|r|r[key].as_f64().unwrap()).collect();metrics.insert(key,json!({"median":med(values.clone()),"values":values}));
    }
    for group in["last_fold_to_stage_return","last_fold_to_stage_return_other_query","stage_other_query_owner_service","query_owner_preinput_foreign_service"]{let mut mm=BTreeMap::new();for key in["elapsed_ms","covered_serial_span_ms","fully_contained_CPU_ms","covered_span_fraction"]{let v:Vec<f64>=rows.iter().filter_map(|r|r[group][key].as_f64()).collect();if !v.is_empty(){mm.insert(key,json!({"median":med(v.clone()),"values":v}));}}metrics.insert(group,json!(mm));}
    let result=json!({"status":"complete","main_graphs":9,"enabled_graphs_decoded":decoded.len(),"records":records_total,"raw_bytes":bytes_total,"byte_exact_reconstruction":true,"metrics":metrics,"graphs":rows,"limits":"Fold CPU includes clocks/counts and observer; span concurrency is not CPU utilization. Partial serial-leaf CPU stays unknown; uncovered gaps are not idle/removable work. Main observer qualification remains authoritative in analysis.json."});
    save(r("decoded-events.json"),&json!(decoded));save(r("preparation-detail.json"),&result);
    println!("Decoded {records_total} records/{bytes_total} bytes; nine detailed graphs");println!("{}",result["metrics"]);
}
#[cfg(test)]mod tests{
 use super::*;
 fn e(id:u64,parent:u64,start:u64,end:u64,cpu:u64)->E{[id,parent,33,1,0,start,end,2,100,100+cpu]}
 #[test]fn clipped_cpu_is_unknown_and_owner_is_physical(){let es=[e(1,0,10,30,18),e(2,0,35,45,7),{let mut a=e(3,0,12,24,11);a[7]=3;a}];let v=coverage(&es,2,20,50);assert_eq!(v["covered_serial_span_ms"],20.0/1e6);assert_eq!(v["fully_contained_CPU_ms"],7.0/1e6);assert_eq!(v["partially_clipped_leaves_CPU_unknown"],1);}
 #[test]fn span_union_and_concurrency_handle_touching_endpoints(){assert_eq!(concurrency(&[e(1,0,10,20,9),e(2,0,15,30,14),e(3,0,30,35,4),e(4,0,17,17,0)]),(25,2));}
 #[test]fn logical_ownership_crosses_physical_workers(){let mut a=e(1,0,10,90,0);a[7]=1;let b=e(2,1,20,80,0);let mut c=e(3,2,30,40,8);c[7]=3;let map=[a,b,c].into_iter().map(|e|(e[0],e)).collect();assert_eq!(ancestors(3,&map),[3,2,1]);}
}
