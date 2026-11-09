mod kernel;
use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{BigInt, PrimeField, Zero};
use zopro_bn254::{Field, Fq, Fr};

fn random_words(state: &mut u64) -> [u64; 4] {
    core::array::from_fn(|_| {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    })
}
fn scalar(state: &mut u64) -> (Fr, ark_bn254::Fr) {
    loop {
        let w = random_words(state);
        if let Some(native) = Fr::from_canonical(w) {
            return (native, ark_bn254::Fr::from_bigint(BigInt(w)).unwrap());
        }
    }
}
fn base(state: &mut u64) -> Fq {
    loop { if let Some(x) = Fq::from_canonical(random_words(state)) { if !x.is_zero() { return x; } } }
}
fn fq(x: ark_bn254::Fq) -> Fq { Fq::from_canonical(x.into_bigint().0).unwrap() }
fn g1(x: ark_bn254::G1Affine) -> zopro_bn254::G1Affine {
    if x.is_zero() { zopro_bn254::G1Affine::IDENTITY } else { zopro_bn254::G1Affine::new(fq(x.x), fq(x.y)) }
}
fn fq2(x: ark_bn254::Fq2) -> zopro_bn254::Fq2 { zopro_bn254::Fq2::new(fq(x.c0), fq(x.c1)) }
fn g2(x: ark_bn254::G2Affine) -> zopro_bn254::G2Affine {
    if x.is_zero() { zopro_bn254::G2Affine::IDENTITY } else { zopro_bn254::G2Affine::new(fq2(x.x), fq2(x.y)) }
}

#[derive(Default, Clone, Copy, Debug)]
struct Counts { outputs: usize, units: usize, nonunits: usize, identities: usize, same: usize, opposite: usize, transitions: usize }
impl Counts {
    fn sum(&mut self, b: Self) { self.outputs+=b.outputs; self.units+=b.units; self.nonunits+=b.nonunits; self.identities+=b.identities; self.same+=b.same; self.opposite+=b.opposite; self.transitions+=b.transitions; }
}

