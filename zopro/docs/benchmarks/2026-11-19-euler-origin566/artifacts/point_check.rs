use ark_bn254::{Fr, G1Projective as Point};
use ark_ec::PrimeGroup;
use ark_ff::{AdditiveGroup, Field, FftField, Zero};

fn random(state: &mut u64) -> Fr {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    Fr::from(*state)
}

fn powers(base: Fr) -> impl Iterator<Item=Fr> {
    std::iter::successors(Some(Fr::from(1)), move |previous| Some(*previous * base))
}

// The production Euler output is N times X f'(X) evaluated on the domain.
// Direct inverse DFT, coefficient weighting and forward evaluation supply an oracle.
fn derivative(values: &[Fr], root: Fr) -> Vec<Fr> {
    let n = values.len();
    let coefficients: Vec<Fr> = (0..n).map(|k| {
        let inv = root.inverse().unwrap().pow([k as u64]);
        let total: Fr = values.iter().zip(powers(inv)).map(|(&x,f)|x*f).sum();
        Fr::from(k as u64)*total
    }).collect();
    (0..n).map(|j| coefficients.iter().zip(powers(root.pow([j as u64])))
              .map(|(&x,f)|x*f).sum()).collect()
}

fn product(a: &[Fr], b: &[Fr], root: Fr) -> Vec<Fr> {
    derivative(a,root).iter().zip(derivative(b,root)).enumerate()
        .map(|(i,(da,db))| *da*b[i]+a[i]*db).collect()
}

fn msm(values: &[Fr], points: &[Point]) -> Point {
    assert_eq!(values.len(),points.len());
    values.iter().zip(points).map(|(s,p)|*p * *s).sum()
}

fn dot(a: &[Fr], b: &[Fr]) -> Fr {
    a.iter().zip(b).map(|(a,b)|*a * b).sum()
}

fn main() {
    let mut rng = 0xb48e17a5840b1cu64;
    let mut checked = 0;
    for n in [4,8,16] {
        let root=Fr::get_root_of_unity(n as u64).unwrap();
        let wires = 5;
        for kind in 0..4 {
            let z:Vec<_>=(0..n).map(|_|Point::generator()*random(&mut rng)).collect();
            let mut a0:Vec<_>=(0..n).map(|_|random(&mut rng)).collect();
            let mut b0:Vec<_>=(0..n).map(|_|random(&mut rng)).collect();
            let mut left:Vec<Vec<Fr>>=(0..n).map(|_|(0..wires).map(|_|random(&mut rng)).collect()).collect();
            let mut right=left.clone();
            for row in &mut right {for c in row {*c=random(&mut rng)}}
            if kind==0 {a0.fill(Fr::ZERO);}
            if kind==1 {b0.fill(Fr::from(1));}
            if kind==2 {for i in n/2..n {a0[i]=Fr::ZERO;b0[i]=Fr::ZERO;left[i].fill(Fr::ZERO);right[i].fill(Fr::ZERO);}}
            let da=derivative(&a0,root);let db=derivative(&b0,root);
            let mut wire_points=vec![Point::zero();wires];
            for j in 0..n {
                let mut unit=vec![Fr::ZERO;n];unit[j]=Fr::from(1);
                let col=derivative(&unit,root);
                let mut ga=z[j]*db[j];let mut gb=z[j]*da[j];
                for i in 0..n {ga+=z[i]*(col[i]*b0[i]);gb+=z[i]*(col[i]*a0[i]);}
                for k in 0..wires {wire_points[k]+=ga*left[j][k]+gb*right[j][k];}
            }
            // Signed constant comes from expressing the correction against original wire values.
            let origin=msm(&product(&a0,&b0,root),&z);
            for j in 0..5 {
                let mut w:Vec<_>=(0..wires).map(|_|random(&mut rng)).collect();
                if j==0 {w.fill(Fr::ZERO)} else {w[0]=Fr::from(1);}
                let a:Vec<_>=left.iter().map(|r|dot(r,&w)).collect();
                let b:Vec<_>=right.iter().map(|r|dot(r,&w)).collect();
                let x:Vec<_>=a.iter().zip(&a0).map(|(v,c)|*v-c).collect();
                let y:Vec<_>=b.iter().zip(&b0).map(|(v,c)|*v-c).collect();
                let direct=msm(&product(&a,&b,root),&z);
                let folded=msm(&product(&x,&y,root),&z)+msm(&w,&wire_points)-origin;
                assert_eq!(direct,folded);
                if !origin.is_zero() {assert_ne!(direct,folded+origin);}
                checked+=1;
            }
        }
    }
    println!("verified {checked} folded Euler G1 identities");
}
