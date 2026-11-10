#![recursion_limit="256"]
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
fn absorb(a:&mut Stats,b:&Stats){a.generic+=b.generic;a.calls+=b.calls;a.lanes+=b.lanes;a.tails+=b.tails;a.prefix+=b.prefix;a.zero+=b.zero;a.projective_add_calls+=b.projective_add_calls;a.projective_nonzero_operands+=b.projective_nonzero_operands;a.projective_double_calls+=b.projective_double_calls;for(&m,&n)in &b.hist{*a.hist.entry(m).or_default()+=n;}}
#[derive(Default)]struct Stream{arrivals:u64,first:u64,queued_to_projective:u64,projective_first:u64,projective_mixed:u64,finish_joins:u64,normalize:u64,stats:Stats,tree:Stats,completions:u64,tree_inputs:u64,max_inputs:usize,max_depth:usize}
fn sum_tree(parts:&[Stream])->Stats{let mut t=Stats::default();for s in parts{absorb(&mut t,&s.tree);}t}
struct Window{alive:Vec<bool>,values:Vec<i128>,busy:Vec<bool>,batch:Vec<(usize,i128)>,queue:Vec<(usize,i128)>,projective:Vec<usize>,s:Stream}
impl Window{
 fn new(n:usize)->Self{Self{alive:vec![false;n],values:vec![0;n],busy:vec![false;n],batch:vec![],queue:vec![],projective:vec![0;n],s:Stream::default()}}
 fn add(&mut self,b:usize,value:i128){if !self.alive[b]{self.alive[b]=true;self.values[b]=value;self.s.first+=1;}else{assert!(!self.busy[b]);self.busy[b]=true;self.batch.push((b,value));}}
 fn execute(&mut self){if self.batch.is_empty(){return;}assert!(self.batch.len()<=200);self.s.stats.batch(self.batch.len());for(b,value)in self.batch.drain(..){self.busy[b]=false;self.values[b]+=value;}}
 fn spill(&mut self){
  if self.queue.is_empty(){return;}assert!(self.queue.len()<=199);
  // Finish pending bucket dependencies before any root attachment.
  self.execute();self.s.completions+=1;self.s.tree_inputs+=self.queue.len()as u64;self.s.max_inputs=self.s.max_inputs.max(self.queue.len());
  let mut groups=BTreeMap::<usize,Vec<i128>>::new();for(b,value)in self.queue.drain(..){assert!(self.alive[b]);groups.entry(b).or_default().push(value);}
  let mut shape=groups.values().map(|g|vec![true;g.len()]).collect::<Vec<_>>();self.s.tree.tree(&mut shape);
  for(b,mut points)in groups{let depth=points.len().next_power_of_two().trailing_zeros()as usize;self.s.max_depth=self.s.max_depth.max(depth);while points.len()>1{points=points.chunks(2).map(|p|p.iter().sum()).collect();}self.add(b,points[0]);}
  // At most one root per destination; attachment has no collision queue.
  self.execute();assert!(self.busy.iter().all(|b|!*b));
 }
 fn take(&mut self){while let Some(&(b,value))=self.queue.last(){if self.busy[b]{return;}self.queue.pop();self.add(b,value);}}
 fn feed_events(&mut self,events:impl Iterator<Item=(i16,i128)>){for(d,value)in events{if d==0{continue;}self.s.arrivals+=1;let b=d.unsigned_abs()as usize-1;let value=if d<0{-value}else{value};if self.busy[b]{self.queue.push((b,value));if self.queue.len()==199{self.spill();}continue;}self.add(b,value);if self.batch.len()==200{self.execute();self.take();}}self.execute();self.spill();}
 fn feed(&mut self,digits:impl Iterator<Item=i16>){self.feed_events(digits.enumerate().map(|(i,d)|(d,(i+1)as i128)));}
 fn finish(self)->(Vec<bool>,Stream){assert!(self.queue.is_empty()&&self.batch.is_empty());assert!(self.projective.iter().all(|&n|n==0));(self.alive,self.s)}
}
fn covered(v:&Value)->Value{let w=&v["candidate_work"];let u=|k:&str|w[k].as_u64().unwrap();let norm_calls=36;let mixed=u("stream_projective_mixed_plus_finish");let n=u("stream_normalization_points");json!({"M":8*u("V")+u("scalar_M")+8*mixed+2*n+2*norm_calls+v["candidate_unit_map_Fq_products"].as_u64().unwrap()+1+38,"F":8*u("F")+u("scalar_fused2")+14*mixed+12*n,"S":u("scalar_S")+2*norm_calls,"I":u("inverse_calls")+norm_calls,"not_runtime":true,"weighted_projective_packing_memory_scheduling_unpriced":true})}

