
#[cfg(zopro_query_neutralization)]
mod diagnostic_proofs {
    use super::*;
    use zopro_prover::zopro_core::query_neutralization::{Controller,Mode,Snapshot};
    fn check(s:Snapshot,mask:u8) {
        assert!(s.cache_ready);assert_eq!(s.requests,1);assert_eq!(s.declines,0);assert_eq!(s.faults,0);assert_eq!(s.misses,[0;5]);
        assert!(s.learned_topology==19||s.learned_topology==29);
        for q in 0..5 {let active=(s.learned_topology>>q)&1;let bit=match q{0=>1,4=>2,_=>4};assert_eq!(s.calls[q],u64::from(active));assert_eq!(s.hits[q],u64::from(active)*u64::from(mask&bit!=0));assert_eq!(s.computed[q],u64::from(active)*u64::from(mask&bit==0));}
    }
    #[test]
    fn actual_cubic_masks_verify_and_preserve_fresh_blinding() {
        let mut prover=Prover::load(&key()).unwrap();let witness=prover.witness(&assignment(3,35)).unwrap();let control=Controller::new();let mut proofs=std::collections::BTreeSet::new();
        {let _scope=control.enter(Mode::Learn).unwrap();let proved=prover.prove(&witness).unwrap();assert!(prover.verify(&proved.proof,&proved.public));assert!(proofs.insert(proved.proof.to_gnark()));}
        let learned=control.snapshot().unwrap();assert!(learned.cache_ready);assert_eq!(learned.faults,0);assert_eq!(learned.hits,[0;5]);
        for mask in 0..8 {control.reset_counts().unwrap();{let _scope=control.enter(Mode::Replay(mask)).unwrap();let proved=prover.prove(&witness).unwrap();assert_eq!(proved.public,[Fr::from(35u64)]);assert!(prover.verify(&proved.proof,&proved.public));assert_eq!(proved.proof.to_gnark().len(),164);assert!(proofs.insert(proved.proof.to_gnark()),"fresh randomized bytes repeated");}check(control.snapshot().unwrap(),mask);}
        println!("QUERY_DIAGNOSTIC cubic_masks=8 verified_fresh_proofs=9 topology={} path={} coordinate_bytes={} controller_bytes={}",learned.learned_topology,learned.learned_path_kind,Controller::coordinate_payload_bytes(),Controller::payload_bytes());
    }
    #[test]
    fn actual_cubic_changed_witness_key_and_gpu_drop_decline() {
        let bytes=key();let mut prover=Prover::load(&bytes).unwrap();let witness=prover.witness(&assignment(3,35)).unwrap();let c=Controller::new();{let _scope=c.enter(Mode::Learn).unwrap();prover.prove(&witness).unwrap();}assert!(c.snapshot().unwrap().cache_ready);
        c.reset_counts().unwrap();{let _scope=c.enter(Mode::Replay(7)).unwrap();let changed=prover.witness(&assignment(4,73)).unwrap();let proof=prover.prove(&changed).unwrap();assert_eq!(proof.public,[Fr::from(73u64)]);assert!(prover.verify(&proof.proof,&proof.public));}let s=c.snapshot().unwrap();assert_eq!(s.requests,1);assert_eq!(s.declines,1);assert_eq!(s.hits,[0;5]);assert_eq!(s.faults,0);
        let mut other=Prover::load(&bytes).unwrap();c.reset_counts().unwrap();{let _scope=c.enter(Mode::Replay(7)).unwrap();let proof=other.prove(&witness).unwrap();assert!(other.verify(&proof.proof,&proof.public));}let s=c.snapshot().unwrap();assert_eq!(s.declines,1);assert_eq!(s.hits,[0;5]);assert_eq!(s.faults,0);
        prover.drop_gpu();c.reset_counts().unwrap();{let _scope=c.enter(Mode::Replay(7)).unwrap();let proof=prover.prove(&witness).unwrap();assert!(prover.verify(&proof.proof,&proof.public));}let s=c.snapshot().unwrap();assert_eq!(s.declines,1);assert_eq!(s.hits,[0;5]);assert_eq!(s.faults,0);
        println!("QUERY_DIAGNOSTIC actual_cubic_changed_witness_new_key_mutated_key_full_fallback verified=3");
    }
}
