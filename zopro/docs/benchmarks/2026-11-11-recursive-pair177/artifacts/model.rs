use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
use zopro_bn254::Fr;
#[path="/home/ubuntu/src/zopro-perf-work/tau9-ring-work176/split.rs"] mod split;
const R: &str = "/home/ubuntu/src/zopro-perf-work/recursive-pair177";
const LAMBDA: Fr = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
fn hash(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn read(p: &str) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn field(x: i128) -> Fr { let m=x.unsigned_abs(); let v=Fr::from_canonical([m as u64,(m>>64)as u64,0,0]).unwrap(); if x<0 {-v} else {v} }
fn codes(w: [u64;4]) -> [i16;22] {
    let s=split::phi_split(w);
    let pair: [i128;2]=std::array::from_fn(|c| { let n=s[2*c]as u128|((s[2*c+1]&0x7fff_ffff_ffff_ffff)as u128)<<64; let n=i128::try_from(n).unwrap(); if s[2*c+1]>>63!=0 {-n} else {n} });
    let out=std::array::from_fn(|j| { let c=j/11; let m=pair[c].unsigned_abs(); let d=split::booth(&[m as u64,(m>>64)as u64,0,0],j%11,12); if pair[c]<0 {-d} else {d} });
    for c in 0..2 { assert_eq!((0..11).rev().fold(0i128, |v,j| v*4096+i128::from(out[11*c+j])), pair[c]); }
    assert_eq!(field(pair[0])+LAMBDA*field(pair[1]),Fr::from_canonical(w).unwrap()); out
}
fn key(a:i16,b:i16)->Option<(i16,i16)> { if a==0||b==0 {None} else if a<0 {Some((-a,-b))} else {Some((a,b))} }
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node<const W:usize> { digits:[i16;W], edge:Option<(usize,usize,i16)>, depth:usize }
fn arrivals<const W:usize>(nodes:&[Node<W>])->u64 { nodes.iter().flat_map(|n|n.digits).filter(|&d|d!=0).count() as u64 }
fn choose<const W:usize>(nodes:&[Node<W>],used:&[bool],a:usize,b:usize,naive:bool)->Vec<(usize,usize)> {
    if !naive {
        let mut groups=BTreeMap::<(i16,i16),Vec<usize>>::new();
        for (i,n) in nodes.iter().enumerate() { if !used[i] { if let Some(k)=key(n.digits[a],n.digits[b]) {groups.entry(k).or_default().push(i);} } }
        groups.values().flat_map(|g|g.chunks_exact(2).map(|p|(p[0],p[1]))).collect()
    } else {
        // Independent sorting/adjacent-run grouping for the small-input oracle.
        let mut records:Vec<_>=nodes.iter().enumerate().filter_map(|(i,n)| {
            if used[i] || n.digits[a]==0 || n.digits[b]==0 {return None}
            let s=n.digits[a].signum(); Some(((s*n.digits[a],s*n.digits[b]),i))
        }).collect();
        records.sort_unstable(); let mut pairs=Vec::new(); let mut i=0;
        while i<records.len() {let mut j=i+1;while j<records.len() && records[j].0==records[i].0 {j+=1;} for k in (i..j-1).step_by(2) {pairs.push((records[k].1,records[k+1].1));}i=j;} pairs
    }
}
fn compress<const W:usize>(rows:&[[i16;W]],naive:bool)->(Vec<Node<W>>,Vec<Value>) {
    let mut nodes:Vec<_>=rows.iter().map(|&digits|Node{digits,edge:None,depth:0}).collect();
    let initial=arrivals(&nodes); let mut rounds=Vec::new(); let mut total=0u64;
    loop {
        let n=nodes.len(); let before=arrivals(&nodes); let mut used=vec![false;n]; let mut created=Vec::new(); let mut matched=0u64; let mut recursive=0;
        for a in 0..W { for b in a+1..W {
            for (i,j) in choose(&nodes[..n],&used,a,b,naive) {
                assert!(!used[i]&&!used[j]); let sign=if nodes[i].digits[a]==nodes[j].digits[a] {1} else {-1};
                let mut digits=[0i16;W]; let mut count=0;
                for w in 0..W {if nodes[i].digits[w]!=0 && nodes[i].digits[w]==sign*nodes[j].digits[w] {digits[w]=nodes[i].digits[w];nodes[i].digits[w]=0;nodes[j].digits[w]=0;count+=1;}}
                assert!(count>=2); assert!(digits[a]!=0&&digits[b]!=0); matched+=count;
                recursive+=usize::from(i>=rows.len()||j>=rows.len());
                created.push(Node{digits,edge:Some((i,j,sign)),depth:1+nodes[i].depth.max(nodes[j].depth)});used[i]=true;used[j]=true;
            }
        }}
        let formations=created.len() as u64; total+=formations; nodes.extend(created);
        assert_eq!(before-arrivals(&nodes),matched); assert!(2*total<=initial);
        rounds.push(json!({"generation":rounds.len()+1,"nodes_before":n,"formations":formations,"recursive_formations":recursive,"removed_arrivals":matched,"net_saved":matched-formations,"arrivals_after":arrivals(&nodes)}));
        if formations==0 {break}
    }
    (nodes,rounds)
}
fn verify<const W:usize>(original:&[[i16;W]],nodes:&[Node<W>])->usize {
    // Reverse DAG propagation reconstructs every coefficient of every input.
    // Each operation is checked i64 arithmetic; no random fingerprint.
    for w in 0..W {
        let mut coeff:Vec<i64>=nodes.iter().map(|n|i64::from(n.digits[w])).collect();
        for i in (original.len()..nodes.len()).rev() {
            let (a,b,s)=nodes[i].edge.unwrap(); assert!(a<i&&b<i);
            let v=coeff[i]; coeff[a]=coeff[a].checked_add(v).unwrap();coeff[b]=coeff[b].checked_add(v.checked_mul(i64::from(s)).unwrap()).unwrap();
        }
        for i in 0..original.len() {assert_eq!(coeff[i],i64::from(original[i][w]),"coefficient {i}/{w}");}
    }
    // A fixed point has no pair of rows sharing two oriented nonzero columns.
    for a in 0..W {for b in a+1..W {let mut seen=std::collections::BTreeSet::new();for n in nodes {if let Some(k)=key(n.digits[a],n.digits[b]) {assert!(seen.insert(k));}}}}
    original.len()*W
}
fn main() {
    assert!(!std::path::Path::new(&format!("{R}/result.json")).exists());
    for (p,h) in read(&format!("{R}/pins.json")).as_object().unwrap() {assert_eq!(hash(&fs::read(p).unwrap()),h.as_str().unwrap());}
    let input=read("/home/ubuntu/src/zopro-perf-work/tau9-ring-work176/inputs.json");
    let prior=read("/home/ubuntu/src/zopro-perf-work/global-window-pair-bound176/analysis.json"); let mut cases=Vec::new();
    for (name,v) in input.as_object().unwrap() {
        let bytes=fs::read(v["file"].as_str().unwrap()).unwrap();assert_eq!(hash(&bytes),v["sha256"]);assert_eq!(&bytes[..8],b"B2SCAN01");let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;assert_eq!(bytes.len(),16+160*n);
        let mut classes=BTreeMap::<[u64;4],usize>::new();let mut raw_arrivals=0u64;let mut nonzero=0;
        for i in 0..n {if bytes[16+128*i..16+128*(i+1)].iter().all(|&b|b==0) {continue}
            let word=std::array::from_fn(|j|u64::from_le_bytes(bytes[16+128*n+32*i+8*j..16+128*n+32*i+8*j+8].try_into().unwrap()));
            if word==[0;4] {continue} let ds=codes(word);raw_arrivals+=ds.iter().filter(|&&d|d!=0).count()as u64;nonzero+=1;*classes.entry(word).or_default()+=1;
        }
        let rows:Vec<_>=classes.keys().copied().map(codes).collect();let initial=rows.iter().flatten().filter(|&&d|d!=0).count()as u64;
        let (nodes,rounds)=compress(&rows,false);let checks=verify(&rows,&nodes);let final_arrivals=arrivals(&nodes);let formations=nodes.len()-rows.len();let grouping_formations=nonzero-rows.len();let saved=initial-final_arrivals-formations as u64;
        let old=prior["cases"].as_array().unwrap().iter().find(|v|v["case"]==*name).unwrap();
        assert_eq!(raw_arrivals,old["raw_nonzero_digit_arrivals"]);assert_eq!(initial,old["after_exact_scalar_grouping_arrivals"]);
        assert_eq!(rounds[0]["formations"],old["fixed_greedy"]["disjoint_two_point_formations"]);assert_eq!(rounds[0]["net_saved"],old["fixed_greedy"]["net_arrival_minus_formation_savings"]);
        let kept=nodes.iter().filter(|n|n.digits.iter().any(|&d|d!=0)).count();
        let combined_saved=raw_arrivals-final_arrivals-formations as u64-grouping_formations as u64;
        cases.push(json!({"case":name,"fixture_sha256":v["sha256"],"input_rows":n,"nonzero_rows":nonzero,"classes":rows.len(),"raw_arrivals":raw_arrivals,"initial_class_arrivals":initial,"exact_group_formations":grouping_formations,"recursive_formations":formations,"final_arrivals":final_arrivals,"net_saved_beyond_exact_groups":saved,"fraction_beyond_exact_groups":saved as f64/initial as f64,"combined_net_saved":combined_saved,"combined_fraction_of_raw":combined_saved as f64/raw_arrivals as f64,"rounds":rounds,"coefficient_checks":checks,"retained_nonempty_rows":kept,"maximum_DAG_depth":nodes.iter().map(|n|n.depth).max().unwrap(),"all_nodes":nodes.len(),"model_node_capacity_bytes":nodes.capacity()*std::mem::size_of::<Node<22>>(),"hypothetical_new_G2_affine_bytes":formations*128,"first_generation_matches_prior":true,"no_eligible_pair_remains":true,"runtime_grouping_policy_replayed":false}));
    }
    let out=json!({"passed":true,"cases":cases,"scalar_model_only":true,"new_proofs":0,"proof_ledger_unchanged":53917,"native_selected":false});fs::write(format!("{R}/result.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("{}",out);
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn exhaustive_three_row_three_column_dags() {
        for mut code in 0..3usize.pow(9) {let mut rows=[[0i16;3];3];for row in &mut rows {for d in row {*d=(code%3)as i16-1;code/=3;}}
            let (a,x)=compress(&rows,false);let (b,y)=compress(&rows,true);assert_eq!(a,b);assert_eq!(x,y);assert_eq!(verify(&rows,&a),9);
        }
    }
    #[test] fn recursive_sum_and_signs_are_reused() {
        let rows=[[1,2,3,0],[1,2,0,4],[-1,-2,5,0],[-1,-2,0,6]];
        let (nodes,rounds)=compress(&rows,false);assert!(rounds.len()>=3);assert!(rounds[1]["recursive_formations"].as_u64().unwrap()>0);assert_eq!(verify(&rows,&nodes),16);
    }
}
