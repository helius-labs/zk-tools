use serde_json::{json, Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs};
#[path="split.rs"] mod split;
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176";
type P=[i128;2];
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn versions(v:P)->[P;6]{let[a,b]=v;[[a,b],[-b,a-b],[b-a,-a],[-a,-b],[b,b-a],[a-b,a]]}
fn residue(v:P)->usize{(v[0].rem_euclid(128)+128*v[1].rem_euclid(128))as usize}
fn norm(v:P)->i128{v[0]*v[0]+v[1]*v[1]-v[0]*v[1]}
fn add(a:P,b:P,p:i128)->P{[(a[0]+b[0]).rem_euclid(p),(a[1]+b[1]).rem_euclid(p)]}
fn scale(a:P,k:i128,p:i128)->P{[(a[0]*k).rem_euclid(p),(a[1]*k).rem_euclid(p)]}
fn j(a:P)->P{[-a[1],a[0]-a[1]]}
fn booth(m:u128,w:usize)->i32{let bit=|i:usize|if i<128{((m>>i)&1)as i32}else{0};if w==0{(0..12).map(|k|bit(k)<<k).sum::<i32>()-(bit(11)<<12)}else{let x=(0..13).map(|k|bit(w*12-1+k)<<k).sum::<i32>();(x+1)/2-(bit(w*12+11)<<12)}}
fn main(){
    assert!(!std::path::Path::new(&format!("{R}/review.json")).exists());
    let pins=read(&format!("{R}/pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let plan=read(&format!("{R}/plan.json"));let mut normalized=plan.clone();normalized.as_object_mut().unwrap().entry("environment").or_insert(json!({}));
    for step in normalized["steps"].as_array_mut().unwrap(){step.as_object_mut().unwrap().entry("required_tests").or_insert(json!([]));}
    assert_eq!(normalized,read(&format!("{R}/runs/protocol.json")));assert_eq!(read(&format!("{R}/runs/attempt-0.json"))["resume"],false);
    for s in plan["steps"].as_array().unwrap(){let name=s["name"].as_str().unwrap();let base=format!("{R}/runs/{name}");let e=read(&format!("{base}/exit.json"));assert_eq!(e["success"],true);assert_eq!(e["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(&format!("{base}/{stream}.log")),e[format!("{stream}_sha256")].as_str().unwrap());}let c=read(&format!("{base}/command.json"));assert!(c["argv"].as_array().unwrap().ends_with(s["argv"].as_array().unwrap()));assert_eq!(c["environment"],json!({}));}
    assert_eq!(read(&format!("{R}/runs/result.json"))["status"],"complete");let tests=read(&format!("{R}/runs/ring-model-tests/tests.json"));assert_eq!(tests["unfiltered"],true);assert_eq!(tests["passed_tests"].as_array().unwrap().len(),3);
    // Derive canonical orbits independently, with lexicographically sorted
    // residue sets rather than the study's incremental table walk.
    let mut orbits=BTreeSet::new();
    for b in 0..128 {for a in 0..128 {if a==0&&b==0{continue;}let mut orbit=versions([a,b]).map(residue).to_vec();orbit.sort_unstable();orbit.dedup();orbits.insert(orbit);}}
    assert_eq!(orbits.len(),2731);
    let mut map=BTreeMap::<usize,(P,usize,P)>::new();let mut maxnorm=0;
    for orbit in orbits{let mut lifts=vec![];for &r in &orbit{let[a,b]=[r as i128%128,r as i128/128];for da in[-128,0]{for db in[-128,0]{lifts.push([a+da,b+db]);}}}lifts.sort_by_key(|&v|(norm(v),v[0],v[1]));let rep=lifts[0];maxnorm=maxnorm.max(norm(rep));for (u,value)in versions(rep).into_iter().enumerate(){let r=residue(value);map.entry(r).or_insert((rep,u,value));}}
    assert_eq!(map.len(),16383);let a=read(&format!("{R}/analysis.json"));assert_eq!(a["maximum_representative_norm"].as_i64().unwrap()as i128,maxnorm);assert_eq!(a["representatives"],2731);
    let inputs=read(&format!("{R}/inputs.json"));let mut checked=0;let mut summaries=vec![];
    for(name,input)in inputs.as_object().unwrap(){let bytes=fs::read(input["file"].as_str().unwrap()).unwrap();let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;let observed=a["cases"].as_array().unwrap().iter().find(|v|v["case"]==name.as_str()).unwrap();assert_eq!(observed["raw_rows"],n);
        let mut records=vec![vec![];19];let mut original=vec![vec![];11];let mut direct=vec![[[0i128;2];2];19];let primes=[65537i128,1000000007];let mut active=0;
        for i in 0..n{if bytes[16+128*i..16+128*(i+1)].iter().all(|&b|b==0){continue;}active+=1;let words=std::array::from_fn(|k|u64::from_le_bytes(bytes[16+128*n+32*i+8*k..16+128*n+32*i+8*k+8].try_into().unwrap()));let s=split::phi_split(words);let mut value=[0i128;2];for c in 0..2{value[c]=(s[2*c]as u128|((s[2*c+1]&0x7fffffffffffffff)as u128)<<64)as i128;if s[2*c+1]>>63!=0{value[c]=-value[c];}}
            let initial=value;let part=usize::from(i>=n.div_ceil(2));let mut digits=vec![];
            for w in 0..19{let r=residue(value);let d=if r==0{[0,0]}else{let &(rep,u,d)=map.get(&r).unwrap();records[w].push((rep,part,u,i));for(jj,&p)in primes.iter().enumerate(){let t=[((i as i128+1)*7919+37)%p,((i as i128+1)*104729+101)%p];direct[w][jj]=add(direct[w][jj],add(scale(t,d[0],p),scale(j(t),d[1],p),p),p);}d};digits.push(d);for c in 0..2{assert_eq!((value[c]-d[c])%128,0);value[c]=(value[c]-d[c])/128;}}
            assert_eq!(value,[0,0]);for c in 0..2{let result=digits.iter().enumerate().map(|(w,d)|d[c].checked_mul(1i128<<(7*w)).unwrap()).try_fold(0i128,|a,v|a.checked_add(v)).unwrap();assert_eq!(result,initial[c]);}
            for c in 0..2{for w in 0..11{let d=booth(initial[c].unsigned_abs(),w);if d!=0{original[w].push((d.unsigned_abs()as usize,part,c));}}}checked+=1;
        }
        assert_eq!(observed["model"]["rows_checked"],active);
        let mut candidate_total=0;let mut candidate_maps=0;
        for w in 0..19{let rows=&records[w];let mut classes=BTreeMap::<P,Vec<(usize,usize,usize)>>::new();for &(rep,part,u,i)in rows{classes.entry(rep).or_default().push((part,u,i));}
            let first=rows.iter().map(|&(rep,part,u,_)|(rep,part,u%3)).collect::<BTreeSet<_>>().len()as u64;
            let phis=rows.iter().filter(|(_,_,u,_)|u%3!=0).map(|&(rep,_,u,_)|(rep,u%3)).collect::<BTreeSet<_>>().len()as u64;
            let mut axes:[BTreeMap<usize,Vec<(P,i128)>>;2]=std::array::from_fn(|_|BTreeMap::new());let mut reference=[0u64;2];let mut reconstructed=[[0i128;2];2];
            for(&rep,members)in &classes{for axis in 0..2{if rep[axis]!=0{reference[axis]+=1;axes[axis].entry(rep[axis].unsigned_abs()as usize).or_default().push((rep,rep[axis].signum()));}}
                for(jj,&p)in primes.iter().enumerate(){let mut token=[0i128;2];for&(_,u,i)in members{let t=[((i as i128+1)*7919+37)%p,((i as i128+1)*104729+101)%p];let t=match u%3{0=>t,1=>j(t),2=>j(j(t)),_=>unreachable!()};token=add(token,scale(t,if u>=3{-1}else{1},p),p);}reconstructed[jj]=add(reconstructed[jj],add(scale(token,rep[0],p),scale(j(token),rep[1],p),p),p);}}
            assert_eq!(reconstructed,direct[w]);
            let groups=axes.each_ref().map(|m|m.len()as u64);let slots=axes.each_ref().map(|m|m.keys().next_back().copied().unwrap_or(0));
            let joins=reference.iter().sum::<u64>()-groups.iter().sum::<u64>();let fold=slots.iter().map(|&n|2*(n as u64).saturating_sub(1)).sum::<u64>();let combine=u64::from(groups.iter().all(|&n|n>0));let merges=first-classes.len()as u64;let adds=rows.len()as u64-first+merges+joins+fold+combine;
            let row=json!({"position":w,"arrivals":rows.len(),"first_assignments":first,"unit_and_part_merges":merges,"merged_representatives":classes.len(),"axis_references":reference,"axis_groups":groups,"axis_group_joins":joins,"axis_dense_fold_slots":slots,"axis_dense_fold_adds":fold,"axis_result_combinations":combine,"generic_adds":adds,"unit_phi_maps":phis,"unit_phi_Fq_multiplications":2*phis,"formal_token_checks":2});assert_eq!(row,observed["model"]["ring"]["rows"][w]);candidate_total+=adds;candidate_maps+=phis;
        }
        let mut base_total=0;let mut base_maps=0;
        for w in 0..11{let rows=&original[w];let slots=rows.iter().map(|&(b,_,_)|b).max().unwrap_or(0);let occupied=rows.iter().map(|&(b,p,_)|(b,p)).collect::<BTreeSet<_>>();let overlap=occupied.iter().filter(|&&(b,p)|p==0&&occupied.contains(&(b,1))).count()as u64;let maps=rows.iter().filter(|(_,_,c)|*c==1).map(|&(b,p,_)|(b,p)).collect::<BTreeSet<_>>().len()as u64;let adds=rows.len()as u64-occupied.len()as u64+overlap+2*(slots as u64).saturating_sub(1);let row=json!({"position":w,"arrivals":rows.len(),"first_assignments":occupied.len(),"part_merges":overlap,"dense_fold_slots":slots,"generic_adds":adds,"unit_phi_maps":maps,"unit_phi_Fq_multiplications":2*maps});assert_eq!(row,observed["model"]["baseline_full_dense"]["rows"][w]);base_total+=adds;base_maps+=maps;}
        assert_eq!(observed["model"]["ring"]["generic_addition_model_total"],candidate_total);assert_eq!(observed["model"]["ring"]["unit_phi_maps"],candidate_maps);assert_eq!(observed["model"]["baseline_full_dense"]["generic_addition_model_total"],base_total);assert_eq!(observed["model"]["baseline_full_dense"]["unit_phi_maps"],base_maps);assert_eq!(observed["model"]["necessary_generic_work_gate_passed"],candidate_total<base_total);assert_eq!(observed["model"]["ring_over_baseline_dense"].as_f64().unwrap(),candidate_total as f64/base_total as f64);
        summaries.push(json!({"case":name,"checked_rows":active,"ring_adds":candidate_total,"baseline_dense_adds":base_total,"ring_phi_maps":candidate_maps,"baseline_phi_maps":base_maps}));
    }
    assert_eq!(a["necessary_primary_generic_work_gate_passed"],true);assert_eq!(a["native_implementation_selected"],false);assert_eq!(a["adopted"],false);
    let out=json!({"passed":true,"pins":pins.as_object().unwrap().len(),"production_originals_exact":176,"successful_workflow_commands":4,"model_tests":3,"checked_real_scalar_rows":checked,"independent_ring_orbits":2731,"independent_residue_mappings":16383,"review_method":"sorted unit orbits; explicit unit-coordinate formulas; direct quotient/subtraction; weighted integer reconstruction; sorted population sets; independent per-bit Booth extraction; independent two-prime token sums; no study import","shared_decomposition_inherited":true,"necessary_primary_generic_work_gate_passed":true,"native_implementation_selected":false,"cases":summaries,"new_proofs":0,"proof_ledger":30708,"adopted":false,"goal_achieved":false});
    fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("Review PASS: {} pins, {} real rows, all ring and baseline counts exact; native not selected",pins.as_object().unwrap().len(),checked);
}
