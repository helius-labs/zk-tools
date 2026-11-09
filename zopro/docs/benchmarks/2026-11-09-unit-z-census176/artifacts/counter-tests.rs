use zopro_bn254::{Affine,CurveConfig,Field,Projective};
fn once<C: CurveConfig>() {
    let g=Projective::<C>::generator();
    let p=g.to_affine();let h=g.double().to_affine();
    let two=C::Base::ONE.double();
    let scaled=Projective::<C>::new(p.x*two.square(),p.y*two.square()*two,two);
    let id=Affine::<C>::IDENTITY;
    for (a,b) in [(Projective::IDENTITY,id),(Projective::IDENTITY,p),(g,id),(g,p),(g,-p),(g,h),(scaled,id),(scaled,p),(scaled,-p),(scaled,h)] {
        std::hint::black_box(a.add_affine(&b));
    }
}
fn both(){once::<zopro_bn254::G1Config>();once::<zopro_bn254::G2Config>();}
#[test]
fn classifications_reset_disabled_and_concurrent_workers(){
    assert_eq!(zopro_bn254::unit_z_census::snapshot(),[[0;9];2]);both();assert_eq!(zopro_bn254::unit_z_census::snapshot(),[[0;9];2]);
    zopro_bn254::unit_z_census::begin();both();
    let want=[10,3,1,1,1,1,1,1,1];
    assert_eq!(zopro_bn254::unit_z_census::finish(),[want;2]);both();assert_eq!(zopro_bn254::unit_z_census::snapshot(),[want;2]);
    zopro_bn254::unit_z_census::begin();
    let mut workers_seen=0;
    for n in [1,4,16,32] {
        let pool=rayon::ThreadPoolBuilder::new().num_threads(n).stack_size(512*1024).build().unwrap();
        let seen=std::sync::Mutex::new(std::collections::BTreeSet::new());
        pool.broadcast(|ctx|{seen.lock().unwrap().insert(ctx.index());both();});
        assert_eq!(seen.into_inner().unwrap().len(),n);workers_seen+=n;
    }
    let pool=rayon::ThreadPoolBuilder::new().num_threads(8).stack_size(512*1024).build().unwrap();
    std::thread::scope(|scope|{for _ in 0..4{let pool=&pool;scope.spawn(move||{for _ in 0..3{pool.install(both);}});}});
    let out=zopro_bn254::unit_z_census::finish();
    assert_eq!(workers_seen,53);assert_eq!(out,[want.map(|v|v*65);2]);
    for r in out{assert_eq!(r[0],r[1..].iter().sum());}
    println!("COUNTER_TEST_PASS isolated10+threaded650calls_per_curve;53broadcastworkers+4callersX3/shared8pool;all9branches;disabled+reset");
}
