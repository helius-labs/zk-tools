use ark_ec::{AdditiveGroup,AffineRepr,CurveGroup,PrimeGroup,short_weierstrass::{Affine,Projective,SWCurveConfig}};
use ark_ff::{Field,PrimeField,Zero,BigInt};
use serde_json::{json,Value};
use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/weighted-sumdiff176";

#[derive(Default,Clone,Copy,Debug)]
struct Counts{ordinary:usize,shared:usize,identity:usize,equal:usize,opposite:usize,projective_adds:usize,doubles:usize}
impl Counts{
 fn value(&self)->Value{json!({"ordinary":self.ordinary,"shared_pairs":self.shared,"identities":self.identity,"equal":self.equal,"opposite":self.opposite,"projective_adds":self.projective_adds,"doubles":self.doubles})}
}
fn classify<C:SWCurveConfig>(a:&Affine<C>,b:&Affine<C>,c:&mut Counts)->bool{
 if a.is_zero()||b.is_zero(){c.identity+=1;false}
 else if a.x==b.x {if a.y==b.y{c.equal+=1;}else{c.opposite+=1;}false}
 else{true}
}
fn ordinary<C:SWCurveConfig>(a:Affine<C>,b:Affine<C>,c:&mut Counts)->Affine<C>{
 if classify(&a,&b,c){c.ordinary+=1;}
 (a.into_group()+b).into_affine()
}
fn pair<C:SWCurveConfig>(a:Affine<C>,b:Affine<C>,c:&mut Counts)->(Affine<C>,Affine<C>){
 if !classify(&a,&b,c){return((a.into_group()+b).into_affine(),(a.into_group()-b).into_affine());}
 c.shared+=1;
 let inv=(b.x-a.x).inverse().unwrap();let p=(b.y-a.y)*inv;let m=(-b.y-a.y)*inv;let sum=a.x+b.x;
 let px=p.square()-sum;let mx=m.square()-sum;
 (Affine::new_unchecked(px,p*(a.x-px)-a.y),Affine::new_unchecked(mx,m*(a.x-mx)-a.y))
}
// Source schedule of accepted msm.rs::weighted_sum, with complete group
// operations in place of the native packed layer. No candidate recurrence.
fn accepted<C:SWCurveConfig>(input:&[Affine<C>],c:&mut Counts)->(Projective<C>,Projective<C>){
 let n=input.len();assert!(n.is_power_of_two());let mut points=vec![Affine::<C>::zero();2*n-1];points[..n].copy_from_slice(input);
 let mut starts=vec![0usize];let mut counts=vec![n];let mut next=n;
 while counts[0]>1{
  let half=counts[0]/2;for i in 0..half{points[next+i]=points[2*i];}
  for j in 0..starts.len(){let start=starts[j];let count=counts[j];for i in 0..count/2{points[start+i]=ordinary(points[start+2*i],points[start+2*i+1],c);}counts[j]=count.div_ceil(2);}
  starts.push(next);counts.push(half);next+=half;
 }
 let total=points[0].into_group();let mut sum=total;
 for start in starts[1..].iter().rev(){sum=sum.double();c.doubles+=1;sum-=points[*start];c.projective_adds+=1;}
 (sum,total)
}
fn sum_in_place<C:SWCurveConfig>(points:&mut[Affine<C>],c:&mut Counts)->Projective<C>{
 let mut count=points.len();while count>1{for i in 0..count/2{points[i]=ordinary(points[2*i],points[2*i+1],c);}if count%2!=0{points[count/2]=points[count-1];}count=count.div_ceil(2);}points[0].into_group()
}
fn candidate<C:SWCurveConfig>(points:&mut[Affine<C>],c:&mut Counts)->(Projective<C>,Projective<C>){
 let n=points.len();assert!(n.is_power_of_two());if n==1{let p=points[0].into_group();return(p,p);}
 if n==2{let total=ordinary(points[0],points[1],c).into_group();c.doubles+=1;c.projective_adds+=1;return(total.double()-points[0],total);}
 let h=n/2;let(a,b)=points.split_at_mut(h);for i in 0..h{(a[i],b[i])=pair(a[i],b[i],c);}
 let d=sum_in_place(b,c);let(w,total)=candidate(a,c);let mut correction=total-d;c.projective_adds+=1;
 for _ in 0..(h/2).ilog2(){correction=correction.double();c.doubles+=1;}
 c.projective_adds+=1;(w+correction,total)
}
fn weights()->Vec<ark_bn254::Fr>{
 let mut seed=0x1c7a04fdb639825eu64;(0..256).map(|_|loop{let words=core::array::from_fn(|_|{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed});if let Some(s)=ark_bn254::Fr::from_bigint(BigInt(words)){if !s.is_zero(){break s;}}}).collect()
}
fn curve<C:SWCurveConfig<ScalarField=ark_bn254::Fr>>(label:&str)->(Vec<Value>,usize,usize){
 let g=Projective::<C>::generator();let weights=weights();let points=Projective::<C>::normalize_batch(&weights.iter().map(|s|g*s).collect::<Vec<_>>());
 let mut pair_checks=0;let mut pair_counts=Counts::default();
 let mut sample=points[..29].to_vec();sample.extend([Affine::<C>::zero(),points[0],-points[0]]);
 for a in &sample{for b in &sample{let(p,m)=pair(*a,*b,&mut pair_counts);assert_eq!(p,(a.into_group()+b).into_affine());assert_eq!(m,(a.into_group()-b).into_affine());pair_checks+=2;}}
 assert_eq!(pair_checks,2048);assert!(pair_counts.ordinary==0&&pair_counts.shared>0&&pair_counts.identity>0&&pair_counts.equal>0&&pair_counts.opposite>0);
 let mut rows=Vec::new();let mut checks=0;
 for n in[1usize,2,4,8,16,32,64,128,256]{for pattern in["dense","identities","equal","opposites","half-equal","half-opposite"]{
  let ws=(0..n).map(|i|match pattern{
   "dense"=>weights[i],"identities"=>if i%3==0{ark_bn254::Fr::zero()}else{weights[i]},"equal"=>weights[0],"opposites"=>if i%2==0{weights[0]}else{-weights[0]},
   "half-equal"=>weights[i%(n/2).max(1)],"half-opposite"=>if i<n/2{weights[i]}else{-weights[i.saturating_sub(n/2)]},_=>unreachable!()}).collect::<Vec<_>>();
  let input=Projective::<C>::normalize_batch(&ws.iter().map(|s|g*s).collect::<Vec<_>>());let original=input.clone();let mut scratch=input.clone();let ptr=scratch.as_ptr();let capacity=scratch.capacity();
  let mut off=Counts::default();let mut on=Counts::default();let(ow,ot)=accepted(&input,&mut off);let(nw,nt)=candidate(&mut scratch,&mut on);
  let total:ark_bn254::Fr=ws.iter().copied().sum();let weighted:ark_bn254::Fr=ws.iter().enumerate().map(|(i,w)|*w*ark_bn254::Fr::from(i as u64+1)).sum();let wantw=g*weighted;let wantt=g*total;
  assert_eq!(ow,wantw,"{label}/{n}/{pattern}/oldW");assert_eq!(nw,wantw,"{label}/{n}/{pattern}/newW");assert_eq!(ot,wantt);assert_eq!(nt,wantt);assert_eq!(input,original);assert_eq!(scratch.as_ptr(),ptr);assert_eq!(scratch.capacity(),capacity);assert_eq!(scratch.len(),n);checks+=4;
  rows.push(json!({"curve":label,"n":n,"pattern":pattern,"accepted":off.value(),"candidate":on.value(),"weighted_and_total_oracles_passed":true,"input_immutable":true,"candidate_point_slice_reused":true}));
 }}
 (rows,checks,pair_checks)
}
fn topology(n:usize)->(usize,usize,usize,usize,usize,usize,usize){
 let levels=n.ilog2()as usize;let old=2*n-levels-2;let mut pairs=0;let mut ordinary=0;let mut adds=1;let mut doubles=1;let mut size=n;
 while size>2{let h=size/2;pairs+=h;ordinary+=h-1;adds+=2;doubles+=(h/2).ilog2()as usize;size=h;}
 ordinary+=1;(old,levels,levels,pairs,ordinary,adds,doubles)
}
fn main(){
 assert!(!Path::new(&format!("{R}/result.json")).exists());let(g1,a,b)=curve::<ark_bn254::g1::Config>("G1");let(g2,c,d)=curve::<ark_bn254::g2::Config>("G2");let mut rows=g1;rows.extend(g2);assert_eq!(rows.len(),108);assert_eq!(a+c,432);assert_eq!(b+d,4096);
 let mut work=Vec::new();for n in[32usize,64,128,256]{let(old,old_add,old_double,pairs,ordinary,new_add,new_double)=topology(n);let row=rows.iter().find(|r|r["curve"]=="G1"&&r["pattern"]=="dense"&&r["n"]==n).unwrap();
  assert_eq!(row["accepted"]["ordinary"],old);assert_eq!(row["accepted"]["projective_adds"],old_add);assert_eq!(row["accepted"]["doubles"],old_double);assert_eq!(row["candidate"]["shared_pairs"],pairs);assert_eq!(row["candidate"]["ordinary"],ordinary);assert_eq!(row["candidate"]["projective_adds"],new_add);assert_eq!(row["candidate"]["doubles"],new_double);
  let old_m=5*old;let old_s=old;let new_m=7*pairs+5*ordinary;let new_s=2*pairs+ordinary;let pass=new_m+new_s<old_m+old_s&&new_add<=old_add&&new_double<=old_double;
  work.push(json!({"n":n,"accepted_affine_adds":old,"candidate_shared_pairs":pairs,"candidate_other_affine_adds":ordinary,"accepted_M":old_m,"accepted_S":old_s,"candidate_M":new_m,"candidate_S":new_s,"candidate_over_accepted_variable_MS":(new_m+new_s)as f64/(old_m+old_s)as f64,"accepted_projective_adds":old_add,"candidate_projective_adds":new_add,"accepted_doubles":old_double,"candidate_doubles":new_double,"necessary_gate_passed":pass,"G1_optimistic_amortized_work_only":true}));
 }
 assert!(work.iter().all(|v|v["necessary_gate_passed"]==false));
 let result=json!({"mathematical_checks_passed":true,"curve_cases":108,"weighted_total_output_checks":432,"paired_output_checks":4096,"all_output_comparisons":4528,"input_points_per_curve":256,"input_population_reused_across_sizes_patterns":true,"candidate_slice_stable":true,"necessary_work_gate_passed":false,"failed_work_cases":4,"dense_G1_variable_work":work,"rows":rows,"native_SIMD_qualified":false,"speed_measured":false,"adopted":false,"production_unchanged":true,"SDK_requests":0,"new_Gnark_exports":0,"canonical_proofs":43073,"goal_active":true,"goal_complete":false});fs::write(format!("{R}/result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("SHARED_SUMDIFF_MODEL_PASS108cases/4528outputchecks;necessaryworkgateFAIL4of4;nativeandtimingnotselected;canonical43073");
}
