use ark_bn254::Fr;use ark_ff::{AdditiveGroup,PrimeField};
fn main(){let mut checked=0u64;let mut seed=0x2d47a891u64;
    for _ in 0..16384{let mut next=||{let mut bytes=[0u8;32];for chunk in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}Fr::from_le_bytes_mod_order(&bytes)};let(a,da,db)=(next(),next(),next());
        for b in[a,-a,Fr::ZERO,next()]{for aa in[a,Fr::ZERO]{let original=da*b+aa*db;let factored=if aa==Fr::ZERO{da*b}else if b==Fr::ZERO{aa*db}else if aa==b{aa*(da+db)}else{original};assert_eq!(original,factored);checked+=1;}}
    }
    assert_eq!(checked,131072);println!("Independent Arkworks Fr oracle: {checked} factor identities pass; unequal derivatives retained");
}
