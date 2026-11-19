use std::collections::HashMap;
use std::{env,fs};

fn ordinary(values: &[[u64; 4]]) -> Vec<u32> {
    let mut groups = HashMap::new();
    values.iter().map(|x| {
        if *x == [0; 4] { return u32::MAX; }
        let next = groups.len() as u32;
        *groups.entry(*x).or_insert(next)
    }).collect()
}

fn predicted<const SLOTS: usize>(values: &[[u64; 4]], reference: &[u32]) -> (usize, usize, usize) {
    assert!(SLOTS.is_power_of_two());
    let mut groups: HashMap<[u64;4], (u32,u32)> = HashMap::new();
    let mut classes = vec![u32::MAX;values.len()];
    let mut lag = [0usize; SLOTS];
    let (mut nonzero, mut checked, mut hits) = (0,0,0);
    for (i, x) in values.iter().enumerate() {
        if *x == [0;4] { continue; }
        nonzero += 1;
        let slot = i & (SLOTS-1);
        let stride = lag[slot];
        if stride > 0 && stride <= i {
            checked += 1;
            if values[i-stride] == *x {
                classes[i] = classes[i-stride];
                hits += 1;
                continue;
            }
        }
        let next = groups.len() as u32;
        let (class,last) = groups.entry(*x).or_insert((next,i as u32));
        classes[i] = *class;
        if *last < i as u32 {
            lag[slot] = i - *last as usize;
            *last = i as u32;
        }
    }
    assert_eq!(classes,reference);
    (nonzero,checked,hits)
}

fn main() {
    let xs:Vec<_>=(0..256).map(|i|[if i%31==0{0}else{1+(i%7)},0,0,0]).collect();
    let r=ordinary(&xs);
    assert!(predicted::<1>(&xs,&r).2>200);
    predicted::<8>(&xs,&r);
    for path in env::args().skip(1) {
        let raw=fs::read(&path).unwrap();
        assert_eq!(raw.len()%32,0);
        let values:Vec<[u64;4]>=raw.chunks_exact(32).map(|v|std::array::from_fn(|j|
            u64::from_le_bytes(v[j*8..j*8+8].try_into().unwrap()))).collect();
        assert!(values.len()<u32::MAX as usize);
        let reference=ordinary(&values);
        let distinct=reference.iter().copied().filter(|x| *x != u32::MAX).max().map_or(0,|x| x+1);
        let (n,c,h)=predicted::<1>(&values,&reference);
        let (_,d,j)=predicted::<8>(&values,&reference);
        println!("{} {} {} {} {} {} {}",values.len(),n,distinct,c,h,d,j);
    }
}
