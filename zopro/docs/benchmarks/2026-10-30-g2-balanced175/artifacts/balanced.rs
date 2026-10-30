//! Balanced Phi2 G2 spans; original Window and field arithmetic.
use super::*;
use rayon::prelude::*;
#[derive(Clone,Copy)]
struct Span{start:usize,width:usize}
fn spans()->[Span;11]{core::array::from_fn(|i|{let start=i*127/11;Span{start,width:(i+1)*127/11-start}})}
fn digit_at(s: &[u64; 4], span: Span) -> i16 {
    let Span { start, width } = span;
    if start == 0 {
        let value = s[0] & ((1u64 << width) - 1);
        return (value as i64 - ((value >> (width - 1)) << width) as i64) as i16;
    }
    let offset = start - 1;
    let (word, shift) = (offset / 64, offset % 64);
    let mut value = if word < 4 { s[word] >> shift } else { 0 };
    if shift + width + 1 > 64 && word < 3 {
        value |= s[word + 1] << (64 - shift);
    }
    value &= (1u64 << (width + 1)) - 1;
    (((value + 1) >> 1) as i64 - ((value >> width) << width) as i64) as i16
}
fn digit(s:&[u64;4],high:bool,span:Span)->i16{let i=usize::from(high)*2;let d=digit_at(&[s[i],s[i+1]&0x7fffffffffffffff,0,0],span);if s[i+1]>>63!=0{-d}else{d}}
fn feed<'a>(
        state: &mut Window<'a>,
        bases: &'a [G2Affine],
        short: &[[u64; 4]],
        high: bool,
        span: Span,
        size: usize,
    ) {
        for (p, s) in bases.iter().zip(short) {
            let d = digit(s, high, span);
            if d == 0 || p.is_identity() {
                continue;
            }
            let b = d.unsigned_abs() as usize - 1;
            let encoded = b as u32 | if d < 0 { 1 << 31 } else { 0 };
            if state.busy(b) {
                state.queue.push((encoded, p));
                if state.queue.len() == size - 1 {
                    state.flush_queue();
                }
                continue;
            }
            state.add(encoded, p);
            if state.batch.len() == size {
                state.execute();
                state.take_from_queue();
            }
        }
        state.execute();
        state.flush_queue();
    }
fn window(bases:&[G2Affine],short:&[[u64;4]],span:Span)->Vec<G2Affine>{let size=batch_size(span.width).max(32);let mut state=Window::new(1usize<<(span.width-1),size,BatchAdd::default());feed(&mut state,bases,short,true,span,size);for p in &mut state.affine{if !p.is_identity(){p.x=p.x.mul_by_fp(&BETA);}}for p in &mut state.projective{p.x=p.x.mul_by_fp(&BETA);}feed(&mut state,bases,short,false,span,size);state.finish()}
pub(crate) fn raw(bases:&[G2Affine],words:&mut[[u64;4]])->G2Projective{assert_eq!(bases.len(),words.len());if bases.is_empty(){return G2Projective::IDENTITY;}words.par_iter_mut().for_each(|s|*s=super::super::g2_phi::phi_split(*s));let cover=spans();let chunk=bases.len().div_ceil(2);let sums:Vec<_>=cover.par_iter().map(|&span|{let mut buckets:Vec<_>=(0..2).into_par_iter().map(|part|{let start=(part*chunk).min(bases.len());let end=(start+chunk).min(bases.len());window(&bases[start..end],&words[start..end],span)}).collect();let mut combined=buckets.swap_remove(0);for b in buckets{super::super::merge_buckets(&mut combined,&b);}crate::msm::weighted_affine(&combined)}).collect();let mut result=G2Projective::IDENTITY;for(span,sum)in cover.iter().zip(&sums).rev(){for _ in 0..span.width{result=result.double();}result+=*sum;}result}
#[cfg(test)]mod tests{
 use super::*;use zopro_bn254::Fr;
 fn random(seed:&mut u64)->Fr{let mut b=[0;32];for c in b.chunks_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;c.copy_from_slice(&seed.to_le_bytes());}Fr::from_le_bytes_mod_order(&b)}
 fn fixture(n:usize,mode:usize)->(Vec<G2Affine>,Vec<[u64;4]>,G2Projective){let mut seed=0x62616c616e636532u64;let mut coeff=vec![];let mut scalars=vec![];let mut expected=Fr::ZERO;for i in 0..n{let c=if mode==1{match i%7{0=>Fr::ZERO,1..=3=>Fr::ONE,4=>-Fr::ONE,_=>Fr::from((i%31+1)as u64)}}else{Fr::from((i*37+991)as u64)};let s=match (mode,i%9){(_,0)=>Fr::ZERO,(_,1)=>Fr::ONE,(_,2)=>-Fr::ONE,(2,_)=>Fr::from((i%17+1)as u64),_=>random(&mut seed)};expected+=c*s;coeff.push(G2Projective::generator()*c);scalars.push(s.to_canonical());}(G2Projective::batch_to_affine(&coeff),scalars,G2Projective::generator()*expected)}
 #[test]fn balanced_actual_curve_full_tail_sums(){for n in[0usize,1,7,8,9,31,199,200,201,1025]{let(p,mut s,want)=fixture(n,0);let before=p.clone();assert_eq!(raw(&p,&mut s),want,"n={n}");assert_eq!(p,before);}}
 #[test]fn balanced_actual_identity_double_opposite_and_sparse(){for mode in[1usize,2]{for n in[8usize,199,1025]{let(p,mut s,want)=fixture(n,mode);let before=p.clone();assert_eq!(raw(&p,&mut s),want,"mode={mode} n={n}");assert_eq!(p,before);}}}
 #[test]fn balanced_complete_msm_in_custom_pools(){let(p,words,want)=fixture(1025,1);let before=p.clone();for workers in[1usize,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();let mut s=words.clone();assert_eq!(pool.install(||raw(&p,&mut s)),want,"workers={workers}");assert_eq!(p,before);}}
}
