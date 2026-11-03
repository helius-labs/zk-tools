//! A global allocator for hosts of the prover (`#[global_allocator]`).
//!
//! On Apple's systems the system allocator reuses the memory of freed large
//! blocks but keeps it charged to the process: its footprint, which iOS
//! holds against an app's memory limit, only grows. A key's first load,
//! which decompresses its points and builds its solver, left 480 MB of
//! freed vectors charged on an M5 Pro and 200 MB on an iPad, and each
//! proof's transient vectors stay charged at their peak. Here blocks of
//! 1 MB and more are mapped and unmapped directly, so a freed one goes back
//! to the system at once; smaller blocks, and every block elsewhere (Linux
//! and Android give large blocks back), are the system allocator's.
use std::alloc::{GlobalAlloc, Layout, System};

pub struct Allocator;

/// The smallest block mapped directly.
#[cfg(target_vendor = "apple")]
const LARGE: usize = 1 << 20;

/// Whether `layout`'s block is mapped directly: a large block whose
/// alignment a mapping's page provides.
#[inline]
fn mapped(layout: Layout) -> bool {
    #[cfg(target_vendor = "apple")]
    return layout.size() >= LARGE && layout.align() <= 4096;
    #[cfg(not(target_vendor = "apple"))]
    {
        let _ = layout;
        false
    }
}

#[cfg(target_vendor = "apple")]
unsafe fn map(size: usize) -> *mut u8 {
    // SAFETY: an anonymous private mapping of `size` bytes, zeroed.
    let p = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANON | libc::MAP_PRIVATE,
            -1,
            0,
        )
    };
    if p == libc::MAP_FAILED {
        std::ptr::null_mut()
    } else {
        p.cast()
    }
}

// SAFETY: blocks are mapped and unmapped whole, with the size and alignment
// they were asked for; every other block is the system allocator's, which
// `mapped` tells apart by its layout alone.
unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(target_vendor = "apple")]
        if mapped(layout) {
            return unsafe { map(layout.size()) };
        }
        unsafe { System.alloc(layout) }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        #[cfg(target_vendor = "apple")]
        if mapped(layout) {
            // A fresh mapping is zeroed.
            return unsafe { map(layout.size()) };
        }
        unsafe { System.alloc_zeroed(layout) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        #[cfg(target_vendor = "apple")]
        if mapped(layout) {
            unsafe { libc::munmap(ptr.cast(), layout.size()) };
            return;
        }
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: the caller's new size with the block's alignment is a
        // valid layout (GlobalAlloc::realloc's contract).
        let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
        if !mapped(layout) && !mapped(new_layout) {
            return unsafe { System.realloc(ptr, layout, new_size) };
        }
        let new = unsafe { self.alloc(new_layout) };
        if !new.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(ptr, new, layout.size().min(new_size));
                self.dealloc(ptr, layout);
            }
        }
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Blocks on both sides of the threshold, grown and shrunk across it,
    /// keep their contents.
    #[test]
    fn keeps_contents_across_the_threshold() {
        let a = Allocator;
        for (from, to) in [(1000, 2 << 20), (2 << 20, 1000), (3 << 20, 5 << 20), (100, 200)] {
            unsafe {
                let layout = Layout::from_size_align(from, 8).unwrap();
                let p = a.alloc(layout);
                assert!(!p.is_null());
                for i in 0..from.min(to) {
                    *p.add(i) = (i % 251) as u8;
                }
                let q = a.realloc(p, layout, to);
                assert!(!q.is_null());
                for i in 0..from.min(to) {
                    assert_eq!(*q.add(i), (i % 251) as u8);
                }
                a.dealloc(q, Layout::from_size_align(to, 8).unwrap());
                let z = a.alloc_zeroed(Layout::from_size_align(to, 8).unwrap());
                assert!((0..to).all(|i| *z.add(i) == 0));
                a.dealloc(z, Layout::from_size_align(to, 8).unwrap());
            }
        }
    }
}
