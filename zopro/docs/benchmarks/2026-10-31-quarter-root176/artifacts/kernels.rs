// Original mathematical butterfly bodies extracted verbatim; new products below.
#[inline(always)]
fn dif_within(a: &mut Pair, b: &mut Pair, pair: &Pair) {
    // Two apart: (a0 a1 b0 b1) with (a2 a3 b2 b3), twiddles (1, t, 1, t).
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 1, 4, 5>(a, b),
        Pair::shuffle_pair::<2, 3, 6, 7>(a, b),
    );
    let sum = lo.add(&hi);
    let difference = lo.sub(&hi).mul(pair);
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &difference);
    // One apart: (a0 a2 b0 b2) with (a1 a3 b1 b3), twiddle one.
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
}
#[inline(always)]
fn dit_within(a: &mut Pair, b: &mut Pair, pair: &Pair, single: Option<&Pair>) {
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
    let hi = single.map_or(hi, |root| hi.mul(root));
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 1, 4, 5>(a, b),
        Pair::shuffle_pair::<2, 3, 6, 7>(a, b),
    );
    let product = hi.mul(pair);
    let (sum, difference) = (lo.add(&product), lo.sub(&product));
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &difference);
}
#[inline(always)]
fn dif_odd<const FIXED:bool>(a: &mut Pair, b: &mut Pair, pair: &Pair) {
    // Two apart: (a0 a1 b0 b1) with (a2 a3 b2 b3), twiddles (1, t, 1, t).
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 1, 4, 5>(a, b),
        Pair::shuffle_pair::<2, 3, 6, 7>(a, b),
    );
    let sum = lo.add(&hi);
    let difference = quarter::<FIXED,true>(&lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &difference);
    // One apart: (a0 a2 b0 b2) with (a1 a3 b1 b3), twiddle one.
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
}
#[inline(always)]
fn dit_odd<const FIXED:bool>(a: &mut Pair, b: &mut Pair, pair: &Pair, single: Option<&Pair>) {
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
    let hi = single.map_or(hi, |root| hi.mul(root));
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 1, 4, 5>(a, b),
        Pair::shuffle_pair::<2, 3, 6, 7>(a, b),
    );
    let product = quarter::<FIXED,false>(&hi);
    let (sum, difference) = (lo.add(&product), lo.sub(&product));
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &difference);
}

const SMALL:Fr=Fr::from_canonical([2364373463638985163,14648454017743979685,12953715498635827032,0]).unwrap();
#[inline(always)]fn quarter<const FIXED:bool,const NEGATIVE:bool>(value:&Pair)->Pair{
 let(lo,hi)=value.halves();let even=Quad::shuffle::<0,2,4,6>(&lo,&hi);let odd=Quad::shuffle::<1,3,5,7>(&lo,&hi);
 let product=if FIXED{
  #[cfg(quarter_root)]{odd.mul_quarter_small()}
  #[cfg(not(quarter_root))]{panic!("new quarter-root method unavailable")}
 }else{odd.mul(&Quad::new(&[SMALL;4]))};let product=if NEGATIVE{Quad::zero().sub(&product)}else{product};
 Pair::from_halves(&Quad::shuffle::<0,4,1,5>(&even,&product),&Quad::shuffle::<2,6,3,7>(&even,&product))
}
struct Roots{forward:Pair,inverse:Pair}
#[inline(never)]fn ordinary(values:&mut[Pair],roots:&Roots){for pair in values.as_chunks_mut::<2>().0{let(a,b)=pair.split_at_mut(1);dif_within(&mut a[0],&mut b[0],&roots.forward);dit_within(&mut a[0],&mut b[0],&roots.inverse,None);}}
#[inline(never)]fn odd(values:&mut[Pair],roots:&Roots){for pair in values.as_chunks_mut::<2>().0{let(a,b)=pair.split_at_mut(1);dif_odd::<false>(&mut a[0],&mut b[0],&roots.forward);dit_odd::<false>(&mut a[0],&mut b[0],&roots.inverse,None);}}
#[inline(never)]fn fixed(values:&mut[Pair],roots:&Roots){for pair in values.as_chunks_mut::<2>().0{let(a,b)=pair.split_at_mut(1);dif_odd::<true>(&mut a[0],&mut b[0],&roots.forward);dit_odd::<true>(&mut a[0],&mut b[0],&roots.inverse,None);}}
fn roots()->Roots{let i=Fr::root_of_unity(4).unwrap();assert_eq!(i,-SMALL);Roots{forward:Pair::new(&[Fr::ONE,i,Fr::ONE,i,Fr::ONE,i,Fr::ONE,i]),inverse:Pair::new(&[Fr::ONE,SMALL,Fr::ONE,SMALL,Fr::ONE,SMALL,Fr::ONE,SMALL])}}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn one_way_dif_matches_independent_four_point_dft(){let roots=roots();let i=Fr::root_of_unity(4).unwrap();for seed in 0..64u64{let input:[Fr;16]=std::array::from_fn(|j|if j%7==0{Fr::ZERO}else{Fr::from(seed+j as u64+1).pow(&[233])});for mode in 0..3{let(mut a,mut b)=(Pair::new(input[..8].try_into().unwrap()),Pair::new(input[8..].try_into().unwrap()));match mode{0=>dif_within(&mut a,&mut b,&roots.forward),1=>dif_odd::<false>(&mut a,&mut b,&roots.forward),_=>dif_odd::<true>(&mut a,&mut b,&roots.forward)}let got=[a.get(),b.get()].concat();for poly in 0..4{for out in 0..4{let k=(out>>1)|((out&1)<<1);let expected=(0..4).fold(Fr::ZERO,|sum,j|sum+input[poly*4+j]*i.pow(&[(j*k)as u64]));assert_eq!(got[poly*4+out],expected);}}match mode{0=>dit_within(&mut a,&mut b,&roots.inverse,None),1=>dit_odd::<false>(&mut a,&mut b,&roots.inverse,None),_=>dit_odd::<true>(&mut a,&mut b,&roots.inverse,None)}let got=[a.get(),b.get()].concat();for j in 0..16{assert_eq!(got[j],input[j]*Fr::from(4u64));}}}}
 #[test]fn buffers_and_dependent_cycles_preserve_scalar_oracles(){let roots=roots();for n in[2usize,6,64,1024]{let input=(0..n).map(|j|std::array::from_fn(|i|Fr::from((i+j*8+1)as u64).pow(&[251]))).collect::<Vec<[Fr;8]>>();for kernel in[ordinary as fn(&mut[Pair],&Roots),odd,fixed]{let mut values=input.iter().map(Pair::new).collect::<Vec<_>>();for pass in 1..=7{kernel(&mut values,&roots);let factor=Fr::from(4u64).pow(&[pass]);for(v,p)in values.iter().zip(&input){assert_eq!(v.get(),p.map(|s|s*factor));}}}}}
}
