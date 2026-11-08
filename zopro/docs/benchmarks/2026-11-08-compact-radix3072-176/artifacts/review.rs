use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/compact-radix3072-176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn get(v:&Value,k:&str)->u64{v[k].as_u64().unwrap_or(0)}
fn add(m:&mut BTreeMap<String,u64>,v:&Value){for(k,v)in v.as_object().unwrap(){*m.entry(k.clone()).or_default()+=v.as_u64().unwrap();}}
fn weighted_check(a:&[u64])->(u64,u64){
    let mut lists=vec![a.to_vec()];let mut joins=0;
    while lists[0].len()>1{let copied=lists[0].iter().step_by(2).copied().collect();for list in&mut lists{*list=list.chunks_exact(2).map(|p|{joins+=u64::from(p[0]!=0&&p[1]!=0);p[0]+p[1]}).collect();}lists.push(copied);}
    let mut weighted=lists[0][0];for list in lists[1..].iter().rev(){weighted=2*weighted-list[0];}(weighted,joins)
}
fn main(){
    let r=Path::new(R);let pins=read(r.join("pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let plan=read(r.join("plan.json"));let protocol=read(r.join("runs/protocol.json"));assert_eq!(plan["steps"].as_array().unwrap().len(),1);assert_eq!(plan["steps"][0]["argv"],protocol["steps"][0]["argv"]);assert_eq!(plan["source_pins"],protocol["source_pins"]);
    let exit=read(r.join("runs/codec-and-execution-type-screen/exit.json"));assert_eq!(exit["returncode"],0);assert_eq!(exit["success"],true);
    for(file,k)in[("stdout.log","stdout_sha256"),("stderr.log","stderr_sha256")]{assert_eq!(hash(r.join("runs/codec-and-execution-type-screen").join(file)),exit[k].as_str().unwrap());}
    assert_eq!(read(r.join("runs/result.json"))["status"],"complete");let a=read(r.join("analysis.json"));assert_eq!(a["new_proofs"],0);assert_eq!(a["new_timings"],0);assert_eq!(a["production_changed"],false);assert_eq!(a["proof_ledger_count"],37023);assert_eq!(a["baseline_primary_native_work_census_matches"],true);
    let codec=&a["codec_checks"];assert_eq!(codec["bytes_per_scalar_row"],32);assert_eq!(codec["reciprocal_quotient_checks"],118098*11);assert_eq!(codec["adjacent_digit_cases"],3072u64*3072);
    let limit=(1u128<<126)-1;assert_eq!(limit/3072u128.pow(10),1136);assert!(1136<1536);assert!(2*3u64.pow(10)<1<<17);assert!(118098u128*59049<(1u128<<40));
    // Independently check telescoping for every carry pattern, avoiding the
    // screen's reciprocal decoder and fixture/component implementation.
    let mut telescoping=0;
    for mask in 0usize..1<<10{let mut digits=[0i128;11];let mut unsigned=[0i128;11];for i in 0..10{unsigned[i]=if mask>>i&1!=0{1536}else{1535};}unsigned[10]=1136;
        for i in 0..11{let incoming=i128::from(i!=0&&unsigned[i-1]>=1536);digits[i]=unsigned[i]-3072*i128::from(unsigned[i]>=1536)+incoming;}
        let eval=|d:&[i128;11]|d.iter().rev().fold(0i128,|v,&d|v*3072+d);assert_eq!(eval(&digits),eval(&unsigned));assert!(digits.iter().all(|d|d.abs()<=1536));telescoping+=1;
    }
    let mut weighted_masks=0;
    for n in[1usize,2,4,8]{for mask in 0usize..1<<n{let points=(0..n).map(|i|u64::from(mask>>i&1!=0)).collect::<Vec<_>>();let(weight,joins)=weighted_check(&points);assert_eq!(weight,points.iter().enumerate().map(|(i,&v)|(i as u64+1)*v).sum::<u64>());let dense=(0..n.ilog2()).map(|i|(n>>i)as u64-1).sum::<u64>();assert!(joins<=dense);weighted_masks+=1;}}
    for n in[1usize,2,4,8,16,32,64,128,256]{let(_,joins)=weighted_check(&vec![1;n]);assert_eq!(joins,(0..n.ilog2()).map(|i|(n>>i)as u64-1).sum::<u64>());}
    let mut component_checks=0;
    for row in a["fixture_rows"].as_array().unwrap(){component_checks+=row["component_checks"].as_u64().unwrap();let mut increases=BTreeMap::new();
        for arm in["baseline","candidate"]{let v=&row[arm];let mut all=BTreeMap::new();let mut ordinary=BTreeMap::new();let mut top=BTreeMap::new();assert_eq!(v["windows"].as_array().unwrap().len(),11);assert_eq!(v["part_records"].as_array().unwrap().len(),22);
            for w in v["windows"].as_array().unwrap(){add(&mut all,&w["counts"]);add(if w["window"]==10{&mut top}else{&mut ordinary},&w["counts"]);}
            assert_eq!(json!(all),v["total"]);assert_eq!(json!(ordinary),v["ordinary"]);assert_eq!(json!(top),v["top"]);
            for p in v["part_records"].as_array().unwrap(){let c=&p["counts"];if get(c,"terms")==0{continue;}assert_eq!(get(c,"terms"),get(c,"first_writes")+get(c,"stream_batch_pairs")+get(c,"flushed_points"));assert_eq!(get(c,"queued"),get(c,"drained")+get(c,"flushed_points"));assert_eq!(get(c,"flushed_points"),get(c,"projective_accumulator_adds")+get(c,"projective_initializations"));assert_eq!(get(c,"normalized_projective_points"),get(c,"projective_initializations"));}
            let t=&v["total"];for prefix in["stream_batch","merge_batch","reduction_layer"]{assert_eq!(get(t,&format!("{prefix}_pairs")),get(t,&format!("{prefix}_vector_pairs"))+get(t,&format!("{prefix}_scalar_tail_pairs")));}
            assert_eq!(v["generic_mathematical_joins"],get(t,"stream_batch_pairs")+get(t,"flushed_points")+get(t,"merge_batch_pairs")+get(t,"reduction_layer_pairs"));
        }
        let baseline=&row["baseline"]["total"];let candidate=&row["candidate"]["total"];for k in["flushed_points","normalized_projective_points","stream_batch_calls","stream_batch_pairs","projective_accumulator_adds"]{assert!(get(candidate,k)>get(baseline,k));increases.insert(k,json!([get(baseline,k),get(candidate,k)]));}
        assert!(row["generic_join_ratio"].as_f64().unwrap()<1.0);println!("{}: fewer mathematical joins but increased execution-type counts {}",row["shape"],json!(increases));
    }
    assert_eq!(component_checks,117882);
    let output=json!({"pins_checked":pins.as_object().unwrap().len(),"workflow_receipt_and_stream_hashes_match":true,"aggregate_conservation_and_partitions_match":true,"independent_top_and_reciprocal_error_bounds_pass":true,"independent_carry_patterns":telescoping,"independent_weighted_identity_masks":weighted_masks,"independent_dense_weighted_sizes":9,"saved_component_checks":component_checks,"same_agent_separately_written_reviewer":true,"candidate_mathematical_joins_lower_but_expensive_types_increase_both_shapes":true,"decision":"Close fixed compact-radix3072/batch200/streaming-top proposal without native timing. Codec is mathematically valid; no supported latency or whole-prover memory gain.","new_proofs":0,"new_timings":0,"production_changed":false,"goal_achieved":false});
    let out=r.join("review.json");assert!(!out.exists());fs::write(out,serde_json::to_vec_pretty(&output).unwrap()).unwrap();
    println!("Independent source/receipt/count/algebra review PASS; fixed proposal closed, no native selection");
}
