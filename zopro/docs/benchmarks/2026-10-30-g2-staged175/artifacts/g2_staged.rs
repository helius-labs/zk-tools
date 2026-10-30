//! Bounded operator stages across several existing G2 reverse groups.
//! All operands, field formulas, inverse recurrence and scatter stay original.
use super::*;
use core::mem::{size_of,MaybeUninit};

struct Record{
    indices:[u32;LANES],
    ax:[FqBatch;2],ay:[FqBatch;2],bx:[FqBatch;2],
    dx:[FqBatch;2],dy:[FqBatch;2],factor:FqBatch,
}
// At most the existing 200-pair norm/prefix payload per leaf, independent of N.
// This is a cache tile, not a RAM cap or key-sized retained allocation.
const TILE:usize=(200/LANES*size_of::<[FqBatch;2]>())/size_of::<Record>();
const _:()=assert!(TILE>=2);

#[inline(never)]
fn slopes(tile:&mut[Record]){
    for r in tile{
        r.dx=Fq2::lanes_scaled_numerator(&r.dy,&r.dx,&r.factor);
    }
}
#[inline(never)]
fn xs(tile:&mut[Record]){
    for r in tile{
        r.bx=Fq2::lanes_sub(&Fq2::lanes_square(&r.dx),&Fq2::lanes_add(&r.ax,&r.bx));
    }
}
#[inline(never)]
fn finish<C:CurveConfig<Base=Fq2>>(buckets:&mut[Affine<C>],tile:&[Record]){
    for r in tile{
        let y=Fq2::lanes_sub(&Fq2::lanes_mul(&r.dx,&Fq2::lanes_sub(&r.ax,&r.bx)),&r.ay);
        let(x,y)=(fq2_unpack(r.bx),fq2_unpack(y));
        for i in 0..LANES{buckets[r.indices[i]as usize]=Affine::new(x[i],y[i]);}
    }
}

pub(super) fn reverse<C:CurveConfig<Base=Fq2>,const FAST:bool,F:Fn(usize)->(u32,Affine<C>)>(
    buckets:&mut[Affine<C>],full:usize,prefix:&[[FqBatch;2]],inverse:&mut FqBatch,incoming:F,
){
    assert_eq!(full%LANES,0);
    assert_eq!(prefix.len(),full/LANES);
    #[cfg(test)] HITS.fetch_add(full/LANES,std::sync::atomic::Ordering::Relaxed);
    let mut storage:[MaybeUninit<Record>;TILE]=[const{MaybeUninit::uninit()};TILE];
    let mut end=full/LANES;
    while end>0{
        let len=end.min(TILE);
        for(i,slot)in storage[..len].iter_mut().enumerate(){
            let j=end-1-i;let k=j*LANES;
            let pairs:[(u32,Affine<C>);LANES]=core::array::from_fn(|i|incoming(k+i));
            let indices=pairs.map(|p|p.0);
            let a:[Affine<C>;LANES]=indices.map(|i|buckets[i as usize]);
            let b=pairs.map(|p|p.1);
            let ax=fq2_pack(a.map(|p|p.x));let ay=fq2_pack(a.map(|p|p.y));
            let bx=fq2_pack(b.map(|p|p.x));let by=fq2_pack(b.map(|p|p.y));
            let dx=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
            let dy=core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]));
            let factor=prefix[j][0].mul(inverse);
            *inverse=inverse.mul(&prefix[j][1]);
            slot.write(Record{indices,ax,ay,bx,dx,dy,factor});
        }
        // SAFETY: exactly the first len<=TILE elements above have been fully
        // initialized as Records. MaybeUninit preserves Record layout/alignment.
        // The unique storage borrow covers this slice until all stages return;
        // no trailing uninitialized element is read. Record has no destructor.
        let tile=unsafe{core::slice::from_raw_parts_mut(storage.as_mut_ptr().cast::<Record>(),len)};
        slopes(tile);xs(tile);finish(buckets,tile);
        end-=len;
    }
}

#[cfg(test)]static HITS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::{G2Config,G2Affine,G2Projective};
    fn points(n:usize,start:u64)->Vec<G2Affine>{let g=G2Projective::generator();let mut p=g*Fr::from(start);let v:Vec<_>=(0..n).map(|_|{let q=p;p+=g;q}).collect();G2Projective::batch_to_affine(&v)}
    fn check(n:usize,signed:bool,canonical:bool,vector_inverse:bool){
        let size=n.max(1).next_power_of_two();let initial=points(size,73);let src=points(n,700003);let before=src.clone();let mut actual=initial.clone();let mut expected=initial;
        let mut s=Fq2AddScratch{canonical,vector_inverse,..Default::default()};
        let owned:Vec<_>=src.iter().enumerate().map(|(i,p)|((i*37%size)as u32,if signed&&i%3==0{-*p}else{*p})).collect();
        let borrowed:Vec<_>=src.iter().enumerate().map(|(i,p)|(((i*37%size)as u32)|if signed&&i%3==0{1<<31}else{0},p)).collect();
        for _ in 0..3{
            for &(i,p)in &owned{assert_ne!(expected[i as usize].x,p.x);expected[i as usize]=(expected[i as usize]+p).to_affine();}
            if signed{if canonical{g2_borrowed::apply::<G2Config,true>(&mut actual,&borrowed,&mut s)}else{g2_borrowed::apply::<G2Config,false>(&mut actual,&borrowed,&mut s)}}else if canonical{g2_prefix::apply::<G2Config,true>(&mut actual,&owned,&mut s)}else{g2_prefix::apply::<G2Config,false>(&mut actual,&owned,&mut s)}
            assert_eq!(actual,expected,"n={n} signed={signed} canonical={canonical} inverse={vector_inverse}");assert_eq!(src,before);
        }
    }
    #[test]fn staged_owned_and_signed_tails_and_reuse(){
        for n in[0,1,7,8,9,15,16,24,31,32,33,39,40,200,352,511,512,513]{for sign in[false,true]{for canonical in[false,true]{for inverse in[false,true]{check(n,sign,canonical,inverse);}}}}
        if cfg!(zopro_g2_staged){assert!(HITS.load(std::sync::atomic::Ordering::Relaxed)>0);}
        assert!(size_of::<[MaybeUninit<Record>;TILE]>()<=200/LANES*size_of::<[FqBatch;2]>());
    }
    #[test]fn staged_concurrent_small_stacks_and_partial_tiles(){
        for workers in[1,4,16,32]{rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap().install(||{use rayon::prelude::*;(0..workers*2).into_par_iter().for_each(|i|check([8,24,32,40,200,352][i%6],i%2==1,true,i%3!=0));});}
    }
}
