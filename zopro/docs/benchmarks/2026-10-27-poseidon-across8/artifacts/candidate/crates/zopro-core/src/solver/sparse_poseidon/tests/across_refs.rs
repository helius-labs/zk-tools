use super::*;
use super::super::component::{Probe,forms_width};
#[test]fn complete_combined_layout_matches_original_fixture(){
 let(n,forms)=forms_width(3);
 for reverse in [false,true]{let original=fixture(3,n,&forms,reverse);for materialize in [false,true]{let mut p=Probe::new(1,reverse,materialize,false);p.run(2).unwrap();let inputs=p.plan.n_inputs;let stride=3+3*n;
  for lane in 0..8{let expected=execute(&original,None,lane,materialize);assert!(expected.0.is_ok());let raw=&expected.1;let private=inputs+lane*stride;
   for i in 0..3{assert_eq!(p.w[1+3*lane+i],raw[0][1+i]);assert_eq!(p.w[private+i],raw[0][1+i]);}
   for i in 0..3*n{assert_eq!(p.w[private+3+i],raw[0][4+i]);assert_eq!(p.a[lane*stride+3+i],raw[1][i]);assert_eq!(p.b[lane*stride+3+i],raw[2][i]);if materialize{assert_eq!(p.c[lane*stride+3+i],raw[3][i]);}}
   for i in 0..n{assert_eq!(p.w[p.plan.n_wires+lane*n+i],raw[0][original.n_wires+i]);}
  }
 }}
}
#[test]fn eight_lanes_all_wires_rows_prefixes_and_ragged_chains(){
 for groups in [1,16]{for reverse in [false,true]{for materialize in [false,true]{for ragged in [false,true]{let mut p=Probe::new(groups,reverse,materialize,ragged);let seeds=if groups==1{32}else{2};for seed in 0..seeds{p.reset(seed);p.run(2).unwrap();let expected=p.snapshot();p.reset(seed);p.run(0).unwrap();assert_eq!(p.snapshot(),expected);p.reset(seed);p.run(1).unwrap();assert_eq!(p.snapshot(),expected,"groups={groups} reverse={reverse} C={materialize} ragged={ragged} seed={seed}");}}}}}
}
#[test]fn untagged_and_unsupported_width_fall_back(){
 for materialize in [false,true]{for t in [3,4,13]{let mut p=Probe::with_width(1,true,materialize,true,t);p.reset(19);p.run(2).unwrap();let expected=p.snapshot();p.reset(19);p.run(1).unwrap();assert_eq!(p.snapshot(),expected,"width{t}");
  for code in &mut p.tasks[0]{*code&=INDEX;}p.reset(19);p.run(1).unwrap();assert_eq!(p.snapshot(),expected,"untagged width{t}");
 }}
}
