//! Materialize norm inverses before packing the point-finishing inputs.
use super::*;

pub(super) fn apply<C:CurveConfig<Base=Fq2>,const FAST:bool>(
    buckets:&mut[Affine<C>],batch:&[(u32,&Affine<C>)],s:&mut Fq2AddScratch,
) {
    debug_assert!(batch.len()>=64);
    let full=batch.len()/LANES*LANES;
    s.prefix_records.clear();
    s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES) {
        let ax=fq2_pack(core::array::from_fn(|i|buckets[(batch[k+i].0&0x7fffffff)as usize].x));
        let bx=fq2_pack(core::array::from_fn(|i|batch[k+i].1.x));
        let dx=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        s.prefix_records.push([FqBatch::one(),Fq2::lanes_norm(&dx)]);
    }
    let mut roots=[FqBatch::one();4];
    for (j,record) in s.prefix_records.iter_mut().enumerate() {
        record[0]=roots[j%4];
        roots[j%4]=roots[j%4].mul(&record[1]);
    }
    let ab=roots[0].mul(&roots[1]);
    let cd=roots[2].mul(&roots[3]);
    let joint=ab.mul(&cd);
    s.tail.clear();
    let inverse=if let Some(value)=direct_chain_inverse(&joint,full==batch.len(),s.vector_inverse) {
        value
    } else {
        let mut tail=joint.get();
        for k in full..batch.len() {
            let (encoded,b)=batch[k];
            let b=if encoded>>31!=0{-*b}else{*b};
            let a=buckets[(encoded&0x7fffffff)as usize];
            let denominator=b.x-a.x;
            let norm=denominator.c0.square()+denominator.c1.square();
            let numerator=((b.y-a.y)*denominator.conjugate()).mul_by_fp(&tail[k%LANES]);
            tail[k%LANES]*=norm;
            s.tail.push([a.x,a.y,a.x+b.x,Fq2::new(norm,Fq::ZERO),numerator]);
        }
        let mut inverse=invert_lanes(tail,s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax,ay,xsum,norm,numerator]=s.tail[k-full];
            let slope=numerator.mul_by_fp(&inverse[k%LANES]);
            inverse[k%LANES]*=norm.c0;
            let x=slope.square()-xsum;
            let y=slope*(ax-x)-ay;
            buckets[(batch[k].0&0x7fffffff)as usize]=Affine::new(x,y);
        }
        FqBatch::new(&inverse)
    };
    let abi=inverse.mul(&cd);
    let cdi=inverse.mul(&ab);
    let mut inverses=[abi.mul(&roots[1]),abi.mul(&roots[0]),cdi.mul(&roots[3]),cdi.mul(&roots[2])];
    for (j,record) in s.prefix_records.iter_mut().enumerate().rev() {
        let factor=record[0].mul(&inverses[j%4]);
        inverses[j%4]=inverses[j%4].mul(&record[1]);
        record[0]=factor;
    }
    for j in (0..full/LANES).rev() {
        let k=j*LANES;
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|buckets[(batch[k+i].0&0x7fffffff)as usize]);
        let b:[&Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);
        let ax=fq2_pack(a.map(|p|p.x));let ay=fq2_pack(a.map(|p|p.y));
        let bx=fq2_pack(b.map(|p|p.x));
        let by=fq2_pack(core::array::from_fn(|i|if batch[k+i].0>>31!=0{-b[i].y}else{b[i].y}));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let dy=core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]));
        let factor=s.prefix_records[j][0];
        let slope=Fq2::lanes_scaled_numerator(&dy,&denominator,&factor);
        let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&Fq2::lanes_add(&ax,&bx));
        let y=Fq2::lanes_sub(&Fq2::lanes_mul(&slope,&Fq2::lanes_sub(&ax,&x)),&ay);
        let (x,y)=(fq2_unpack(x),fq2_unpack(y));
        for i in 0..LANES {buckets[(batch[k+i].0&0x7fffffff)as usize]=Affine::new(x[i],y[i]);}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::G2Config;
    fn points(n:usize,offset:u64)->Vec<Affine<G2Config>> {
        let g=Projective::<G2Config>::generator();
        Projective::batch_to_affine(&(0..n).map(|i|g*Fr::from(offset+i as u64)).collect::<Vec<_>>())
    }
    fn run<const FAST:bool>(n:usize,inverse:bool,s:&mut Fq2AddScratch) {
        s.vector_inverse=inverse;
        let size=n.next_power_of_two();
        let mut actual=points(size,12345);
        let incoming=points(n,900001);
        let batch:Vec<_>=incoming.iter().enumerate().map(|(i,p)|((i*37%size)as u32|if i%3==0{1<<31}else{0},p)).collect();
        let mut expected=actual.clone();
        let norms:Vec<_>=batch.iter().map(|&(id,p)|{let d=p.x-actual[(id&0x7fffffff)as usize].x;d.c0.square()+d.c1.square()}).collect();
        for &(id,p) in &batch {
            let i=(id&0x7fffffff)as usize;let p=if id>>31!=0{-*p}else{*p};
            expected[i]=(expected[i]+p).to_affine();
        }
        apply::<G2Config,FAST>(&mut actual,&batch,s);
        assert_eq!(actual,expected,"n={n} FAST={FAST} inverse={inverse}");
        for (j,record) in s.prefix_records.iter().enumerate() {
            let values=record[0].get();let stored=record[1].get();
            for i in 0..LANES {assert_eq!(stored[i],norms[j*LANES+i]);assert_eq!(values[i]*stored[i],Fq::ONE);}
        }
    }
    #[test]
    fn phased_norm_factors_and_complete_point_results() {
        for n in [64,65,71,72,73,200,203,352,355,511] {
            for inverse in [false,true] {
                run::<false>(n,inverse,&mut Fq2AddScratch::default());
                run::<true>(n,inverse,&mut Fq2AddScratch::default());
            }
        }
    }
    #[test]
    fn phased_records_reuse_capacity_without_stale_tail_state() {
        let mut scratch=Fq2AddScratch::default();
        run::<true>(512,true,&mut scratch);
        let pointer=scratch.prefix_records.as_ptr();let capacity=scratch.prefix_records.capacity();
        for n in [200,65,352,71,64,355,512,73] {
            run::<true>(n,true,&mut scratch);
            assert_eq!(scratch.prefix_records.as_ptr(),pointer);
            assert_eq!(scratch.prefix_records.capacity(),capacity);
            assert_eq!(scratch.groups.capacity(),0);
        }
    }
}