macro_rules! curve_cases {
    ($name:ident, $cfg:ty, $projective:ty, $ark:ty, $convert:ident, $scale:expr, $entry:path) => {
        fn $name(seed: u64, rounds: usize) -> Counts {
            type Point = zopro_bn254::Affine<$cfg>;
            type Proj = zopro_bn254::Projective<$cfg>;
            type Base = <$cfg as zopro_bn254::CurveConfig>::Base;
            let g = <$ark>::generator();
            let mut state = seed;
            let mut c = Counts::default();
            for _ in 0..rounds {
                let (_, a) = scalar(&mut state);
                let (_, b) = scalar(&mut state);
                let p = (g*a).into_affine();
                let q = (g*b).into_affine();
                let inputs = [<$ark>::zero().into_affine(), p, -p];
                let incoming = [<$ark>::zero().into_affine(), p, -p, q, -q];
                let random_scale: Base = ($scale)(&mut state);
                let scales = [Base::ONE, -Base::ONE, Base::ONE.double(), random_scale];
                for ap in inputs {
                    let np = $convert(ap);
                    for z in scales {
                        let left = if np.is_identity() { Proj::IDENTITY } else { Proj::new(np.x*z.square(), np.y*z.square()*z, z) };
                        for aq in incoming {
                            let right = $convert(aq);
                            let before = (left.x, left.y, left.z, right.x, right.y);
                            let want = $convert((ap.into_group()+aq).into_affine());
                            let original = left.add_affine(&right);
                            let actual = $entry(&left, &right);
                            assert_eq!((actual.x,actual.y,actual.z), (original.x,original.y,original.z), "exact Jacobian output");
                            let affine = actual.to_affine();
                            assert_eq!(affine, want, "independent Arkworks oracle");
                            assert!(affine.is_on_curve());
                            assert_eq!((left.x,left.y,left.z,right.x,right.y), before);
                            c.outputs+=1;
                            if np.is_identity() || right.is_identity() { c.identities+=1; }
                            else if z==Base::ONE {c.units+=1;} else {c.nonunits+=1;}
                            if !np.is_identity() && !right.is_identity() && ap.x==aq.x {
                                if ap.y==aq.y {c.same+=1;} else {c.opposite+=1;}
                            }
                        }
                    }
                }
                // A changing chain reaches identity, a fresh unit-Z state,
                // ordinary addition, doubling and nonunit mixed additions.
                let mut actual = Proj::IDENTITY;
                let mut expected = <$ark>::zero();
                for aq in [p,-p,q,q,p,-q,-p,<$ark>::zero().into_affine()] {
                    let native = $convert(aq);
                    let original = actual.add_affine(&native);
                    let next = kernel::add(&actual,&native);
                    expected += aq;
                    assert_eq!((next.x,next.y,next.z), (original.x,original.y,original.z));
                    assert_eq!(next.to_affine(), $convert(expected.into_affine()));
                    actual=next; c.transitions+=1;
                }
                // Explicit noncanonical identity representation must preserve
                // the original identity guard instead of entering mmadd.
                let weird = Proj::new(Base::ZERO, Base::ONE.double(), Base::ZERO);
                let id = Point::IDENTITY;
                let out = kernel::add(&weird,&id);
                assert_eq!((out.x,out.y,out.z),(weird.x,weird.y,weird.z));
            }
            assert_eq!(c.outputs, rounds*60);
            assert_eq!(c.transitions, rounds*8);
            assert!(c.units>0 && c.nonunits>0 && c.identities>0 && c.same>0 && c.opposite>0);
            c
        }
    }
}
curve_cases!(case_g1, zopro_bn254::G1Config, zopro_bn254::G1Projective, ark_bn254::G1Projective, g1, |s: &mut u64| base(s), kernel::unit_z_mixed_g1);
curve_cases!(case_g2, zopro_bn254::G2Config, zopro_bn254::G2Projective, ark_bn254::G2Projective, g2, |s: &mut u64| zopro_bn254::Fq2::new(base(s),base(s)), kernel::unit_z_mixed_g2);

#[test]
fn exact_and_independent_group_outputs() {
    let a=case_g1(0x936b5ac0124de78f,96);
    let b=case_g2(0x41bd8f62a9073ce5,96);
    println!("G1={a:?};G2={b:?};INDEPENDENT_OUTPUTS={};CHAIN_TRANSITIONS={}",a.outputs+b.outputs,a.transitions+b.transitions);
}

#[cfg(unit_z_native_test)]
#[test]
fn all_workers_and_concurrent_callers() {
    let mut all=Counts::default();
    for workers in [1,4,16,32] {
        let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
        let seen=std::sync::Mutex::new(std::collections::BTreeSet::new());
        let output=pool.broadcast(|ctx| {
            seen.lock().unwrap().insert(ctx.index());
            let mut c=case_g1(0x712d3c4a58b960ef+ctx.index() as u64,1);
            c.sum(case_g2(0xb0d863254f9cae71+ctx.index() as u64,1)); c
        });
        assert_eq!(seen.into_inner().unwrap().len(),workers);
        for c in output {all.sum(c);}
    }
    let pool=rayon::ThreadPoolBuilder::new().num_threads(8).stack_size(512*1024).build().unwrap();
    let out=std::thread::scope(|scope| {
        let mut jobs=Vec::new();
        for i in 0..4u64 {let pool=&pool;jobs.push(scope.spawn(move || pool.install(|| {
            let mut c=case_g1(0x1ef34abc652789d0+i,3);c.sum(case_g2(0x76cda1b9502348ef+i,3));c
        })));}
        jobs.into_iter().map(|j|j.join().unwrap()).collect::<Vec<_>>()
    });
    for c in out {all.sum(c);}
    assert_eq!(all.outputs,(53+12)*120);
    assert_eq!(all.transitions,(53+12)*16);
    println!("ACTUAL_WORKERS=53;CONCURRENT_CALLERS=4;SHARED_POOL_WORKERS=8;COUNTS={all:?}");
}
