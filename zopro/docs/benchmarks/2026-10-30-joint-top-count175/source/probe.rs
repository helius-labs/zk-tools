use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs};
use zopro_bn254::{Fq,Fq2,Fr,G2Affine};
mod split;
type Word=[u64;4];
type Digit=(i16,i16);
const K:usize=129*129;
fn groups(words:&[Word])->Option<(Vec<Word>,Vec<usize>)>{let n=words.len();if n<256{return None}let sample=n.min(2048);let stride=n.div_ceil(sample);let seen=words.iter().step_by(stride).take(sample).collect::<BTreeSet<_>>();let observed=n.div_ceil(stride).min(sample);if seen.len()*200>observed*199{return None}let percent=if(32768..65536).contains(&n){95}else{80};let zeros=words.iter().step_by(stride).take(sample).filter(|s|**s==[0;4]).count();let nonzero=observed-zeros;if seen.len()-usize::from(zeros!=0)==nonzero&&nonzero*100>observed*percent{return None}let mut map=BTreeMap::new();let mut values=vec![];let mut classes=vec![usize::MAX;n];for(i,&v)in words.iter().enumerate(){if v==[0;4]{continue}let class=*map.entry(v).or_insert_with(||{values.push(v);values.len()-1});classes[i]=class;}if values.len()*100>n*percent{None}else{Some((values,classes))}}
fn scalar(w:&Word)->Fr{Fr::from_le_bytes_mod_order(&w.iter().flat_map(|v|v.to_le_bytes()).collect::<Vec<_>>())}
fn signed(v:i128)->Fr{let x=Fr::from_le_bytes_mod_order(&v.unsigned_abs().to_le_bytes());if v<0{-x}else{x}}
fn token(seed:&mut u64)->Fr{let mut b=[0;32];for c in b.chunks_exact_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;c.copy_from_slice(&seed.to_le_bytes());}Fr::from_le_bytes_mod_order(&b)}
fn digits(w:&Word)->Digit{
    let s=split::phi_split(*w);let mut component=[0i128;2];
    for h in 0..2{for j in 0..11{component[h]+=(split::digit_of(&s,h==1,j,12)as i128)*(1i128<<(12*j));}
        let i=2*h;let mag=s[i]as u128|((s[i+1]&0x7fff_ffff_ffff_ffff)as u128)<<64;
        assert!(mag<1u128<<126);assert_eq!(component[h],if s[i+1]>>63!=0{-(mag as i128)}else{mag as i128});}
    assert_eq!(signed(component[0])+split::phi_lambda()*signed(component[1]),scalar(w));
    let d=(split::digit_of(&s,false,10,12),split::digit_of(&s,true,10,12));assert!(d.0.unsigned_abs()<=64&&d.1.unsigned_abs()<=64);d
}
fn bin(d:i16)->Option<usize>{if d==0{None}else{assert!(d.unsigned_abs()<=64);Some(d.unsigned_abs()as usize-1+if d<0{64}else{0})}}
#[derive(Default,Clone,Debug,PartialEq,Eq)]
struct Stats{additions:usize,tree_layers:usize,prefix_layers:usize,lane_groups:usize,scalar_pairs:usize,zero_pair_layers:usize,max_layer_pairs:usize,leaf_jobs:usize,max_leaf_points:usize,histogram:BTreeMap<usize,usize>}
impl Stats{
    fn tree(&mut self,counts:&[usize]){
        if counts.iter().sum::<usize>()==0{return;}
        self.leaf_jobs+=1;self.max_leaf_points=self.max_leaf_points.max(counts.iter().sum());
        let mut counts=counts.to_vec();
        while counts.iter().any(|&n|n>1){
            let m=counts.iter().map(|n|n/2).sum::<usize>();self.additions+=m;self.tree_layers+=1;
            self.prefix_layers+=usize::from(m>=32);self.lane_groups+=m/8;self.scalar_pairs+=m%8;
            self.zero_pair_layers+=usize::from(m==0);self.max_layer_pairs=self.max_layer_pairs.max(m);
            *self.histogram.entry(m).or_default()+=1;
            for n in &mut counts{*n=n.div_ceil(2);}
        }
    }
    fn grouped(&mut self,counts:&[usize]){for chunk in counts.chunks(256){self.tree(chunk);}}
    fn indexed(&mut self,counts:&[usize]){
        let mut partials=Vec::with_capacity(counts.len());
        for &n in counts{let mut first=0;let mut tiles=0;while first<n{let len=(n-first).min(1024);self.tree(&[len]);tiles+=1;first+=len;}partials.push(tiles);}
        self.grouped(&partials);
    }
    fn json(&self)->Value{json!({"generic_additions":self.additions,"tree_layer_calls":self.tree_layers,"prefix_selected_layers_m_ge32":self.prefix_layers,"SIMD_lane_groups":self.lane_groups,"scalar_tail_pairs":self.scalar_pairs,"zero_pair_layers":self.zero_pair_layers,"max_ordinary_pairs_per_layer":self.max_layer_pairs,"nonempty_leaf_jobs":self.leaf_jobs,"max_leaf_point_payload":self.max_leaf_points,"ordinary_pairs_per_layer_histogram":self.histogram})}
}
fn counts(ds:&[Digit],active:&[bool])->(BTreeMap<Digit,usize>,[Vec<usize>;2],usize){
    let mut classes=BTreeMap::new();let mut marginal=[vec![0;128],vec![0;128]];let mut both=0;
    for (&d,&live)in ds.iter().zip(active){if !live{continue;}if d!=(0,0){*classes.entry(d).or_default()+=1;}
        both+=usize::from(d.0!=0&&d.1!=0);for(c,v)in [d.0,d.1].into_iter().enumerate(){if let Some(b)=bin(v){marginal[c][b]+=1;}}}
    (classes,marginal,both)
}
fn partition(ds:&[Digit],active:&[bool])->Value{
    let (classes,marginal,both)=counts(ds,active);let u=classes.len();let mut class_marginal=[vec![0;128],vec![0;128]];
    let occupied_both=classes.keys().filter(|d|d.0!=0&&d.1!=0).count();
    for &d in classes.keys(){for(c,v)in [d.0,d.1].into_iter().enumerate(){if let Some(b)=bin(v){class_marginal[c][b]+=1;}}}
    for c in 0..2{for b in 0..128{assert_eq!(marginal[c][b]!=0,class_marginal[c][b]!=0);}}
    let mut original=Stats::default();let mut coalesced=Stats::default();let mut joint=Stats::default();
    for c in 0..2{original.indexed(&marginal[c]);coalesced.grouped(&marginal[c]);}
    joint.grouped(&classes.values().copied().collect::<Vec<_>>());
    for c in 0..2{joint.indexed(&class_marginal[c]);}
    assert_eq!(original.additions,coalesced.additions);
    assert_eq!(original.additions-joint.additions,both-occupied_both);
    assert_eq!(original.additions,marginal.iter().flatten().map(|n|n.saturating_sub(1)).sum::<usize>());
    assert!(u<=ds.len().min(K-1));
    let union=classes.values().sum::<usize>();
    let route=8*K+18*u+8+4*ds.len();let first_hold=146*u+8+4*ds.len();let worst=ds.len().min(K-1);
    json!({"input_rows":ds.len(),"nonzero_pair_rows":union,"rows_with_both_digits_nonzero":both,"occupied_pair_classes":u,
        "occupied_both_nonzero_pair_classes":occupied_both,"generic_additions_saved_closed_form":both-occupied_both,
        "original_marginal_arrivals":marginal.iter().flatten().sum::<usize>(),"class_marginal_arrivals":class_marginal.iter().flatten().sum::<usize>(),
        "original":original.json(),"coalesced_independent_attribution":coalesced.json(),"joint":joint.json(),
        "routing_requested_payload_bytes":route,"first_reduction_held_payload_bytes":first_hold,
        "worst_route_requested_payload_bytes":8*K+18*worst+8+4*ds.len(),"worst_first_reduction_held_payload_bytes":146*worst+8+4*ds.len(),
        "second_marginal_and_tree_scratch_not_in_payload":true,"occupied_marginal_sign_bins_unchanged":true})
}
fn oracle(ds:&[Digit],active:&[bool],tokens:&[Fr])->Fr{
    assert_eq!(ds.len(),tokens.len());let mut original=[vec![Fr::ZERO;128],vec![Fr::ZERO;128]];
    let mut classes=BTreeMap::<Digit,Fr>::new();let mut target=Fr::ZERO;let lambda=split::phi_lambda();
    for ((&d,&live),&t)in ds.iter().zip(active).zip(tokens){if !live{assert_eq!(t,Fr::ZERO);continue;}
        target+=(signed(d.0 as i128)+lambda*signed(d.1 as i128))*t;
        if d!=(0,0){*classes.entry(d).or_insert(Fr::ZERO)+=t;}
        for(c,v)in [d.0,d.1].into_iter().enumerate(){if let Some(b)=bin(v){original[c][b]+=t;}}
    }
    let mut actual=[vec![Fr::ZERO;128],vec![Fr::ZERO;128]];
    for (&d,&t)in &classes{for(c,v)in [d.0,d.1].into_iter().enumerate(){if let Some(b)=bin(v){actual[c][b]+=t;}}}
    assert_eq!(original,actual);let sum=|values:&[Fr]|(0..64).map(|b|Fr::from((b+1)as u64)*(values[b]-values[b+64])).fold(Fr::ZERO,|a,b|a+b);
    let out=(sum(&actual[0])+lambda*sum(&actual[1]))*Fr::from(2u64).pow(&[120]);
    assert_eq!(out,target*Fr::from(2u64).pow(&[120]));out
}
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn census(shape:&str)->Value{
    let root="/home/ubuntu/src/zopro-perf-work/g2-queue-scan175";let path=format!("{root}/inputs/{shape}.b2q");let b=fs::read(&path).unwrap();
    let receipt=read(&format!("{root}/oracle-fixed-runs/oracle-{shape}/stdout.log"));let hash=format!("{:x}",Sha256::digest(&b));
    assert_eq!(hash,receipt["fixture_sha256"]);assert_eq!(receipt["independent_Arkworks_complete_MSM"],true);assert_eq!(&b[..8],b"B2SCAN01");
    let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(n,receipt["points"]);assert_eq!(b.len(),16+160*n);
    let live=b[16..16+128*n].chunks_exact(128).map(|raw|{let f=raw.chunks_exact(32).map(|v|Fq::from_be_bytes(v.try_into().unwrap()).unwrap()).collect::<Vec<_>>();let p=G2Affine::new(Fq2::new(f[1],f[0]),Fq2::new(f[3],f[2]));assert!(p.is_on_curve());!p.is_identity()}).collect::<Vec<_>>();
    let original=b[16+128*n..].chunks_exact(32).map(|v|core::array::from_fn(|i|u64::from_le_bytes(v[i*8..i*8+8].try_into().unwrap()))).collect::<Vec<Word>>();
    for w in &original{assert_eq!(scalar(w).to_canonical(),*w);}drop(b);
    let g=groups(&original);let words=g.as_ref().map_or_else(||original.clone(),|x|x.0.clone());
    let active=if let Some((_,map))=&g{let mut a=vec![false;words.len()];for(&i,&v)in map.iter().zip(&live){if i!=usize::MAX{a[i]|=v;}}a}else{live.clone()};
    let ds=words.iter().map(digits).collect::<Vec<_>>();let chunk=ds.len().div_ceil(2);
    let parts=(0..2).map(|p|{let first=(p*chunk).min(ds.len());let last=(first+chunk).min(ds.len());let mut v=partition(&ds[first..last],&active[first..last]);v["part"]=json!(p);v}).collect::<Vec<_>>();
    for tag in 0..2{let mut seed=0x6a6f696e74746f70u64^tag;let raw=live.iter().map(|&v|{let t=token(&mut seed);if v{t}else{Fr::ZERO}}).collect::<Vec<_>>();
        let tokens=if let Some((_,map))=&g{let mut t=vec![Fr::ZERO;words.len()];for(&i,&v)in map.iter().zip(&raw){if i!=usize::MAX{t[i]+=v;}}t}else{raw.clone()};
        let expected=original.iter().zip(&raw).map(|(w,&t)|{let d=digits(w);(signed(d.0 as i128)+split::phi_lambda()*signed(d.1 as i128))*t}).fold(Fr::ZERO,|a,b|a+b)*Fr::from(2u64).pow(&[120]);
        let mut got=Fr::ZERO;for p in 0..2{let first=(p*chunk).min(ds.len());let last=(first+chunk).min(ds.len());got+=oracle(&ds[first..last],&active[first..last],&tokens[first..last]);}assert_eq!(got,expected);
    }
    let sum=|policy:&str,k:&str|parts.iter().map(|p|p[policy][k].as_u64().unwrap()).sum::<u64>();
    let original_adds=sum("original","generic_additions");let joint_adds=sum("joint","generic_additions");
    let original_layers=sum("original","tree_layer_calls");let joint_layers=sum("joint","tree_layer_calls");
    json!({"shape":shape,"fixture_sha256":hash,"original_rows":n,"post_group_rows":words.len(),"grouping_admitted":g.is_some(),"input_identities":live.iter().filter(|&&v|!v).count(),"grouped_curve_cancellation_unmodeled":g.is_some(),
        "parts":parts,"generic_top_additions_before":original_adds,"generic_top_additions_after":joint_adds,"generic_top_additions_saved":original_adds-joint_adds,
        "generic_top_addition_fraction_saved":(original_adds-joint_adds)as f64/original_adds.max(1)as f64,
        "tree_layers_before":original_layers,"tree_layers_after":joint_layers,"prefix_layers_before":sum("original","prefix_selected_layers_m_ge32"),"prefix_layers_after":sum("joint","prefix_selected_layers_m_ge32"),
        "scalar_tail_pairs_before":sum("original","scalar_tail_pairs"),"scalar_tail_pairs_after":sum("joint","scalar_tail_pairs"),
        "necessary_gate":joint_adds<original_adds&&joint_layers<=original_layers,"two_token_oracles":true,"Phi_Booth_exact":true,"closed_form_and_tree_additions_agree":true})
}
fn main(){let rows=["1x2","2x2"].map(census);for r in &rows{println!("ROW {}",r);}println!("COMPLETE {}",json!({"necessary_gate":rows.iter().all(|r|r["necessary_gate"]==true),"production_changed":false,"performance_claim":false,"new_solver_calls":0,"SDK_requests":0,"new_independent_exports":0,"proof_ledger":25231,"goal_achieved":false}));}
#[cfg(test)]mod tests{
    use super::*;
    fn check(ds:Vec<Digit>){let active=vec![true;ds.len()];let p=partition(&ds,&active);let mut seed=1597;let tokens=(0..ds.len()).map(|_|token(&mut seed)).collect::<Vec<_>>();oracle(&ds,&active,&tokens);assert_eq!(p["original"]["generic_additions"].as_u64().unwrap()-p["joint"]["generic_additions"].as_u64().unwrap(),p["generic_additions_saved_closed_form"].as_u64().unwrap());}
    #[test]fn signed_zero_extreme_skew_and_odd_top_oracles(){for n in [0,1,2,3,7,8,9,63,257,1023,1024,1025,2049]{for d in [(0,0),(0,64),(-64,0),(-64,64),(1,-1)]{check(vec![d;n]);}check((0..n).map(|i|match i%7{0=>(0,0),1=>(1,-1),2=>(-64,64),3=>(0,-7),4=>(5,0),5=>(7,-3),_=>(-7,3)}).collect());}}
    #[test]fn dense_pair_classes_and_group_boundary(){for n in [255,256,257,511,512,513,16641]{check((0..n).map(|i|((i/129)as i16-64,(i%129)as i16-64)).collect());}}
    #[test]fn topology_additions_equal_tree_edges_and_independent_tiles(){for n in [0usize,1,2,7,8,9,31,32,33,1023,1024,1025,2048,2049,4097]{let mut t=Stats::default();t.indexed(&[n,0,1,3]);assert_eq!(t.additions,n.saturating_sub(1)+2);assert_eq!(8*t.lane_groups+t.scalar_pairs,t.additions);}let mut t=Stats::default();t.grouped(&[0,1,0,1]);assert_eq!(t.tree_layers,0);assert_eq!(t.additions,0);}
    #[test]fn inactive_original_points_and_empty_classes_preserve_bins(){let ds=[(64,-64),(3,4),(0,0)];let live=[false,true,true];let tokens=[Fr::ZERO,Fr::from(17),Fr::from(19)];oracle(&ds,&live,&tokens);let p=partition(&ds,&live);assert_eq!(p["occupied_pair_classes"],1);assert_eq!(p["generic_additions_saved_closed_form"],0);}
}
