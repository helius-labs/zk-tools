//! Private coarse observer: one caller-owned record, borrowed across original joins.
use std::{
    fmt::{self, Write},
    sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    time::Instant,
};
const N: usize = 8;
const NAMES: [&str; N] = [
    "solve", "prepare", "fft", "queries", "A", "KH", "G2", "output",
];
#[repr(align(64))]
struct Slot {
    state: AtomicUsize,
    begin: AtomicU64,
    end: AtomicU64,
}
impl Slot {
    fn new() -> Self {
        Self {
            state: AtomicUsize::new(0),
            begin: AtomicU64::new(0),
            end: AtomicU64::new(0),
        }
    }
}
pub(crate) struct Trace {
    origin: Option<Instant>,
    slots: [Slot; N],
    invalid: AtomicBool,
    route: AtomicUsize,
    workers: usize,
    emit: bool,
}
pub(crate) struct Span<'a> {
    trace: &'a Trace,
    index: usize,
    live: bool,
}
struct Buffer {
    bytes: [u8; 4096],
    len: usize,
}
impl Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.len + s.len() > self.bytes.len() {
            return Err(fmt::Error);
        }
        self.bytes[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
        self.len += s.len();
        Ok(())
    }
}
impl Trace {
    pub(crate) fn new() -> Self {
        Self::with_enabled(
            std::env::var("ZOPRO_BENCH_MSM_PHASES").as_deref() == Ok("1"),
            true,
        )
    }
    fn with_enabled(enabled: bool, emit: bool) -> Self {
        Self {
            origin: enabled.then(Instant::now),
            slots: std::array::from_fn(|_| Slot::new()),
            invalid: AtomicBool::new(false),
            route: AtomicUsize::new(0),
            workers: rayon::current_num_threads(),
            emit,
        }
    }
    fn now(&self) -> u64 {
        self.origin
            .as_ref()
            .unwrap()
            .elapsed()
            .as_nanos()
            .try_into()
            .unwrap()
    }
    pub(crate) fn route(&self, route: usize) {
        if self.origin.is_some() {
            self.route.store(route, Ordering::Relaxed);
        }
    }
    pub(crate) fn enter(&self, index: usize) -> Span<'_> {
        let mut live = false;
        if self.origin.is_some() {
            if let Some(s) = self.slots.get(index) {
                if s.state
                    .compare_exchange(0, 1, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    s.begin.store(self.now(), Ordering::Relaxed);
                    live = true
                } else {
                    self.invalid.store(true, Ordering::Relaxed)
                }
            } else {
                self.invalid.store(true, Ordering::Relaxed)
            }
        }
        Span {
            trace: self,
            index,
            live,
        }
    }
    pub(crate) fn span<T>(&self, index: usize, f: impl FnOnce() -> T) -> T {
        let _span = self.enter(index);
        f()
    }
    fn valid(&self) -> bool {
        self.route.load(Ordering::Relaxed) == 1
            && !self.invalid.load(Ordering::Relaxed)
            && self
                .slots
                .iter()
                .all(|s| s.state.load(Ordering::Acquire) == 2)
    }
    fn render(&self) -> Result<Buffer, fmt::Error> {
        let mut out = Buffer {
            bytes: [0; 4096],
            len: 0,
        };
        write!(out,"ZOPRO_MSM_PHASES {{\"version\":1,\"workers\":{},\"route\":{},\"valid\":{},\"trace_bytes\":{},\"total_ns\":{},\"rows\":[",self.workers,self.route.load(Ordering::Relaxed),self.valid(),std::mem::size_of::<Self>(),self.now())?;
        for (i, s) in self.slots.iter().enumerate() {
            write!(
                out,
                "{}{{\"phase\":\"{}\",\"state\":{},\"begin_ns\":{},\"end_ns\":{}}}",
                if i == 0 { "" } else { "," },
                NAMES[i],
                s.state.load(Ordering::Acquire),
                s.begin.load(Ordering::Relaxed),
                s.end.load(Ordering::Relaxed)
            )?;
        }
        writeln!(out, "]}}")?;
        Ok(out)
    }
}
impl Drop for Span<'_> {
    fn drop(&mut self) {
        if self.live {
            if std::thread::panicking() {
                self.trace.invalid.store(true, Ordering::Relaxed);
            }
            let s = &self.trace.slots[self.index];
            s.end.store(self.trace.now(), Ordering::Relaxed);
            s.state.store(2, Ordering::Release);
        }
    }
}
impl Drop for Trace {
    fn drop(&mut self) {
        if self.origin.is_some() && self.emit {
            use std::io::Write;
            match self.render() {
                Ok(b) => {
                    let _ = std::io::stderr().lock().write_all(&b.bytes[..b.len]);
                }
                Err(_) => eprintln!("ZOPRO_MSM_PHASES {{\"version\":1,\"format_overflow\":true}}"),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disabled_preserves_value() {
        let t = Trace::with_enabled(false, false);
        assert_eq!(t.span(200, || 17), 17);
        assert!(t.origin.is_none());
        assert!(t.slots.iter().all(|s| s.state.load(Ordering::Relaxed) == 0));
    }
    #[test]
    fn duplicate_and_out_of_range_flagged() {
        let t = Trace::with_enabled(true, false);
        t.span(0, || ());
        t.span(0, || ());
        assert!(t.invalid.load(Ordering::Relaxed));
        let u = Trace::with_enabled(true, false);
        u.span(N, || ());
        assert!(u.invalid.load(Ordering::Relaxed));
    }
    #[test]
    fn unwind_flagged() {
        let t = Trace::with_enabled(true, false);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            t.span(0, || panic!("expected"))
        }));
        assert!(result.is_err());
        assert!(t.invalid.load(Ordering::Relaxed));
        assert_eq!(t.slots[0].state.load(Ordering::Acquire), 2);
    }
    #[test]
    fn independent_parallel_callers() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let p = &pool;
                scope.spawn(move || {
                    p.install(|| {
                        let t = Trace::with_enabled(true, false);
                        t.route(1);
                        for i in 0..3 {
                            t.span(i, || ());
                        }
                        t.span(3, || {
                            rayon::scope(|s| {
                                for i in 4..7 {
                                    let t = &t;
                                    s.spawn(move |_| {
                                        t.span(i, || std::hint::black_box(i));
                                    });
                                }
                            })
                        });
                        t.span(7, || ());
                        assert!(t.valid());
                        let parent = &t.slots[3];
                        for i in 4..7 {
                            assert!(
                                t.slots[i].begin.load(Ordering::Relaxed)
                                    >= parent.begin.load(Ordering::Relaxed)
                            );
                            assert!(
                                t.slots[i].end.load(Ordering::Relaxed)
                                    <= parent.end.load(Ordering::Relaxed)
                            );
                        }
                        assert!(t.render().unwrap().len < 4096);
                    })
                });
            }
        });
    }
    #[test]
    fn incomplete_route_rejected() {
        let t = Trace::with_enabled(true, false);
        for i in 0..N {
            t.span(i, || ());
        }
        assert!(!t.valid());
        t.route(1);
        assert!(t.valid());
        t.route(2);
        assert!(!t.valid());
    }
}
