//! Experimental late multiplier application; only immutable key references are queued.
use super::{batch_size, BatchAdd, BETA, G2Affine, G2Config, G2Projective, Window};
use rayon::prelude::*;
type Codes = [u16; 20];
const NEG: u16 = 1 << 15;
const THREE: u16 = 1 << 14;
const MAG: u16 = (1 << 13) - 1;

#[inline]
fn decompose(t: i64) -> (i64, i64, i64) {
    let r = t.rem_euclid(8192);
    let x = r.min(8192-r);
    if x==0 {return (0,1,t/8192);}
    let odd=x>>x.trailing_zeros();
    let m=if matches!(odd%8,1|7) {1} else {3};
    let b=if m==1 {x} else {let z=r*2731%8192;z.min(8192-z)};
    let d=if m*b%8192==r {b} else {-b};
    (d,m,(t-m*d)/8192)
}
fn encode(words: [u64;4]) -> Codes {
    let split=super::super::g2_phi::phi_split(words);
    let mut out=[0;20];
    for (comp,index) in [2,0].into_iter().enumerate() {
        let negative=split[index+1]>>63!=0;
        let magnitude=split[index]as u128 | (((split[index+1]&0x7fff_ffff_ffff_ffff)as u128)<<64);
        let mut carry=0;
        for col in 0..10 {
            let t=((magnitude>>(13*col))&8191)as i64+carry;
            let (mut d,m,c)=if col==9 {(t,1,0)} else {decompose(t)};
            carry=c;
            if negative {d=-d;}
            debug_assert!(d.unsigned_abs()<=if col==9 {1024} else {4096});
            out[comp*10+col]=d.unsigned_abs()as u16 | if d<0 {NEG} else {0} | if m==3 {THREE} else {0};
        }
    }
    out
}
#[inline]
fn signed(code:u16)->i16 {
    let d=(code&MAG)as i16;
    if code&NEG!=0 {-d} else {d}
}
impl<'a> Window<'a> {
    fn late_feed(&mut self,bases:&'a[G2Affine],codes:&[Codes],comp:usize,col:usize,m3:bool,size:usize) {
        for (p,s) in bases.iter().zip(codes) {
            let code=s[comp*10+col];
            if (code&THREE!=0)!=m3 || code&MAG==0 || p.is_identity() {continue;}
            let b=(code&MAG)as usize-1;
            let encoded=b as u32 | if code&NEG!=0 {1<<31} else {0};
            if self.busy(b) {
                self.queue.push((encoded,p));
                if self.queue.len()==size-1 {self.flush_queue();}
                continue;
            }
            self.add(encoded,p);
            if self.batch.len()==size {self.execute();self.take_from_queue();}
        }
        self.execute();self.flush_queue();
    }
}
fn ordinary(bases:&[G2Affine],codes:&[Codes],col:usize,m3:bool)->Vec<G2Affine> {
    let size=batch_size(13).max(32);
    let mut state=Window::new(4096,size,BatchAdd::<G2Config>::default());
    state.late_feed(bases,codes,0,col,m3,size);
    for p in &mut state.affine {if !p.is_identity() {p.x=p.x.mul_by_fp(&BETA);}}
    for p in &mut state.projective {p.x=p.x.mul_by_fp(&BETA);}
    state.late_feed(bases,codes,1,col,m3,size);
    state.finish()
}
// Same signed indexed reduction as the original leading component; only digits differ.
fn top_component(bases:&[G2Affine],codes:&[Codes],comp:usize)->Vec<G2Affine> {
    let n=1024;
    let bucket=|d:i16|d.unsigned_abs()as usize-1+if d<0 {n} else {0};
    let mut counts=vec![0usize;2*n];
    for (base,s) in bases.iter().zip(codes) {
        let d=signed(s[comp*10+9]);assert!(d.unsigned_abs()as usize<=n);
        if d!=0&&!base.is_identity() {counts[bucket(d)]+=1;}
    }
    let mut starts=Vec::with_capacity(2*n+1);starts.push(0);
    for count in counts {starts.push(starts.last().unwrap()+count);}
    let len=*starts.last().unwrap();
    if len==0 {return vec![G2Affine::IDENTITY;n];}
    let mut positions=starts[..2*n].to_vec();let mut order=vec![0u32;len];
    for (i,(base,s)) in bases.iter().zip(codes).enumerate() {
        let d=signed(s[comp*10+9]);
        if d!=0&&!base.is_identity() {let b=bucket(d);order[positions[b]]=i as u32;positions[b]+=1;}
    }
    let mut reduced=crate::msm::reduce_indexed_buckets(bases,&order,&starts);
    let mut negative=reduced.split_off(n);
    for p in &mut negative {p.y=-p.y;}
    super::super::merge_buckets(&mut reduced,&negative);reduced
}
fn top(bases:&[G2Affine],codes:&[Codes])->Vec<G2Affine> {
    let mut high=top_component(bases,codes,0);
    for p in &mut high {if !p.is_identity() {p.x=p.x.mul_by_fp(&BETA);}}
    let low=top_component(bases,codes,1);
    super::super::merge_buckets(&mut high,&low);high
}
fn plane(bases:&[G2Affine],codes:&[Codes],col:usize,m3:bool)->G2Projective {
    let chunk=bases.len().div_ceil(2);
    let mut buckets:Vec<_>=(0..2).into_par_iter().map(|part| {
        let start=(part*chunk).min(bases.len());let end=(start+chunk).min(bases.len());
        if col==9 {top(&bases[start..end],&codes[start..end])}
        else {ordinary(&bases[start..end],&codes[start..end],col,m3)}
    }).collect();
    let mut combined=buckets.swap_remove(0);
    for b in buckets {super::super::merge_buckets(&mut combined,&b);}
    crate::msm::weighted_affine(&combined)
}
fn admitted(bases:&[G2Affine],words:&[[u64;4]],peak:Option<usize>)->Option<(G2Projective,usize)> {
    assert_eq!(bases.len(),words.len());
    if !BatchAdd::<G2Config>::default().borrowed_g2_enabled() {return None;}
    let peak=peak?;
    // No optional allocations or input mutation precede admission; the token
    // remains live through every fill, nested reduction and final Horner pass.
    let _owner=crate::preparation::Reservation::analysis(Some(peak))?;
    let codes:Vec<Codes>=words.par_iter().map(|w|encode(*w)).collect();
    let sums:Vec<_>=(0..10).into_par_iter().map(|col| {
        let one=plane(bases,&codes,col,false);
        if col==9 {one} else {let three=plane(bases,&codes,col,true);one+three.double()+three}
    }).collect();
    let mut result=G2Projective::IDENTITY;
    for sum in sums.into_iter().rev() {for _ in 0..13 {result=result.double();}result+=sum;}
    Some((result,peak))
}
pub(super) fn try_query(bases:&[G2Affine],words:&[[u64;4]])->Option<(G2Projective,usize)> {
    admitted(bases,words,super::super::workspace::late_m3_peak(bases.len(),rayon::current_num_threads()))
}

