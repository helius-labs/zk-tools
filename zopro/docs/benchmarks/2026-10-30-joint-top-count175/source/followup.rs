#![allow(dead_code)]
mod original {
    include!("probe.rs");
    fn coalesced(ds:&[Digit],active:&[bool])->Value{
        let(classes,marginal,both)=counts(ds,active);let u=classes.len();
        let mut cm=[vec![0;128],vec![0;128]];
        for &d in classes.keys(){for(c,v)in[d.0,d.1].into_iter().enumerate(){if let Some(b)=bin(v){cm[c][b]+=1;}}}
        for c in 0..2{for b in 0..128{assert_eq!(marginal[c][b]!=0,cm[c][b]!=0);}}
        let mut first=Stats::default();first.grouped(&classes.values().copied().collect::<Vec<_>>());
        let mut total=first.clone();for c in 0..2{total.grouped(&cm[c]);}
        let baseline=marginal.iter().flatten().map(|n|n.saturating_sub(1)).sum::<usize>();
        let occupied_both=classes.keys().filter(|d|d.0!=0&&d.1!=0).count();
        assert_eq!(baseline-total.additions,both-occupied_both);
        json!({"input_rows":ds.len(),"occupied_pair_classes":u,"first_stage":first.json(),"joint_both_coalesced":total.json(),
            "generic_additions_saved_closed_form":both-occupied_both,"generic_original_additions_closed_form":baseline,
            "coalesced_marginal_retained_payload_before_leaf_bytes":134*u+3080,
            "worst_coalesced_marginal_retained_payload_before_leaf_bytes":134*ds.len().min(K-1)+3080,
            "new_leaf_may_retain_U_original_derived_point_copies":true,"leaf_and_outputs_not_in_payload":true})
    }
    fn run_case(shape:&str,old:&Value)->Value{
        let root="/home/ubuntu/src/zopro-perf-work/g2-queue-scan175";let b=fs::read(format!("{root}/inputs/{shape}.b2q")).unwrap();
        assert_eq!(format!("{:x}",Sha256::digest(&b)),old["fixture_sha256"]);assert_eq!(&b[..8],b"B2SCAN01");
        let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(b.len(),16+160*n);assert_eq!(n,old["original_rows"]);
        let live=b[16..16+128*n].chunks_exact(128).map(|raw|{let f=raw.chunks_exact(32).map(|v|Fq::from_be_bytes(v.try_into().unwrap()).unwrap()).collect::<Vec<_>>();let p=G2Affine::new(Fq2::new(f[1],f[0]),Fq2::new(f[3],f[2]));assert!(p.is_on_curve());!p.is_identity()}).collect::<Vec<_>>();
        let words=b[16+128*n..].chunks_exact(32).map(|v|core::array::from_fn(|i|u64::from_le_bytes(v[i*8..i*8+8].try_into().unwrap()))).collect::<Vec<Word>>();drop(b);
        let g=groups(&words);assert_eq!(g.is_some(),old["grouping_admitted"]);
        let selected=g.as_ref().map_or_else(||words.clone(),|x|x.0.clone());assert_eq!(selected.len(),old["post_group_rows"]);
        let active=if let Some((_,map))=&g{let mut a=vec![false;selected.len()];for(&i,&v)in map.iter().zip(&live){if i!=usize::MAX{a[i]|=v;}}a}else{live.clone()};
        let ds=selected.iter().map(digits).collect::<Vec<_>>();let chunk=ds.len().div_ceil(2);
        let parts=(0..2).map(|p|{let first=(p*chunk).min(ds.len());let last=(first+chunk).min(ds.len());let mut v=coalesced(&ds[first..last],&active[first..last]);
            let prev=&old["parts"][p];assert_eq!(v["occupied_pair_classes"],prev["occupied_pair_classes"]);assert_eq!(v["joint_both_coalesced"]["generic_additions"],prev["joint"]["generic_additions"]);
            assert_eq!(v["generic_original_additions_closed_form"],prev["original"]["generic_additions"]);v["part"]=json!(p);v}).collect::<Vec<_>>();
        for tag in 0..2{let mut seed=0x636f616c65736365u64^tag;let raw=live.iter().map(|&v|{let t=token(&mut seed);if v{t}else{Fr::ZERO}}).collect::<Vec<_>>();
            let tokens=if let Some((_,map))=&g{let mut t=vec![Fr::ZERO;selected.len()];for(&i,&v)in map.iter().zip(&raw){if i!=usize::MAX{t[i]+=v;}}t}else{raw.clone()};
            let lambda=split::phi_lambda();let target=words.iter().zip(&raw).map(|(w,&t)|{let d=digits(w);(signed(d.0 as i128)+lambda*signed(d.1 as i128))*t}).fold(Fr::ZERO,|a,b|a+b)*Fr::from(2u64).pow(&[120]);
            let got=(0..2).map(|p|{let first=(p*chunk).min(ds.len());let last=(first+chunk).min(ds.len());oracle(&ds[first..last],&active[first..last],&tokens[first..last])}).fold(Fr::ZERO,|a,b|a+b);assert_eq!(got,target);
        }
        let sum=|k:&str|parts.iter().map(|p|p["joint_both_coalesced"][k].as_u64().unwrap()).sum::<u64>();
        let after=sum("generic_additions");let layers=sum("tree_layer_calls");let before=old["generic_top_additions_before"].as_u64().unwrap();let old_layers=old["tree_layers_before"].as_u64().unwrap();
        json!({"shape":shape,"parts":parts,"generic_top_additions_before_inherited":before,"generic_top_additions_after":after,"generic_top_additions_saved":before-after,
            "generic_top_addition_fraction_saved":(before-after)as f64/before as f64,"tree_layers_before_inherited":old_layers,"tree_layers_after":layers,
            "prefix_layers_after":sum("prefix_selected_layers_m_ge32"),"scalar_tail_pairs_after":sum("scalar_tail_pairs"),
            "original_count_gate":old["necessary_gate"],"followup_necessary_gate":after<before&&layers<=old_layers,
            "same_joint_classes_as_original_count":true,"two_token_oracles":true,"new_performance_claim":false})
    }
    pub fn run(){
        let root="/home/ubuntu/src/zopro-perf-work/joint-top-count175";let old=fs::read_to_string(format!("{root}/runs/count-two-fixtures/stdout.log")).unwrap();
        let previous=old.lines().filter_map(|l|l.strip_prefix("ROW ")).map(|s|serde_json::from_str::<Value>(s).unwrap()).collect::<Vec<_>>();assert_eq!(previous.len(),2);
        let rows=["1x2","2x2"].iter().zip(&previous).map(|(shape,old)|{assert_eq!(old["shape"],*shape);run_case(shape,old)}).collect::<Vec<_>>();for row in &rows{println!("ROW {}",row);}
        println!("COMPLETE {}",json!({"original_gate":false,"followup_necessary_gate":rows.iter().all(|r|r["followup_necessary_gate"]==true),"production_changed":false,"performance_claim":false,"new_solver_calls":0,"SDK_requests":0,"new_independent_exports":0,"proof_ledger":25231,"goal_achieved":false}));
    }
    #[cfg(test)]mod followup_tests{
        use super::*;
        #[test]fn coalesced_marginals_signed_sparse_dense_and_skewed_oracles(){
            for n in[0,1,3,257,1025,16641]{let ds=(0..n).map(|i|match i%3{0=>(0,0),1=>(-64,64),_=>((i/129)as i16-64,(i%129)as i16-64)}).collect::<Vec<_>>();let active=vec![true;n];let p=coalesced(&ds,&active);let mut seed=1597;let tokens=(0..n).map(|_|token(&mut seed)).collect::<Vec<_>>();oracle(&ds,&active,&tokens);let original=marginal_original(&ds,&active);assert_eq!(p["generic_original_additions_closed_form"],original);}
        }
        fn marginal_original(ds:&[Digit],active:&[bool])->usize{let(_,m,_)=counts(ds,active);m.iter().flatten().map(|n|n.saturating_sub(1)).sum()}
        #[test]fn revised_layer_geometry_at_group_and_tile_boundaries(){
            for n in[1,2,255,256,257,1023,1024,1025,2049]{let ds=vec![(7,-9);n];let p=coalesced(&ds,&vec![true;n]);let c=&p["joint_both_coalesced"];assert_eq!(c["generic_additions"],n-1);assert_eq!(c["tree_layer_calls"],if n<=1{0}else{usize::BITS as usize-(n-1).leading_zeros()as usize});assert_eq!(c["SIMD_lane_groups"].as_u64().unwrap()*8+c["scalar_tail_pairs"].as_u64().unwrap(),(n-1)as u64);}
        }
    }
}
fn main(){original::run();}
