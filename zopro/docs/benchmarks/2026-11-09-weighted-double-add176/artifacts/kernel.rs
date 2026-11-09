#![deny(unsafe_code)]
use zopro_bn254::{G1Affine, G1Projective};

#[derive(Clone, Copy, Debug, Default)]
pub struct Routes { pub fast: usize, pub first_guard: usize, pub second_guard: usize, pub inverse_roots: usize }
pub struct Output { pub points: [G1Affine; 8], pub routes: Routes }

fn original(p: G1Affine, q: G1Affine) -> G1Affine { p.to_projective().double().add_affine(&q).to_affine() }

pub fn apply(p: &[G1Affine; 8], q: &[G1Affine; 8], active: usize) -> Output {
    assert!(active <= 8);
    #[cfg(all(target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    { native::apply(p, q, active) }
    #[cfg(not(all(target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma))))]
    { let mut points=*p; for i in 0..active {points[i]=original(p[i],q[i]);} Output {points, routes:Routes {first_guard:active,..Routes::default()}} }
}

#[cfg(all(target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
mod native {
    use super::*;
    use zopro_bn254::{Field, Fq, FqConfig, ifma8::Ifma8};
    type V = Ifma8<FqConfig, true>;

    // Exact accepted FqBatch::inverse_nonzero product tree; no new field primitive.
    fn inverse_nonzero(x: &V) -> V {
        let swapped=V::shuffle_pair::<1,0,3,2>(x,x);
        let pairs=x.mul(&swapped);
        let other_pair=V::shuffle_pair::<2,3,0,1>(&pairs,&pairs);
        let quads=pairs.mul(&other_pair);
        let products=quads.get();
        let inverse=(products[0]*products[4]).inverse().unwrap();
        let (lo,hi)=quads.halves();
        V::new(&[inverse;8]).mul(&V::from_halves(&hi,&lo)).mul(&other_pair).mul(&swapped)
    }

    pub(super) fn apply(p:&[G1Affine;8],q:&[G1Affine;8],active:usize)->Output {
        let mut routes=Routes::default();let mut points=*p;
        let mut valid=core::array::from_fn::<_,8,_>(|i| i<active && !p[i].is_identity() && !q[i].is_identity() && p[i].x!=q[i].x);
        for i in 0..active {if !valid[i] {points[i]=original(p[i],q[i]);routes.first_guard+=1;}}
        if !valid.iter().any(|v|*v) {return Output {points,routes};}
        let px=V::new(&p.map(|p|p.x));let py=V::new(&p.map(|p|p.y));
        let qx=V::new(&q.map(|p|p.x));let qy=V::new(&q.map(|p|p.y));
        let first_den=V::new(&core::array::from_fn(|i|if valid[i]{q[i].x-p[i].x}else{Fq::ONE}));
        let u=qy.sub(&py).mul(&inverse_nonzero(&first_den));routes.inverse_roots+=1;
        let mid=u.mul(&u).sub(&px).sub(&qx);let middle=mid.get();
        for i in 0..active {if valid[i]&&middle[i]==p[i].x {valid[i]=false;points[i]=original(p[i],q[i]);routes.second_guard+=1;}}
        if !valid.iter().any(|v|*v) {return Output {points,routes};}
        let second_den=V::new(&core::array::from_fn(|i|if valid[i]{middle[i]-p[i].x}else{Fq::ONE}));
        let v=V::zero().sub(&u).sub(&py.add(&py).mul(&inverse_nonzero(&second_den)));routes.inverse_roots+=1;
        let x=v.mul(&v).sub(&px).sub(&mid);
        let y=v.mul(&px.sub(&x)).sub(&py);let (x,y)=(x.get(),y.get());
        for i in 0..active {if valid[i] {points[i]=G1Affine::new(x[i],y[i]);routes.fast+=1;}}
        Output {points,routes}
    }
}

pub fn fold(roots:&[[G1Affine;9];8],active:usize)->([G1Affine;8],Routes) {
    let mut points=roots.map(|r|r[0]);let mut total=Routes::default();
    for step in 1..9 {
        let corrections=roots.map(|r|-r[step]);let out=apply(&points,&corrections,active);points=out.points;
        total.fast+=out.routes.fast;total.first_guard+=out.routes.first_guard;total.second_guard+=out.routes.second_guard;total.inverse_roots+=out.routes.inverse_roots;
    }
    (points,total)
}

pub fn original_fold(roots:&[G1Affine;9])->G1Projective {
    let mut out=roots[0].to_projective();for q in &roots[1..] {out=out.double().add_affine(&-*q);}out
}
