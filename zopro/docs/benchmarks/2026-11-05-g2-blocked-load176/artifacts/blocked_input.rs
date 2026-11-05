//! Same-payload blocked G2 coordinate words. Native codec experiment only.
use core::{arch::x86_64::*,marker::PhantomData};
use crate::{Fq,FqConfig,FpConfig,ifma8::Ifma8};
pub type Pair=[Ifma8<FqConfig,true>;2];
/// Borrowed raw words, validated below p, blocked eight points at a time.
/// A partial final block uses its actual length as the word-plane stride.
pub struct Blocked<'a>{words:&'a[u64],points:usize}
impl<'a> Blocked<'a>{
    pub fn encode(points:&'a mut [[u64;16]])->Result<Self,&'static str>{
        // Validate before the first write, so rejected input is unchanged.
        for p in points.iter(){for f in p.chunks_exact(4){let v:[u64;4]=f.try_into().unwrap();if !crate::portable::less(v,FqConfig::P){return Err("noncanonical coordinate word value");}}}
        let n=points.len();
        for block in points.chunks_mut(8){
            let k=block.len();let mut tile=[[0u64;16];8];tile[..k].copy_from_slice(block);
            let flat=block.as_flattened_mut();for w in 0..16{for lane in 0..k{flat[w*k+lane]=tile[lane][w];}}
        }
        Ok(Self{words:points.as_flattened(),points:n})
    }
    pub fn checked(words:&'a[u64],points:usize)->Option<Self>{
        if points.checked_mul(16)?!=words.len(){return None;}
        let view=Self{words,points};
        for i in 0..points{let p=view.point_words(i);for f in p.chunks_exact(4){if !crate::portable::less(f.try_into().unwrap(),FqConfig::P){return None;}}}
        Some(view)
    }
    pub fn points(&self)->usize{self.points}
    pub fn payload_words(&self)->usize{self.words.len()}
    #[inline]pub fn point_words(&self,i:usize)->[u64;16]{
        assert!(i<self.points);let block=i/8;let k=(self.points-block*8).min(8);
        core::array::from_fn(|w|self.words[block*128+w*k+i%8])
    }
    #[inline(always)]fn word(&self,block:usize,w:usize)->__m512i{
        let k=(self.points-block*8).min(8);let offset=block*128+w*k;
        // SAFETY: pack checks every source index. block therefore exists,
        // w<16, and this word plane has exactly k initialized u64 values.
        // The partial mask reads only those k lanes, including the last word.
        unsafe{if k==8{_mm512_loadu_si512(self.words.as_ptr().add(offset).cast())}else{_mm512_maskz_loadu_epi64(((1u16<<k)-1)as u8,self.words.as_ptr().add(offset).cast())}}
    }
    /// FIELD=0 selects X, FIELD=2 selects Y; both components are returned.
    #[inline(always)]pub fn pack<const FIELD:usize>(&self,indices:[usize;8])->Pair{
        const{assert!(FIELD==0||FIELD==2);}
        for i in indices{assert!(i<self.points,"source index outside blocked root");}
        let b0=indices[0]/8;let second=indices.iter().map(|i|i/8).find(|&b|b!=b0);
        let columns:[__m512i;8];
        // SAFETY: native module guards require AVX512 IFMA/VL/DQ. All source
        // ordinals were checked above; word() handles partial last blocks.
        // Each permute index selects a lane in its one or two supplied vectors.
        unsafe{
            if let Some(b1)=second{
                if indices.iter().all(|i|i/8==b0||i/8==b1){
                    let lane:[u64;8]=core::array::from_fn(|j|(indices[j]%8+8*usize::from(indices[j]/8==b1))as u64);
                    let lane=_mm512_loadu_si512(lane.as_ptr().cast());
                    columns=core::array::from_fn(|j|_mm512_permutex2var_epi64(self.word(b0,FIELD*4+j),lane,self.word(b1,FIELD*4+j)));
                }else{
                    let base:[u64;8]=core::array::from_fn(|j|((indices[j]/8)*128+indices[j]%8)as u64);
                    let stride:[u64;8]=core::array::from_fn(|j|(self.points-(indices[j]/8)*8).min(8)as u64);
                    let base=_mm512_loadu_si512(base.as_ptr().cast());let stride=_mm512_loadu_si512(stride.as_ptr().cast());
                    let mut position=_mm512_add_epi64(base,_mm512_mullo_epi64(stride,_mm512_set1_epi64((FIELD*4)as i64)));
                    columns=core::array::from_fn(|_|{let v=_mm512_i64gather_epi64::<8>(position,self.words.as_ptr().cast());position=_mm512_add_epi64(position,stride);v});
                }
            }else if indices.iter().enumerate().all(|(j,&i)|i==b0*8+j){
                columns=core::array::from_fn(|j|self.word(b0,FIELD*4+j));
            }else{
                let lane:[u64;8]=indices.map(|i|(i%8)as u64);let lane=_mm512_loadu_si512(lane.as_ptr().cast());
                columns=core::array::from_fn(|j|_mm512_permutexvar_epi64(lane,self.word(b0,FIELD*4+j)));
            }
        }
        [from_columns(columns[..4].try_into().unwrap()),from_columns(columns[4..].try_into().unwrap())]
    }
}
#[inline(always)]fn from_columns(w:[__m512i;4])->Ifma8<FqConfig,true>{
    // SAFETY: all lanes are original canonical Montgomery coordinate words.
    // This is the same bit split as Ifma8::raw/PackedIfma8::expand. Each digit
    // is normalized; the top digit fits 48 bits and the full value stays <p.
    unsafe{let [a,b,c,d]=w;let mask=_mm512_set1_epi64(((1u64<<52)-1)as i64);Ifma8([
        _mm512_and_si512(a,mask),
        _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<52>(a),_mm512_slli_epi64::<12>(b)),mask),
        _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<40>(b),_mm512_slli_epi64::<24>(c)),mask),
        _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<28>(c),_mm512_slli_epi64::<36>(d)),mask),
        _mm512_srli_epi64::<16>(d)],PhantomData)}
}
/// Standalone original-constructor comparator, not a new curve API.
#[inline(always)]pub fn pack_aos<const FIELD:usize>(points:&[[u64;16]],indices:[usize;8])->Pair{
    const{assert!(FIELD==0||FIELD==2);}
    core::array::from_fn(|part|Ifma8::new(&core::array::from_fn(|i|{
        let p=&points[indices[i]];let w=(FIELD+part)*4;Fq::from_montgomery(p[w..w+4].try_into().unwrap())
    })))
}
#[cfg(test)]mod tests{
    use super::*;
    fn points(n:usize)->Vec<[u64;16]>{
        let p=FqConfig::P;let pm1=crate::portable::sub4(p,[1,0,0,0]).0;
        let cases=[[0;4],[1,0,0,0],pm1,FqConfig::R,[u64::MAX,0,0,0],[0,u64::MAX,0,0],[0,0,u64::MAX,0]];
        let mut state=Fq::from(72913u64);
        (0..n).map(|i|{let mut row=[0;16];for f in 0..4{state=state*Fq::from(6364136223846793005u64)+Fq::from(1u64);let words=if i<cases.len(){cases[(i+f)%cases.len()]}else{state.0};row[f*4..f*4+4].copy_from_slice(&words);}row}).collect()
    }
    fn check(view:&Blocked<'_>,aos:&[[u64;16]],ids:[usize;8]){
        for field in[0,2]{let old=if field==0{pack_aos::<0>(aos,ids)}else{pack_aos::<2>(aos,ids)};let got=if field==0{view.pack::<0>(ids)}else{view.pack::<2>(ids)};
            for part in 0..2{
                let a=old[part].get();let b=got[part].get();assert_eq!(a,b);
                for lane in 0..8{let expected:[u64;4]=aos[ids[lane]][(field+part)*4..(field+part+1)*4].try_into().unwrap();assert_eq!(b[lane].0,expected);}
                for digit in 0..5{let a:[u64;8]=unsafe{core::mem::transmute(old[part].0[digit])};let b:[u64;8]=unsafe{core::mem::transmute(got[part].0[digit])};assert_eq!(a,b);assert!(b.iter().all(|&v|v<(1u64<<if digit==4{48}else{52})));}
            }
        }
    }
    #[test]fn exact_storage_tail_bounds_and_rejection_before_mutation(){
        for n in[0,1,7,8,9,15,16,17,31,32,33,200,352,511,512]{let original=points(n);let mut changed=original.clone();let address=changed.as_ptr();let cap=changed.capacity();{
            let view=Blocked::encode(&mut changed).unwrap();assert_eq!(view.points(),n);assert_eq!(view.payload_words(),16*n);for i in 0..n{assert_eq!(view.point_words(i),original[i]);}
            if n>0{check(&view,&original,[n-1;8]);}let bad=std::panic::catch_unwind(||view.pack::<0>([n;8]));assert!(bad.is_err());
        }assert_eq!(changed.as_ptr(),address);assert_eq!(changed.capacity(),cap);let view=Blocked::checked(changed.as_flattened(),n).unwrap();for i in 0..n{assert_eq!(view.point_words(i),original[i]);}assert!(Blocked::checked(changed.as_flattened(),usize::MAX).is_none());}
        let mut invalid=points(9);invalid[8][12..16].copy_from_slice(&FqConfig::P);let before=invalid.clone();assert!(Blocked::encode(&mut invalid).is_err());assert_eq!(invalid,before);
    }
    #[test]fn original_constructor_and_scalar_word_oracles_all_dispatch_classes(){
        let original=points(257);let mut changed=original.clone();let view=Blocked::encode(&mut changed).unwrap();
        for start in 0..250{check(&view,&original,core::array::from_fn(|i|start+i));}
        for block in 0..32{check(&view,&original,core::array::from_fn(|i|block*8+7-i));check(&view,&original,[block*8+3;8]);}
        for blocks in 1..=8{for offset in 0..8{check(&view,&original,core::array::from_fn(|i|(i%blocks)*8+(7-i+offset)%8));}}
        check(&view,&original,[256,0,8,16,24,32,40,48]);check(&view,&original,[256,255,256,254,256,253,256,252]);
        let mut seed=918273u64;for _ in 0..8192{let ids=core::array::from_fn(|_|{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;(seed as usize)%original.len()});check(&view,&original,ids);}
    }
}
