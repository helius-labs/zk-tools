//! Private Linux diagnostic: one active request, bounded process-wide stamps.
//! RUSAGE_SELF includes every process thread; it is not caller CPU accounting.
use std::sync::{Mutex, atomic::{AtomicBool, Ordering}};

pub const CAPACITY: usize = 32;

#[derive(Clone, Copy, Debug)]
pub struct Stamp {
    pub phase: &'static str,
    pub clock_before_ns: u64,
    pub clock_after_ns: u64,
    pub user_us: i64,
    pub system_us: i64,
    pub minor_faults: i64,
    pub major_faults: i64,
    pub voluntary_switches: i64,
    pub involuntary_switches: i64,
    pub tid: i64,
}

#[derive(Clone, Copy, Debug)]
pub struct Snapshot {
    pub stamps: [Option<Stamp>; CAPACITY],
    pub len: usize,
}

struct State {
    generation: u64,
    occupied: bool,
    recording: bool,
    failed: bool,
    snapshot: Snapshot,
}
static ENABLED: AtomicBool = AtomicBool::new(false);
static STATE: Mutex<State> = Mutex::new(State {
    generation: 0, occupied: false, recording: false, failed: false,
    snapshot: Snapshot { stamps: [None; CAPACITY], len: 0 },
});

fn clock() -> Option<u64> {
    let mut time = std::mem::MaybeUninit::<libc::timespec>::uninit();
    // SAFETY: CLOCK_MONOTONIC writes the whole timespec on success.
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, time.as_mut_ptr()) } != 0 {
        return None;
    }
    let time = unsafe { time.assume_init() };
    u64::try_from(time.tv_sec).ok()?.checked_mul(1_000_000_000)?
        .checked_add(u64::try_from(time.tv_nsec).ok()?)
}

fn stamp(phase: &'static str) -> Option<Stamp> {
    let clock_before_ns = clock()?;
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    // SAFETY: getrusage writes the whole rusage on successful return.
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
        return None;
    }
    let usage = unsafe { usage.assume_init() };
    let clock_after_ns = clock()?;
    let micros = |t: libc::timeval| t.tv_sec.checked_mul(1_000_000)?.checked_add(t.tv_usec);
    Some(Stamp {
        phase, clock_before_ns, clock_after_ns,
        user_us: micros(usage.ru_utime)?, system_us: micros(usage.ru_stime)?,
        minor_faults: usage.ru_minflt, major_faults: usage.ru_majflt,
        voluntary_switches: usage.ru_nvcsw, involuntary_switches: usage.ru_nivcsw,
        // SAFETY: gettid has no arguments and returns the calling Linux TID.
        tid: unsafe { libc::syscall(libc::SYS_gettid) },
    })
}

#[inline]
pub fn mark(phase: &'static str) {
    if !ENABLED.load(Ordering::Relaxed) { return; }
    let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if !state.recording { return; }
    if state.snapshot.len == CAPACITY {
        state.failed = true;
        return;
    }
    if let Some(value) = stamp(phase) {
        let index = state.snapshot.len;
        state.snapshot.stamps[index] = Some(value);
        state.snapshot.len += 1;
    } else {
        state.failed = true;
    }
}

pub struct Capture { generation: u64, finished: bool }
impl Capture {
    pub fn start(enabled: bool) -> Result<Self, &'static str> {
        let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
        if state.occupied { return Err("phase capture already has an active owner"); }
        state.generation = state.generation.checked_add(1).ok_or("phase generation overflow")?;
        state.occupied = true;
        state.recording = enabled;
        state.failed = false;
        state.snapshot = Snapshot { stamps: [None; CAPACITY], len: 0 };
        ENABLED.store(enabled, Ordering::SeqCst);
        Ok(Self { generation: state.generation, finished: false })
    }

    pub fn finish(mut self) -> Result<Snapshot, &'static str> {
        let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
        assert!(state.occupied && state.generation == self.generation);
        ENABLED.store(false, Ordering::SeqCst);
        state.recording = false;
        state.occupied = false;
        self.finished = true;
        if state.failed { return Err("phase stamp failed or capacity exceeded"); }
        Ok(state.snapshot)
    }
}
impl Drop for Capture {
    fn drop(&mut self) {
        if self.finished { return; }
        let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
        if state.occupied && state.generation == self.generation {
            ENABLED.store(false, Ordering::SeqCst);
            state.recording = false;
            state.occupied = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_cross_thread_process_accounting_ownership_and_unwind() {
        let disabled = Capture::start(false).unwrap();
        mark("disabled");
        assert!(Capture::start(true).is_err());
        assert_eq!(disabled.finish().unwrap().len, 0);

        let pool = rayon::ThreadPoolBuilder::new().num_threads(2)
            .stack_size(512 * 1024).build().unwrap();
        let capture = Capture::start(true).unwrap();
        mark("caller");
        pool.install(|| mark("worker"));
        mark("done");
        let result = capture.finish().unwrap();
        assert_eq!(result.len, 3);
        let values = result.stamps[..result.len].iter().map(|s| s.unwrap()).collect::<Vec<_>>();
        assert_ne!(values[0].tid, values[1].tid);
        assert_eq!(values[0].tid, values[2].tid);
        for event in &values { assert!(event.clock_before_ns <= event.clock_after_ns); }
        for pair in values.windows(2) {
            assert!(pair[0].clock_after_ns <= pair[1].clock_before_ns);
            assert!(pair[0].user_us <= pair[1].user_us);
            assert!(pair[0].system_us <= pair[1].system_us);
            assert!(pair[0].minor_faults <= pair[1].minor_faults);
            assert!(pair[0].voluntary_switches <= pair[1].voluntary_switches);
        }
        let failed = std::panic::catch_unwind(|| {
            let _capture = Capture::start(true).unwrap();
            mark("before_unwind");
            panic!("diagnostic owner unwinds");
        });
        assert!(failed.is_err());
        let overflow = Capture::start(true).unwrap();
        for _ in 0..=CAPACITY { mark("overflow"); }
        assert!(overflow.finish().is_err());
        let reused = Capture::start(true).unwrap();
        mark("reused");
        assert_eq!(reused.finish().unwrap().len, 1);
        mark("disabled_after_finish");
    }
}
