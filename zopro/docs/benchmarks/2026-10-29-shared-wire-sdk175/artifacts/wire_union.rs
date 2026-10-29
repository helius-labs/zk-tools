// Proof-local shared classification of two validated, sorted wire queries.
use super::{grouping_cost, grouping_sample, ScalarGroups};
use hashbrown::{hash_table::Entry, HashTable};
use std::hash::BuildHasher;

type Words = [u64; 4];
const EMPTY: u32 = u32::MAX;

// Charge scratch separately from both unchanged group/consumer reservations.
// Existing required proof forecasts cover FFT scratch. These optional buffers
// may overlap it, so their charge is additional, never substituted for it.
fn scratch_peak(wires: usize, a: usize, b: usize) -> Option<usize> {
    use grouping_cost::{add, hash_table, mul, vector};
    let union = wires.min(a.checked_add(b)?);
    add(
        add(hash_table::<u32>(union)?, mul(vector::<u32>(wires, false)?, 2)?)?,
        add(hash_table::<&Words>(a.min(2048))?, hash_table::<&Words>(b.min(2048))?)?,
    )
}

// Outer None requests the original path. Some([None, ..]) means grouping
// was attempted and declined: callers must not hash those scalars again.
pub(crate) fn scalar_groups_pair(
    frame: &[Words],
    indices: [&[u32]; 2],
    scalars: [&[Words]; 2],
) -> Option<[Option<ScalarGroups>; 2]> {
    if !crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
        || frame.len() >= EMPTY as usize
        || indices.iter().zip(scalars).any(|(ids, s)| ids.len() != s.len() || !(256..EMPTY as usize).contains(&s.len()))
    {
        return None;
    }
    // Plans validate strict ordering and bounds when constructed/deserialized.
    // Check the final bound here too; folded slots can enlarge the live frame.
    if indices.iter().any(|q| q.last().is_some_and(|&i| i as usize >= frame.len())) {
        return None;
    }
    let _scratch = crate::preparation::Reservation::analysis(scratch_peak(frame.len(), indices[0].len(), indices[1].len()))?;
    let zero_aware = std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1");
    let policies: [Option<usize>; 2] = std::array::from_fn(|q| grouping_sample(scalars[q], zero_aware).map(|(_, p)| p));
    // With fewer than two admitted samples there is no cross-query work to
    // remove. Return before allocating the global dictionary/labels.
    if policies.iter().any(Option::is_none) {
        return None;
    }
    let mut reservations = [
        Some(grouping_cost::reserve(indices[0].len(), policies[0]?)?),
        Some(grouping_cost::reserve(indices[1].len(), policies[1]?)?),
    ];
    let labels = classify(frame, indices, ahash::RandomState::new());
    let mut remap = vec![EMPTY; frame.len()];
    Some(std::array::from_fn(|q| {
        remap.fill(EMPTY);
        local_groups(frame, indices[q], &labels, &mut remap, policies[q].unwrap(), reservations[q].take().unwrap())
    }))
}

fn classify<S: BuildHasher>(frame: &[Words], queries: [&[u32]; 2], state: S) -> Vec<u32> {
    let mut labels = vec![EMPTY; frame.len()];
    let mut table = HashTable::<u32>::new();
    let (mut a, mut b) = (0, 0);
    // Merge sorted query indices without materializing a union or scalar copy.
    while a < queries[0].len() || b < queries[1].len() {
        let x = queries[0].get(a).copied().unwrap_or(EMPTY);
        let y = queries[1].get(b).copied().unwrap_or(EMPTY);
        let wire = x.min(y);
        a += usize::from(x == wire);
        b += usize::from(y == wire);
        let value = &frame[wire as usize];
        if *value == [0; 4] { continue; }
        let hash = state.hash_one(value);
        let representative = match table.entry(hash,
            |&rep| frame[rep as usize] == *value,
            |&rep| state.hash_one(&frame[rep as usize])) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => { e.insert(wire); wire }
        };
        labels[wire as usize] = representative;
    }
    labels
}

