use std::sync::Mutex;
pub const N:usize=21;
pub const LABELS:[&str;N]=["terms","first_writes","affine_updates","batches","full_batches","queue_pushes","drained","flush_points","nonempty_flushes","projective_arrivals","projective_initializations","projective_adds","equal_x_same","equal_x_opposite","scalar_tail_pairs","vector_pairs","drain_inspections","overflow_flushes","max_batch","max_queue","feed_calls"];
pub const GLOBAL_LABELS:[&str;12]=["weighted_calls","weighted_slots","weighted_nonidentity","indexed_calls","indexed_inputs","indexed_sign_bins","reduction_layers","reduction_generic_pairs","reduction_identity_or_equal_pairs","reduction_vector_pairs","reduction_scalar_tail_pairs","zero_generic_layers"];
#[derive(Clone,Debug)]pub struct Row{pub half:usize,pub size:usize,pub column:usize,pub multiplier:usize,pub points:usize,pub phase:usize,pub counts:[[u64;N];2],pub histogram:[Vec<u64>;2],pub normalized:usize}
impl Row {
 pub(super)fn new(half:usize,size:usize)->Self{Self{half,size,column:usize::MAX,multiplier:0,points:0,phase:0,counts:[[0;N];2],histogram:std::array::from_fn(|_|vec![0;size+1]),normalized:0}}
 pub(super)fn event(&mut self,i:usize,n:usize){self.counts[self.phase][i]+=n as u64;}
 pub(super)fn maximum(&mut self,i:usize,n:usize){self.counts[self.phase][i]=self.counts[self.phase][i].max(n as u64);}
 pub(super)fn feed(&mut self,col:usize,m:usize,phase:usize,points:usize){self.column=col;self.multiplier=m;self.phase=phase;self.points=points;self.event(20,1);}
 pub fn validate(&self){let mut normalized=0;for(comp,c)in self.counts.iter().enumerate(){assert_eq!(c[0],c[1]+c[2]+c[7]+c[12]+c[13]);assert_eq!(c[5],c[6]+c[7]);assert_eq!(c[9],c[7]+c[12]);assert_eq!(c[9],c[10]+c[11]);assert_eq!(c[2],c[14]+c[15]);assert_eq!(c[15]%8,0);assert!(c[18]<=self.size as u64&&c[19]<self.size as u64);assert_eq!(self.histogram[comp].iter().sum::<u64>(),c[3]);assert_eq!(self.histogram[comp].iter().enumerate().map(|(n,c)|n as u64*c).sum::<u64>(),c[2]);assert_eq!(self.histogram[comp][self.size],c[4]);normalized+=c[10];}assert_eq!(self.normalized as u64,normalized);}
}
static ROWS:Mutex<Vec<Row>>=Mutex::new(Vec::new());
static GLOBAL:Mutex<[u64;12]>=Mutex::new([0;12]);
pub fn begin(){ROWS.lock().unwrap().clear();*GLOBAL.lock().unwrap()=[0;12];}
pub fn finish()->(Vec<Row>,[u64;12]){let rows=std::mem::take(&mut *ROWS.lock().unwrap());for r in&rows{r.validate();}(rows,*GLOBAL.lock().unwrap())}
pub(super)fn emit(row:Row){row.validate();let mut rows=ROWS.lock().unwrap();assert!(rows.len()<128);rows.push(row);}
pub(crate)fn weighted(slots:usize,live:usize){let mut g=GLOBAL.lock().unwrap();g[0]+=1;g[1]+=slots as u64;g[2]+=live as u64;}
pub(crate)fn indexed(inputs:usize,bins:usize){let mut g=GLOBAL.lock().unwrap();g[3]+=1;g[4]+=inputs as u64;g[5]+=bins as u64;}
pub(crate)fn layer(generic:usize,total:usize){assert!(generic<=total);let mut g=GLOBAL.lock().unwrap();g[6]+=1;g[7]+=generic as u64;g[8]+=(total-generic)as u64;g[9]+=(generic/8*8)as u64;g[10]+=(generic%8)as u64;g[11]+=u64::from(generic==0);}

#[cfg(test)]mod tests{use super::*;use zopro_bn254::{Fr,G2Projective,G2Affine};
 #[test]fn exact_window_counters_preserve_complete_sums_and_conservation(){
  let generator=G2Projective::generator();
  for exceptional in[false,true]{begin();let n=513;let coefficients=(0..n).map(|i|if exceptional{match i%5{0=>Fr::ZERO,1|2=>Fr::ONE,3=>-Fr::ONE,_=>Fr::from(2u64)}}else{Fr::from((i+1234567)as u64)}).collect::<Vec<_>>();let bases=G2Projective::batch_to_affine(&coefficients.iter().map(|c|generator**c).collect::<Vec<_>>());let short=vec![[1,0,1,0];n];let mut state=super::super::Window::new(64,8,crate::msm::BatchAdd::default());state.feed(&bases,&short,true,0,12,8);state.feed(&bases,&short,false,0,12,8);let output=state.finish();let expected=generator*(coefficients.into_iter().fold(Fr::ZERO,|a,b|a+b)*Fr::from(2u64));assert_eq!(output[0],expected.to_affine());assert!(output[1..].iter().all(|p|p.is_identity()));let(rows,_)=finish();assert_eq!(rows.len(),1);assert_eq!(rows[0].counts[0][20],1);assert_eq!(rows[0].counts[1][20],1);if exceptional{assert!(rows[0].counts.iter().map(|c|c[12]+c[13]).sum::<u64>()>0);}else{assert!(rows[0].counts.iter().map(|c|c[7]).sum::<u64>()>0);}}
  begin();let mut buckets=vec![G2Affine::IDENTITY;64];buckets[0]=generator.to_affine();buckets[7]=(generator*Fr::from(3u64)).to_affine();let out=crate::msm::weighted_affine(&buckets);assert_eq!(out.to_affine(),(generator*Fr::from(25u64)).to_affine());let(rows,g)=finish();assert!(rows.is_empty());assert_eq!(&g[..3],&[1,64,2]);assert!(g[6]>0);assert_eq!(g[7],g[9]+g[10]);
 }
}
