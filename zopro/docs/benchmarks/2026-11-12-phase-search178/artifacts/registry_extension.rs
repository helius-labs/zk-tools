// Private diagnostic only. Does not change queue ownership or latch protocols.
// More than one overlapping scope lifts the restriction until all scopes end.
struct SearchScope {
    registry: Arc<Registry>,
}
impl Drop for SearchScope {
    fn drop(&mut self) {
        let mut scopes = self.registry.search_scopes.lock().unwrap();
        assert!(*scopes > 0);
        *scopes -= 1;
        if *scopes == 0 {
            self.registry.search_limit.store(usize::MAX, Ordering::Release);
            // Reuse the injected-job fence/event/wakeup protocol. `false`
            // requests all current sleepers, including a sleepy transition.
            self.registry.sleep.new_injected_jobs(self.registry.num_threads() as u32, false);
        }
    }
}

pub(crate) fn with_search_limit<F: FnOnce() -> R, R>(limit: usize, f: F) -> R {
    assert!(limit > 0);
    let registry = Registry::current();
    {
        let mut scopes = registry.search_scopes.lock().unwrap();
        *scopes += 1;
        if *scopes == 1 {
            registry.search_limit.store(if limit >= registry.num_threads() { usize::MAX } else { limit }, Ordering::Release);
        } else {
            registry.search_limit.store(usize::MAX, Ordering::Release);
            registry.sleep.new_injected_jobs(registry.num_threads() as u32, false);
        }
    }
    let _scope = SearchScope { registry };
    f()
}

pub(crate) fn search_state() -> (usize, usize) {
    let registry=Registry::current();
    let scopes=registry.search_scopes.lock().unwrap();
    (registry.search_limit.load(Ordering::Acquire),*scopes)
}

#[cfg(test)]
mod phase_search_tests {
    use super::*;
    use crate::{join, ThreadPoolBuilder};
    use std::sync::{atomic::AtomicUsize, Barrier};

    fn tree(depth: usize, count: &AtomicUsize) {
        if depth == 0 { count.fetch_add(1, Ordering::Relaxed); }
        else { join(|| tree(depth-1,count), || tree(depth-1,count)); }
    }

    #[test]
    fn scoped_work_and_wake_restore_all_workers() {
        for workers in [1, 4, 16, 32] {
            let pool = ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
            for limit in [1, workers.div_ceil(3), workers] {
                let count=AtomicUsize::new(0);
                pool.install(|| with_search_limit(limit, || {
                    if limit < workers { assert_eq!(Registry::current().search_limit.load(Ordering::Acquire),limit); }
                    tree(10,&count);
                }));
                assert_eq!(count.load(Ordering::Relaxed),1024);
                pool.install(|| assert_eq!(Registry::current().search_limit.load(Ordering::Acquire),usize::MAX));
                let mut ids=pool.broadcast(|cx| cx.index());ids.sort_unstable();assert_eq!(ids,(0..workers).collect::<Vec<_>>());
            }
        }
    }

    #[test]
    fn panic_nested_scopes_and_foreign_pool_restore() {
        let pool=ThreadPoolBuilder::new().num_threads(4).build().unwrap();
        let other=ThreadPoolBuilder::new().num_threads(2).build().unwrap();
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||pool.install(||with_search_limit(1,||{
            other.install(||assert_eq!(Registry::current().search_limit.load(Ordering::Acquire),usize::MAX));
            with_search_limit(2,||assert_eq!(Registry::current().search_limit.load(Ordering::Acquire),usize::MAX));
            // Once overlapping, remain unrestricted until outermost exit.
            assert_eq!(Registry::current().search_limit.load(Ordering::Acquire),usize::MAX);
            panic!("expected callback panic");
        }))));
        assert!(result.is_err());
        pool.install(||{assert_eq!(Registry::current().search_limit.load(Ordering::Acquire),usize::MAX);assert_eq!(*Registry::current().search_scopes.lock().unwrap(),0);});
        assert_eq!(pool.install(||join(||17,||29)),(17,29));
    }

    #[test]
    fn concurrent_external_callers_lift_restriction() {
        let pool=ThreadPoolBuilder::new().num_threads(4).build().unwrap();
        let ready=Barrier::new(2);let finish=Barrier::new(2);
        std::thread::scope(|scope| {
            for _ in 0..2 {scope.spawn(||pool.install(||with_search_limit(1,||{
                ready.wait();assert_eq!(Registry::current().search_limit.load(Ordering::Acquire),usize::MAX);
                let count=AtomicUsize::new(0);tree(7,&count);assert_eq!(count.load(Ordering::Relaxed),128);finish.wait();
            })) );}
        });
        pool.install(|| assert_eq!(*Registry::current().search_scopes.lock().unwrap(),0));
    }
}
