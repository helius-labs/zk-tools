#[path = "split.rs"] mod split;
use std::{collections::BTreeMap, fs};
use serde_json::{json, Value};
use zopro_bn254::Fr;
type Word = [u64;4];
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/component-alias-count176";
const LAMBDA: Fr = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
fn signed_component(s: Word, c: usize) -> (u128,bool) {
    let k = c*2;
    (s[k] as u128 | (((s[k+1]&0x7fff_ffff_ffff_ffff) as u128)<<64), s[k+1]>>63!=0)
}
fn scalar(m: u128, neg: bool) -> Fr {
    let x=Fr::from_canonical([m as u64,(m>>64)as u64,0,0]).unwrap();
    if neg {-x} else {x}
}
fn check_split(w: Word) -> Word {
    let s=split::phi_split(w);
    let(a,an)=signed_component(s,0); let(b,bn)=signed_component(s,1);
    assert_eq!(scalar(a,an)+LAMBDA*scalar(b,bn),Fr::from_canonical(w).unwrap());
    s
}
fn normalized(w: Word) -> (Word,bool) {
    let s=Fr::from_canonical(w).unwrap(); let n=(-s).to_canonical();
    if n.iter().rev().cmp(w.iter().rev()).is_lt() {(n,true)} else {(w,false)}
}
fn digits(m: u128) -> [i16;11] {
    let w=[m as u64,(m>>64)as u64,0,0];
    std::array::from_fn(|col|split::booth(&w,col,12))
}
fn add_mod(a:u64,b:u64,p:u64)->u64 {(a+b)%p}
fn neg_mod(a:u64,neg:bool,p:u64)->u64 {if neg {(p-a)%p} else {a}}
fn mul_signed(a:u64,b:i16,p:u64)->u64 {neg_mod(a*b.unsigned_abs()as u64%p,b<0,p)}
#[derive(Default,Debug,PartialEq,Eq)]
struct Counts { classes:usize,members:usize,formation:usize,removed:usize,net:usize,one_window_duplicates:usize,zero_components:usize,hist:BTreeMap<(usize,usize),usize> }
impl Counts {fn value(&self)->Value {json!({"eligible_classes":self.classes,"eligible_members":self.members,"extra_formation_generic_additions":self.formation,"avoided_bucket_arrivals":self.removed,"net_generic_additions_saved":self.net,"duplicate_one_window_classes_skipped":self.one_window_duplicates,"zero_components":self.zero_components,"class_histogram_members_windows_classes":self.hist.iter().map(|(&(n,w),&k)|vec![n,w,k]).collect::<Vec<_>>()})}}
// Row tokens represent already-contracted signed full-scalar classes. Linear grouping
// is checked against individual insertion, not against a curve arithmetic oracle.
fn analyze(rows:&[(Word,u64)], p:u64)->Counts {
    let mut count=Counts::default();
    for c in 0..2 {
        let previously_removed=count.removed;
        let mut classes=BTreeMap::<u128,Vec<(bool,u64)>>::new();
        let mut before=BTreeMap::<(usize,usize),u64>::new();
        let mut before_presence=BTreeMap::<(usize,usize),usize>::new();
        for &(s,token) in rows {
            let(m,neg)=signed_component(s,c);
            if m==0 {count.zero_components+=1;continue;}
            classes.entry(m).or_default().push((neg,token));
            for (j,d) in digits(m).into_iter().enumerate(){if d!=0{
                let k=(j,d.unsigned_abs()as usize);
                let old=*before.get(&k).unwrap_or(&0);before.insert(k,add_mod(old,mul_signed(neg_mod(token,neg,p),d,p),p));
                *before_presence.entry(k).or_default()+=1;
            }}
        }
        let mut after=BTreeMap::<(usize,usize),u64>::new();
        let mut after_presence=BTreeMap::<(usize,usize),usize>::new();
        for (m,v) in classes {
            let ds=digits(m);let weight=ds.iter().filter(|&&d|d!=0).count();
            *count.hist.entry((v.len(),weight)).or_default()+=1;
            let eligible=v.len()>=2&&weight>=2;
            if v.len()>=2&&weight==1{count.one_window_duplicates+=1;}
            if eligible {count.classes+=1;count.members+=v.len();count.formation+=v.len()-1;count.removed+=(v.len()-1)*weight;count.net+=(v.len()-1)*(weight-1);}
            let mut emit=|token:u64| {for (j,d)in ds.into_iter().enumerate(){if d!=0{
                let k=(j,d.unsigned_abs()as usize);
                let old=*after.get(&k).unwrap_or(&0);after.insert(k,add_mod(old,mul_signed(token,d,p),p));
                *after_presence.entry(k).or_default()+=1;
            }}};
            if eligible {emit(v.iter().fold(0,|acc,&(neg,t)|add_mod(acc,neg_mod(t,neg,p),p)));}
            else {for(neg,t)in v {emit(neg_mod(t,neg,p));}}
        }
        assert_eq!(before,after); // Every signed bucket, not just a final checksum.
        assert_eq!(before_presence.keys().collect::<Vec<_>>(),after_presence.keys().collect::<Vec<_>>());
        let old_pairs:usize=before_presence.values().map(|n|n-1).sum();
        let new_pairs:usize=after_presence.values().map(|n|n-1).sum();
        assert!(old_pairs>=new_pairs);
        let removed=old_pairs-new_pairs;
        assert_eq!(removed,count.removed-previously_removed);
        // Per-component value has one nonzero-window weight; conservation is also
        // independently reconstructed in review from sorted adjacent classes.
        assert_eq!(removed,before_presence.values().sum::<usize>()-after_presence.values().sum::<usize>());
    }
    assert_eq!(count.removed-count.formation,count.net);
    count
}
fn read(file:&str)->(Vec<Word>,Vec<bool>) {
    let b=fs::read(file).unwrap();assert_eq!(&b[..8],b"B2SCAN01");
    let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(b.len(),16+160*n);
    let words=b[16+128*n..].chunks_exact(32).map(|v|std::array::from_fn(|i|u64::from_le_bytes(v[8*i..8*i+8].try_into().unwrap()))).collect();
    let ids=b[16..16+128*n].chunks_exact(128).map(|p|p.iter().all(|&b|b==0)).collect();(words,ids)
}
fn main(){
    let inputs:Value=serde_json::from_slice(&fs::read(format!("{ROOT}/inputs.json")).unwrap()).unwrap();
    let mut results=Vec::new();let mut primary_pass=None;let mut reconstructions=0;
    for (name,input)in inputs.as_object().unwrap(){
        let(w,ids)=read(input["file"].as_str().unwrap());
        for &s in &w{check_split(s);reconstructions+=1;}
        let part=w.len().div_ceil(2);let mut parts=Vec::new();let mut net=0;
        for start in (0..w.len()).step_by(part){let end=(start+part).min(w.len());
            let mut groups=BTreeMap::<Word,Vec<(usize,bool)>>::new();
            for i in start..end {if ids[i]||w[i]==[0;4]{continue;}let(k,neg)=normalized(w[i]);groups.entry(k).or_default().push((i,neg));}
            let mut by_prime=Vec::new();let mut previous=None;
            for p in [65537u64,1000000007] {
                let rows=groups.iter().map(|(&k,v)|{
                    let t=v.iter().fold(0,|acc,&(i,neg)|add_mod(acc,neg_mod(((i as u64+1)*7919+37)%p,neg,p),p));
                    (check_split(k),t)
                }).collect::<Vec<_>>();
                let counts=analyze(&rows,p);
                if let Some(old)=previous.as_ref(){assert_eq!(old,&counts);}
                by_prime.push(counts.value());previous=Some(counts);
            }
            net+=previous.unwrap().net;
            parts.push(json!({"start":start,"end":end,"active_original_rows":groups.values().map(|v|v.len()).sum::<usize>(),"signed_full_scalar_classes":groups.len(),"counts":by_prime[0],"two_small_prime_bucket_oracles_agree":true}));
        }
        if name=="1x2-B" {primary_pass=Some(net>0);}
        results.push(json!({"case":name,"raw_rows":w.len(),"parts":parts,"net_additional_generic_additions_saved":net}));
    }
    let out=json!({"complete":true,"necessary_primary_novelty_gate_passed":primary_pass.unwrap(),"rows":results,"raw_scalar_split_reconstructions":reconstructions,"width":12,"partitions":2,"signed_full_scalar_contraction":"optimistic conditional comparison, not production dispatch","new_proofs":0,"proof_ledger":29953,"adopted":false,"new_latency_claim":false,"goal_achieved":false});
    fs::write(format!("{ROOT}/analysis.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("{}",serde_json::to_string(&out).unwrap());
}
#[cfg(test)]mod tests{
    use super::*;
    fn packed(a:i128,b:i128)->Word{let e=|v:i128|v.unsigned_abs()|if v<0{1<<127}else{0};let(a,b)=(e(a),e(b));[a as u64,(a>>64)as u64,b as u64,(b>>64)as u64]}
    #[test]fn distinct_full_pairs_share_one_multwindow_component(){
        let rows=vec![(packed(4097,7),13),(packed(-4097,19),5),(packed(0,0),71)];
        for p in[65537,1000000007]{let c=analyze(&rows,p);assert_eq!((c.classes,c.members,c.formation,c.removed,c.net),(1,2,1,2,1));assert_eq!(c.zero_components,2);}
    }
    #[test]fn zero_unit_and_opposite_tokens_preserve_bucket_algebra(){
        for n in[0,1,7,31,1025]{let rows=(0..n).map(|i|(packed(if i%2==0{1}else{-1},if i%3==0{0}else{(1<<80)+4097}),if i%2==0{17}else{65537-17})).collect::<Vec<_>>();let a=analyze(&rows,65537);let b=analyze(&rows,1000000007);assert_eq!(a,b);assert_eq!(a.net,a.removed-a.formation);}
    }
    #[test]fn accepted_split_reconstructs_scalar_edges_and_changing_values(){
        let mut cases=vec![Fr::ZERO,Fr::ONE,-Fr::ONE,LAMBDA,LAMBDA+Fr::ONE,LAMBDA-Fr::ONE];
        for i in 0..254{let mut w=[0;4];w[i/64]=1<<(i%64);let s=Fr::from_canonical(w).unwrap();cases.extend([s,-s,s+Fr::ONE,s-Fr::ONE]);}
        let mut x=Fr::from(191u64);for _ in 0..4096{x=x*x+Fr::from(17u64);cases.push(x);}
        for s in cases{let(k,neg)=normalized(s.to_canonical());assert_eq!(if neg{-Fr::from_canonical(k).unwrap()}else{Fr::from_canonical(k).unwrap()},s);check_split(s.to_canonical());check_split(k);}
    }
}
