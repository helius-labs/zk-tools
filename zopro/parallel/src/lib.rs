#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Scoped task and data parallelism on an existing Rayon pool.
//!
//! [`join`] expresses independent tasks. [`for_each_chunk_mut`] distributes the
//! ranges inside a task using indexed splitting with a minimum task grain.
//! Both use the current Rayon pool and finish their work before
//! returning. Calling from outside a pool can initialize Rayon's global pool.
//!
//! ```
//! use zopro_parallel::{for_each_chunk_mut, join};
//!
//! let mut left = [0_u64; 257];
//! let mut right = [0_u64; 513];
//! join(
//!     || for_each_chunk_mut(&mut left, 64, |start, chunk| {
//!         for (offset, value) in chunk.iter_mut().enumerate() {
//!             *value = (start + offset) as u64;
//!         }
//!     }),
//!     || for_each_chunk_mut(&mut right, 64, |start, chunk| {
//!         for (offset, value) in chunk.iter_mut().enumerate() {
//!             *value = 2 * (start + offset) as u64;
//!         }
//!     }),
//! );
//! assert_eq!(left[256], 256);
//! assert_eq!(right[512], 1024);
//! ```
//!
//! This experimental API does not promise a speedup over ordinary Rayon
//! iterators. Coarser tasks can reduce scheduling costs but limit load balancing;
//! measure the complete application with a suitable chunk size. The crate creates no collection
//! buffers or per-call pools. Rayon and callbacks can perform allocations.

use rayon::prelude::*;

/// Execute two independent tasks in the current Rayon pool.
///
/// This re-exports Rayon's scoped fork/join operation, including its borrowing,
/// return-value and panic semantics. A task may call [`for_each_chunk_mut`].
pub use rayon::join;

/// Apply `f` once to each disjoint mutable chunk of `values`.
///
/// The callback receives the chunk's starting index in the original slice and
/// its mutable elements. All chunks except possibly the last contain
/// `chunk_size` elements. Callback order and worker assignment are unspecified.
/// Every chunk completes before this function returns normally.
///
/// Rayon splits the indexed chunk iterator with a minimum task grain of
/// `ceil(number_of_chunks / (2 * current_pool_size))`. This limits splitting
/// while retaining Rayon's work stealing. The policy applies to this call;
/// callbacks may independently create more parallel work.
///
/// Empty slices perform no work. A single chunk is processed directly, without
/// initializing a Rayon pool. A one-worker pool processes chunks sequentially.
///
/// # Panics
///
/// Panics if `chunk_size` is zero. Callback panics propagate through Rayon after
/// outstanding parallel work completes. Other callbacks may have run, and
/// mutations are retained; this function does not provide rollback or promise
/// cancellation of remaining chunks.
///
/// # Example
///
/// ```
/// let mut values = [0_usize; 11];
/// zopro_parallel::for_each_chunk_mut(&mut values, 4, |start, chunk| {
///     for (offset, value) in chunk.iter_mut().enumerate() {
///         *value = start + offset;
///     }
/// });
/// assert_eq!(values, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// ```
pub fn for_each_chunk_mut<T, F>(values: &mut [T], chunk_size: usize, f: F)
where
    T: Send,
    F: Fn(usize, &mut [T]) + Send + Sync,
{
    assert_ne!(chunk_size, 0, "chunk_size must be nonzero");
    if values.is_empty() {
        return;
    }
    if values.len() <= chunk_size {
        f(0, values);
        return;
    }

    let workers = rayon::current_num_threads();
    if workers == 1 {
        for (index, chunk) in values.chunks_mut(chunk_size).enumerate() {
            f(index * chunk_size, chunk);
        }
        return;
    }

    let chunks = values.len().div_ceil(chunk_size);
    let min_chunks_per_task = chunks.div_ceil(workers.saturating_mul(2));
    values
        .par_chunks_mut(chunk_size)
        .with_min_len(min_chunks_per_task)
        .enumerate()
        .for_each(|(index, chunk)| f(index * chunk_size, chunk));
}