fn stream_value(parts:&[Stream])->Value {let sum=|f:fn(&Stream)->u64|parts.iter().map(f).sum::<u64>();json!({"arrivals":sum(|s|s.arrivals),"first_assignments":sum(|s|s.first),"affine_pairs":sum(|s|s.stats.generic),"affine_batches":sum(|s|s.stats.calls),"full_8lane_groups":sum(|s|s.stats.lanes),"scalar_tail_pairs":sum(|s|s.stats.tails),"queued_points_to_projective":sum(|s|s.queued_to_projective),"projective_first_assignments":sum(|s|s.projective_first),"projective_mixed_additions":sum(|s|s.projective_mixed),"finish_projective_affine_joins":sum(|s|s.finish_joins),"normalization_points":sum(|s|s.normalize),"queue_tree_completions":sum(|s|s.completions),"queue_tree_input_points":sum(|s|s.tree_inputs),"queue_tree_stages":sum(|s|s.tree.calls),"queue_tree_max_input_points":parts.iter().map(|s|s.max_inputs).max().unwrap_or(0),"queue_tree_max_levels":parts.iter().map(|s|s.max_depth).max().unwrap_or(0),"queue_tree_stats":sum_tree(parts).value()})}
fn signed_top(ds:impl Iterator<Item=i16>,slots:usize,s:&mut Stats)->Vec<bool>{let mut counts=vec![0usize;2*slots];for d in ds{if d!=0{counts[d.unsigned_abs()as usize-1+if d<0{slots}else{0}]+=1;}}let mut out=s.indexed(&counts);let neg=out.split_off(slots);s.merge(&mut out,&neg);out}
// This suffix follows the exact inherited recoder/Stats/Window helper prefix.
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/ring-queue-tree176";
const REPS:usize=2731;

fn work(stream:&Value,tree:&Value,top:&Value)->Value {
    let u=|v:&Value,k:&str|v[k].as_u64().unwrap();
    let blocks=u(stream,"full_8lane_groups")+u(tree,"full_8lane_groups")+u(top,"full_8lane_groups");
    let calls=u(stream,"affine_batches")+u(tree,"batch_or_tree_layer_calls")+u(top,"batch_or_tree_layer_calls");
    let tree_groups=u(tree,"full_8lane_groups")+u(top,"full_8lane_groups");
    let layers=u(tree,"batch_or_tree_layer_calls")+u(top,"batch_or_tree_layer_calls");
    // Below32 ordinary pairs the original record path adds one V per group.
    let extra=|v:&Value|v["ordinary_pair_count_histogram"].as_object().unwrap().iter()
        .filter_map(|(m,c)|{let m=m.parse::<u64>().unwrap();(m<32).then_some((m/8)*c.as_u64().unwrap())}).sum::<u64>();
    let v=7*blocks+5*calls+extra(tree)+extra(top);
    let fused=5*blocks;
    let stream_tails=u(stream,"scalar_tail_pairs");let tree_tails=u(tree,"scalar_tail_pairs")+u(top,"scalar_tail_pairs");
    json!({"V":v,"F":fused,"vector_IFMA_proxy":100*v+150*fused,"inverse_calls":calls,
        "stream_scalar_tail_pairs":stream_tails,"tree_scalar_tail_pairs":tree_tails,
        "scalar_M":8*stream_tails+7*tree_tails+calls,"scalar_S":2*stream_tails,"scalar_fused2":4*stream_tails+5*tree_tails,
        "generic_affine_pairs":u(stream,"affine_pairs")+u(tree,"generic_affine_pairs")+u(top,"generic_affine_pairs"),
        "tree_groups":tree_groups,"tree_layers":layers,
        "stream_projective_mixed_plus_finish":u(stream,"projective_mixed_additions")+u(stream,"finish_projective_affine_joins"),
        "stream_normalization_points":u(stream,"normalization_points"),
        "not_complete_CPU_cost":true})
}

