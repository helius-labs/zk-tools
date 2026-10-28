use zopro_bn254::{ifma8::Ifma8, Fp, FpConfig};
pub(crate) struct Batch<C: FpConfig>(Ifma8<C>);
impl<C: FpConfig> Batch<C> {
    #[inline(always)]
    pub(crate) fn new(v: &[Fp<C>; 8]) -> Self {
        Self(Ifma8::new(v))
    }
    #[inline(always)]
    pub(crate) fn get(&self) -> [Fp<C>; 8] {
        self.0.get()
    }
    #[inline(always)]
    fn mul(&self, b: &Self) -> Self {
        Self(self.0.mul(&b.0))
    }
    #[inline]
    pub(crate) fn inverse_nonzero(&self) -> Self {
        use zopro_bn254::ifma8::Ifma8;
        let swapped = Self(Ifma8::shuffle_pair::<1, 0, 3, 2>(&self.0, &self.0));
        let pairs = self.mul(&swapped);
        let other_pair = Self(Ifma8::shuffle_pair::<2, 3, 0, 1>(&pairs.0, &pairs.0));
        let quads = pairs.mul(&other_pair);
        let products = quads.get();
        let inverse = (products[0] * products[4]).inverse().unwrap();
        let (lo, hi) = quads.0.halves();
        let other_quad = Self(Ifma8::from_halves(&hi, &lo));
        Self::new(&[inverse; 8])
            .mul(&other_quad)
            .mul(&other_pair)
            .mul(&swapped)
    }
}
