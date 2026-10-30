use serde_json::{json,Value};
use std::collections::{BTreeMap,BTreeSet};
use zopro_bn254::Fr;
pub type Word=[u64;4];
pub const TILE:usize=1024;
mod split {
    include!("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/split.rs");
    pub fn full_digit(s:&[u64;4],w:usize)->i16{booth_digit(s,w,12)}
}

pub fn scalar(w:Word)->Fr{Fr::from_canonical(w).expect("canonical scalar")}
fn signed(v:i16)->Fr{if v<0{-Fr::from(v.unsigned_abs()as u64)}else{Fr::from(v as u64)}}
#[derive(Clone,Copy)]pub enum Query{A,B}
impl Query{
    fn parts(self)->usize{match self{Self::A=>1,Self::B=>2}}
    fn columns(self)->usize{22}
    fn digits(self,w:Word)->Vec<i16>{
        let s=match self{Self::A=>unreachable!("full-scalar reader"),Self::B=>split::phi_split(w)};
        match self{Self::A=>unreachable!("full-scalar reader"),
            Self::B=>(0..11).flat_map(|j|[split::digit_of(&s,true,j,12),split::digit_of(&s,false,j,12)]).collect()}
    }
}
pub fn digits(q:Query,w:Word)->Vec<i16>{match q{Query::A=>(0..22).map(|j|split::full_digit(&w,j)).collect(),Query::B=>q.digits(w)}}
pub fn grouping_admitted(words:&[Word])->bool{
    let n=words.len();if n<256{return false}let sample=n.min(2048);let stride=n.div_ceil(sample);
    let set=words.iter().step_by(stride).take(sample).collect::<BTreeSet<_>>();let observed=n.div_ceil(stride).min(sample);
    if set.len()*200>observed*199{return false}let percent=if(32768..65536).contains(&n){95}else{80};
    let zeros=words.iter().step_by(stride).take(sample).filter(|x|**x==[0;4]).count();let nonzero=observed-zeros;
    if set.len()-usize::from(zeros!=0)==nonzero&&nonzero*100>observed*percent{return false}
    let all=words.iter().filter(|x|**x!=[0;4]).collect::<BTreeSet<_>>();all.len()*100<=n*percent
}
#[derive(Clone)]struct Entry{word:Word,rows:Vec<usize>}
fn coalesce(q:Query,words:&[Word],ids:&[bool])->(Vec<Vec<Entry>>,usize,usize){
    let part=words.len().div_ceil(q.parts());let mut result=vec![];let(mut formations,mut useful_classes)=(0,0);
    for p in 0..q.parts(){
        let first=p*part;let end=((p+1)*part).min(words.len());let mut entries=vec![];
        for start in(first..end).step_by(TILE){
            let stop=(start+TILE).min(end);let mut classes:Vec<Entry>=vec![];let mut map=BTreeMap::new();
            for i in start..stop{
                if words[i]==[0;4]||ids[i]{continue}
                let c=*map.entry(words[i]).or_insert_with(||{classes.push(Entry{word:words[i],rows:vec![]});classes.len()-1});classes[c].rows.push(i);
            }
            for e in classes{
                if e.rows.len()>1&&digits(q,e.word).iter().filter(|x|**x!=0).count()>1{
                    formations+=e.rows.len()-1;useful_classes+=1;entries.push(e);
                }else{for i in e.rows{entries.push(Entry{word:e.word,rows:vec![i]})}}
            }
        }
        result.push(entries);
    }
    (result,formations,useful_classes)
}
fn token(seed:&mut u64)->Fr{let mut bytes=[0;32];for b in bytes.chunks_exact_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;b.copy_from_slice(&seed.to_le_bytes());}Fr::from_le_bytes_mod_order(&bytes)}
pub fn census(q:Query,words:&[Word],ids:&[bool])->Value{
    assert_eq!(words.len(),ids.len());let admitted=grouping_admitted(words);
    if admitted{return json!({"original_rows":words.len(),"global_grouping_admitted":true,"bounded_path_selected":false,"reason":"retain original admitted grouping","new_grouping_pass_or_point_work":0})}
    let (candidate,formation,useful)=coalesce(q,words,ids);let part=words.len().div_ceil(q.parts());
    let lambda=split::phi_lambda();let mut stats=None;
    for tag in 0..2{
        let mut seed=0x636f616c65736365u64^tag;let tokens=(0..words.len()).map(|_|token(&mut seed)).collect::<Vec<_>>();
        let direct=words.iter().zip(ids).zip(&tokens).fold(Fr::ZERO,|v,((&w,&id),&t)|if id{v}else{v+scalar(w)*t});
        let mut before_total=Fr::ZERO;let mut after_total=Fr::ZERO;let mut rows=vec![];
        for p in 0..q.parts(){
            let first=p*part;let end=((p+1)*part).min(words.len());
            let mut row_seen=BTreeSet::new();let mut projected=Fr::ZERO;
            for e in &candidate[p]{
                let mut value=Fr::ZERO;
                for &i in &e.rows{assert!((first..end).contains(&i));assert_eq!((i-first)/TILE,(e.rows[0]-first)/TILE);assert!(row_seen.insert(i));assert_eq!(e.word,words[i]);assert!(!ids[i]&&words[i]!=[0;4]);value+=tokens[i];}
                projected+=scalar(e.word)*value;
            }
            let original=(first..end).filter(|&i|!ids[i]&&words[i]!=[0;4]).collect::<BTreeSet<_>>();assert_eq!(row_seen,original);
            let target=(first..end).filter(|&i|!ids[i]).fold(Fr::ZERO,|v,i|v+scalar(words[i])*tokens[i]);assert_eq!(projected,target);
            let mut prior_occupancy:Option<Vec<usize>>=None;
            for col in 0..q.columns(){
                let top=match q{Query::A=>col==21,Query::B=>col>=20};
                let bins=if top{match q{Query::A=>4,Query::B=>128}}else{2048};
                let index=|d:i16|{assert_ne!(d,0);if top{match q{Query::A=>{assert!(d>0&&d<=4);d as usize-1},Query::B=>{assert!(d.unsigned_abs()<=64);d.unsigned_abs()as usize-1+if d<0{64}else{0}}}}else{assert!(d.unsigned_abs()<=2048);d.unsigned_abs()as usize-1}};
                let mut before=vec![Fr::ZERO;bins];let mut after=before.clone();let mut bc=vec![0usize;bins];let mut ac=bc.clone();
                for i in first..end{if ids[i]{continue}let d=digits(q,words[i])[col];if d==0{continue}let b=index(d);bc[b]+=1;before[b]+=tokens[i]*signed(d.signum());}
                for e in &candidate[p]{let d=digits(q,e.word)[col];if d==0{continue}let b=index(d);ac[b]+=1;let t=e.rows.iter().fold(Fr::ZERO,|v,&i|v+tokens[i]);after[b]+=t*signed(d.signum());}
                assert_eq!(before,after);assert_eq!(bc.iter().map(|x|*x>0).collect::<Vec<_>>(),ac.iter().map(|x|*x>0).collect::<Vec<_>>());
                let weighted=|v:&[Fr]|v.iter().enumerate().fold(Fr::ZERO,|s,(i,&t)|s+t*Fr::from((if top&&matches!(q,Query::B){i%64+1}else{i+1})as u64));
                let factor=match q{Query::A=>Fr::from(2u64).pow(&[(12*col)as u64]),Query::B=>Fr::from(2u64).pow(&[(12*(col/2))as u64])*if col%2==0{lambda}else{Fr::ONE}};
                before_total+=weighted(&before)*factor;after_total+=weighted(&after)*factor;
                let additions=|c:&[usize]|c.iter().map(|n|n.saturating_sub(1)).sum::<usize>();
                rows.push(json!({"part":p,"column":col,"route":if top{"indexed-top within-sign pairs"}else{"ordinary component generic pairs"},"original_arrivals":bc.iter().sum::<usize>(),"candidate_arrivals":ac.iter().sum::<usize>(),"original_generic_pairs":additions(&bc),"candidate_generic_pairs":additions(&ac),"same_formal_occupancy":true,"every_signed_bin_exact":true}));
                if matches!(q,Query::B)&&!top{
                    if col%2==0{prior_occupancy=Some(bc.clone())}
                    else{
                        let prior=prior_occupancy.take().unwrap();let shared=prior.iter().zip(&bc).filter(|(a,b)|**a>0&&**b>0).count();
                        // The actual high-then-low filler shares one bucket
                        // set. Add back the seeds counted free twice above.
                        rows.push(json!({"part":p,"column":col,"route":"shared high/low seeding correction","original_arrivals":0,"candidate_arrivals":0,"original_generic_pairs":shared,"candidate_generic_pairs":shared,"same_formal_occupancy":true,"every_signed_bin_exact":true}));
                    }
                }
            }
        }
        assert_eq!(before_total,direct,"before recoding");assert_eq!(after_total,direct,"after coalescing");
        if let Some(old)=&stats{assert_eq!(old,&rows)}else{stats=Some(rows)}
    }
    let rows=stats.unwrap();let before:usize=rows.iter().map(|r|r["original_generic_pairs"].as_u64().unwrap()as usize).sum();let after:usize=rows.iter().map(|r|r["candidate_generic_pairs"].as_u64().unwrap()as usize).sum();
    let orig_arrivals:usize=rows.iter().map(|r|r["original_arrivals"].as_u64().unwrap()as usize).sum();let arrivals:usize=rows.iter().map(|r|r["candidate_arrivals"].as_u64().unwrap()as usize).sum();
    assert_eq!(before-after,orig_arrivals-arrivals);assert_eq!(candidate.iter().flatten().map(|e|e.rows.len()-1).sum::<usize>(),formation);
    let net=before as i64-after as i64-formation as i64;
    json!({"original_rows":words.len(),"global_grouping_admitted":false,"bounded_path_selected":true,"parts":q.parts(),"tile_rows":TILE,"useful_repeated_classes":useful,"formation_additions":formation,"emitted_terms":candidate.iter().map(Vec::len).sum::<usize>(),"original_generic_pairs":before,"candidate_generic_pairs_excluding_formation":after,"candidate_generic_pairs_including_formation":after+formation,"net_generic_pairs_saved":net,"generic_pair_saving_fraction":net as f64/before as f64,"necessary_opportunity_positive":net>0,"original_arrivals":orig_arrivals,"candidate_arrivals":arrivals,"rows":rows,"two_independent_token_streams":true,"every_row_once_and_partition_tile_preserved":true,"every_signed_bin_and_full_scalar_dot_product_exact":true,"performance_claim":false})
}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn digit_reconstruction_full_width_and_boundaries(){
  let mut seed=0x7363616c617273u64;let lambda=split::phi_lambda();
  for i in 0..4096{let s=match i{0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,_=>token(&mut seed)};for q in[Query::A,Query::B]{let d=digits(q,s.to_canonical());let mut sum=Fr::ZERO;for(c,v)in d.iter().enumerate(){let pow=if matches!(q,Query::A){c}else{c/2};sum+=signed(*v)*Fr::from(2u64).pow(&[(pow*12)as u64])*if matches!(q,Query::B)&&c%2==0{lambda}else{Fr::ONE};}assert_eq!(sum,s);}}
 }
 #[test]fn coalescing_is_bounded_and_charges_only_useful_classes(){
  for q in[Query::A,Query::B]{for n in[0,1,7,1023,1024,1025,2049]{let mut seed=811;let shared=token(&mut seed).to_canonical();let words=(0..n).map(|i|match i%9{0=>[0;4],1=>Fr::ONE.to_canonical(),2..=4=>shared,_=>token(&mut seed).to_canonical()}).collect::<Vec<_>>();let ids=(0..n).map(|i|i%17==0).collect::<Vec<_>>();let (entries,f,k)=coalesce(q,&words,&ids);let mut seen=BTreeSet::new();let mut actual=0;let part=n.div_ceil(q.parts());for(p,es)in entries.iter().enumerate(){for e in es{actual+=e.rows.len()-1;let first=e.rows[0];for &i in &e.rows{assert!(seen.insert(i));assert_eq!((i-p*part)/TILE,(first-p*part)/TILE);assert_eq!(words[i],e.word);}if e.rows.len()>1{assert!(digits(q,e.word).iter().filter(|d|**d!=0).count()>1)}}}assert_eq!(actual,f);assert_eq!(seen.len(),words.iter().zip(&ids).filter(|(w,id)|**w!=[0;4]&&!**id).count());assert!(k<=f);}}
 }
 #[test]fn every_bucket_matches_for_skewed_and_boundary_tiles(){
  for q in[Query::A,Query::B]{for n in[31,1025,2049]{let mut seed=9101;let repeated=token(&mut seed).to_canonical();let words=(0..n).map(|i|if i%31==0{[0;4]}else if i%29==0{Fr::ONE.to_canonical()}else if i%23==0{repeated}else{token(&mut seed).to_canonical()}).collect::<Vec<_>>();let ids=(0..n).map(|i|i%19==0).collect::<Vec<_>>();let v=census(q,&words,&ids);assert_eq!(v["bounded_path_selected"],true);assert_eq!(v["every_signed_bin_and_full_scalar_dot_product_exact"],true);}}
 }
}