fn phase_counts(ring:&[[i16;19]],reps:&[[i128;2]],old:&Value)->Value {
    let n=ring.len();let part=n.div_ceil(2);assert_eq!(reps.len(),REPS);
    let mut streams=vec![];let mut tree=Stats::default();let mut top=Stats::default();
    let mut affine_maps=0usize;let mut projective_maps=0usize;let mut top_maps=0usize;
    for w in 0..19 {
        let mut parts=vec![];
        for first in(0..n).step_by(part){let last=(first+part).min(n);
            if w==18 { parts.push(signed_top(ring[first..last].iter().map(|d|d[w]),3*REPS,&mut top)); }
            else {
                let mut win=Window::new(REPS);
                for unit in(0..3).rev(){
                    win.feed(ring[first..last].iter().filter_map(|s|{
                        let d=s[w];if d==0{return None;}let index=d.unsigned_abs()as usize-1;
                        if index/REPS!=unit{return None;}
                        let v=(index%REPS+1)as i16;Some(if d<0{-v}else{v})
                    }));
                    if unit!=0 {affine_maps+=win.alive.iter().filter(|&&b|b).count();projective_maps+=win.projective.iter().filter(|&&n|n>0).count();}
                }
                let(p,s)=win.finish();absorb(&mut tree,&s.tree);parts.push(p);streams.push(s);
            }
        }
        let mut p=parts.remove(0);for other in parts{tree.merge(&mut p,&other);}
        let merged=if w==18 {
            top_maps+=p[REPS..].iter().filter(|&&b|b).count();let mut out=p[..REPS].to_vec();
            tree.merge(&mut out,&p[REPS..2*REPS]);tree.merge(&mut out,&p[2*REPS..]);out
        }else{p};
        assert_eq!(merged.len(),REPS);
        for axis in 0..2{let mut counts=vec![0usize;128];for(i,rep)in reps.iter().enumerate(){if merged[i]&&rep[axis]!=0{counts[rep[axis].unsigned_abs()as usize-1]+=1;}}let sums=tree.grouped(&counts);tree.weighted(&sums);}
        tree.projective_add_calls+=1;
    }
    // The leading algorithm is unchanged, including every ordinary pair,
    // tail and histogram. Check its recomposed candidate counts against the
    // saved parent observation; no parent model/test invocation is repeated.
    assert_eq!(top.value(),old["ring_indexed_top"]);
    let stream=stream_value(&streams);let tree=tree.value();let top=top.value();
    let candidate=work(&stream,&tree,&top);
    let accepted=work(&old["original_stream"],&old["original_merges_and_weighted"],&old["original_indexed_top"]);
    let parent=work(&old["ring_stream"],&old["ring_merges_axes_and_weighted"],&old["ring_indexed_top"]);
    let u=|v:&Value,k:&str|v[k].as_u64().unwrap();
    // Every noncancelling mathematical addition is either affine or an
    // exceptional-projective spill/finish. Changing phases cannot erase it.
    assert_eq!(u(&candidate,"generic_affine_pairs")+u(&candidate,"stream_projective_mixed_plus_finish"),u(&parent,"generic_affine_pairs")+u(&parent,"stream_projective_mixed_plus_finish"));
    let permitted=u(&candidate,"vector_IFMA_proxy")<u(&accepted,"vector_IFMA_proxy")&&u(&candidate,"inverse_calls")<=u(&accepted,"inverse_calls");
    json!({"rows":n,"candidate_stream":stream,"candidate_merges_axes_weighted":tree,"unchanged_top":top,
        "accepted_work":accepted,"old_ring_work":parent,"candidate_work":candidate,
        "candidate_ordinary_affine_phi_maps":affine_maps,"candidate_ordinary_projective_phi_maps":projective_maps,
        "unchanged_top_affine_phi_maps":top_maps,"candidate_unit_map_Fq_products":2*(affine_maps+projective_maps+top_maps),
        "old_ring_unit_map_Fq_products":2*old["ring_affine_unit_phi_maps"].as_u64().unwrap(),
        "accepted_unit_map_Fq_products":2*old["original_phi_affine_and_projective_maps"].as_u64().unwrap(),
        "axis_result_phi_maps_each_recipe":19,"Horner_double_calls_unchanged":133,
        "old_ordinary_code_row_tests":18*n,"candidate_ordinary_code_row_tests":3*18*n,
        "point_identity_coordinate_tests_before_unit_filter":false,
        "ordinary_bucket_slots_old_per_part":3*REPS,"ordinary_bucket_slots_candidate_per_part":REPS,
        "all_part_window_affine_coordinate_payload_old_bytes":2*19*3*REPS*128,
        "all_part_window_affine_coordinate_payload_candidate_bytes":2*(18*REPS+3*REPS)*128,
        "native_resource_correctness_investigation_permitted":permitted,
        "native_measurement_selected":false,"generic_noncancelling_model_only":true})
}

