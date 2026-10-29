fn descends(mut id:u64,ancestor:u64,map:&BTreeMap<u64,Event>)->bool{while id!=0{if id==ancestor{return true}id=map[&id][1];}false}
fn window_identity(mut id:u64,map:&BTreeMap<u64,Event>)->u64{while id!=0{let e=map[&id];if [10,11,12].contains(&e[2]){return id}id=e[1];}0}
fn window_gap(events:&[Event],window:Event)->Value{
 let map:BTreeMap<_,_>=events.iter().map(|e|(e[0],*e)).collect();let last=events.iter().filter(|e|e[8]!=0&&descends(e[0],window[0],&map)).map(|e|e[6]).max().unwrap();
 let mut details=vec![];let(mut other_wall,mut other_cpu,mut partial)=(0u64,0u64,0usize);for e in events.iter().filter(|e|e[8]!=0&&e[7]==window[7]){let wall=overlap(last,window[6],e[5],e[6]);if wall==0{continue}let wid=window_identity(e[0],&map);assert_ne!(wid,window[0]);let full=e[5]>=last&&e[6]<=window[6];other_wall+=wall;if full{other_cpu+=e[9]-e[8]}else{partial+=1;}details.push(json!({"leaf_id":e[0],"logical_query":query(e[0],&map),"logical_window_id":wid,"service_span_overlap_ns":wall,"full_leaf":full,"CPU_ns":if full{json!(e[9]-e[8])}else{Value::Null}}));}
 let gap=window[6]-last;assert!(other_wall<=gap);json!({"window_id":window[0],"query":query(window[0],&map),"index_or_bit":window[3],"width":window[4],"owner_worker":window[7],"last_own_serial_end_ns":last,"callback_end_ns":window[6],"gap_ns":gap,"other_window_service_span_overlap_ns":other_wall,"other_window_fully_contained_CPU_ns":other_cpu,"partial_service_leaves":partial,"uncovered_elapsed_ns":gap-other_wall,"other_window_span_fraction":if gap==0{0.}else{other_wall as f64/gap as f64},"records":details})
}
#[cfg(test)]mod expanded_tests{
use super::*;
#[test]fn window_ownership_separates_same_query_help(){
let id=|w:u64,s:u64|(w<<32)|s;
let events=vec![[id(0,1),0,0,0,0,0,110,0,0,0],[id(1,1),id(0,1),2,0,0,5,105,1,0,0],[id(2,1),id(1,1),11,0,0,10,100,2,0,0],[id(3,1),id(1,1),11,13,0,20,95,3,0,0],[id(2,2),id(2,1),21,0,0,12,40,2,100,120],[id(2,3),id(3,1),21,0,0,50,80,2,130,155]];
assert!(decode(&encode(&events)).is_ok());let v=window_gap(&events,events[2]);assert_eq!(v["gap_ns"],60);assert_eq!(v["other_window_service_span_overlap_ns"],30);assert_eq!(v["other_window_fully_contained_CPU_ns"],25);assert_eq!(v["records"][0]["logical_query"],"kh");assert_eq!(v["records"][0]["logical_window_id"],id(3,1));assert_eq!(coverage(&events,2,"kh",40,100)["foreign_service_span_overlap_ns"],0);
}
}
