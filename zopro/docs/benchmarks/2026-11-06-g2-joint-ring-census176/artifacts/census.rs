use serde_json::{json,Value};
use std::{collections::BTreeMap,fs};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-joint-ring-census176";
mod recipe {
    include!("/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176/study.rs");
    pub fn codes(words:&[[u64;4]])->(Vec<[i16;22]>,Vec<[i16;19]>,Vec<[i128;2]>) {
        let table=Table::new();let mut original=vec![];let mut ring=vec![];
        for &word in words {let s=split::phi_split(word);let pair=[component(s,0),component(s,1)];
            original.push(std::array::from_fn(|j|{let c=j/11;let w=j%11;let m=pair[c].unsigned_abs();let d=split::booth(&[m as u64,(m>>64)as u64,0,0],w,12);if pair[c]<0{-d}else{d}}));
            ring.push(table.recode(pair).map(|d|d.map(|d|{let n=(d.plane*table.reps.len()+d.rep+1)as i16;if d.negative{-n}else{n}}).unwrap_or(0)));
        }
        (original,ring,table.reps)
    }
}
#[derive(Default)]struct Stats{generic:u64,calls:u64,lanes:u64,tails:u64,prefix:u64,zero:u64,projective_add_calls:u64,projective_nonzero_operands:u64,projective_double_calls:u64,hist:BTreeMap<usize,u64>}
impl Stats{
    fn batch(&mut self,m:usize){self.generic+=m as u64;self.calls+=1;self.lanes+=(m/8)as u64;self.tails+=(m%8)as u64;self.prefix+=u64::from(m>=32);self.zero+=u64::from(m==0);*self.hist.entry(m).or_default()+=1;}
    fn tree(&mut self,groups:&mut[Vec<bool>]) {while groups.iter().any(|v|v.len()>1){let mut ordinary=0;for g in groups.iter_mut(){let mut next=Vec::with_capacity(g.len().div_ceil(2));for p in g.chunks(2){ordinary+=usize::from(p.len()==2&&p[0]&&p[1]);next.push(p.iter().any(|&x|x));}*g=next;}self.batch(ordinary);}}
    fn grouped(&mut self,counts:&[usize])->Vec<bool>{let mut result=vec![];for chunk in counts.chunks(256){let mut groups=chunk.iter().map(|&n|vec![true;n]).collect::<Vec<_>>();self.tree(&mut groups);result.extend(groups.iter().map(|v|!v.is_empty()&&v[0]));}result}
    fn indexed(&mut self,counts:&[usize])->Vec<bool>{let mut partials=vec![];for &n in counts{let mut pieces=0;for first in(0..n).step_by(1024){let mut group=vec![vec![true;(n-first).min(1024)]];self.tree(&mut group);pieces+=1;}partials.push(pieces);}self.grouped(&partials)}
    fn merge(&mut self,a:&mut[bool],b:&[bool]){for(x,y)in a.chunks_mut(512).zip(b.chunks(512)){let m=x.iter().zip(y).filter(|(a,b)|**a&&**b).count();if m>0{self.batch(m);}for(a,&b)in x.iter_mut().zip(y){*a|=b;}}}
    fn weighted(&mut self,buckets:&[bool]) {
        for chunk in buckets.chunks(256){let n=chunk.len();assert!(n.is_power_of_two());let mut groups=vec![chunk.to_vec()];
            while groups[0].len()>1{let odd=groups[0].iter().step_by(2).copied().collect::<Vec<_>>();let mut ordinary=0;for g in &mut groups{let mut next=vec![];for p in g.chunks(2){ordinary+=usize::from(p.len()==2&&p[0]&&p[1]);next.push(p.iter().any(|&v|v));}*g=next;}self.batch(ordinary);groups.push(odd);}
            for g in groups.iter().skip(1).rev(){self.projective_double_calls+=1;self.projective_add_calls+=1;self.projective_nonzero_operands+=u64::from(g[0]);}
        }
        let chunks=buckets.len().div_ceil(256);self.projective_add_calls+=(3*chunks+1)as u64;self.projective_double_calls+=8;
        // Calls with identity operands remain counted, but no universal
        // generic-projective arithmetic cost is assigned to these calls.
    }
    fn value(&self)->Value{json!({"generic_affine_pairs":self.generic,"batch_or_tree_layer_calls":self.calls,"full_8lane_groups":self.lanes,"scalar_tail_pairs":self.tails,"prefix_eligible_m_ge32":self.prefix,"zero_pair_layers":self.zero,"projective_add_calls":self.projective_add_calls,"projective_nonidentity_subtrahends":self.projective_nonzero_operands,"projective_double_calls":self.projective_double_calls,"ordinary_pair_count_histogram":self.hist})}
}
#[derive(Default)]struct Stream{arrivals:u64,first:u64,queued_to_projective:u64,projective_first:u64,projective_mixed:u64,finish_joins:u64,normalize:u64,stats:Stats}
struct Window {alive:Vec<bool>,busy:Vec<bool>,batch:Vec<usize>,queue:Vec<usize>,projective:Vec<usize>,s:Stream}
impl Window {
    fn new(n:usize)->Self{Self{alive:vec![false;n],busy:vec![false;n],batch:vec![],queue:vec![],projective:vec![0;n],s:Stream::default()}}
    fn add(&mut self,b:usize){if !self.alive[b]{self.alive[b]=true;self.s.first+=1;}else{assert!(!self.busy[b]);self.busy[b]=true;self.batch.push(b);}}
    fn execute(&mut self){if self.batch.is_empty(){return;}self.s.stats.batch(self.batch.len());for b in self.batch.drain(..){self.busy[b]=false;}}
    fn spill(&mut self){for b in self.queue.drain(..){self.s.queued_to_projective+=1;if self.projective[b]==0{self.s.projective_first+=1;}else{self.s.projective_mixed+=1;}self.projective[b]+=1;}}
    fn take(&mut self){while let Some(&b)=self.queue.last(){if self.busy[b]{return;}self.queue.pop();self.add(b);}}
    fn feed(&mut self,digits:impl Iterator<Item=i16>){for d in digits{if d==0{continue;}self.s.arrivals+=1;let b=d.unsigned_abs()as usize-1;if self.busy[b]{self.queue.push(b);if self.queue.len()==199{self.spill();}continue;}self.add(b);if self.batch.len()==200{self.execute();self.take();}}self.execute();self.spill();}
    fn finish(mut self)->(Vec<bool>,Stream){for(n,&a)in self.projective.iter().zip(&self.alive){if *n>0{self.s.normalize+=1;self.s.finish_joins+=u64::from(a);}}(self.alive,self.s)}
}
fn stream_value(parts:&[Stream])->Value {let sum=|f:fn(&Stream)->u64|parts.iter().map(f).sum::<u64>();json!({"arrivals":sum(|s|s.arrivals),"first_assignments":sum(|s|s.first),"affine_pairs":sum(|s|s.stats.generic),"affine_batches":sum(|s|s.stats.calls),"full_8lane_groups":sum(|s|s.stats.lanes),"scalar_tail_pairs":sum(|s|s.stats.tails),"queued_points_to_projective":sum(|s|s.queued_to_projective),"projective_first_assignments":sum(|s|s.projective_first),"projective_mixed_additions":sum(|s|s.projective_mixed),"finish_projective_affine_joins":sum(|s|s.finish_joins),"normalization_points":sum(|s|s.normalize)})}
fn signed_top(ds:impl Iterator<Item=i16>,slots:usize,s:&mut Stats)->Vec<bool>{let mut counts=vec![0usize;2*slots];for d in ds{if d!=0{counts[d.unsigned_abs()as usize-1+if d<0{slots}else{0}]+=1;}}let mut out=s.indexed(&counts);let neg=out.split_off(slots);s.merge(&mut out,&neg);out}
fn run(original:&[[i16;22]],ring:&[[i16;19]],reps:&[[i128;2]])->Value{
    let n=original.len();let part=n.div_ceil(2);let r=reps.len();let mut original_stream=vec![];let mut ring_stream=vec![];let mut original_tree=Stats::default();let mut ring_tree=Stats::default();let mut original_top=Stats::default();let mut ring_top=Stats::default();let mut original_phi=0;let mut ring_phi=0;
    for w in 0..11{let mut parts=vec![];for first in(0..n).step_by(part){let last=(first+part).min(n);let p=if w==10{let mut high=signed_top(original[first..last].iter().map(|d|d[11+w]),64,&mut original_top);original_phi+=high.iter().filter(|&&b|b).count();let low=signed_top(original[first..last].iter().map(|d|d[w]),64,&mut original_top);original_top.merge(&mut high,&low);high}else{let mut win=Window::new(2048);win.feed(original[first..last].iter().map(|d|d[11+w]));original_phi+=win.alive.iter().filter(|&&b|b).count()+win.projective.iter().filter(|&&n|n>0).count();win.feed(original[first..last].iter().map(|d|d[w]));let(p,s)=win.finish();original_stream.push(s);p};parts.push(p);}let mut p=parts.remove(0);for other in parts{original_tree.merge(&mut p,&other);}original_tree.weighted(&p);}
    for w in 0..19{let mut parts=vec![];for first in(0..n).step_by(part){let last=(first+part).min(n);let p=if w==18{signed_top(ring[first..last].iter().map(|d|d[w]),3*r,&mut ring_top)}else{let mut win=Window::new(3*r);win.feed(ring[first..last].iter().map(|d|d[w]));let(p,s)=win.finish();ring_stream.push(s);p};parts.push(p);}let mut p=parts.remove(0);for other in parts{ring_tree.merge(&mut p,&other);}ring_phi+=p[r..].iter().filter(|&&b|b).count();let mut merged=p[..r].to_vec();ring_tree.merge(&mut merged,&p[r..2*r]);ring_tree.merge(&mut merged,&p[2*r..]);
        for axis in 0..2{let mut counts=vec![0usize;128];for(i,rep)in reps.iter().enumerate(){if merged[i]&&rep[axis]!=0{counts[rep[axis].unsigned_abs()as usize-1]+=1;}}let sums=ring_tree.grouped(&counts);ring_tree.weighted(&sums);}ring_tree.projective_add_calls+=1;
    }
    let a=stream_value(&original_stream);let b=stream_value(&ring_stream);let important=|v:&Value,k:&str|v[k].as_u64().unwrap();let affine_gain=important(&b,"affine_pairs")<important(&a,"affine_pairs");let no_spill_explosion=important(&b,"projective_mixed_additions")+important(&b,"finish_projective_affine_joins")<=important(&a,"projective_mixed_additions")+important(&a,"finish_projective_affine_joins");
    json!({"rows":n,"original_stream":a,"ring_stream":b,"original_indexed_top":original_top.value(),"ring_indexed_top":ring_top.value(),"original_merges_and_weighted":original_tree.value(),"ring_merges_axes_and_weighted":ring_tree.value(),"original_phi_affine_and_projective_maps":original_phi,"ring_affine_unit_phi_maps":ring_phi,"ring_projective_axis_phi_maps":19,"original_Horner_double_calls":11*12,"ring_Horner_double_calls":19*7,"native_correctness_design_permitted":affine_gain&&no_spill_explosion,"equal_x_and_cancellation_modeled":false})
}
fn main(){assert!(!std::path::Path::new(&format!("{R}/analysis.json")).exists());let inputs:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176/inputs.json").unwrap()).unwrap();let mut cases=vec![];for(name,input)in inputs.as_object().unwrap(){let bytes=fs::read(input["file"].as_str().unwrap()).unwrap();let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;assert_eq!(&bytes[..8],b"B2SCAN01");assert_eq!(bytes.len(),16+160*n);let words=bytes[16+128*n..].chunks_exact(32).map(|b|std::array::from_fn(|j|u64::from_le_bytes(b[8*j..8*j+8].try_into().unwrap()))).collect::<Vec<_>>();let(mut a,mut b,reps)=recipe::codes(&words);for i in 0..n{if bytes[16+128*i..16+128*(i+1)].iter().all(|&b|b==0){a[i]=[0;22];b[i]=[0;19];}}cases.push(json!({"case":name,"counts":run(&a,&b,&reps)}));}fs::write(format!("{R}/analysis.json"),serde_json::to_vec_pretty(&json!({"complete":true,"cases":cases,"native_measurement_selected":false,"new_proofs":0,"proof_ledger":30708,"production_changed":false,"adopted":false,"goal_achieved":false})).unwrap()).unwrap();println!("Structural stream/tree census complete, {} raw fixtures; no timing/proofs",cases.len());}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn queue_spills_preserve_complete_addition_accounting(){for bins in[1usize,8,64,8193]{for n in[0usize,1,8,199,200,201,1000,20000]{let mut w=Window::new(bins);w.feed((0..n).map(|i|if i%11==0{0}else{((i*37%bins)+1)as i16}));let(live,s)=w.finish();let occupied=live.iter().filter(|&&v|v).count()as u64;assert_eq!(s.stats.generic+s.projective_mixed+s.finish_joins,s.arrivals-occupied);assert_eq!(s.stats.lanes*8+s.stats.tails,s.stats.generic);assert_eq!(s.normalize,s.projective_first);assert_eq!(s.projective_first+s.projective_mixed,s.queued_to_projective);}}let mut w=Window::new(1);w.feed(std::iter::repeat_n(1,1000));let(_,s)=w.finish();assert_eq!(s.stats.generic,1);assert_eq!(s.queued_to_projective,998);assert_eq!(s.projective_mixed,997);}
    #[test]fn actual_weighted_tree_topology_full_and_empty(){for n in[1usize,2,4,8,64,128,256,2048]{let mut s=Stats::default();s.weighted(&vec![true;n]);let group=n.min(256);let chunks=n.div_ceil(256);assert_eq!(s.generic,(chunks*(2*group-group.ilog2()as usize-2))as u64);assert_eq!(s.lanes*8+s.tails,s.generic);let mut z=Stats::default();z.weighted(&vec![false;n]);assert_eq!(z.generic,0);assert_eq!(z.projective_nonzero_operands,0);}}
    #[test]fn indexed_tile_and_group_boundaries(){let counts=vec![0usize,1,2,255,256,257,1023,1024,1025,2049];let mut s=Stats::default();let out=s.indexed(&counts);assert_eq!(s.generic,counts.iter().map(|&n|n.saturating_sub(1)as u64).sum::<u64>());assert_eq!(out,counts.iter().map(|&n|n!=0).collect::<Vec<_>>());assert_eq!(s.lanes*8+s.tails,s.generic);}
}
