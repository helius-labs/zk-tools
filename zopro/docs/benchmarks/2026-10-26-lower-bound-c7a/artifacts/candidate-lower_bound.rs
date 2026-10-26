//! An occupied-bit count is a lower bound on distinct nonzero inputs.
//! Collisions can only cause fallback to the existing exact grouping pass.
use std::hash::BuildHasher;

fn words(n: usize, percent: usize) -> Option<usize> {
    // Small query sizes where map setup is material. Larger queries retain
    // their existing policy and allocate no bitmap. No key table is retained.
    if percent != 80 || !(8192..32768).contains(&n) { return None; }
    n.checked_mul(16)?.checked_next_power_of_two()?.checked_div(64)
}

pub(super) fn declines(scalars: &[[u64; 4]], percent: usize) -> bool {
    if words(scalars.len(), percent).is_none() { return false; }
    with_hasher(scalars, percent, &ahash::RandomState::new())
}

fn with_hasher<S: BuildHasher>(scalars: &[[u64; 4]], percent: usize, state: &S) -> bool {
    let Some(count) = words(scalars.len(), percent) else { return false; };
    // This phase runs after grouping's existing checked reservation and before
    // any full-map buffers. Its allocation (including the vector allowance)
    // is no larger than the already forecast classes Vec<u32> for these n.
    let mut bitmap = vec![0u64; count];
    let mask = count * 64 - 1;
    let cutoff = scalars.len() * percent / 100;
    let mut occupied = 0;
    for scalar in scalars {
        if *scalar == [0; 4] { continue; }
        let h = state.hash_one(scalar) as usize & mask;
        let word = &mut bitmap[h >> 6];
        let bit = 1u64 << (h & 63);
        if *word & bit == 0 {
            *word |= bit;
            occupied += 1;
            if occupied > cutoff { return true; }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeSet, hash::{BuildHasherDefault, Hasher}};
    #[derive(Default)] struct Identity(u64);
    impl Hasher for Identity {
        fn finish(&self)->u64{self.0}
        fn write(&mut self,b:&[u8]){for chunk in b.chunks(8){let mut v=[0u8;8];v[..chunk.len()].copy_from_slice(chunk);self.0^=u64::from_ne_bytes(v);}}
        fn write_u64(&mut self,x:u64){self.0^=x;}
        fn write_usize(&mut self,x:usize){self.0^=x as u64;}
    }
    #[derive(Default)]struct Collision;
    impl Hasher for Collision {fn finish(&self)->u64{0}fn write(&mut self,_:&[u8]){}}
    #[test] fn exact_cutoff_and_zero_boundaries() {
        for n in [8192usize,8193,16384,21006,30089,32767] {
            let cutoff=n*80/100;
            for unique in [0,1,cutoff-1,cutoff,cutoff+1,n] {
                let x:Vec<_>=(0..n).map(|i|if unique==0{[0;4]}else{[(i%unique+1)as u64,0,0,0]}).collect();
                assert_eq!(with_hasher(&x,80,&BuildHasherDefault::<Identity>::default()),unique>cutoff,"n={n} unique={unique}");
            }
        }
    }
    #[test] fn collisions_only_fall_back() {
        let x:Vec<_>=(1..=30089u64).map(|i|[i,0,0,0]).collect();
        assert!(!with_hasher(&x,80,&BuildHasherDefault::<Collision>::default()));
        assert!(with_hasher(&x,80,&BuildHasherDefault::<Identity>::default()));
    }
    #[test] fn randomized_words_never_decline_acceptable_cardinality() {
        let mut seed=928371u64;
        let mut cases=0;
        for n in [8192usize,16385,21006,30089,32767] {
            for density in [0usize,1,25,50,79,80,81,87,95,100] {
                let u=(n*density/100).max(1);
                let pool:Vec<_>=(0..u).map(|_|{let mut word=[0;4];for x in &mut word{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;*x=seed;}word}).collect();
                let x:Vec<_>=(0..n).map(|i|if density==0||i%31==0{[0;4]}else{pool[i%u]}).collect();
                let unique=x.iter().filter(|x|**x!=[0;4]).collect::<BTreeSet<_>>().len();
                for _ in 0..4{if declines(&x,80){assert!(unique*100>n*80);}}
                cases+=1;
            }
        }
        assert_eq!(cases,50);
    }
    #[test] fn bitmap_fits_existing_classes_phase_and_large_paths_skip() {
        for n in 8192usize..32768 {
            let count=words(n,80).unwrap();
            assert!(count.is_power_of_two());
            assert!(count*8<=n*4);
            assert!(count*8<=65536);
        }
        for n in [0usize,256,8191,32768,65536,211_000_000,usize::MAX] {assert!(words(n,80).is_none());}
        for percent in [0usize,79,81,95,100,usize::MAX]{assert!(words(30089,percent).is_none());}
        for n in [8192usize,21006,30089,32767] {
            let bitmap=crate::msm_affine::grouping_cost::vector::<u64>(words(n,80).unwrap(),false).unwrap();
            let classes=crate::msm_affine::grouping_cost::vector::<u32>(n,false).unwrap();
            assert!(bitmap<=classes);
        }
    }
}