fn local_groups(frame: &[Words], indices: &[u32], labels: &[u32], remap: &mut [u32], percent: usize, reservation: crate::preparation::Reservation) -> Option<ScalarGroups> {
    let mut values = Vec::new();
    let mut counts = Vec::<usize>::new();
    for &wire in indices {
        let rep = labels[wire as usize];
        if rep == EMPTY { continue; }
        let class = &mut remap[rep as usize];
        if *class == EMPTY {
            *class = values.len() as u32;
            values.push(frame[wire as usize]);
            counts.push(0);
        }
        counts[*class as usize] += 1;
    }
    if values.len() * 100 > indices.len() * percent { return None; }
    let mut starts = Vec::with_capacity(counts.len() + 1);
    starts.push(0);
    for &count in &counts { starts.push(starts.last().unwrap() + count); }
    let mut positions = starts[..counts.len()].to_vec();
    let mut order = vec![0; *starts.last().unwrap()];
    for (i, &wire) in indices.iter().enumerate() {
        let rep = labels[wire as usize];
        if rep == EMPTY { continue; }
        let class = remap[rep as usize] as usize;
        order[positions[class]] = i as u32;
        positions[class] += 1;
    }
    Some(ScalarGroups { len: indices.len(), values, counts, starts, order, _reservation: reservation })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::Hasher;
    struct Constant;
    struct ConstantHasher;
    impl Hasher for ConstantHasher { fn write(&mut self, _: &[u8]) {} fn finish(&self) -> u64 { 7 } }
    impl BuildHasher for Constant { type Hasher = ConstantHasher; fn build_hasher(&self) -> ConstantHasher { ConstantHasher } }
    fn compare(a: Option<ScalarGroups>, b: Option<ScalarGroups>) {
        match (a,b) {
            (None,None) => (),
            (Some(a),Some(b)) => {
                assert_eq!(a.len,b.len); assert_eq!(a.values,b.values);
                assert_eq!(a.counts,b.counts); assert_eq!(a.starts,b.starts); assert_eq!(a.order,b.order);
            },
            _ => panic!("group acceptance differs"),
        }
    }
    #[test]
    fn shared_metadata_matches_original_including_forced_collisions() {
        let mut cases=0;
        for n in [0usize,1,7,256,1025] {
            for kind in 0..5 {
                let frame:Vec<_>=(0..n).map(|i|match kind {
                    0=>[0;4], 1=>[7,8,9,10], 2=>[(i%17)as u64,4,3,2],
                    3=>[i as u64,4,3,!i as u64],
                    _=>if i%5==0 {[0;4]} else {[i as u64*1597,i as u64*65537,!i as u64,7]},
                }).collect();
                for geometry in 0..3 {
                    let a:Vec<_>=(0..n).filter(|i|geometry==0||i%2==0).map(|i|i as u32).collect();
                    let b:Vec<_>=(0..n).filter(|i|geometry==0||geometry==1&&i%2!=0).map(|i|i as u32).collect();
                    let labels=classify(&frame,[&a,&b],Constant);
                    for ids in [&a,&b] {
                        let words:Vec<_>=ids.iter().map(|&i|frame[i as usize]).collect();
                        let original=super::super::scalar_groups_policy(&words,true);
                        let sample=if (256..u32::MAX as usize).contains(&words.len()) {grouping_sample(&words,true).map(|(_,p)|p)}else{None};
                        let result=sample.and_then(|p|local_groups(&frame,ids,&labels,&mut vec![EMPTY;n],p,grouping_cost::reserve(ids.len(),p).unwrap()));
                        compare(original,result);
                    }
                    cases+=1;
                }
            }
        }
        assert_eq!(cases,75);
    }
    #[test]
    fn full_pair_and_checked_resource_bounds() {
        assert_eq!(scratch_peak(usize::MAX,usize::MAX,usize::MAX),None);
        let frame:Vec<_>=(0..4096).map(|i|[(i%17)as u64,2,3,4]).collect();
        let a:Vec<_>=(0..4096u32).step_by(2).collect();
        let b:Vec<_>=(0..4096u32).collect();
        let va:Vec<_>=a.iter().map(|&i|frame[i as usize]).collect();
        let vb:Vec<_>=b.iter().map(|&i|frame[i as usize]).collect();
        if !crate::simd::cpu_option("ZOPRO_CPU_DEDUP") { return; }
        let [ga,gb]=scalar_groups_pair(&frame,[&a,&b],[&va,&vb]).unwrap();
        compare(ga,super::super::scalar_groups(&va));
        compare(gb,super::super::scalar_groups(&vb));
        assert!(scalar_groups_pair(&frame,[&a,&b],[&vb,&va]).is_none());
    }
}
