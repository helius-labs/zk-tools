//! Diagnostic counts only; no scalar or point is changed or exported.
use std::collections::BTreeMap;

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Class { count:usize, chunks:usize, first:usize, last:usize, category:usize }
#[derive(Debug,PartialEq,Eq)]
struct Census { n:usize, zeros:usize, classes:Vec<Class>, per_chunk:Vec<usize>, saves:Vec<usize> }
fn census(words:&[[u64;4]],chunk:usize)->Census {
 assert!(chunk>0);let mut classes=Vec::<Class>::new();let mut index=BTreeMap::new();let mut zeros=0;let mut per_chunk=vec![0;words.len().div_ceil(chunk)];
 for(i,&s)in words.iter().enumerate(){if s==[0;4]{zeros+=1;continue;}let c=i/chunk;let next=classes.len();let k=*index.entry(s).or_insert_with(||{classes.push(Class{first:c,last:c,category:if s==[1,0,0,0]{0}else if s[1..]==[0,0,0]{1}else{2},..Class::default()});next});let v=&mut classes[k];v.count+=1;if v.chunks==0||v.last!=c{v.chunks+=1;per_chunk[c]+=1;}v.last=c;}
 let mut extras=classes.iter().map(|c|c.chunks-1).collect::<Vec<_>>();extras.sort_unstable_by(|a,b|b.cmp(a));let saves=[8,16,32].map(|n|extras.iter().take(n).sum()).to_vec();Census{n:words.len(),zeros,classes,per_chunk,saves}
}

pub(crate) fn observe(name:&str,words:&[[u64;4]]) {
 let c=census(words,1024);let mut hist=BTreeMap::<(usize,usize,usize,usize),usize>::new();for v in&c.classes{*hist.entry((v.count,v.chunks,v.last-v.first+1,v.category)).or_default()+=1;}
 let rows=hist.into_iter().map(|((count,chunks,span,category),classes)|vec![count,chunks,span,category,classes]).collect::<Vec<_>>();
 let global=super::scalar_groups(words);let admitted=global.as_ref().map(|g|g.values.len());drop(global);
 let local=words.chunks(1024).map(|w|{let g=super::scalar_groups(w);g.as_ref().map_or(-1i64,|g|g.values.len()as i64)}).collect::<Vec<_>>();
 eprintln!("CHUNK_GROUP {{\"query\":\"{}\",\"n\":{},\"zeros\":{},\"distinct_nonzero\":{},\"local_distinct_sum\":{},\"per_chunk_distinct\":{:?},\"save_class_instances_top8_16_32\":{:?},\"global_admitted_values\":{},\"chunk_admitted_values_or_minus1\":{:?},\"class_histogram_count_chunks_span_category_classes\":{:?}}}",name,c.n,c.zeros,c.classes.len(),c.per_chunk.iter().sum::<usize>(),c.per_chunk,c.saves,admitted.map_or("null".to_owned(),|n|n.to_string()),local,rows);
}

#[cfg(test)] mod tests {
 use super::*;
 fn independent(w:&[[u64;4]],chunk:usize)->(usize,Vec<(usize,usize,usize,usize)>,Vec<usize>){let mut seen=Vec::new();let mut rows=Vec::new();for s in w{if *s==[0;4]||seen.contains(s){continue;}seen.push(*s);let positions=w.iter().enumerate().filter(|(_,v)|*v==s).map(|(i,_)|i).collect::<Vec<_>>();let mut chunks=positions.iter().map(|i|i/chunk).collect::<Vec<_>>();chunks.dedup();rows.push((positions.len(),chunks.len(),chunks[0],*chunks.last().unwrap()));}let distinct=(0..w.len().div_ceil(chunk)).map(|c|{let mut seen=Vec::new();for v in &w[c*chunk..((c+1)*chunk).min(w.len())]{if *v!=[0;4]&&!seen.contains(v){seen.push(*v);}}seen.len()}).collect();(w.iter().filter(|s|**s==[0;4]).count(),rows,distinct)}
 #[test]fn empty_zero_and_partial_boundaries(){for n in[0,1,1023,1024,1025,2049]{let c=census(&vec![[0;4];n],1024);assert_eq!(c.zeros,n);assert!(c.classes.is_empty());assert_eq!(c.per_chunk,vec![0;n.div_ceil(1024)]);assert_eq!(c.saves,[0,0,0]);}}
 #[test]fn cross_chunk_classes_and_top_retention(){let mut w=vec![[1,0,0,0];4097];for i in(1..w.len()).step_by(3){w[i]=[7,0,0,0];}let c=census(&w,1024);assert_eq!(c.classes.len(),2);assert_eq!(c.per_chunk,[2,2,2,2,1]);assert_eq!(c.saves,[7,7,7]);assert_eq!(c.classes[0].category,0);assert_eq!(c.classes[1].category,1);}
 #[test]fn class_membership_matches_pairwise_equivalence_oracle(){for chunk in[1,3,16,1024]{let w=(0..1031).map(|i|match i%9{0=>[0;4],1=>[1,0,0,0],2|3=>[7,0,0,0],_=>[(i%53)as u64,(i%7)as u64,0,0]}).collect::<Vec<_>>();let c=census(&w,chunk);let(z,rows,counts)=independent(&w,chunk);assert_eq!(c.zeros,z);assert_eq!(c.per_chunk,counts);assert_eq!(c.classes.iter().map(|v|(v.count,v.chunks,v.first,v.last)).collect::<Vec<_>>(),rows);assert_eq!(c.classes.iter().map(|c|c.count).sum::<usize>()+z,w.len());assert_eq!(c.classes.iter().map(|c|c.chunks).sum::<usize>(),counts.iter().sum::<usize>());}}
}
