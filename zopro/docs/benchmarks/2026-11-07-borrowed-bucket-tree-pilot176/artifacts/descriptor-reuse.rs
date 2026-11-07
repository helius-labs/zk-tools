fn empty<'old, 'new>(batch: Vec<(u32, &'old u64)>) -> Vec<(u32, &'new u64)> {
    batch
        .into_iter()
        .map(|_| -> (u32, &'new u64) { unreachable!("take(0) never calls this map") })
        .take(0)
        .collect()
}
fn main() {
    let mut cases = 0;
    for capacity in [1, 8, 17, 128, 2048] {
        for len in [0, 1, capacity / 2, capacity] {
            let mut referent = 77u64;
            let mut batch = Vec::with_capacity(capacity);
            batch.extend((0..len).map(|i| (i as u32, &referent)));
            let address = batch.as_ptr() as usize;
            let allocated = batch.capacity();
            let mut fresh = empty(batch);
            referent = 91;
            assert!(fresh.is_empty());
            assert_eq!(fresh.capacity(), allocated);
            assert_eq!(fresh.as_ptr() as usize, address);
            fresh.push((3, &referent));
            assert_eq!(*fresh[0].1, 91);
            cases += 1;
        }
    }
    println!("{cases} descriptor allocation/lifetime reuse cases pass");
}
