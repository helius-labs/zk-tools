//! Private prototype: provenance for outputs built from certified G2 inputs.
use rayon::prelude::*;
use std::sync::OnceLock;
use zopro_bn254::G2Affine;

/// A generated raw proof with a private construction certificate.
/// External raw proof parsing never consumes this certificate.
pub struct GeneratedProof {
    raw: Vec<u8>,
    g2_certified: bool,
}
impl GeneratedProof {
    pub(crate) fn new(raw: Vec<u8>, g2_certified: bool) -> Self { Self { raw, g2_certified } }
    pub fn into_proof(self) -> Result<crate::verify::Proof, &'static str> {
        if self.g2_certified { probe::fast(); }
        crate::verify::Proof::from_generated_raw(&self.raw, self.g2_certified)
    }
}

#[derive(Default)]
pub(crate) struct G2InputCertificate(OnceLock<bool>);
impl G2InputCertificate {
    /// The owner must keep this exact point population immutable for its lifetime.
    pub(crate) fn check(&self, points: &[G2Affine], fixed: bool) -> bool {
        if let Some(&valid)=self.0.get(){return valid;}
        // Rayon may execute another caller while this scan waits for its tasks.
        // Do not hold a OnceLock initializer across that cooperative wait.
        // First callers may duplicate only cold work; publish a completed bool.
        let start=std::time::Instant::now();
        let valid=points.par_iter().with_min_len(64).all(|p|p.is_on_curve()&&p.is_in_subgroup());
        probe::scan(fixed,start.elapsed().as_nanos().try_into().unwrap());
        if let Err(other)=self.0.set(valid){debug_assert_eq!(other,valid);}
        let published=*self.0.get().unwrap();debug_assert_eq!(published,valid);published
    }
}

pub mod probe {
    use std::sync::atomic::{AtomicU64,Ordering};
    static FAST:AtomicU64=AtomicU64::new(0);
    static KEYS:AtomicU64=AtomicU64::new(0);
    static FIXED:AtomicU64=AtomicU64::new(0);
    static NS:AtomicU64=AtomicU64::new(0);
    pub(super)fn fast(){FAST.fetch_add(1,Ordering::Relaxed);}
    pub(super)fn scan(fixed:bool,ns:u64){if fixed{FIXED.fetch_add(1,Ordering::Relaxed);}else{KEYS.fetch_add(1,Ordering::Relaxed);}NS.fetch_add(ns,Ordering::Relaxed);}
    pub fn snapshot()->(u64,u64,u64,u64){(FAST.load(Ordering::Relaxed),KEYS.load(Ordering::Relaxed),FIXED.load(Ordering::Relaxed),NS.load(Ordering::Relaxed))}
}

#[cfg(test)]mod tests {
    use super::*;
    use zopro_bn254::{CurveConfig,Fq,Fq2,Fr,G1Affine,G2Config};
    fn outside()->G2Affine {
        for i in 0..100u64 {let x=Fq2::new(Fq::from(i),Fq::ONE);if let Some(y)=(x.square()*x+G2Config::B).sqrt(){let p=G2Affine::new(x,y);assert!(p.is_on_curve());if !p.is_in_subgroup(){return p;}}}panic!("no raw twist fixture")
    }
    #[test]fn input_certificates_keep_invalid_and_distinct_owners_separate(){
        let valid=[G2Affine::IDENTITY,G2Affine::GENERATOR,-G2Affine::GENERATOR];
        assert!(G2InputCertificate::default().check(&valid,false));
        let offcurve=G2Affine::new(Fq2::ONE,Fq2::ONE);assert!(!offcurve.is_on_curve());
        assert!(!G2InputCertificate::default().check(&[offcurve],false));
        let bad=outside();assert!(!G2InputCertificate::default().check(&[bad],false));
        let mut fixed=crate::groth16::FixedPoints{alpha:G1Affine::GENERATOR,beta:G1Affine::GENERATOR,delta:G1Affine::GENERATOR,beta2:G2Affine::GENERATOR,delta2:G2Affine::GENERATOR};
        let good=crate::groth16::PreparedFixedPoints::new(fixed);assert!(good.generated_g2_certified());
        fixed.beta2=bad;let bad_fixed=crate::groth16::PreparedFixedPoints::new(fixed);assert!(!bad_fixed.generated_g2_certified());assert!(good.generated_g2_certified());
    }
    #[test]fn external_and_generated_parsers_preserve_point_checks(){
        for i in 0..12u64 {let proof=crate::verify::Proof{ar:(G1Affine::GENERATOR*Fr::from(i)).to_affine(),krs:(G1Affine::GENERATOR*Fr::from(i+3)).to_affine(),bs:(G2Affine::GENERATOR*Fr::from(i+7)).to_affine()};let raw=proof.to_raw();assert_eq!(crate::verify::Proof::from_raw(&raw).unwrap(),proof);assert_eq!(GeneratedProof::new(raw.clone(),true).into_proof().unwrap(),proof);assert_eq!(GeneratedProof::new(raw,false).into_proof().unwrap(),proof);}
        let proof=crate::verify::Proof{ar:G1Affine::GENERATOR,krs:G1Affine::GENERATOR,bs:outside()};let raw=proof.to_raw();assert!(crate::verify::Proof::from_raw(&raw).is_err());assert!(GeneratedProof::new(raw,false).into_proof().is_err());
        let mut raw=crate::verify::Proof{bs:G2Affine::GENERATOR,..proof}.to_raw();raw[..32].fill(255);assert!(GeneratedProof::new(raw,true).into_proof().is_err());
        let offcurve=crate::verify::Proof{ar:G1Affine::new(Fq::ONE,Fq::ONE),bs:G2Affine::GENERATOR,..proof}.to_raw();assert!(GeneratedProof::new(offcurve,true).into_proof().is_err());
        let offcurve=crate::verify::Proof{ar:G1Affine::GENERATOR,bs:G2Affine::new(Fq2::ONE,Fq2::ONE),..proof}.to_raw();assert!(GeneratedProof::new(offcurve,true).into_proof().is_err());
    }
    #[test]fn certificate_concurrent_initialization_small_worker_stacks(){
        let points=(1..=129u64).map(|i|(G2Affine::GENERATOR*Fr::from(i)).to_affine()).collect::<Vec<_>>();
        for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();let certificate=G2InputCertificate::default();let barrier=std::sync::Barrier::new(4);std::thread::scope(|scope|{for _ in 0..4{let(pool,certificate,points,barrier)=(&pool,&certificate,&points,&barrier);scope.spawn(move||{barrier.wait();assert!(pool.install(||certificate.check(points,false)));assert_eq!(certificate.0.get(),Some(&true));});}});}
    }
}
