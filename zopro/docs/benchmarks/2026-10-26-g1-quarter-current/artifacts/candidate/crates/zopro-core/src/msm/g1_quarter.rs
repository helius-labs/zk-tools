//! Rebase of the prior quarter-slope formula on current CIOS and shared scratch.
use super::{FqAddScratch,FqBatch};
use zopro_bn254::{ifma8::Ifma8,Affine,CurveConfig,Fq,FqConfig};
type V=Ifma8<FqConfig,false>;
const INV4:Fq=Fq::from_decimal("5472060717959818805561601436314318772174077789324455915672259473661306552146");
#[inline(always)]fn index<const SIGNED:bool>(i:u32)->usize{if SIGNED{(i&0x7fff_ffff)as usize}else{i as usize}}
#[inline]
fn inverse_quarter(a:V)->V{
    let swapped=V::shuffle_pair::<1,0,3,2>(&a,&a);let pairs=a.mul(&swapped);let other_pair=V::shuffle_pair::<2,3,0,1>(&pairs,&pairs);let quads=pairs.mul(&other_pair);let products=quads.get();let inverse=(products[0]*products[4]).inverse().unwrap()*INV4;let(lo,hi)=quads.halves();let other_quad=V::from_halves(&hi,&lo);
    V::new(&[inverse;8]).mul(&other_quad).mul(&other_pair).mul(&swapped)
}
pub(super) fn stream<C:CurveConfig<Base=Fq>,const SIGNED:bool>(buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut FqAddScratch){
    if s.quarter_trace&&!s.quarter_traced{s.quarter_traced=true;eprintln!("G1_QUARTER selected=true signed={SIGNED} batch={} group_bytes={} scratch_bytes={}",batch.len(),core::mem::size_of::<super::FqAddGroup>(),core::mem::size_of::<FqAddScratch>());}
    let full=batch.len()/8*8;let mut chains=V::one();s.groups.clear();s.groups.reserve(full/8);
    for k in (0..full).step_by(8){let a:[Affine<C>;8]=core::array::from_fn(|i|buckets[index::<SIGNED>(batch[k+i].0)]);let b:[Affine<C>;8]=core::array::from_fn(|i|batch[k+i].1);
        // The SAME1600-byte record and Vec are reused by both kernels. Bit
        // bridges perform no arithmetic; only this function interprets these
        // slots as raw point coordinates and uniform R260 chain values.
        unsafe{let slot=s.groups.as_mut_ptr().add(k/8);let ax=V::point_raw_host(&a.map(|p|p.x));core::ptr::addr_of_mut!((*slot).ax).write(FqBatch::from_point_domain(ax));let bx=V::point_raw_host(&b.map(|p|p.x));let denominator=bx.sub_canonical(&ax);core::ptr::addr_of_mut!((*slot).xsum).write(FqBatch::from_point_domain(ax.add_canonical(&bx)));core::ptr::addr_of_mut!((*slot).denominator).write(FqBatch::from_point_domain(denominator));let ay=V::point_raw_host(&a.map(|p|p.y));core::ptr::addr_of_mut!((*slot).ay).write(FqBatch::from_point_domain(ay));let by=V::point_raw_host(&b.map(|p|p.y));let numerator=if SIGNED{by.point_signed_scaled(&ay,&chains,super::signs::<SIGNED,C>(&batch[k..k+8]))}else{by.sub_canonical(&ay).mul(&chains)};core::ptr::addr_of_mut!((*slot).numerator).write(FqBatch::from_point_domain(numerator));chains=chains.mul(&denominator);s.groups.set_len(k/8+1);}
    }
    s.tail.clear();let mut inverse=if full==batch.len(){inverse_quarter(chains)}else{
        let mut tail=chains.get();for k in full..batch.len(){let(i,b)=batch[k];let b=if SIGNED&&i>>31!=0{-b}else{b};let a=buckets[index::<SIGNED>(i)];let denominator=b.x-a.x;let numerator=(b.y-a.y)*tail[k%8];tail[k%8]*=denominator;s.tail.push([a.x,a.y,a.x+b.x,denominator,numerator]);}
        let mut inverse=super::invert_lanes(tail,true);for k in (full..batch.len()).rev(){let[ax,ay,xsum,denominator,numerator]=s.tail[k-full];let slope=numerator*inverse[k%8];inverse[k%8]*=denominator;let x=slope.square()-xsum;let y=slope*(ax-x)-ay;buckets[index::<SIGNED>(batch[k].0)]=Affine::new(x,y);}
        V::new(&inverse.map(|x|x*INV4))
    };
    for(j,g)in s.groups.iter().enumerate().rev(){let slope=g.numerator.point_domain().mul(&inverse);inverse=inverse.mul(&g.denominator.point_domain());let x=slope.mul(&slope).sub(&g.xsum.point_domain());let y=slope.point_mul_times4(&g.ax.point_domain().sub(&x)).sub(&g.ay.point_domain());let(x,y)=(x.point_raw_get(),y.point_raw_get());for i in 0..8{buckets[index::<SIGNED>(batch[j*8+i].0)]=Affine::new(x[i],y[i]);}}
}
#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::{Fr,G1Affine,G1Config,G1Projective};
    fn check<const SIGNED:bool>(){
        assert_eq!(INV4*Fq::from(4u64),Fq::ONE);assert_eq!(core::mem::size_of::<super::super::FqAddGroup>(),1600);assert_eq!(core::mem::size_of::<FqAddScratch>(),56);
        let mut s=FqAddScratch{quarter_enabled:true,write_groups:true,canonical:true,vector_inverse:true,..Default::default()};
        for n in [513,1,0,7,8,9,15,16,17,127,128,129,150,151,152,199,200,201,351,352,353,511,512]{
            let points:Vec<_>=(0..n).map(|i|G1Affine::GENERATOR*Fr::from((i+1)as u64)).collect();let mut a=G1Projective::batch_to_affine(&points);let addends:Vec<_>=points.iter().map(|p|*p+G1Affine::GENERATOR*Fr::from(100001u64)).collect();let b=G1Projective::batch_to_affine(&addends);let batch:Vec<_>=(0..n).map(|i|((n-1-i)as u32|if SIGNED&&i%3==0{1<<31}else{0},b[i])).collect();
            for round in 0..3{let mut expected=a.clone();for &(i,p)in &batch{let p=if SIGNED&&i>>31!=0{-p}else{p};let j=index::<SIGNED>(i);assert_ne!(a[j].x,p.x);expected[j]=(a[j]+p).to_affine();}super::super::stream_g1_impl::<G1Config,true,SIGNED>(&mut a,&batch,&mut s);assert_eq!(a,expected,"n={n} round={round}");}
        }
    }
    #[test]fn current_quarter_unsigned_tail_and_shared_scratch_reuse(){check::<false>();}
    #[test]fn current_quarter_signed_tail_and_shared_scratch_reuse(){check::<true>();}
    #[test]fn switching_kernels_reuses_the_same_record_allocation(){
        let n=128;let mut a=vec![G1Affine::GENERATOR;n];let p=(G1Affine::GENERATOR*Fr::from(17u64)).to_affine();let batch:Vec<_>=(0..n).map(|i|(i as u32,p)).collect();let mut s=FqAddScratch{quarter_enabled:false,write_groups:true,canonical:true,vector_inverse:true,..Default::default()};
        let mut expected=a.clone();for enabled in [false,true,false,true]{for v in &mut expected{*v=(*v+p).to_affine();}s.quarter_enabled=enabled;super::super::stream_g1_impl::<G1Config,true,false>(&mut a,&batch,&mut s);assert_eq!(a,expected);assert_eq!(s.groups.capacity(),16);}
    }
}
