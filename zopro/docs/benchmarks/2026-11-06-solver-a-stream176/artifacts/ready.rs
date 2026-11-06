//! Joined-epoch publication of initialized persistent A-query wires.
use super::*;
use crate::msm_affine::borrowed_pending::early_stream::{Chunk, CHUNK};
use std::sync::Arc;

pub(crate) struct Ready<'plan> {
    plan: &'plan Plan,
    epochs: Vec<u16>,
    epoch_count: usize,
    _reservation: crate::preparation::Reservation,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Published {
    pub chunks: usize,
    pub values: usize,
    pub early_chunks: usize,
    pub early_values: usize,
}

fn derive_epochs(writers: &[u16], indices: &[u32], chunk: usize) -> Option<Vec<u16>> {
    indices
        .chunks(chunk)
        .map(|indices| {
            indices.iter().try_fold(0, |epoch, &wire| {
                let writer = *writers.get(wire as usize)?;
                (writer != u16::MAX).then_some(epoch.max(writer))
            })
        })
        .collect()
}

impl<'plan> Ready<'plan> {
    pub(crate) fn prepare(plan: &'plan Plan) -> Option<Self> {
        let schedule = plan.schedule.get()?;
        if solve_epochs() == 0
            || plan.ops.len() < 4096
            || schedule.epochs.is_empty()
            || !schedule.refined.is_empty()
            || schedule.epochs.len() >= u16::MAX as usize
        {
            return None;
        }
        let n = plan.n_wires.checked_add(plan.n_slots)?;
        let retained = plan
            .a_indices
            .len()
            .div_ceil(CHUNK)
            .checked_mul(2)?
            .checked_add(core::mem::size_of::<Self>())?
            .checked_add(128)?;
        // Persistent-wire map plus a worst-sized transient writes() vector.
        // The existing definite-assignment checker owns its own reservation.
        let peak = plan
            .n_wires
            .checked_mul(2)?
            .checked_add(n.checked_mul(16)?)?
            .checked_add(retained)?;
        let reservation = crate::preparation::Reservation::analysis(Some(peak))?;
        if !*plan.definite.get_or_init(|| plan.definitely_writes_all()) {
            return None;
        }
        let mut writers = vec![u16::MAX; plan.n_wires];
        writers.get_mut(..plan.n_inputs)?.fill(0);
        for (epoch, tasks) in schedule.epochs.iter().enumerate() {
            for task in tasks {
                let encoded = plan.poseidon_task(task);
                for &raw in task {
                    let op = if encoded { raw & ((1 << 27) - 1) } else { raw };
                    for wire in plan.writes(plan.ops.get(op as usize)?) {
                        if wire >= plan.n_wires {
                            continue;
                        }
                        if writers[wire] != u16::MAX {
                            return None;
                        }
                        writers[wire] = (epoch + 1) as u16;
                    }
                }
            }
        }
        let epochs = derive_epochs(&writers, &plan.a_indices, CHUNK)?;
        drop(writers);
        let retained = epochs
            .capacity()
            .checked_mul(2)?
            .checked_add(core::mem::size_of::<Self>())?
            .checked_add(128)?;
        let reservation = reservation.retain(Some(retained));
        Some(Self {
            plan,
            epochs,
            epoch_count: schedule.epochs.len(),
            _reservation: reservation,
        })
    }

    pub(crate) fn chunk_count(&self) -> usize {
        self.epochs.len()
    }

    pub(crate) fn solve<F>(
        &self,
        witness: &[Fr],
        domain: usize,
        slots: &[usize],
        materialize_c: bool,
        sink: F,
    ) -> Result<(Solution, Published), String>
    where
        F: FnMut(Arc<Chunk>) -> Result<(), String>,
    {
        let mut delivery = Delivery {
            ready: self,
            seen: vec![false; self.epochs.len()],
            sink,
            published: Published::default(),
        };
        let solution =
            self.plan
                .solve_with_a_feed(witness, domain, slots, materialize_c, &mut delivery)?;
        if delivery.seen.iter().any(|sent| !sent) {
            return Err("solver left an A chunk unpublished".into());
        }
        Ok((solution, delivery.published))
    }
}

struct Delivery<'ready, 'plan, F> {
    ready: &'ready Ready<'plan>,
    seen: Vec<bool>,
    sink: F,
    published: Published,
}

/// Read only the individually initialized indices. No slice of the full
/// allocation is formed while the original Vec length may still be zero.
///
/// # Safety
/// Each selected wire is within w's allocation, initialized, has a unique
/// writer, and that writer's complete epoch has joined. No later op writes it.
unsafe fn copy_chunk(view: View, indices: &[u32], start: usize, end: usize) -> Chunk {
    let mut words = Vec::with_capacity(end - start);
    for &wire in &indices[start..end] {
        // SAFETY: the caller establishes the per-index initialization and
        // unique-writer epoch conditions above; Fr is Copy.
        let value = unsafe { view.w.add(wire as usize).read() };
        words.push(value.to_canonical());
    }
    Chunk { start, words }
}

impl<F: FnMut(Arc<Chunk>) -> Result<(), String>> Delivery<'_, '_, F> {
    /// The solver calls this only after input initialization or a joined epoch.
    unsafe fn publish(
        &mut self,
        epoch: usize,
        all_finished: bool,
        view: View,
    ) -> Result<(), String> {
        for index in 0..self.ready.epochs.len() {
            if self.seen[index] || (!all_finished && self.ready.epochs[index] as usize > epoch) {
                continue;
            }
            let start = index * CHUNK;
            let end = (start + CHUNK).min(self.ready.plan.a_indices.len());
            // SAFETY: Ready proves unique persistent writers. Its epoch map
            // permits only fields whose writes have joined; all_finished is
            // used only after every operation succeeds, before slot compaction.
            let chunk = unsafe { copy_chunk(view, &self.ready.plan.a_indices, start, end) };
            (self.sink)(Arc::new(chunk))?;
            self.seen[index] = true;
            self.published.chunks += 1;
            self.published.values += end - start;
            if !all_finished && epoch < self.ready.epoch_count {
                self.published.early_chunks += 1;
                self.published.early_values += end - start;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn epochs_cover_only_initialized_bounded_wires() {
        let writers = [0, 3, 1, 2, u16::MAX, 4];
        assert_eq!(
            derive_epochs(&writers, &[0, 2, 3, 1, 5], 2),
            Some(vec![1, 3, 4])
        );
        assert!(derive_epochs(&writers, &[4], 2).is_none());
        assert!(derive_epochs(&writers, &[6], 2).is_none());
        assert_eq!(derive_epochs(&writers, &[], CHUNK), Some(vec![]));
    }
    #[test]
    fn copy_does_not_require_a_fully_initialized_vector() {
        let mut storage = vec![core::mem::MaybeUninit::<Fr>::uninit(); 4096];
        for (wire, value) in [(3, 7u64), (4095, 11), (9, 13)] {
            storage[wire].write(Fr::from(value));
        }
        let view = View {
            w: storage.as_mut_ptr().cast(),
            a: core::ptr::null_mut(),
            b: core::ptr::null_mut(),
            c: core::ptr::null_mut(),
        };
        // SAFETY: only these three cells have been initialized, and no writer
        // exists while the narrow copy runs. Every other cell remains unread.
        let chunk = unsafe { copy_chunk(view, &[3, 4095, 9], 0, 3) };
        assert_eq!(
            chunk.words,
            [7u64, 11, 13].map(|v| Fr::from(v).to_canonical())
        );
    }
}
