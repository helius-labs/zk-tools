//! Prefix records for nonaliasing streaming batches; private component only.
use super::*;

pub(super) fn g1<C:CurveConfig<Base=Fq>,const FAST:bool,const SIGNED:bool>(
    buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut FqAddScratch,
) {
    let full=batch.len()/LANES*LANES;
    let mut chains=FqBatch::one();
    s.prefix_records.clear();s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES) {
        let ax=FqBatch::new(&core::array::from_fn(|i|buckets[batch_index::<SIGNED>(batch[k+i].0)].x));
        let bx=FqBatch::new(&core::array::from_fn(|i|batch[k+i].1.x));
        let denominator=canonical_sub::<FAST>(&bx,&ax);
        s.prefix_records.push(chains);chains=chains.mul(&denominator);
    }
    s.tail.clear();
    let mut inverse = if let Some(value)=direct_chain_inverse(&chains,full==batch.len(),s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..batch.len() {
            let (index, b) = batch[k];
            let b = if SIGNED && index>>31!=0 {-b}else{b};
            let a = buckets[batch_index::<SIGNED>(index)];
            let denominator = b.x - a.x;
            let numerator = (b.y - a.y) * tail[k % LANES];
            tail[k % LANES] *= denominator;
            s.tail.push([a.x, a.y, a.x + b.x, denominator, numerator]);
        }
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax, ay, xsum, denominator, numerator] = s.tail[k - full];
            let slope = numerator * inverse[k % LANES];
            inverse[k % LANES] *= denominator;
            let x = slope.square() - xsum;
            let y = slope * (ax - x) - ay;
            buckets[batch_index::<SIGNED>(batch[k].0)] = Affine::new(x, y);
        }
        FqBatch::new(&inverse)
    };
    for j in (0..full/LANES).rev() {
        let k=j*LANES;
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|buckets[batch_index::<SIGNED>(batch[k+i].0)]);
        let b:[Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);
        let ax=FqBatch::new(&a.map(|p|p.x));let ay=FqBatch::new(&a.map(|p|p.y));
        let bx=FqBatch::new(&b.map(|p|p.x));let by=FqBatch::new(&b.map(|p|p.y));
        let denominator=canonical_sub::<FAST>(&bx,&ax);
        let factor=s.prefix_records[j].mul(&inverse);
        inverse=inverse.mul(&denominator);
        let slope=scaled_signed_dy::<FAST,SIGNED>(&by,&ay,&factor,signs::<SIGNED,C>(&batch[k..k+LANES]));
        let x=slope.mul(&slope).sub(&canonical_add::<FAST>(&ax,&bx));
        let y=slope.mul(&ax.sub(&x)).sub(&ay);
        let(x,y)=(x.get(),y.get());
        for i in 0..LANES {buckets[batch_index::<SIGNED>(batch[k+i].0)]=Affine::new(x[i],y[i]);}
    }
}