#[cfg(test)]mod tests {
    use super::*;
    use zopro_bn254::{Fr,Field};
    fn random(seed:&mut u64)->Fr {
        let mut b=[0;32];for c in b.chunks_mut(8) {*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;c.copy_from_slice(&seed.to_le_bytes());}
        Fr::from_le_bytes_mod_order(&b)
    }
    #[test]fn digit_encoding_reconstructs_components_and_carries() {
        for t in -1..=8193 {let(d,m,c)=decompose(t);assert_eq!(t,d*m+8192*c);assert!((-1..=2).contains(&c));if d!=0 {let x=d.unsigned_abs();assert!(matches!((x>>x.trailing_zeros())%8,1|7));}}
        let mut seed=0x6d335f636f646573;
        let mut inputs=vec![Fr::ZERO,Fr::ONE,-Fr::ONE];
        for bit in 0..254 {let p=Fr::from(2u64).pow(&[bit]);inputs.extend([p-Fr::ONE,p,p+Fr::ONE]);}
        inputs.extend((0..16384).map(|_|random(&mut seed)));
        for s in inputs {
            let split=super::super::super::g2_phi::phi_split(s.to_canonical());let codes=encode(s.to_canonical());
            for (comp,index) in [2,0].into_iter().enumerate() {
                let mag=split[index]as u128|(((split[index+1]&0x7fff_ffff_ffff_ffff)as u128)<<64);
                let expected=if split[index+1]>>63!=0 {-(mag as i128)} else {mag as i128};
                let actual=(0..10).rev().fold(0i128,|v,col|v*8192+signed(codes[comp*10+col])as i128*if codes[comp*10+col]&THREE!=0 {3} else {1});
                assert_eq!(actual,expected);assert_eq!(codes[comp*10+9]&THREE,0);
            }
        }
    }
    #[test]fn complete_queries_match_generator_oracle_at_thread_and_tail_boundaries() {
        let g=G2Projective::generator();let mut seed=0x6d335f7175657279;
        for n in [1024usize,2051] {
            let coefficients=(0..n).map(|i|match i%9 {0=>Fr::ZERO,1..=3=>Fr::ONE,4=>-Fr::ONE,_=>Fr::from((i+1234567)as u64)}).collect::<Vec<_>>();
            let bases=G2Projective::batch_to_affine(&coefficients.iter().map(|c|g**c).collect::<Vec<_>>());
            let scalars=(0..n).map(|i|match i%11 {0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,3=>Fr::from(2u64),4=>-Fr::from(2u64),_=>random(&mut seed)}).collect::<Vec<_>>();
            let expected=g*scalars.iter().zip(&coefficients).fold(Fr::ZERO,|a,(s,c)|a+*s**c);
            let words=scalars.iter().map(|s|s.to_canonical()).collect::<Vec<_>>();let original=bases.clone();
            for workers in [1,4,16,32] {
                let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();
                let (out,_)=pool.install(||try_query(&bases,&words).expect("native scratch admitted"));
                assert_eq!(out.to_affine(),expected.to_affine());assert_eq!(bases,original);
            }
        }
    }
    #[test]fn resource_overflow_denial_and_empty_planes_preserve_inputs() {
        assert!(super::super::super::workspace::late_m3_peak(usize::MAX,32).is_none());
        let bases=vec![G2Affine::IDENTITY;1024];let words=vec![[1,0,0,0];1024];let original=words.clone();
        assert!(admitted(&bases,&words,None).is_none());assert!(admitted(&bases,&words,Some(usize::MAX)).is_none());
        let pool=rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
        let(out,_) = pool.install(||try_query(&bases,&words).unwrap());assert!(out.is_identity());assert_eq!(words,original);
        let bases=vec![G2Projective::generator().to_affine();1024];let words=vec![[0;4];1024];
        let(out,_) = pool.install(||try_query(&bases,&words).unwrap());assert!(out.is_identity());
    }
}
