//! Linux allocator routing. Policy can change while allocations
//! remain live; each eligible block records its own allocation backend.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

const LARGE: usize = 128 << 10;
static RECLAIM_USERS: AtomicUsize = AtomicUsize::new(0);

pub struct Guard(bool);
impl Guard {
    pub fn enter() -> Self {
        Self(
            RECLAIM_USERS
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
                .is_ok(),
        )
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        if self.0 {
            RECLAIM_USERS.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

// Count active proof forecasts, not all loaded keys. An unknown/overflowing
// forecast or unreadable headroom chooses prompt reclamation conservatively.
static ACTIVE_BYTES: AtomicUsize = AtomicUsize::new(0);
pub(crate) struct ProofScope {
    bytes: Option<usize>,
    _reclaim: Option<Guard>,
}
impl ProofScope {
    /// Release a large unused tail while this proof's pressure guard is held.
    /// Keeping both layouts eligible lets our allocator shrink with mremap.
    pub(crate) fn release_unused<T>(&self, values: &mut Vec<T>) {
        let size = core::mem::size_of::<T>();
        let kept = values.len().saturating_mul(size);
        let unused = (values.capacity() - values.len()).saturating_mul(size);
        if self._reclaim.is_some() && kept >= LARGE && unused >= LARGE {
            values.shrink_to_fit();
        }
    }
    pub(crate) fn enter(workspace: Option<usize>) -> Self {
        let bytes = workspace.filter(|&bytes| {
            ACTIVE_BYTES
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                    n.checked_add(bytes)
                })
                .is_ok()
        });
        let reclaim = bytes.is_none()
            || needs_reclaim(
                crate::preparation::headroom(),
                ACTIVE_BYTES.load(Ordering::Acquire),
            );
        let reclaim = reclaim.then(Guard::enter);
        // A server may have warmed up with ample RAM before pressure arrived.
        // Return free pages retained by that earlier heap phase as well as
        // promptly releasing new large buffers. This does not change malloc's
        // process-wide configuration or free any live allocation.
        #[cfg(target_env = "gnu")]
        if reclaim.is_some() {
            unsafe {
                libc::malloc_trim(0);
            }
        }
        Self {
            bytes,
            _reclaim: reclaim,
        }
    }
}
impl Drop for ProofScope {
    fn drop(&mut self) {
        if let Some(bytes) = self.bytes {
            ACTIVE_BYTES.fetch_sub(bytes, Ordering::AcqRel);
        }
    }
}
fn needs_reclaim(available: Option<usize>, forecast: usize) -> bool {
    available.map_or(true, |available| forecast > available)
}

pub struct Allocator;
#[inline]
fn eligible(layout: Layout) -> bool {
    layout.size() >= LARGE && layout.align() <= 4096
}

// The backend byte follows the payload. The caller receives the backend's
// original pointer and alignment; there is no prefix or pointer adjustment.
// Both backends reserve the byte so ownership does not depend on live policy.
#[inline]
fn backing(layout: Layout) -> Option<Layout> {
    Layout::from_size_align(layout.size().checked_add(1)?, layout.align()).ok()
}

impl Allocator {
    unsafe fn allocate(&self, layout: Layout, zero: bool) -> *mut u8 {
        if !eligible(layout) {
            return if zero {
                unsafe { System.alloc_zeroed(layout) }
            } else {
                unsafe { System.alloc(layout) }
            };
        }
        let Some(backing) = backing(layout) else {
            return std::ptr::null_mut();
        };
        let mapped = RECLAIM_USERS.load(Ordering::Relaxed) != 0;
        let ptr = if mapped {
            let p = unsafe {
                libc::mmap(
                    std::ptr::null_mut(),
                    backing.size(),
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                    -1,
                    0,
                )
            };
            if p == libc::MAP_FAILED {
                return std::ptr::null_mut();
            }
            p.cast::<u8>()
        } else if zero {
            unsafe { System.alloc_zeroed(backing) }
        } else {
            unsafe { System.alloc(backing) }
        };
        if !ptr.is_null() {
            // The tag is outside the requested payload, including alloc_zeroed.
            unsafe { ptr.add(layout.size()).write(u8::from(mapped)); }
        }
        ptr
    }
}

// SAFETY: every eligible allocation owns one byte beyond its payload. Reads
// use the layout that belongs to that live allocation, independent of policy.
// Reallocation reads the old tag before moving/shrinking and writes a new tag
// only after success. Failure leaves the old pointer, tag and payload intact.
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.allocate(layout, false) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { self.allocate(layout, true) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if !eligible(layout) {
            unsafe { System.dealloc(ptr, layout); }
            return;
        }
        let Some(backing) = backing(layout) else {
            std::process::abort();
        };
        if unsafe { ptr.add(layout.size()).read() } == 1 {
            unsafe { libc::munmap(ptr.cast(), backing.size()); }
        } else {
            unsafe { System.dealloc(ptr, backing); }
        }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let Ok(next) = Layout::from_size_align(new_size, layout.align()) else {
            return std::ptr::null_mut();
        };
        if !eligible(layout) && !eligible(next) {
            return unsafe { System.realloc(ptr, layout, new_size) };
        }
        if eligible(layout) && eligible(next) {
            let Some(old_backing) = backing(layout) else {
                std::process::abort();
            };
            let Some(new_backing) = backing(next) else {
                return std::ptr::null_mut();
            };
            let was_mapped = unsafe { ptr.add(layout.size()).read() } == 1;
            let map_now = RECLAIM_USERS.load(Ordering::Relaxed) != 0;
            if !was_mapped && !map_now {
                let next_ptr = unsafe { System.realloc(ptr, old_backing, new_backing.size()) };
                if !next_ptr.is_null() {
                    unsafe { next_ptr.add(new_size).write(0); }
                }
                return next_ptr;
            }
            if was_mapped && map_now {
                // Linux mremap avoids simultaneously holding old and new copies.
                let next_ptr = unsafe {
                    libc::mremap(
                        ptr.cast(),
                        old_backing.size(),
                        new_backing.size(),
                        libc::MREMAP_MAYMOVE,
                    )
                };
                if next_ptr == libc::MAP_FAILED {
                    return std::ptr::null_mut();
                }
                let next_ptr = next_ptr.cast::<u8>();
                unsafe { next_ptr.add(new_size).write(1); }
                return next_ptr;
            }
        }
        let next_ptr = unsafe { self.alloc(next) };
        if !next_ptr.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(ptr, next_ptr, layout.size().min(new_size));
                self.dealloc(ptr, layout);
            }
        }
        next_ptr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pressure_scope_releases_only_large_unused_capacity() {
        let mut values = Vec::with_capacity(LARGE / 8 * 4);
        values.extend((0..LARGE / 8 * 2).map(|i| i as u64 ^ 0x91823));
        let original_capacity = values.capacity();
        let unpressured = ProofScope { bytes: None, _reclaim: None };
        unpressured.release_unused(&mut values);
        assert_eq!(values.capacity(), original_capacity);
        let pressured = ProofScope::enter(None);
        pressured.release_unused(&mut values);
        assert_eq!(values.capacity(), values.len());
        assert!(values.iter().enumerate().all(|(i, &v)| v == i as u64 ^ 0x91823));
        let mut small_tail = Vec::<u64>::with_capacity(LARGE / 8 + 1);
        small_tail.resize(LARGE / 8, 37);
        let before = small_tail.capacity();
        pressured.release_unused(&mut small_tail);
        assert_eq!(small_tail.capacity(), before);
        let mut zero = Vec::<()>::with_capacity(10);
        zero.resize(3, ());
        pressured.release_unused(&mut zero);
        assert_eq!(zero.len(), 3);
    }
    #[test]
    fn forecast_policy_uses_available_memory_and_releases_charges() {
        assert!(!needs_reclaim(Some(100), 99));
        assert!(!needs_reclaim(Some(100), 100));
        assert!(needs_reclaim(Some(100), 101));
        assert!(needs_reclaim(None, 1));
        let unknown = ProofScope::enter(None);
        assert!(unknown._reclaim.is_some());
        drop(unknown);
        let first = ProofScope::enter(Some(17));
        let second = ProofScope::enter(Some(31));
        assert_eq!(ACTIVE_BYTES.load(Ordering::Acquire), 48);
        drop(first);
        assert_eq!(ACTIVE_BYTES.load(Ordering::Acquire), 31);
        drop(second);
        assert_eq!(ACTIVE_BYTES.load(Ordering::Acquire), 0);
        let maximum = ProofScope::enter(Some(usize::MAX));
        let overflow = ProofScope::enter(Some(1));
        assert!(overflow.bytes.is_none());
        assert!(overflow._reclaim.is_some());
        drop(maximum);
        drop(overflow);
        assert_eq!(ACTIVE_BYTES.load(Ordering::Acquire), 0);
        assert_eq!(RECLAIM_USERS.load(Ordering::Relaxed), 0);
    }
    unsafe fn route(ptr: *mut u8, layout: Layout) -> bool {
        eligible(layout) && unsafe { ptr.add(layout.size()).read() == 1 }
    }
    #[test]
    fn layouts_zeroing_and_policy_transitions_preserve_contents() {
        let a = Allocator;
        let mut count = 0;
        for align in [1, 8, 16, 64, 256, 4096, 8192, 65536] {
            for (from, to) in [
                (17, 31),
                (LARGE - 1, LARGE),
                (LARGE, LARGE - 1),
                (LARGE, LARGE + 4096),
                (LARGE + 4096, LARGE),
                (LARGE - 1, LARGE + 1),
                (LARGE + 7, LARGE - 3),
                (LARGE, LARGE * 2 + 3),
                (LARGE * 2 + 17, LARGE + 1),
            ] {
                for initial in [false, true] {
                    for later in [false, true] {
                        let layout = Layout::from_size_align(from, align).unwrap();
                        let guard = initial.then(Guard::enter);
                        let p = unsafe { a.alloc_zeroed(layout) };
                        assert!(!p.is_null());
                        assert_eq!(p as usize % align, 0);
                        assert_eq!(unsafe { route(p, layout) }, initial && eligible(layout));
                        assert!(unsafe { std::slice::from_raw_parts(p, from) }
                            .iter()
                            .all(|&b| b == 0));
                        for i in 0..from {
                            unsafe {
                                p.add(i).write((i % 251) as u8);
                            }
                        }
                        drop(guard);
                        let guard = later.then(Guard::enter);
                        let q = unsafe { a.realloc(p, layout, to) };
                        assert!(!q.is_null());
                        assert_eq!(q as usize % align, 0);
                        let next = Layout::from_size_align(to, align).unwrap();
                        assert_eq!(unsafe { route(q, next) }, later && eligible(next));
                        for i in 0..from.min(to) {
                            assert_eq!(unsafe { q.add(i).read() }, (i % 251) as u8);
                        }
                        drop(guard);
                        unsafe {
                            a.dealloc(q, next);
                        }
                        count += 1;
                    }
                }
            }
        }
        assert_eq!(count, 288);
        assert_eq!(RECLAIM_USERS.load(Ordering::Relaxed), 0);
    }
    #[test]
    fn nested_guards_and_cross_thread_deallocation() {
        let a = Allocator;
        let layout = Layout::from_size_align(LARGE + 17, 64).unwrap();
        let first = Guard::enter();
        let second = Guard::enter();
        drop(first);
        let p = unsafe { a.alloc(layout) };
        assert!(unsafe { route(p, layout) });
        unsafe {
            p.write(99);
            p.add(layout.size() - 1).write(17);
        }
        drop(second);
        let address = p as usize;
        std::thread::spawn(move || {
            let p = address as *mut u8;
            assert_eq!(unsafe { p.read() }, 99);
            assert_eq!(unsafe { p.add(layout.size() - 1).read() }, 17);
            unsafe {
                Allocator.dealloc(p, layout);
            }
        })
        .join()
        .unwrap();
        assert_eq!(RECLAIM_USERS.load(Ordering::Relaxed), 0);
    }
    #[test]
    fn concurrent_policy_changes_keep_each_block_owned_by_its_backend() {
        std::thread::scope(|scope| {
            for thread in 0..8 {
                scope.spawn(move || {
                    for i in 0..64 {
                        let guard = ((thread + i) % 2 == 0).then(Guard::enter);
                        let len = LARGE + (thread * 64 + i) * 32;
                        let layout = Layout::from_size_align(len, 64).unwrap();
                        let p = unsafe { Allocator.alloc(layout) };
                        assert!(!p.is_null());
                        unsafe {
                            p.write(thread as u8);
                            p.add(len - 1).write(i as u8);
                        }
                        drop(guard);
                        std::thread::yield_now();
                        let guard = ((thread + i) % 3 == 0).then(Guard::enter);
                        let q = unsafe { Allocator.realloc(p, layout, len * 2) };
                        assert!(!q.is_null());
                        assert_eq!(unsafe { q.read() }, thread as u8);
                        assert_eq!(unsafe { q.add(len - 1).read() }, i as u8);
                        drop(guard);
                        unsafe {
                            Allocator.dealloc(q, Layout::from_size_align(len * 2, 64).unwrap());
                        }
                    }
                });
            }
        });
        assert_eq!(RECLAIM_USERS.load(Ordering::Relaxed), 0);
    }
    #[test]
    fn oversized_tail_failure_keeps_old_block_valid() {
        for mapped in [false, true] {
            let guard = mapped.then(Guard::enter);
            let layout = Layout::from_size_align(LARGE, 8).unwrap();
            let p = unsafe { Allocator.alloc(layout) };
            assert!(!p.is_null());
            unsafe {
                p.write(17);
                p.add(LARGE - 1).write(91);
            }
            let impossible = isize::MAX as usize - 7;
            assert!(Layout::from_size_align(impossible, 8).is_ok());
            assert!(
                unsafe { Allocator.alloc(Layout::from_size_align(impossible, 8).unwrap()) }
                    .is_null()
            );
            assert!(unsafe { Allocator.realloc(p, layout, impossible) }.is_null());
            assert_eq!(unsafe { p.read() }, 17);
            assert_eq!(unsafe { p.add(LARGE - 1).read() }, 91);
            drop(guard);
            unsafe {
                Allocator.dealloc(p, layout);
            }
        }
    }
    #[test]
    fn mapped_free_returns_the_region_after_scope_ends() {
        let guard = Guard::enter();
        let layout = Layout::from_size_align(LARGE, 64).unwrap();
        let p = unsafe { Allocator.alloc(layout) };
        assert!(!p.is_null());
        let base = p;
        let tail_page = unsafe { p.add(LARGE) };
        unsafe {
            p.write(1);
        }
        drop(guard);
        unsafe {
            Allocator.dealloc(p, layout);
        }
        let mut residency = 0u8;
        let rc = unsafe { libc::mincore(base.cast(), 4096, &mut residency) };
        assert_eq!(rc, -1);
        assert_eq!(unsafe { *libc::__errno_location() }, libc::ENOMEM);
        // A page-multiple payload puts the backend byte on an extra page.
        // Deallocation must release that page too.
        let rc = unsafe { libc::mincore(tail_page.cast(), 4096, &mut residency) };
        assert_eq!(rc, -1);
        assert_eq!(unsafe { *libc::__errno_location() }, libc::ENOMEM);
    }
}
