use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};
use zopro_bn254::Fr;
#[path = "split.rs"] mod split;
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176";
const RADIX: i128 = 128;
const WINDOWS: usize = 19;
const LAMBDA: Fr = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
type Pair = [i128; 2];
fn turn(v: Pair) -> Pair { [-v[1], v[0] - v[1]] }
fn neg(v: Pair) -> Pair { [-v[0], -v[1]] }
fn norm(v: Pair) -> i128 { v[0]*v[0] - v[0]*v[1] + v[1]*v[1] }
fn residue(v: Pair) -> usize { (v[0].rem_euclid(RADIX) + RADIX*v[1].rem_euclid(RADIX)) as usize }
fn unit(v: Pair, plane: usize, negative: bool) -> Pair {
    let v = (0..plane).fold(v, |v, _| turn(v));
    if negative { neg(v) } else { v }
}
#[derive(Clone, Copy, Debug)] struct Digit { rep: usize, plane: usize, negative: bool, value: Pair }
struct Table { reps: Vec<Pair>, map: Vec<Option<Digit>> }
impl Table {
    fn new() -> Self {
        let mut out = Self { reps: vec![], map: vec![None; (RADIX*RADIX) as usize] };
        for n in 1..out.map.len() {
            if out.map[n].is_some() { continue; }
            let seed = [n as i128 % RADIX, n as i128 / RADIX];
            let mut candidates = vec![];
            for plane in 0..3 { for negative in [false, true] {
                let v = unit(seed, plane, negative);
                let v = [v[0].rem_euclid(RADIX), v[1].rem_euclid(RADIX)];
                for da in [-RADIX, 0] { for db in [-RADIX, 0] { candidates.push([v[0]+da, v[1]+db]); } }
            } }
            let rep = candidates.into_iter().min_by_key(|&v| (norm(v),v[0],v[1])).unwrap();
            let index = out.reps.len(); out.reps.push(rep);
            for plane in 0..3 { for negative in [false, true] {
                let value = unit(rep, plane, negative); let r = residue(value);
                if let Some(old) = out.map[r] { assert_eq!(old.rep,index); }
                else { out.map[r] = Some(Digit {rep:index, plane, negative, value}); }
            } }
        }
        assert!(out.map[0].is_none()); assert!(out.map[1..].iter().all(Option::is_some));
        // Exact conservative rational bound on the terminal ring norm radius:
        // 2*2^127/128^19 + ceil(sqrt(maxnorm))/127 < 1.
        let max = out.reps.iter().map(|&v|norm(v)).max().unwrap();
        let mut radius=0i128; while radius*radius<max {radius+=1;}
        assert!(2*127 + 64*radius < 64*127);
        out
    }
    fn recode(&self, value: Pair) -> [Option<Digit>; WINDOWS] {
        assert!(value.iter().all(|v| v.unsigned_abs() < (1u128<<127)));
        let mut q=value;
        let out=std::array::from_fn(|_| {
            let r=residue(q); let digit=self.map[r];
            let d=digit.map(|d|d.value).unwrap_or([0,0]);
            for j in 0..2 { let low=q[j].rem_euclid(RADIX); assert_eq!((low-d[j])%RADIX,0);
                q[j]=q[j].div_euclid(RADIX)+(low-d[j])/RADIX; }
            digit
        });
        assert_eq!(q,[0,0]);
        let reconstructed=out.iter().rev().fold([0i128;2],|q,d| std::array::from_fn(|j|
            q[j].checked_mul(RADIX).unwrap().checked_add(d.map(|d|d.value[j]).unwrap_or(0)).unwrap()));
        assert_eq!(reconstructed,value); out
    }
}
fn component(s:[u64;4], c:usize) -> i128 {
    let m=s[2*c] as u128 | ((s[2*c+1]&0x7fffffffffffffff) as u128)<<64;
    let x=i128::try_from(m).unwrap(); if s[2*c+1]>>63!=0 {-x} else {x}
}
fn token(i:usize,p:i128)->Pair { [((i as i128+1)*7919+37)%p, ((i as i128+1)*104729+101)%p] }
fn add(a:Pair,b:Pair,p:i128)->Pair {std::array::from_fn(|j|(a[j]+b[j]).rem_euclid(p))}
fn scale(a:Pair,k:i128,p:i128)->Pair {std::array::from_fn(|j|(a[j]*k).rem_euclid(p))}
fn apply(v:Pair,coeff:Pair,p:i128)->Pair {add(scale(v,coeff[0],p),scale(turn(v),coeff[1],p),p)}
#[derive(Clone,Default)] struct Bucket { n:u64, token:[Pair;2] }
fn fold_cost(slots:usize)->u64 {2*(slots as u64).saturating_sub(1)}
fn model(table:&Table, rows:&[(usize,[u64;4],Pair)], n:usize) -> Value {
    let size=table.reps.len();
    let mut ring=vec![vec![vec![Bucket::default();size];6];WINDOWS];
    let mut original=vec![vec![vec![0u64;2048];2];11];
    let mut original_high=vec![vec![vec![false;2048];2];11];
    let mut expected=vec![[[0i128;2];2];WINDOWS]; let primes=[65537i128,1000000007];
    let mut checked=0;
    for &(index,split,value) in rows {
        let part=usize::from(index>=n.div_ceil(2)); let digits=table.recode(value); checked+=1;
        for (w,d) in digits.into_iter().enumerate() { if let Some(d)=d {
            let b=&mut ring[w][3*part+d.plane][d.rep]; b.n+=1;
            for (j,&p) in primes.iter().enumerate() {
                let t=token(index,p); b.token[j]=add(b.token[j],scale(t,if d.negative{-1}else{1},p),p);
                expected[w][j]=add(expected[w][j],apply(t,d.value,p),p);
            }
        } }
        for c in 0..2 { let m=value[c].unsigned_abs(); let words=[m as u64,(m>>64)as u64,0,0];
            for w in 0..11 { let d=split::booth(&words,w,12); if d!=0 {let i=d.unsigned_abs()as usize-1;original[w][part][i]+=1;if c==1{original_high[w][part][i]=true;}} }
        }
        let _=split;
    }
    let mut result=vec![]; let mut total=0; let mut phi=0;
    for w in 0..WINDOWS {
        let arrivals=ring[w].iter().flatten().map(|b|b.n).sum::<u64>();
        let first=ring[w].iter().flatten().filter(|b|b.n>0).count()as u64;
        let mut occupied=0;let mut references=[0u64;2];let mut axes:[BTreeMap<usize,([Pair;2],u64)>;2]=std::array::from_fn(|_|BTreeMap::new());
        let mut phi_maps=0;
        for (i,&rep) in table.reps.iter().enumerate() {
            let live=(0..6).filter(|&plane|ring[w][plane][i].n>0).collect::<Vec<_>>();
            if live.is_empty(){continue;} occupied+=1;
            let mut point=[[0i128;2];2];
            for &plane in &live {
                // Part merge can precede unit mapping: at most one map per
                // nonzero unit plane and representative, across both parts.
                if plane%3!=0 && (plane<3 || ring[w][plane-3][i].n==0) {phi_maps+=1;}
                for(j,&p)in primes.iter().enumerate(){let t=(0..plane%3).fold(ring[w][plane][i].token[j],|t,_|turn(t));point[j]=add(point[j],t,p);}
            }
            for axis in 0..2 {let k=rep[axis];if k==0{continue;}references[axis]+=1;
                let e=axes[axis].entry(k.unsigned_abs()as usize).or_insert(([[0;2];2],0));e.1+=1;
                for(j,&p)in primes.iter().enumerate(){e.0[j]=add(e.0[j],scale(point[j],k.signum(),p),p);}
            }
        }
        let mut totals=[[[0i128;2];2];2];
        for axis in 0..2 {for (&k,(value,_))in &axes[axis]{for(j,&p)in primes.iter().enumerate(){totals[axis][j]=add(totals[axis][j],scale(value[j],k as i128,p),p);}}}
        for(j,&p)in primes.iter().enumerate(){assert_eq!(add(totals[0][j],turn(totals[1][j]),p),expected[w][j]);}
        let occupied_axes=axes.each_ref().map(|a|a.len()as u64);
        let max=axes.each_ref().map(|a|a.keys().next_back().copied().unwrap_or(0));
        let axis_grouping=references.iter().sum::<u64>()-occupied_axes.iter().sum::<u64>();
        let fold=max.map(fold_cost).iter().sum::<u64>();
        let combination=u64::from(!axes[0].is_empty()&&!axes[1].is_empty());
        let merging=first-occupied;
        let adds=arrivals-first+merging+axis_grouping+fold+combination;
        total+=adds; phi+=phi_maps;
        result.push(json!({"position":w,"arrivals":arrivals,"first_assignments":first,"unit_and_part_merges":merging,"merged_representatives":occupied,"axis_references":references,"axis_groups":occupied_axes,"axis_group_joins":axis_grouping,"axis_dense_fold_slots":max,"axis_dense_fold_adds":fold,"axis_result_combinations":combination,"generic_adds":adds,"unit_phi_maps":phi_maps,"unit_phi_Fq_multiplications":2*phi_maps,"formal_token_checks":2}));
    }
    let mut base=0;let mut base_rows=vec![];let mut base_phi=0;
    for(w,parts)in original.iter().enumerate(){let arrivals=parts.iter().flatten().sum::<u64>();let first=parts.iter().flatten().filter(|&&x|x>0).count()as u64;
        let overlap=(0..2048).filter(|&i|parts[0][i]>0&&parts[1][i]>0).count()as u64;
        let max=(0..2048).rev().find(|&i|parts[0][i]>0||parts[1][i]>0).map(|i|i+1).unwrap_or(0);
        let adds=arrivals-first+overlap+fold_cost(max);base+=adds;
        let maps=original_high[w].iter().flatten().filter(|&&v|v).count()as u64;base_phi+=maps;
        base_rows.push(json!({"position":w,"arrivals":arrivals,"first_assignments":first,"part_merges":overlap,"dense_fold_slots":max,"generic_adds":adds,"unit_phi_maps":maps,"unit_phi_Fq_multiplications":2*maps}));
    }
    json!({"rows_checked":checked,"ring":{"generic_addition_model_total":total,"unit_phi_maps":phi,"unit_phi_Fq_multiplications":2*phi,"rows":result,"dense_Horner_doublings":7*18,"axis_phi_result_maps_separate":19},"baseline_full_dense":{"generic_addition_model_total":base,"unit_phi_maps":base_phi,"unit_phi_Fq_multiplications":2*base_phi,"rows":base_rows,"dense_Horner_doublings":12*10},"ring_over_baseline_dense":total as f64/base as f64,"necessary_generic_work_gate_passed":total<base})
}
fn main(){
    assert!(!std::path::Path::new(&format!("{ROOT}/analysis.json")).exists());
    let table=Table::new(); let inputs:Value=serde_json::from_slice(&fs::read(format!("{ROOT}/inputs.json")).unwrap()).unwrap();
    let mut cases=vec![];
    for(name,input)in inputs.as_object().unwrap(){let bytes=fs::read(input["file"].as_str().unwrap()).unwrap();assert_eq!(&bytes[..8],b"B2SCAN01");let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;assert_eq!(bytes.len(),16+160*n);let mut rows=vec![];
        for i in 0..n {let words=std::array::from_fn(|j|u64::from_le_bytes(bytes[16+128*n+32*i+8*j..16+128*n+32*i+8*j+8].try_into().unwrap()));let raw=Fr::from_canonical(words).unwrap();if bytes[16+128*i..16+128*(i+1)].iter().all(|&x|x==0){continue;}let s=split::phi_split(words);let value=[component(s,0),component(s,1)];let fr=|v:i128|{let m=v.unsigned_abs();let f=Fr::from_canonical([m as u64,(m>>64)as u64,0,0]).unwrap();if v<0{-f}else{f}};assert_eq!(fr(value[0])+LAMBDA*fr(value[1]),raw);rows.push((i,s,value));}
        cases.push(json!({"case":name,"raw_rows":n,"model":model(&table,&rows,n)}));
    }
    let primary=cases.iter().find(|c|c["case"]=="1x2-B").unwrap()["model"]["necessary_generic_work_gate_passed"].as_bool().unwrap();
    let out=json!({"complete":true,"radix":RADIX,"windows":WINDOWS,"representatives":table.reps.len(),"unit_plane_slots":3*table.reps.len(),"residues":table.map.len(),"maximum_representative_norm":table.reps.iter().map(|&v|norm(v)).max().unwrap(),"cases":cases,"necessary_primary_generic_work_gate_passed":primary,"native_implementation_selected":false,"further_complete_cost_design_permitted":primary,"new_proofs":0,"proof_ledger":30708,"production_changed":false,"adopted":false,"goal_achieved":false,"limitations":"Both comparators are hypothetical dense models including all positions, not actual indexed-top work/timing. Unit phi maps charged separately; field/cancellation/queue/tail/inversion/codec/allocation/scheduler/native memory unpriced. Raw2x2 is not grouped runtime."});
    fs::write(format!("{ROOT}/analysis.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("Ring work screen complete: {} representatives, primary gate {primary}; no native/timing/proofs",table.reps.len());
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn all_residues_and_nontrivial_unit_orbits(){let t=Table::new();assert_eq!(t.reps.len(),2731);let mut seen=vec![0usize;t.reps.len()];for(r,d)in t.map.iter().enumerate().skip(1){let d=d.unwrap();assert_eq!(residue(d.value),r);assert_eq!(unit(t.reps[d.rep],d.plane,d.negative),d.value);seen[d.rep]+=1;t.recode([r as i128%RADIX,r as i128/RADIX]);}assert!(seen.iter().all(|&n|n==3||n==6));assert_eq!(seen.iter().filter(|&&n|n==3).count(),1);}
    #[test] fn signed_coordinate_boundaries_and_random(){let t=Table::new();for a in [-((1i128<<126)-1),-129,-128,-127,-1,0,1,127,128,129,(1i128<<126)-1]{for b in [-((1i128<<126)-1),-128,-1,0,1,128,(1i128<<126)-1]{t.recode([a,b]);}}let mut s=0x1200e635c7a24001u128;for _ in 0..20000{s^=s<<23;s^=s>>17;s^=s<<26;let a=(s&((1<<126)-1))as i128;s=s.rotate_left(53);let b=(s&((1<<126)-1))as i128;t.recode([if s&1==0{a}else{-a},if s&2==0{b}else{-b}]);}}
    #[test] fn weighted_planes_preserve_phi_tokens(){let t=Table::new();let rows=(0..257).map(|i|{let a=if i%3==0{0}else{(i as i128*7919)%100000-50000};let b=if i%5==0{0}else{(i as i128*104729)%100000-50000};(i,[0;4],[a,b])}).collect::<Vec<_>>();let v=model(&t,&rows,257);assert_eq!(v["rows_checked"],257);for row in v["ring"]["rows"].as_array().unwrap(){assert_eq!(row["formal_token_checks"],2);}}
}