fn main(){assert!(!std::path::Path::new(&format!("{ROOT}/analysis.json")).exists());
    let inputs:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176/inputs.json").unwrap()).unwrap();
    let prior:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/g2-joint-ring-census176/analysis.json").unwrap()).unwrap();let mut cases=vec![];
    for(name,input)in inputs.as_object().unwrap(){let b=fs::read(input["file"].as_str().unwrap()).unwrap();assert_eq!(&b[..8],b"B2SCAN01");let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(b.len(),16+160*n);
        let words=b[16+128*n..].chunks_exact(32).map(|b|std::array::from_fn(|j|u64::from_le_bytes(b[j*8..j*8+8].try_into().unwrap()))).collect::<Vec<_>>();
        let(_,mut ring,reps)=recipe::codes(&words);for i in 0..n{if b[16+128*i..16+128*(i+1)].iter().all(|&b|b==0){ring[i]=[0;19];}}
        let old=&prior["cases"].as_array().unwrap().iter().find(|v|v["case"]==name.as_str()).unwrap()["counts"];
        cases.push(json!({"case":name,"counts":phase_counts(&ring,&reps,old)}));
    }
    let comparator:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/ring-affine-completion176/analysis.json").unwrap()).unwrap();
 let mut checks=BTreeMap::new();
 for c in &mut cases{let name=c["case"].as_str().unwrap().to_owned();let v=&c["counts"];checks.insert(format!("{name}/original-necessary-screen"),v["native_resource_correctness_investigation_permitted"]==true);let s=&v["candidate_stream"];checks.insert(format!("{name}/bounded-tree"),s["queue_tree_max_levels"].as_u64().unwrap()<=8&&s["queue_tree_max_input_points"].as_u64().unwrap()<=199);let baseline=&comparator["cases"].as_array().unwrap().iter().find(|x|x["case"]==name).unwrap()["counts"];let a=covered(v);let b=covered(baseline);let mut strict=false;for k in ["M","F","S","I"]{let x=a[k].as_u64().unwrap();let y=b[k].as_u64().unwrap();strict|=x<y;checks.insert(format!("{name}/{k}-not-increased"),x<=y);}checks.insert(format!("{name}/strict-work-reduction"),strict);c["covered_candidate"]=a;c["covered_linear_completion"]=b;}
 let passed=checks.values().all(|&b|b);
    fs::write(format!("{ROOT}/analysis.json"),serde_json::to_vec_pretty(&json!({"passed_data_validation":true,"necessary_screen_passed":passed,"checks":checks,"cases":cases,"old_baseline_counts_read_not_reexecuted":true,"native_measurement_selected":false,"new_proofs":0,"canonical_proofs":44265,"production_changed":false,"adopted":false,"goal_complete":false})).unwrap()).unwrap();println!("phasecensuscomplete: necessaryscreen={passed}, zero newproof/timing/native measurement");
}

