//! Complete a=0 short-Weierstrass accumulation using cached Z² and Z³.
//! Sutherland's 2008 formulas, as documented in the Explicit-Formulas Database:
//! https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html
use zopro_bn254::{Affine, CurveConfig, Field};
#[derive(Clone, Copy, Debug)]
pub(crate) struct Xyzz<C: CurveConfig> {
    pub(crate) x: C::Base,
    pub(crate) y: C::Base,
    pub(crate) zz: C::Base,
    pub(crate) zzz: C::Base,
}
impl<C: CurveConfig> Xyzz<C> {
    pub(crate) fn identity() -> Self {
        Self {
            x: C::Base::ZERO,
            y: C::Base::ZERO,
            zz: C::Base::ZERO,
            zzz: C::Base::ZERO,
        }
    }
    pub(crate) fn is_identity(&self) -> bool {
        self.zz.is_zero()
    }
    pub(crate) fn from_affine(p: Affine<C>) -> Self {
        if p.is_identity() {
            Self::identity()
        } else {
            Self {
                x: p.x,
                y: p.y,
                zz: C::Base::ONE,
                zzz: C::Base::ONE,
            }
        }
    }
    pub(crate) fn add_affine(&self, p: &Affine<C>) -> Self {
        if p.is_identity() {
            return *self;
        }
        if self.is_identity() {
            return Self::from_affine(*p);
        }
        let u = p.x * self.zz;
        let s = p.y * self.zzz;
        let h = u - self.x;
        let r = s - self.y;
        if h.is_zero() {
            return if r.is_zero() {
                self.double()
            } else {
                Self::identity()
            };
        }
        let hh = h.square();
        let hhh = h * hh;
        let v = self.x * hh;
        let x = r.square() - hhh - v.double();
        Self {
            x,
            y: r * (v - x) - self.y * hhh,
            zz: self.zz * hh,
            zzz: self.zzz * hhh,
        }
    }
    pub(crate) fn double(&self) -> Self {
        if self.is_identity() {
            return *self;
        }
        let u = self.y.double();
        let v = u.square();
        let w = u * v;
        let s = self.x * v;
        let xx = self.x.square();
        let m = xx.double() + xx;
        let x = m.square() - s.double();
        Self {
            x,
            y: m * (s - x) - w * self.y,
            zz: v * self.zz,
            zzz: w * self.zzz,
        }
    }
    pub(crate) fn negate(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            zz: self.zz,
            zzz: self.zzz,
        }
    }
    pub(crate) fn to_affine(&self) -> Affine<C> {
        match self.zzz.inverse() {
            None => Affine::IDENTITY,
            Some(inv) => {
                let zi = self.zz * inv;
                Affine::new(self.x * zi.square(), self.y * inv)
            }
        }
    }
    pub(crate) fn batch_to_affine(points: &[Self]) -> Vec<Affine<C>> {
        let mut products = Vec::with_capacity(points.len());
        let mut product = C::Base::ONE;
        for p in points {
            products.push(product);
            if !p.is_identity() {
                product *= p.zzz;
            }
        }
        let mut inverse = product.inverse().expect("nonzero ZZZ product");
        let mut out = vec![Affine::IDENTITY; points.len()];
        for (i, p) in points.iter().enumerate().rev() {
            if p.is_identity() {
                continue;
            }
            let inv = inverse * products[i];
            inverse *= p.zzz;
            let zi = p.zz * inv;
            out[i] = Affine::new(p.x * zi.square(), p.y * inv);
        }
        out
    }
}
