#![cfg(test)]
//! Bounded modified-ordering models, not a model of the complete scheduler,
//! generic volatile payload protocol, index wraparound or epoch collector.
use loom::{sync::{Arc, atomic::{AtomicIsize, AtomicUsize, Ordering, fence}}, thread};

fn model(f: impl Fn() + Send + Sync + 'static) {
    let mut builder = loom::model::Builder::new();
    builder.preemption_bound = Some(3);
    builder.max_branches = 50_000;
    builder.check(move || {
        for name in [std::any::type_name::<Arc<Queue>>(), std::any::type_name::<AtomicIsize>(), std::any::type_name::<AtomicUsize>()] {
            assert!(name.starts_with("loom::"), "unmodelled synchronization {name}");
        }
        f();
    });
}

fn buffering<const COMMON: bool>() {
    model(|| {
        let fronts = Arc::new([AtomicUsize::new(0), AtomicUsize::new(0)]);
        let backs = Arc::new([AtomicUsize::new(0), AtomicUsize::new(0)]);
        let f = fronts.clone(); let b = backs.clone();
        let owner = thread::spawn(move || {
            b[0].store(1, Ordering::Relaxed); b[1].store(1, Ordering::Relaxed);
            fence(Ordering::SeqCst);
            [f[0].load(Ordering::Relaxed), f[1].load(Ordering::Relaxed)]
        });
        let f = fronts.clone(); let b = backs.clone();
        let thief = thread::spawn(move || {
            f[0].store(1, Ordering::Relaxed); f[1].store(1, Ordering::Relaxed);
            let saved = [f[0].load(Ordering::Acquire), f[1].load(Ordering::Acquire)];
            if COMMON { fence(Ordering::SeqCst); }
            let observed = [b[0].load(Ordering::Acquire), b[1].load(Ordering::Acquire)];
            (saved, observed)
        });
        let seen_front = owner.join().unwrap(); let (saved, seen_back) = thief.join().unwrap();
        for i in 0..2 {
            assert_eq!(saved[i], 1);
            assert!(seen_front[i] != 0 || seen_back[i] != 0, "forbidden store-buffering pair {i}");
        }
    });
}

#[derive(Debug)] enum Attempt { Value(usize), Empty, Retry }
struct Queue {
    front: AtomicIsize, back: AtomicIsize, generation: AtomicUsize,
    // Atomic slots avoid claiming to model the original generic volatile T
    // accesses. Both generations remain allocated for the entire model.
    slots: [[AtomicUsize; 2]; 2],
}
impl Queue {
    fn new(values: &[usize]) -> Arc<Self> {
        assert!(values.len() <= 2);
        Arc::new(Self { front: AtomicIsize::new(0), back: AtomicIsize::new(values.len() as isize), generation: AtomicUsize::new(0),
            slots: [[AtomicUsize::new(values.first().copied().unwrap_or(0)), AtomicUsize::new(values.get(1).copied().unwrap_or(0))], [AtomicUsize::new(0), AtomicUsize::new(0)]] })
    }
    fn extract(&self, front: isize) -> Attempt {
        let back = self.back.load(Ordering::Acquire);
        if back.wrapping_sub(front) <= 0 { return Attempt::Empty; }
        let generation = self.generation.load(Ordering::Acquire);
        let task = self.slots[generation][front as usize & 1].load(Ordering::Relaxed);
        if self.generation.load(Ordering::Acquire) != generation || self.front.compare_exchange(front, front.wrapping_add(1), Ordering::SeqCst, Ordering::Relaxed).is_err() {
            return Attempt::Retry;
        }
        assert!(task != 0, "unpublished task observed");
        Attempt::Value(task)
    }
    fn steal(&self) -> Attempt {
        let front = self.front.load(Ordering::Acquire);
        fence(Ordering::SeqCst);
        self.extract(front)
    }
    fn pop_lifo(&self) -> Attempt {
        let back = self.back.load(Ordering::Relaxed);
        let front = self.front.load(Ordering::Relaxed);
        if back.wrapping_sub(front) <= 0 { return Attempt::Empty; }
        let back = back.wrapping_sub(1);
        self.back.store(back, Ordering::Relaxed);
        fence(Ordering::SeqCst);
        let front = self.front.load(Ordering::Relaxed);
        let len = back.wrapping_sub(front);
        if len < 0 { self.back.store(back.wrapping_add(1), Ordering::Relaxed); return Attempt::Empty; }
        let generation = self.generation.load(Ordering::Relaxed);
        let task = self.slots[generation][back as usize & 1].load(Ordering::Relaxed);
        if len == 0 {
            let won = self.front.compare_exchange(front, front.wrapping_add(1), Ordering::SeqCst, Ordering::Relaxed).is_ok();
            self.back.store(back.wrapping_add(1), Ordering::Relaxed);
            if !won { return Attempt::Empty; }
        }
        assert!(task != 0, "unpublished owner task");
        Attempt::Value(task)
    }
    fn push_from_sole_owner(&self, value: usize) {
        let back = self.back.load(Ordering::Relaxed);
        let generation = self.generation.load(Ordering::Relaxed);
        self.slots[generation][back as usize & 1].store(value, Ordering::Relaxed);
        fence(Ordering::Release);
        self.back.store(back.wrapping_add(1), Ordering::Relaxed);
    }
    fn publish_second_generation_from_sole_owner(&self) {
        let back = self.back.load(Ordering::Relaxed);
        let mut front = self.front.load(Ordering::Relaxed);
        while front != back {
            let index = front as usize & 1;
            let value = self.slots[0][index].load(Ordering::Relaxed);
            self.slots[1][index].store(value, Ordering::Relaxed);
            front = front.wrapping_add(1);
        }
        self.generation.store(1, Ordering::Release);
    }
}

