//! Compact nonaliasing G2 tree buffers and index-only ordinary pairs.
use super::*;
use zopro_bn254::{G2Affine,G2Config,G2Projective};
#[derive(Clone,Copy)]
struct Pair{left:u32,right:u32,destination:u32}

// Same prefix/norm formulas as g2_prefix::apply. Source and destination
// are disjoint; pair records hold indices instead of copied affine points.
fn apply<const FAST:bool>(get:impl Fn(usize)->G2Affine,out:&mut[G2Affine],pairs:&[Pair],s:&mut Fq2AddScratch){
    let full=pairs.len()/LANES*LANES;let mut chains=FqBatch::one();s.prefix_records.clear();s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES){let ax=fq2_pack(core::array::from_fn(|i|get(pairs[k+i].left as usize).x));let bx=fq2_pack(core::array::from_fn(|i|get(pairs[k+i].right as usize).x));let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));let norm=Fq2::lanes_norm(&denominator);s.prefix_records.push([chains,norm]);chains=chains.mul(&norm);}
    s.tail.clear();let mut inverse=if let Some(value)=direct_chain_inverse(&chains,full==pairs.len(),s.vector_inverse){value}else{
        let mut tail=chains.get();for k in full..pairs.len(){let a=get(pairs[k].left as usize);let b=get(pairs[k].right as usize);let denominator=b.x-a.x;let norm=denominator.c0.square()+denominator.c1.square();let numerator=((b.y-a.y)*denominator.conjugate()).mul_by_fp(&tail[k%LANES]);tail[k%LANES]*=norm;s.tail.push([a.x,a.y,a.x+b.x,Fq2::new(norm,Fq::ZERO),numerator]);}
        let mut inverse=invert_lanes(tail,s.vector_inverse);for k in (full..pairs.len()).rev(){let[ax,ay,xsum,norm,numerator]=s.tail[k-full];let slope=numerator.mul_by_fp(&inverse[k%LANES]);inverse[k%LANES]*=norm.c0;let x=slope.square()-xsum;let y=slope*(ax-x)-ay;out[pairs[k].destination as usize]=G2Affine::new(x,y);}FqBatch::new(&inverse)
    };
    for j in (0..full/LANES).rev(){let k=j*LANES;let a:[G2Affine;LANES]=core::array::from_fn(|i|get(pairs[k+i].left as usize));let b:[G2Affine;LANES]=core::array::from_fn(|i|get(pairs[k+i].right as usize));let ax=fq2_pack(a.map(|p|p.x));let ay=fq2_pack(a.map(|p|p.y));let bx=fq2_pack(b.map(|p|p.x));let by=fq2_pack(b.map(|p|p.y));let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));let dy=core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]));let factor=s.prefix_records[j][0].mul(&inverse);inverse=inverse.mul(&s.prefix_records[j][1]);let slope=Fq2::lanes_scaled_numerator(&dy,&denominator,&factor);let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&Fq2::lanes_add(&ax,&bx));let y=Fq2::lanes_sub(&Fq2::lanes_mul(&slope,&Fq2::lanes_sub(&ax,&x)),&ay);let(x,y)=(fq2_unpack(x),fq2_unpack(y));for i in 0..LANES{out[pairs[k+i].destination as usize]=G2Affine::new(x[i],y[i]);}}
}

fn level(get:impl Fn(usize)->G2Affine,out:&mut Vec<G2Affine>,starts:&mut[usize],counts:&mut[usize],pairs:&mut Vec<Pair>,s:&mut Fq2AddScratch){
    let size:usize=counts.iter().map(|n|n.div_ceil(2)).sum();assert!(out.capacity()>=size);out.clear();out.resize(size,G2Affine::IDENTITY);pairs.clear();let mut next=0;
    for j in 0..counts.len(){let start=starts[j];let n=counts[j];let destination=next;let reduced=n.div_ceil(2);starts[j]=destination;counts[j]=reduced;next+=reduced;
        for k in 0..n/2{let left=start+2*k;let right=left+1;let a=get(left);let b=get(right);let dst=destination+k;
            if a.is_identity(){out[dst]=b;}else if b.is_identity(){out[dst]=a;}else if a.x==b.x{out[dst]=if a.y==b.y{a.to_projective().double().to_affine()}else{G2Affine::IDENTITY};}else{pairs.push(Pair{left:left.try_into().unwrap(),right:right.try_into().unwrap(),destination:dst.try_into().unwrap()});}
        }
        if n%2!=0{out[destination+reduced-1]=get(start+n-1);}
    }
    assert_eq!(next,size);if s.canonical{apply::<true>(get,out,pairs,s)}else{apply::<false>(get,out,pairs,s)}
}

