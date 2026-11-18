use std::{collections::BTreeSet, fs};
use serde_json::json;
use sha2::{Digest,Sha256};

fn main() {
    let root = "../g2-queue-scan175/inputs";
    let mut rows = Vec::new();
    for shape in ["1x2","2x2"] {
        let data = fs::read(format!("{root}/{shape}.b2q")).unwrap();
        assert_eq!(&data[..8], b"B2SCAN01");
        let n = u64::from_le_bytes(data[8..16].try_into().unwrap()) as usize;
        assert_eq!(data.len(), 16 + 160*n);
        let words: Vec<[u64;4]> = data[16 + 128*n..].chunks_exact(32).map(|b|
            std::array::from_fn(|i|u64::from_le_bytes(b[i*8..i*8+8].try_into().unwrap()))
        ).collect();
        let exact = words.iter().filter(|s| **s != [0;4]).collect::<BTreeSet<_>>().len();
        let cutoff = n * if n>=32768 {95} else {80} / 100;
        // Zeros and adjacent equal nonzero words cannot give a new distinct scalar.
        let mut repeats = 0;
        let mut impossible_at = None;
        for (i,word) in words.iter().enumerate() {
            if *word == [0;4] || i > 0 && words[i-1] == *word {
                repeats += 1;
                if n-repeats <= cutoff && impossible_at.is_none() {
                    impossible_at = Some(i+1);
                }
            }
        }
        assert!(n-repeats >= exact);
        let mut variants = Vec::new();
        for exp in 15..=20 {
            let bits = 1usize<<exp;
            let mut bitmap = vec![0u64;bits/64];
            let mut count = 0;
            let mut first = None;
            for (i,s) in words.iter().enumerate().filter(|(_,s)| s[0]!=0) {
                let b = (s[0].wrapping_mul(0x9e3779b97f4a7c15)>>(64-exp)) as usize;
                let bit = 1u64<<(b%64);
                let word = &mut bitmap[b/64];
                if *word&bit==0 {count+=1; *word|=bit;}
                if count>cutoff && first.is_none() {first=Some(i+1);}
            }
            assert!(count<=exact);
            assert_eq!(count,bitmap.iter().map(|x|x.count_ones() as usize).sum::<usize>());
            variants.push(json!({"bytes":bits/8,"occupied":count,"first_certified_row":first}));
        }
        rows.push(json!({"shape":shape,"source_sha256":format!("{:x}",Sha256::digest(&data)),"n":n,"exact":exact,"adjacent_or_zero_repeats":repeats,"cannot_certify_after":impossible_at,"cutoff":cutoff,"variants":variants}));
    }
    println!("{}",serde_json::to_string_pretty(&rows).unwrap());
}