#[cfg(test)]mod phase_tests {
    fn phi((a,b):(i64,i64))->(i64,i64){((-b).rem_euclid(1019),(a-b).rem_euclid(1019))}
    fn add((a,b):(i64,i64),(c,d):(i64,i64))->(i64,i64){((a+c).rem_euclid(1019),(b+d).rem_euclid(1019))}
    fn check(events:&[(usize,usize,bool,(i64,i64))],bins:usize){
        let mut expected=vec![(0,0);bins];for &(bucket,unit,negative,p)in events{let mut p=p;for _ in 0..unit{p=phi(p);}if negative{p=(-p.0,-p.1);}expected[bucket]=add(expected[bucket],p);}
        let mut state=vec![(0,0);bins];let mut seen=vec![0u8;events.len()];for unit in(0..3).rev(){for(i,&(bucket,u,negative,p))in events.iter().enumerate(){if u!=unit{continue;}seen[i]+=1;state[bucket]=add(state[bucket],if negative{(-p.0,-p.1)}else{p});}if unit!=0{state.iter_mut().for_each(|p|*p=phi(*p));}}
        assert_eq!(state,expected);assert!(seen.iter().all(|&n|n==1));
        // A separate third-order endomorphism identity check includes zero
        // and inverse pairs; native curve exception dispatch is not tested.
        for p in state{assert_eq!(phi(phi(phi(p))),p);assert_eq!(add(add(p,phi(p)),phi(phi(p))),(0,0));}
    }
    #[test]fn phase_horner_signs_cancellations_and_unit_masks(){
        for mask in 0..64usize{let events=(0..6).filter(|i|mask>>i&1!=0).map(|i|(i%2,i/2,i%2!=0,((i*43+1)as i64,(i*19+7)as i64))).collect::<Vec<_>>();check(&events,2);}
        for bins in[1,7,127,2731]{for n in[0,1,7,8,199,200,201,1024,4099]{let events=(0..n).map(|i|(i*37%bins,i%3,i%5==0,if i%11==0{(0,0)}else{((i/2*17%1019)as i64,(i/2*43%1019)as i64)})).collect::<Vec<_>>();check(&events,bins);}}
    }
    #[test]fn retained_bucket_payload_geometry(){assert_eq!(2*19*3*2731*128,39_850_752);assert_eq!(2*(18*2731+3*2731)*128,14_681_856);assert_eq!(3*2731/2731,3);}
}

#[cfg(test)]mod queue_tests{
 use super::*;
 #[test]fn bounded_tree_payload_conservation(){
  let mut cases=0;for bins in[1usize,2,7,127,2731]{for n in[0,1,7,8,9,198,199,200,201,399,400,4099]{let mut w=Window::new(bins);let mut expected=vec![0i128;bins];for phase in 0..3{let events=(0..n).map(|i|{let b=(i*37+i/7)%bins;let d=if (i+phase)%11==0{0}else{(b+1)as i16*if (i+phase)%3==0{-1}else{1}};let value=if i%9==0{0}else{(i/2+1)as i128*if i%2==0{1}else{-1}};(d,value)}).collect::<Vec<_>>();for &(d,value)in &events{if d!=0{expected[d.unsigned_abs()as usize-1]+=if d<0{-value}else{value};}}w.feed_events(events.into_iter());assert_eq!(w.values,expected);assert!(w.queue.is_empty()&&w.batch.is_empty());assert!(w.s.max_inputs<=199&&w.s.max_depth<=8);assert_eq!(w.s.queued_to_projective,0);cases+=1;}let total=w.s.stats.generic+w.s.tree.generic;assert_eq!(total,w.s.arrivals-w.s.first);}}assert_eq!(cases,180);println!("TOKEN_CASES={cases}; signed/zero/cancelling payloads; queues and batches empty; 199-input/8-level bounds");
 }
 #[test]fn one_hot_queue_depth(){let mut w=Window::new(1);w.feed_events((0..10000).map(|i|(1,i as i128)));assert_eq!(w.values[0],9999*10000/2);assert_eq!(w.s.max_inputs,199);assert_eq!(w.s.max_depth,8);assert_eq!(w.s.stats.generic+w.s.tree.generic,9999);assert!(w.s.tree.calls<=8*w.s.completions);assert_eq!(w.s.queued_to_projective,0);println!("HOT_INPUTS=10000; max199/8; additions9999; completions={}",w.s.completions);}
}
