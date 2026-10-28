//! Diagnostic native caller, isolated from production builds.
use super::*;
use std::hint::black_box;
use zopro_bn254::{G1Config, G2Config};

trait TripleField: BatchInverse {
    type Norm: Copy;
    fn keep_norm(norm: FqBatch) -> Self::Norm;
    fn recover_norm(value: &Self::Lanes, norm: &Self::Norm) -> FqBatch;
    fn h(d2: &Self::Lanes, sum: &Self::Lanes, n: &Self::Lanes) -> Self::Lanes;
    fn sop(a: &Self::Lanes, b: &Self::Lanes, c: &Self::Lanes, d: &Self::Lanes) -> Self::Lanes;
    fn coordinates(x: Self, y: Self) -> [[u64; 4]; 4];
}
impl TripleField for Fq {
    type Norm = ();
    fn keep_norm(_: FqBatch) {}
    #[inline(always)]
    fn recover_norm(value: &FqBatch, _: &()) -> FqBatch { *value }
    #[inline(always)]
    fn h(d2: &FqBatch, sum: &FqBatch, n: &FqBatch) -> FqBatch { d2.difference2(sum, n, n) }
    #[inline(always)]
    fn sop(a: &FqBatch, b: &FqBatch, c: &FqBatch, d: &FqBatch) -> FqBatch { a.sum2(b,c,d) }
    fn coordinates(x: Self, y: Self) -> [[u64; 4]; 4] {
        [x.to_canonical(), y.to_canonical(), [0;4], [0;4]]
    }
}
impl TripleField for Fq2 {
    type Norm = FqBatch;
    #[inline(always)]
    fn keep_norm(norm: FqBatch) -> FqBatch { norm }
    #[inline(always)]
    fn recover_norm(_: &[FqBatch;2], norm: &FqBatch) -> FqBatch { *norm }
    #[inline(always)]
    fn h(d2: &Self::Lanes, sum: &Self::Lanes, n: &Self::Lanes) -> Self::Lanes {
        Self::lanes_sub(&Self::lanes_mul(d2,sum), &Self::lanes_square(n))
    }
    #[inline(always)]
    fn sop(a: &Self::Lanes, b: &Self::Lanes, c: &Self::Lanes, d: &Self::Lanes) -> Self::Lanes {
        Self::lanes_add(&Self::lanes_mul(a,b), &Self::lanes_mul(c,d))
    }
    fn coordinates(x: Self, y: Self) -> [[u64;4];4] {
        [x.c0.to_canonical(),x.c1.to_canonical(),y.c0.to_canonical(),y.c1.to_canonical()]
    }
}
struct Record<F: TripleField> {
    prefix: FqBatch,
    norm: F::Norm,
    denominator: F::Lanes,
    first: F::Lanes,
    second: F::Lanes,
    ax: F::Lanes,
    ay: F::Lanes,
    xsum: F::Lanes,
    rx: F::Lanes,
}
#[inline(always)]
fn pack<F: BatchInverse>(values: [F; LANES]) -> F::Lanes {
    let words = values.map(F::to_limbs);
    F::load(core::array::from_fn(|i| &words[i]))
}
fn complete<C: CurveConfig>(buckets: &mut [Affine<C>], q: &[(u32,Affine<C>)], r: &[(u32,Affine<C>)]) {
    for (first, second) in q.iter().zip(r) {
        let i = first.0 as usize;
        buckets[i] = (buckets[i].to_projective()+first.1+second.1).to_affine();
    }
}
fn binary<C: CurveConfig>(buckets: &mut [Affine<C>], q: &[(u32,Affine<C>)], r: &[(u32,Affine<C>)], scratch: &mut BatchAdd<C>)
where C::Base: BatchInverse {
    if q.is_empty() { return; }
    if q.iter().any(|&(i,p)| p.is_identity() || buckets[i as usize].is_identity() || p.x == buckets[i as usize].x) {
        complete(buckets,q,r); return;
    }
    scratch.apply(buckets,q);
    if r.iter().any(|&(i,p)| p.is_identity() || buckets[i as usize].is_identity() || p.x == buckets[i as usize].x) {
        for &(i,p) in r { buckets[i as usize] = (buckets[i as usize]+p).to_affine(); }
        return;
    }
    scratch.apply(buckets,r);
}
fn direct<C: CurveConfig>(buckets: &mut [Affine<C>], q: &[(u32,Affine<C>)], r: &[(u32,Affine<C>)], records: &mut Vec<Record<C::Base>>)
where C::Base: TripleField {
    records.clear();
    if q.is_empty() { return; }
    if q.len()%LANES != 0 || q.iter().zip(r).any(|(&(i,p),&(_,t))| buckets[i as usize].is_identity() || p.is_identity() || t.is_identity()) {
        complete(buckets,q,r); return;
    }
    records.reserve(q.len()/LANES);
    let mut chains = FqBatch::one();
    for k in (0..q.len()).step_by(LANES) {
        let a: [Affine<C>;LANES] = core::array::from_fn(|i| buckets[q[k+i].0 as usize]);
        let b: [Affine<C>;LANES] = core::array::from_fn(|i| q[k+i].1);
        let c: [Affine<C>;LANES] = core::array::from_fn(|i| r[k+i].1);
        let ax = pack(a.map(|p|p.x)); let ay = pack(a.map(|p|p.y));
        let bx = pack(b.map(|p|p.x)); let by = pack(b.map(|p|p.y));
        let rx = pack(c.map(|p|p.x)); let ry = pack(c.map(|p|p.y));
        let d = C::Base::lanes_sub(&bx,&ax);
        let n = C::Base::lanes_sub(&by,&ay);
        let d2 = C::Base::lanes_square(&d);
        let xsum = C::Base::lanes_add(&ax,&bx);
        let h = C::Base::h(&d2,&C::Base::lanes_add(&xsum,&rx),&n);
        let u = C::Base::sop(&d,&C::Base::lanes_add(&ay,&ry),&n,&C::Base::lanes_sub(&rx,&ax));
        let denominator = C::Base::lanes_mul(&d,&h);
        let norm = C::Base::lanes_norm(&denominator);
        // Includes both denominator exceptions; no output has been overwritten.
        if norm.get().iter().any(Field::is_zero) { complete(buckets,q,r); return; }
        let first = C::Base::lanes_mul(&n,&h);
        let second = C::Base::lanes_mul(&d2,&u);
        records.push(Record { prefix:chains,norm:C::Base::keep_norm(norm),denominator,first,second,ax,ay,xsum,rx });
        chains = chains.mul(&norm);
    }
    let mut inverse = chains.inverse_nonzero();
    for (j,g) in records.iter().enumerate().rev() {
        let factor = g.prefix.mul(&inverse);
        inverse = inverse.mul(&C::Base::recover_norm(&g.denominator,&g.norm));
        let dinv = C::Base::lanes_inverse(&g.denominator,&factor);
        let l = C::Base::lanes_mul(&g.first,&dinv);
        let w = C::Base::lanes_mul(&g.second,&dinv);
        let slope = C::Base::lanes_sub(&w,&l);
        let xs = C::Base::lanes_sub(&C::Base::lanes_square(&l),&g.xsum);
        let x = C::Base::lanes_sub(&C::Base::lanes_sub(&C::Base::lanes_square(&slope),&xs),&g.rx);
        let y = C::Base::lanes_add(&C::Base::sop(&w,&C::Base::lanes_sub(&xs,&x),&l,&C::Base::lanes_sub(&x,&g.ax)),&g.ay);
        let x = C::Base::store(&x).map(C::Base::from_limbs);
        let y = C::Base::store(&y).map(C::Base::from_limbs);
        for i in 0..LANES { buckets[q[j*LANES+i].0 as usize] = Affine::new(x[i],y[i]); }
    }
}
struct Fixture<C: CurveConfig> where C::Base: TripleField {
    source: Vec<Affine<C>>,
    buckets: Vec<Affine<C>>,
    q: Vec<(u32,Affine<C>)>,
    r: Vec<(u32,Affine<C>)>,
    scalars: Vec<[Fr;3]>,
    records: Vec<Record<C::Base>>,
    binary: BatchAdd<C>,
}
impl<C: CurveConfig> Fixture<C> where C::Base: TripleField {
    fn from_scalars(words: &[[[u64;4];3]]) -> Self {
        let n = words.len(); let size = n.max(1).next_power_of_two();
        let scalars: Vec<[Fr;3]> = words.iter().map(|w|w.map(|s|Fr::from_canonical(s).unwrap())).collect();
        let g = Projective::<C>::generator();
        let mut source = vec![Affine::GENERATOR;size];
        let mut q = Vec::with_capacity(n); let mut r = Vec::with_capacity(n);
        let mut seen = vec![false;size];
        for (i,s) in scalars.iter().enumerate() {
            let slot = (i*37+17)%size;
            assert!(!seen[slot]); seen[slot] = true;
            source[slot] = (g*s[0]).to_affine();
            q.push((slot as u32,(g*s[1]).to_affine()));
            r.push((slot as u32,(g*s[2]).to_affine()));
        }
        Self { buckets:source.clone(),source,q,r,scalars,records:Vec::with_capacity(n/LANES),binary:BatchAdd::default() }
    }
    fn new(n: usize) -> Self {
        assert!(n<=128);
        let words:Vec<_> = (0..n).map(|i|[
            Fr::from((1u64<<48)*(i as u64+1)+1800000054).to_canonical(),
            Fr::from(11111+18*1337+i as u64*101).to_canonical(),
            Fr::from(50021+18*1711+i as u64*137).to_canonical()
        ]).collect();
        Self::from_scalars(&words)
    }
    fn reset(&mut self) { self.buckets.copy_from_slice(&self.source); }
    fn run(&mut self, selected: bool, rounds: usize) {
        if selected { for _ in 0..rounds { let s = black_box(&mut *self); direct(&mut s.buckets,&s.q,&s.r,&mut s.records); } }
        else { for _ in 0..rounds { let s = black_box(&mut *self); binary(&mut s.buckets,&s.q,&s.r,&mut s.binary); } }
    }
    fn check(&self, rounds: usize) {
        let g = Projective::<C>::generator(); let mut expected = self.source.clone();
        for (i,s) in self.scalars.iter().enumerate() {
            expected[self.q[i].0 as usize] = (g*(s[0]+Fr::from(rounds as u64)*(s[1]+s[2]))).to_affine();
        }
        assert_eq!(self.buckets,expected);
    }
    fn coordinates(&self) -> Vec<[[u64;4];4]> {
        self.q.iter().map(|&(i,_)| { let p = self.buckets[i as usize]; C::Base::coordinates(p.x,p.y) }).collect()
    }
    fn scratch(&self) -> (usize,usize,usize) {
        let control = self.binary.fq.groups.capacity()*std::mem::size_of::<FqAddGroup>()
            +self.binary.fq.tail.capacity()*std::mem::size_of::<[Fq;5]>()
            +self.binary.fq2.groups.capacity()*std::mem::size_of::<Fq2AddGroup>()
            +self.binary.fq2.prefix_records.capacity()*std::mem::size_of::<[FqBatch;2]>()
            +self.binary.fq2.tail.capacity()*std::mem::size_of::<[Fq2;5]>();
        (self.records.capacity()*std::mem::size_of::<Record<C::Base>>(),control,std::mem::size_of::<Record<C::Base>>())
    }
}
pub struct TripleG1(Fixture<G1Config>);
pub struct TripleG2(Fixture<G2Config>);
macro_rules! api { ($ty:ident) => {
    impl $ty {
        pub fn new(n:usize)->Self { Self(Fixture::new(n)) }
        pub fn from_scalars(w:&[[[u64;4];3]])->Self { Self(Fixture::from_scalars(w)) }
        pub fn reset(&mut self) { self.0.reset(); }
        pub fn run(&mut self,selected:bool,n:usize) { self.0.run(selected,n); }
        pub fn check(&self,n:usize) { self.0.check(n); }
        pub fn coordinates(&self)->Vec<[[u64;4];4]> { self.0.coordinates() }
        pub fn scratch(&self)->(usize,usize,usize) { self.0.scratch() }
    }
}; }
api!(TripleG1); api!(TripleG2);