pub(crate) fn group(bases:&[G2Affine],order:&[u32],starts:&[usize],counts:&[usize])->Vec<G2Affine>{
    const GROUP:usize=256;assert_eq!(starts.len(),counts.len()+1);assert_eq!(*starts.last().unwrap(),order.len());let mut result=vec![G2Affine::IDENTITY;counts.len()];
    result.par_chunks_mut(GROUP).enumerate().for_each(|(group,output)|{let first=group*GROUP;let offset=starts[first];let end=starts[first+output.len()];let n=end-offset;let original=|i:usize|{let index=order[offset+i];let p=bases[(index&0x7fff_ffff)as usize];if index>>31!=0{-p}else{p}};let mut local_starts=[0usize;GROUP];let mut local_counts=[0usize;GROUP];for j in 0..output.len(){local_starts[j]=starts[first+j]-offset;local_counts[j]=counts[first+j];}
        let starts=&mut local_starts[..output.len()];let counts=&mut local_counts[..output.len()];if !counts.iter().any(|&n|n>1){for(j,p)in output.iter_mut().enumerate(){if counts[j]!=0{*p=original(starts[j]);}}return;}
        let n1:usize=counts.iter().map(|n|n.div_ceil(2)).sum();let mut a=Vec::with_capacity(n1);let mut b=Vec::new();let mut pairs=Vec::with_capacity(n/2);let mut s=Fq2AddScratch{canonical:crate::simd::cpu_option("ZOPRO_CPU_CANONICAL_BATCH"),vector_inverse:crate::simd::cpu_option("ZOPRO_CPU_VECTOR_INVERSE"),..Default::default()};
        level(original,&mut a,starts,counts,&mut pairs,&mut s);while counts.iter().any(|&n|n>1){let size:usize=counts.iter().map(|n|n.div_ceil(2)).sum();if b.capacity()<size{assert_eq!(b.capacity(),0,"compact buffers must not grow after their first level");b=Vec::with_capacity(size);}level(|i|a[i],&mut b,starts,counts,&mut pairs,&mut s);core::mem::swap(&mut a,&mut b);}
        assert_eq!(s.groups.capacity(),0);assert!(pairs.capacity()==n/2);for(j,p)in output.iter_mut().enumerate(){if counts[j]!=0{*p=a[starts[j]];}}
    });result
}

pub(crate) fn workspace(n:usize,jobs:usize)->Option<usize>{
    if jobs==0&&n!=0{return None;}use crate::msm_affine::workspace::vector as v;
    let first=n.div_ceil(2).checked_add(jobs.checked_mul(128)?)?;let second=n.div_ceil(4).checked_add(jobs.checked_mul(192)?)?;
    let points=v::<G2Affine>(first,false)?.checked_add(v::<G2Affine>(second,false)?)?;let pairs=v::<Pair>(n/2,false)?;let records=v::<[FqBatch;2]>((n/8).checked_add(jobs.checked_mul(8)?)?,false)?;let tails=v::<[Fq2;5]>(jobs.checked_mul(8)?,false)?;
    let metadata=jobs.checked_mul(core::mem::size_of::<Fq2AddScratch>().checked_add(2*256*core::mem::size_of::<usize>())?.checked_add(4*64)?.checked_add(2*core::mem::size_of::<Vec<G2Affine>>())?.checked_add(core::mem::size_of::<Vec<Pair>>())?)?;
    points.checked_add(pairs)?.checked_add(records)?.checked_add(tails)?.checked_add(metadata)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn signed_gather_groups_cover_exceptions_odd_tails_and_skew(){
        let bases:Vec<_>=(0..11).map(|i|(G2Affine::GENERATOR*Fr::from(i as u64)).to_affine()).collect();let mut order=Vec::new();let mut starts=vec![0];let mut counts=Vec::new();let mut expected=Vec::new();
        for bucket in 0..300{let n=if bucket==5{4097}else{[0,1,2,3,9,5,17][bucket%7]};let mut sum=Fr::ZERO;for j in 0..n{let i=if [2,3].contains(&(bucket%7)){1}else{(bucket+j)%11};let neg=bucket%7!=2&&j%3==0;order.push(i as u32|if neg{1<<31}else{0});let w=Fr::from(i as u64);sum+=if neg{-w}else{w};}counts.push(n);starts.push(order.len());expected.push((G2Affine::GENERATOR*sum).to_affine());}
        for workers in [1,4]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();pool.install(||{assert_eq!(group(&bases,&order,&starts,&counts),expected);assert_eq!(group(&bases,&order,&starts,&counts),super::super::group_g2_prefix(&bases,&order,&starts,&counts));});}
    }
    #[test]fn compact_buffer_and_typed_forecasts_cover_boundaries(){
        assert_eq!(core::mem::size_of::<Pair>(),12);for n in [0usize,1,2,3,7,8,15,16,127,256,4097,30089]{for jobs in [1usize,8]{assert!(workspace(n,jobs).unwrap()>=n/2*12);let buckets=256;for odd in 0..=buckets.min(n){if(n+odd)%2==0{let first=(n+odd)/2;assert!(first<=n.div_ceil(2)+128);let second=(first+first.min(buckets)).div_ceil(2);assert!(second<=n.div_ceil(4)+192);}}}}
        assert!(workspace(usize::MAX,8).is_none());assert!(workspace(4097,usize::MAX).is_none());assert!(workspace(1,0).is_none());
    }
}