pub(super) fn g2<C:CurveConfig<Base=Fq2>,const FAST:bool>(
    buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut Fq2AddScratch,
) {
    let full=batch.len()/LANES*LANES;
    let mut chains=FqBatch::one();
    s.prefix_records.clear();s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES) {
        let ax=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].x));
        let bx=fq2_pack(core::array::from_fn(|i|batch[k+i].1.x));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let norm=Fq2::lanes_norm(&denominator);
        s.prefix_records.push([chains,norm]);chains=chains.mul(&norm);
    }
    s.tail.clear();
    let mut inverse = if let Some(value)=direct_chain_inverse(&chains,full==batch.len(),s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..batch.len() {
            let (index, b) = batch[k];
            let a = buckets[index as usize];
            let denominator = b.x - a.x;
            let norm = denominator.c0.square() + denominator.c1.square();
            let numerator = ((b.y - a.y) * denominator.conjugate()).mul_by_fp(&tail[k % LANES]);
            tail[k % LANES] *= norm;
            s.tail
                .push([a.x, a.y, a.x + b.x, Fq2::new(norm, Fq::ZERO), numerator]);
        }
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax, ay, xsum, norm, numerator] = s.tail[k - full];
            let slope = numerator.mul_by_fp(&inverse[k % LANES]);
            inverse[k % LANES] *= norm.c0;
            let x = slope.square() - xsum;
            let y = slope * (ax - x) - ay;
            buckets[batch[k].0 as usize] = Affine::new(x, y);
        }
        FqBatch::new(&inverse)
    };
    for j in (0..full/LANES).rev() {
        let k=j*LANES;
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|buckets[batch[k+i].0 as usize]);
        let b:[Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);
        let ax=fq2_pack(a.map(|p|p.x));let ay=fq2_pack(a.map(|p|p.y));
        let bx=fq2_pack(b.map(|p|p.x));let by=fq2_pack(b.map(|p|p.y));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let dy=core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]));
        let factor=s.prefix_records[j][0].mul(&inverse);
        inverse=inverse.mul(&s.prefix_records[j][1]);
        let slope=Fq2::lanes_scaled_numerator(&dy,&denominator,&factor);
        let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&Fq2::lanes_add(&ax,&bx));
        let y=Fq2::lanes_sub(&Fq2::lanes_mul(&slope,&Fq2::lanes_sub(&ax,&x)),&ay);
        let(x,y)=(fq2_unpack(x),fq2_unpack(y));
        for i in 0..LANES {buckets[batch[k+i].0 as usize]=Affine::new(x[i],y[i]);}
    }
}

fn points<C:CurveConfig>(n:usize,offset:u64)->Vec<Affine<C>> {
    let generator=Projective::<C>::generator();let mut p=generator.mul(&Fr::from(offset));
    let mut out=Vec::with_capacity(n);for _ in 0..n{out.push(p);p+=generator;}
    Projective::<C>::batch_to_affine(&out)
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn prefix_g1_reference_signed_tails_and_reuse(){
        use zopro_bn254::G1Config;
        for n in [0,1,7,8,9,15,16,80,200,352,504]{
            let size=(n+1usize).next_power_of_two();let input=points::<G1Config>(size,7);let additions=points::<G1Config>(n,50007);
            for canonical in [false,true]{for vector_inverse in [false,true]{for signed in [false,true]{
                let batch:Vec<_>=additions.iter().enumerate().map(|(i,&p)|((i*37%size)as u32|if signed&&i%3==0{1<<31}else{0},p)).collect();
                let mut actual=input.clone();let mut expected=input.clone();let mut s=FqAddScratch{canonical,vector_inverse,..Default::default()};
                for _ in 0..3{
                    for &(id,p)in &batch{let index=(id&0x7fffffff)as usize;let p=if id>>31!=0{-p}else{p};assert_ne!(expected[index].x,p.x);expected[index]=(expected[index]+p).to_affine();}
                    if signed{if canonical{g1::<G1Config,true,true>(&mut actual,&batch,&mut s)}else{g1::<G1Config,false,true>(&mut actual,&batch,&mut s)}}else if canonical{g1::<G1Config,true,false>(&mut actual,&batch,&mut s)}else{g1::<G1Config,false,false>(&mut actual,&batch,&mut s)}
                    assert_eq!(actual,expected,"n={n} canonical={canonical} vector={vector_inverse} signed={signed}");assert_eq!(s.groups.capacity(),0);
                }
            }}}
        }
    }
    #[test]fn prefix_g2_reference_tails_and_reuse(){
        use zopro_bn254::G2Config;
        for n in [0,1,7,8,9,15,16,80,200,352,504]{
            let size=(n+1usize).next_power_of_two();let input=points::<G2Config>(size,7);let additions=points::<G2Config>(n,50007);
            for canonical in [false,true]{for vector_inverse in [false,true]{
                let batch:Vec<_>=additions.iter().enumerate().map(|(i,&p)|((i*37%size)as u32,if i%3==0{-p}else{p})).collect();
                let mut actual=input.clone();let mut expected=input.clone();let mut s=Fq2AddScratch{canonical,vector_inverse,..Default::default()};
                for _ in 0..3{
                    for &(id,p)in &batch{let index=id as usize;assert_ne!(expected[index].x,p.x);expected[index]=(expected[index]+p).to_affine();}
                    if canonical{g2::<G2Config,true>(&mut actual,&batch,&mut s)}else{g2::<G2Config,false>(&mut actual,&batch,&mut s)}
                    assert_eq!(actual,expected,"n={n} canonical={canonical} vector={vector_inverse}");assert_eq!(s.groups.capacity(),0);
                }
            }}
        }
    }
}

