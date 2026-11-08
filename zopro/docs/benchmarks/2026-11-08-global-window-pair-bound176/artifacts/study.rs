use serde_json::{json,Value};
use std::{collections::BTreeMap,fs,path::Path};
use zopro_bn254::Fr;
#[path="/home/ubuntu/src/zopro-perf-work/tau9-ring-work176/split.rs"] mod split;
const R:&str="/home/ubuntu/src/zopro-perf-work/global-window-pair-bound176";
const LAMBDA:Fr=Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
type Codes=[i16;22];
fn field(x:i128)->Fr{let n=x.unsigned_abs();let f=Fr::from_canonical([n as u64,(n>>64)as u64,0,0]).unwrap();if x<0{-f}else{f}}
fn codes(w:[u64;4])->Codes{
 let s=split::phi_split(w);let pair: [i128;2]=std::array::from_fn(|c|{let n=(s[2*c]as u128)|((s[2*c+1]&0x7fff_ffff_ffff_ffff)as u128)<<64;let x=i128::try_from(n).unwrap();if s[2*c+1]>>63!=0{-x}else{x}});
 let out=std::array::from_fn(|j|{let c=j/11;let n=pair[c].unsigned_abs();let d=split::booth(&[n as u64,(n>>64)as u64,0,0],j%11,12);if pair[c]<0{-d}else{d}});
 for c in 0..2{let n=(0..11).rev().fold(0i128,|v,j|v*4096+out[11*c+j]as i128);assert_eq!(n,pair[c]);}
 assert_eq!(field(pair[0])+LAMBDA*field(pair[1]),Fr::from_canonical(w).unwrap());out
}
fn key(a:i16,b:i16)->Option<(i16,i16)>{if a==0||b==0{None}else if a<0{Some((-a,-b))}else{Some((a,b))}}
fn bound<const W:usize>(rows:&[[i16;W]])->(u64,Vec<Value>,usize){
 let mut total=0u64;let mut details=vec![];let mut peak_capacity=0;
 for a in 0..W{for b in a+1..W{
  let mut keys=rows.iter().filter_map(|v|key(v[a],v[b])).collect::<Vec<_>>();peak_capacity=peak_capacity.max(keys.capacity()*std::mem::size_of::<(i16,i16)>());keys.sort_unstable();
  let mut classes=0u64;let mut repeated=0u64;let mut tickets=0u64;let mut maximum=0;let mut i=0;
  while i<keys.len(){let mut j=i+1;while j<keys.len()&&keys[j]==keys[i]{j+=1;}let n=j-i;classes+=1;repeated+=u64::from(n>1);tickets+=(n-1)as u64;maximum=maximum.max(n);i=j;}
  total+=tickets;details.push(json!({"a":a,"b":b,"eligible_rows":keys.len(),"classes":classes,"repeated_classes":repeated,"maximum_population":maximum,"optimistic_tickets":tickets}));
 }}(total,details,peak_capacity)
}
fn weights()->[Fr;22]{std::array::from_fn(|j|{let mut v=if j<11{Fr::ONE}else{LAMBDA};for _ in 0..j%11{v*=Fr::from(4096);}v})}
fn token(i:usize,salt:u64)->Fr{Fr::from((i as u64+1).wrapping_mul(7919+salt*10007).wrapping_add(37+salt))}
fn greedy(rows:&[Codes])->Value{
 let mut used=vec![false;rows.len()];let mut removed=vec![0u32;rows.len()];let mut merges=vec![];let mut saved=0u64;let mut max_matched=0usize;let mut positive=0usize;let mut negative=0usize;
 // Fixed lexicographic window-pair order; each point participates in at most
 // one newly formed two-point sum. Do not optimize this order after results.
 for a in 0..22{for b in a+1..22{let mut groups=BTreeMap::<(i16,i16),Vec<usize>>::new();for(i,v)in rows.iter().enumerate(){if !used[i]{if let Some(k)=key(v[a],v[b]){groups.entry(k).or_default().push(i);}}}
  for list in groups.values(){for pair in list.chunks_exact(2){let(i,j)=(pair[0],pair[1]);assert!(!used[i]&&!used[j]);let sign=if rows[i][a]==rows[j][a]{1i16}else{-1};let matched=(0..22).filter(|&w|rows[i][w]!=0&&rows[i][w]==sign*rows[j][w]).collect::<Vec<_>>();assert!(matched.contains(&a)&&matched.contains(&b));assert!(matched.len()>=2);for &w in &matched{removed[i]|=1<<w;removed[j]|=1<<w;}saved+=(matched.len()-1)as u64;max_matched=max_matched.max(matched.len());if sign>0{positive+=1}else{negative+=1}used[i]=true;used[j]=true;merges.push((i,j,sign,matched));}}
 }}
 let w=weights();for salt in 0..2{let before=rows.iter().enumerate().fold(Fr::ZERO,|acc,(i,ds)|acc+token(i,salt)*ds.iter().zip(w).fold(Fr::ZERO,|v,(&d,w)|v+field(d as i128)*w));let mut after=Fr::ZERO;
  for(i,ds)in rows.iter().enumerate(){for col in 0..22{if removed[i]&(1<<col)==0{after+=token(i,salt)*field(ds[col]as i128)*w[col];}}}
  for(i,j,sign,cols)in &merges{let p=token(*i,salt)+field(*sign as i128)*token(*j,salt);for &col in cols{after+=p*field(rows[*i][col]as i128)*w[col];}}
  assert_eq!(before,after,"partial-digit linear conservation");
 }
 json!({"disjoint_two_point_formations":merges.len(),"positive_formations":positive,"negative_formations":negative,"maximum_common_windows":max_matched,"net_arrival_minus_formation_savings":saved,"independent_weighted_Fr_checks":2,"point_arithmetic_executed":false})
}
fn main(){assert!(!Path::new(&format!("{R}/analysis.json")).exists());let inputs:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/tau9-ring-work176/inputs.json").unwrap()).unwrap();let mut cases=vec![];
 for(name,input)in inputs.as_object().unwrap(){let bytes=fs::read(input["file"].as_str().unwrap()).unwrap();assert_eq!(&bytes[..8],b"B2SCAN01");let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;assert_eq!(bytes.len(),16+160*n);
  let mut multiplicity=BTreeMap::<[u64;4],usize>::new();let mut raw=vec![];let mut zeros=0;let mut identities=0;
  for i in 0..n{if bytes[16+128*i..16+128*(i+1)].iter().all(|&v|v==0){identities+=1;continue;}let words=std::array::from_fn(|j|u64::from_le_bytes(bytes[16+128*n+32*i+8*j..16+128*n+32*i+8*j+8].try_into().unwrap()));let scalar=Fr::from_canonical(words).unwrap();if scalar==Fr::ZERO{zeros+=1;continue;}raw.push(codes(words));*multiplicity.entry(words).or_default()+=1;}
  let unique=multiplicity.keys().copied().map(codes).collect::<Vec<_>>();let raw_arrivals=raw.iter().flatten().filter(|&&d|d!=0).count()as u64;let unique_arrivals=unique.iter().flatten().filter(|&&d|d!=0).count()as u64;let formation=(raw.len()-unique.len())as u64;
  let full_saving=raw_arrivals-unique_arrivals-formation;let(u,detail,peak)=bound(&unique);let g=greedy(&unique);assert!(g["net_arrival_minus_formation_savings"].as_u64().unwrap()<=u);
  let upper=full_saving+u;let combined=full_saving+g["net_arrival_minus_formation_savings"].as_u64().unwrap();
  cases.push(json!({"case":name,"raw_rows":n,"nonidentity_nonzero_rows":raw.len(),"zero_scalar_rows":zeros,"identity_rows":identities,"exact_scalar_classes":unique.len(),"raw_nonzero_digit_arrivals":raw_arrivals,"after_exact_scalar_grouping_arrivals":unique_arrivals,"exact_scalar_point_formations":formation,"exact_group_arrival_minus_formation_savings":full_saving,"unique_class_global_pair_ticket_upper_bound":u,"combined_pipeline_arrival_savings_upper_bound":upper,"combined_pipeline_upper_fraction_of_raw_arrivals":upper as f64/raw_arrivals as f64,"fixed_greedy":g,"combined_pipeline_greedy_arrival_savings":combined,"window_pairs_checked":detail.len(),"window_pair_rows":detail,"largest_pair_key_vector_capacity_payload_bytes":peak,"production_grouping_policy_replayed":false,"raw_2x2_not_grouped_runtime":name=="2x2-B"}));
 }
 fs::write(format!("{R}/analysis.json"),serde_json::to_vec_pretty(&json!({"complete":true,"cases":cases,"one_generation_bound_only":true,"recursive_or_reencoded_reuse_not_bounded":true,"new_timings":0,"new_profiles":0,"new_proofs":0,"canonical_proofs_unchanged":37023,"production_changed":false,"native_selected":false,"adopted":false,"goal_complete":false})).unwrap()).unwrap();println!("Global all-window-pair screen complete, {} fixtures; no native clocks/proofs",cases.len());
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn exhaustive_three_window_four_point_matching_bound(){
  for mut code in 0..3usize.pow(12){let mut rows=[[0i16;3];4];for row in &mut rows{for d in row{*d=(code%3)as i16-1;code/=3;}}
   let(u,_,_)=bound(&rows);let gain=|a:usize,b:usize|[1i16,-1].into_iter().map(|s|{let k=(0..3).filter(|&j|rows[a][j]!=0&&rows[a][j]==s*rows[b][j]).count();k.saturating_sub(1)as u64}).max().unwrap();
   let mut optimum=0;for a in 0..4{for b in a+1..4{optimum=optimum.max(gain(a,b));let rest=(0..4).filter(|&v|v!=a&&v!=b).collect::<Vec<_>>();optimum=optimum.max(gain(a,b)+gain(rest[0],rest[1]));}}assert!(optimum<=u);
  }
 }
}