fn tile(queues: &[Arc<Queue>]) -> Attempt {
    assert!(queues.len() <= 8);
    let mut fronts = [0isize; 8];
    for (i, q) in queues.iter().enumerate() { fronts[i] = q.front.load(Ordering::Acquire); }
    fence(Ordering::SeqCst);
    let mut retry = false;
    for (i, q) in queues.iter().enumerate() {
        match q.extract(fronts[i]) { Attempt::Value(value) => return Attempt::Value(value), Attempt::Retry => retry = true, Attempt::Empty => {} }
    }
    if retry { Attempt::Retry } else { Attempt::Empty }
}

fn validate(mut returned: Vec<Attempt>, queues: &[Arc<Queue>], expected: usize) {
    // All writers/thieves joined first: no test counter participates in the
    // concurrent algorithm or accidentally supplies extra synchronization.
    for q in queues { for _ in 0..4 {
        match q.pop_lifo() { Attempt::Value(value) => returned.push(Attempt::Value(value)), Attempt::Empty => break, Attempt::Retry => unreachable!() }
    }}
    let mut seen = vec![0; expected + 1];
    for result in returned { if let Attempt::Value(value) = result {
        assert!(value > 0 && value <= expected);
        seen[value] += 1;
        assert_eq!(seen[value], 1, "duplicate task {value}");
    }}
    assert!(seen[1..].iter().all(|n| *n == 1), "lost task");
}

#[test] fn common_fence_preserves_both_store_buffering_pairs() { buffering::<true>(); }
#[test] fn missing_common_fence_negative_control_detects_forbidden_pair() {
    let error = std::panic::catch_unwind(buffering::<false>).expect_err("negative control did not detect missing fence");
    let message = error.downcast_ref::<String>().map(String::as_str).or_else(||error.downcast_ref::<&str>().copied()).unwrap();
    assert!(message.contains("forbidden store-buffering pair"), "unrelated model failure: {message}");
}
#[test] fn common_fence_two_thieves_and_owner_last_pop_are_unique() {
    model(|| {
        let queues = [Queue::new(&[]), Queue::new(&[1, 2])];
        let owner = queues[1].clone(); let scalar = queues[1].clone(); let tiles = queues.clone();
        let a = thread::spawn(move || owner.pop_lifo());
        let b = thread::spawn(move || scalar.steal());
        let c = thread::spawn(move || tile(&tiles));
        let returned = vec![a.join().unwrap(), b.join().unwrap(), c.join().unwrap()];
        validate(returned, &queues, 2);
    });
}
#[test] fn release_publication_is_visible_after_saved_front_and_back_acquire() {
    model(|| {
        let queues = [Queue::new(&[]), Queue::new(&[])];
        let owner = queues.clone(); let first = queues.clone(); let second = queues.clone();
        let a = thread::spawn(move || { owner[0].push_from_sole_owner(1); owner[1].push_from_sole_owner(2); });
        let b = thread::spawn(move || tile(&first)); let c = thread::spawn(move || tile(&second));
        a.join().unwrap(); let returned = vec![b.join().unwrap(), c.join().unwrap()];
        validate(returned, &queues, 2);
    });
}
#[test] fn generation_publication_and_two_snapshot_readers_preserve_tasks() {
    model(|| {
        let queues = [Queue::new(&[]), Queue::new(&[1, 2])];
        let owner = queues[1].clone(); let first = queues.clone(); let second = queues.clone();
        let a = thread::spawn(move || owner.publish_second_generation_from_sole_owner());
        let b = thread::spawn(move || tile(&first)); let c = thread::spawn(move || tile(&second));
        a.join().unwrap(); let returned = vec![b.join().unwrap(), c.join().unwrap()];
        validate(returned, &queues, 2);
    });
}