fn bench_g1(mode:u8,n:usize,iterations:usize)->(u64,usize,Vec<u8>){
    let mut buckets=points::<zopro_bn254::G1Config>(4096,7);
    let incoming=points::<zopro_bn254::G1Config>(n,50007);
    let batch:Vec<_>=incoming.into_iter().enumerate().map(|(i,p)|{let id=(i*37%buckets.len())as u32;(id|if i%3==0{1<<31}else{0},p)}).collect();
    let mut scratch=FqAddScratch{canonical:true,vector_inverse:true,write_groups:true,reserve_groups:true,..Default::default()};
    for _ in 0..3{if mode==0{stream_g1_impl::<zopro_bn254::G1Config,true,true>(&mut buckets,&batch,&mut scratch)}else{g1::<zopro_bn254::G1Config,true,true>(&mut buckets,&batch,&mut scratch)}}
    let started=std::time::Instant::now();
    for _ in 0..iterations{if mode==0{stream_g1_impl::<zopro_bn254::G1Config,true,true>(std::hint::black_box(&mut buckets),std::hint::black_box(&batch),&mut scratch)}else{g1::<zopro_bn254::G1Config,true,true>(std::hint::black_box(&mut buckets),std::hint::black_box(&batch),&mut scratch)}}
    let elapsed=started.elapsed().as_nanos()as u64;
    let bytes=if mode==0{scratch.groups.capacity()*std::mem::size_of_val(&scratch.groups[0])}else{scratch.prefix_records.capacity()*std::mem::size_of_val(&scratch.prefix_records[0])};
    let mut checksum=Vec::new();for p in buckets{for f in [p.x,p.y]{for v in [f]{for word in v.to_canonical(){checksum.extend_from_slice(&word.to_le_bytes());}}}}
    (elapsed,bytes,checksum)
}

fn bench_g2(mode:u8,n:usize,iterations:usize)->(u64,usize,Vec<u8>){
    let mut buckets=points::<zopro_bn254::G2Config>(2048,7);
    let incoming=points::<zopro_bn254::G2Config>(n,50007);
    let batch:Vec<_>=incoming.into_iter().enumerate().map(|(i,p)|{let id=(i*37%buckets.len())as u32;(id,if i%3==0{-p}else{p})}).collect();
    let mut scratch=Fq2AddScratch{canonical:true,vector_inverse:true, ..Default::default()};
    for _ in 0..3{if mode==0{stream_g2_impl::<zopro_bn254::G2Config,true>(&mut buckets,&batch,&mut scratch)}else{g2::<zopro_bn254::G2Config,true>(&mut buckets,&batch,&mut scratch)}}
    let started=std::time::Instant::now();
    for _ in 0..iterations{if mode==0{stream_g2_impl::<zopro_bn254::G2Config,true>(std::hint::black_box(&mut buckets),std::hint::black_box(&batch),&mut scratch)}else{g2::<zopro_bn254::G2Config,true>(std::hint::black_box(&mut buckets),std::hint::black_box(&batch),&mut scratch)}}
    let elapsed=started.elapsed().as_nanos()as u64;
    let bytes=if mode==0{scratch.groups.capacity()*std::mem::size_of_val(&scratch.groups[0])}else{scratch.prefix_records.capacity()*std::mem::size_of_val(&scratch.prefix_records[0])};
    let mut checksum=Vec::new();for p in buckets{for f in [p.x,p.y]{for v in [f.c0,f.c1]{for word in v.to_canonical(){checksum.extend_from_slice(&word.to_le_bytes());}}}}
    (elapsed,bytes,checksum)
}
pub(crate) fn benchmark(curve:u8,mode:u8,n:usize,iterations:usize)->(u64,usize,Vec<u8>){assert!(mode<=1&&n>0);match curve{1=>bench_g1(mode,n,iterations),2=>bench_g2(mode,n,iterations),_=>panic!("curve")}}
