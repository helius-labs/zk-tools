// A diagnostic observer for the duration of individual leaves. All workers
// are joined before the caller reads the observations from this proof.
struct LeafClock {
    start: std::time::Instant,
    by_address: Vec<(usize,usize)>,
    nanos: Vec<std::sync::atomic::AtomicU64>,
}
impl LeafClock {
    fn new(s: &Schedule) -> Self {
        let mut addresses=Vec::new();
        let mut add=|task:&Vec<u32>|{let id=addresses.len();addresses.push((task.as_ptr() as usize,id));};
        for outer in s.epochs.iter().flatten() {
            if let Some(waves)=s.refined.get(&(outer[0] as usize)) {
                for leaf in waves.iter().flatten(){add(leaf);}
            } else {add(outer);}
        }
        addresses.sort_unstable();
        assert!(addresses.windows(2).all(|w|w[0].0!=w[1].0));
        let nanos=(0..addresses.len()).map(|_|std::sync::atomic::AtomicU64::new(0)).collect();
        Self{start:std::time::Instant::now(),by_address:addresses,nanos}
    }
    fn record<T>(&self,task:&[u32],f:impl FnOnce()->T)->T{
        let lookup=self.by_address.binary_search_by_key(&(task.as_ptr() as usize),|t|t.0).unwrap();
        let now=std::time::Instant::now();let result=f();
        let ns=now.elapsed().as_nanos().max(1) as u64;
        self.nanos[self.by_address[lookup].1].store(ns,std::sync::atomic::Ordering::Relaxed);
        result
    }
    fn report(self){
        let elapsed=self.start.elapsed();
        let list:Vec<_>=self.nanos.into_iter().map(|v|v.into_inner()).collect();
        assert!(list.iter().all(|&t|t>0));
        eprintln!("LEAF_TIMES {} {:?}",elapsed.as_nanos(),list);
    }
}
