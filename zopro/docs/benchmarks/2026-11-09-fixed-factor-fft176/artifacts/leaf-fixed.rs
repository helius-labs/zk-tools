// Original shuffle graph; only its twiddle products use fixed factors.
#[inline(always)]
fn fixed_dif_quads(lo:&mut[Pair],hi:&mut[Pair],roots:&[super::super::fixed_roots::FixedQuad]) {
    #[cfg(zopro_fixed_fft_diagnostics)] super::super::fixed_roots::diag::product(false,lo.len());
    for j in 0..lo.len() {let difference=lo[j].sub(&hi[j]);lo[j]=lo[j].add(&hi[j]);hi[j]=super::super::fixed_roots::expand(&roots[j]).mul(&difference);}
}
#[inline(always)]
fn fixed_dit_quads(lo:&mut[Pair],hi:&mut[Pair],roots:&[super::super::fixed_roots::FixedQuad]) {
    #[cfg(zopro_fixed_fft_diagnostics)] super::super::fixed_roots::diag::product(true,lo.len());
    for j in 0..lo.len() {let product=super::super::fixed_roots::expand(&roots[j]).mul(&hi[j]);hi[j]=lo[j].sub(&product);lo[j]=lo[j].add(&product);}
}
#[inline(always)]
fn fixed_dif_within(a:&mut Pair,b:&mut Pair,pair:&super::super::fixed_roots::FixedQuad) {
    #[cfg(zopro_fixed_fft_diagnostics)] super::super::fixed_roots::diag::product(false,1);
    let(lo,hi)=(Pair::shuffle_pair::<0,1,4,5>(a,b),Pair::shuffle_pair::<2,3,6,7>(a,b));
    let sum=lo.add(&hi);let difference=super::super::fixed_roots::expand(pair).mul(&lo.sub(&hi));
    *a=Pair::shuffle_pair::<0,1,4,5>(&sum,&difference);*b=Pair::shuffle_pair::<2,3,6,7>(&sum,&difference);
    let(lo,hi)=(Pair::shuffle_pair::<0,2,4,6>(a,b),Pair::shuffle_pair::<1,3,5,7>(a,b));
    let(sum,difference)=(lo.add(&hi),lo.sub(&hi));
    *a=Pair::shuffle_pair::<0,4,1,5>(&sum,&difference);*b=Pair::shuffle_pair::<2,6,3,7>(&sum,&difference);
}
#[inline(always)]
fn fixed_dit_within(a:&mut Pair,b:&mut Pair,pair:&super::super::fixed_roots::FixedQuad,single:Option<&super::super::fixed_roots::FixedQuad>) {
    #[cfg(zopro_fixed_fft_diagnostics)] super::super::fixed_roots::diag::product(true,1+usize::from(single.is_some()));
    let(lo,hi)=(Pair::shuffle_pair::<0,2,4,6>(a,b),Pair::shuffle_pair::<1,3,5,7>(a,b));
    let hi=single.map_or(hi,|r|super::super::fixed_roots::expand(r).mul(&hi));
    let(sum,difference)=(lo.add(&hi),lo.sub(&hi));
    *a=Pair::shuffle_pair::<0,4,1,5>(&sum,&difference);*b=Pair::shuffle_pair::<2,6,3,7>(&sum,&difference);
    let(lo,hi)=(Pair::shuffle_pair::<0,1,4,5>(a,b),Pair::shuffle_pair::<2,3,6,7>(a,b));
    let product=super::super::fixed_roots::expand(pair).mul(&hi);let(sum,difference)=(lo.add(&product),lo.sub(&product));
    *a=Pair::shuffle_pair::<0,1,4,5>(&sum,&difference);*b=Pair::shuffle_pair::<2,3,6,7>(&sum,&difference);
}
