use super::*;
use crate::preparation::Reservation;
use hashbrown::{HashTable, hash_table::Entry};
use std::hash::BuildHasher;

const STRIDE: u32 = 4 * MAX_N as u32;

#[derive(Clone, Copy)]
struct State {
    positions: [u32; 13],
    width: u8,
}

impl State {
    fn positions(&self) -> &[u32] {
        &self.positions[..self.width as usize]
    }

    fn hash(&self, words: &[[u64; 4]], state: &impl BuildHasher) -> u64 {
        let mut values = [[0; 4]; 13];
        for (out, &pos) in values.iter_mut().zip(self.positions()) {
            *out = words[pos as usize];
        }
        state.hash_one(&values[..self.width as usize])
    }

    fn matches(&self, other: &Self, words: &[[u64; 4]]) -> bool {
        self.width == other.width
            && self
                .positions()
                .iter()
                .zip(other.positions())
                .all(|(&a, &b)| words[a as usize] == words[b as usize])
    }
}

pub(crate) struct B2Origins {
    order: Vec<u32>,
    states: Vec<State>,
    covered: usize,
    _memory: Reservation,
}

pub(crate) struct RuntimeAliases<'a> {
    order: &'a [u32],
    tags: Vec<u32>,
    classes: Vec<u32>,
}

impl RuntimeAliases<'_> {
    pub(crate) fn slot(&mut self, position: usize) -> Option<&mut u32> {
        let origin = self.order[position];
        if origin == u32::MAX {
            return None;
        }
        let tag = self.tags[(origin / STRIDE) as usize];
        self.classes
            .get_mut((tag * STRIDE + origin % STRIDE) as usize)
    }
}

impl B2Origins {
    pub(crate) fn residual(&self) -> usize {
        self.order.len() - self.covered
    }

    pub(crate) fn scratch_bytes(&self) -> Option<usize> {
        self.states
            .len()
            .checked_mul((STRIDE as usize).checked_mul(4)?.checked_add(64)?)
            .and_then(|n| n.checked_add(256))
    }

    pub(crate) fn aliases<'a>(&'a self, words: &[[u64; 4]]) -> Option<RuntimeAliases<'a>> {
        if words.len() != self.order.len() {
            return None;
        }
        let mut table = HashTable::<u32>::with_capacity(self.states.len());
        let state = ahash::RandomState::new();
        let mut tags = Vec::with_capacity(self.states.len());
        let mut classes = 0u32;
        for (i, span) in self.states.iter().enumerate() {
            let tag = match table.entry(
                span.hash(words, &state),
                |&j| span.matches(&self.states[j as usize], words),
                |&j| self.states[j as usize].hash(words, &state),
            ) {
                Entry::Occupied(e) => tags[*e.get() as usize],
                Entry::Vacant(e) => {
                    e.insert(i as u32);
                    let tag = classes;
                    classes += 1;
                    tag
                }
            };
            tags.push(tag);
        }
        Some(RuntimeAliases {
            order: &self.order,
            tags,
            classes: vec![u32::MAX; (classes * STRIDE) as usize],
        })
    }
}

impl Plan {
    pub(crate) fn b2_origins(&self) -> Option<&B2Origins> {
        if !(65_536..262_144).contains(&self.b_indices.len()) {
            return None;
        }
        if let Some(origins) = self.b2_origins.get() {
            return origins.as_ref();
        }
        let schedule = self.schedule.get()?;
        let wire_count = self.n_wires.checked_add(self.n_slots)?;
        let estimate = wire_count
            .checked_mul(4)?
            .checked_add(self.b_indices.len().checked_mul(4)?)?
            .checked_add(self.ops.len().checked_mul(4)?)?;
        // A transient memory denial leaves the cache available for a retry.
        let memory = Reservation::analysis(Some(estimate))?;
        let origins = self.build_b2_origins(schedule, wire_count, memory);
        let _ = self.b2_origins.set(origins);
        self.b2_origins.get()?.as_ref()
    }

    pub(crate) fn clear_b2_origins(&mut self) {
        self.b2_origins.take();
    }

    fn build_b2_origins(
        &self,
        schedule: &Schedule,
        wire_count: usize,
        memory: Reservation,
    ) -> Option<B2Origins> {
        let mut index = vec![u32::MAX; wire_count];
        for (j, &i) in self.b_indices.iter().enumerate() {
            let slot = index.get_mut(i as usize)?;
            if *slot != u32::MAX {
                return None;
            }
            *slot = j as u32;
        }
        let mut states = Vec::new();
        let mut order = vec![u32::MAX; self.b_indices.len()];
        let mut covered = 0;
        let mut visit = |task: &[u32]| -> Option<()> {
            if !self.poseidon_task(task) {
                return Some(());
            }
            let mut pos = 0;
            while pos < task.len() {
                let width = (task[pos] >> 28) as usize;
                if width == 0 {
                    pos += 1;
                    continue;
                }
                let (partial, _) = parameters(width);
                let n = 7 * width + partial;
                let leaf = &task[pos..pos + 3 * n];
                let mut s = State {
                    positions: [u32::MAX; 13],
                    width: width as u8,
                };
                for (out, &i) in s.positions.iter_mut().zip(&leaf[2 * width..3 * width]) {
                    *out = *index.get(step(self, i)?.assignment.as_ref()?.wire())?;
                }
                pos += 3 * n;
                if s.positions().contains(&u32::MAX) {
                    continue;
                }
                let code = (states.len() as u32).checked_mul(STRIDE)?;
                states.push(s);
                for k in width..n {
                    let [f, m, l] = positions(width, partial, k);
                    let (x, wires) = if leaf[f] & EMIT_MARK != 0 {
                        (
                            (leaf[f] & !(EMIT_MARK | EMIT_NORMAL)) as usize,
                            unpack_three(leaf[l]),
                        )
                    } else {
                        let [a, b, c] = [f, m, l].map(|i| step(self, leaf[i]));
                        let (a, b, c) = (a?, b?, c?);
                        let assignments =
                            [a, b, c].map(|s| s.assignment.as_ref().map(|a| a.wire()));
                        let [square, fourth, fifth] = assignments;
                        (self.n_wires + a.first_slot(), [square?, fourth?, fifth?])
                    };
                    for (coordinate, wire) in
                        [x, wires[0], wires[1], wires[2]].into_iter().enumerate()
                    {
                        let i = *index.get(wire)?;
                        if i == u32::MAX {
                            continue;
                        }
                        let slot = &mut order[i as usize];
                        if *slot != u32::MAX {
                            return None;
                        }
                        *slot = code + (4 * k + coordinate) as u32;
                        covered += 1;
                    }
                }
            }
            Some(())
        };
        for task in schedule.epochs.iter().flatten() {
            if let Some(waves) = schedule.refined.get(&(task[0] as usize)) {
                for leaf in waves.iter().flatten() {
                    visit(leaf)?;
                }
            } else {
                visit(task)?;
            }
        }
        drop(index);
        if covered < order.len() / 8 {
            return None;
        }
        #[cfg(zopro_live_preparation_charge)]
        let memory = memory.retain(
            states
                .capacity()
                .checked_mul(core::mem::size_of::<State>())?
                .checked_add(order.capacity().checked_mul(4)?)
                .and_then(|n| n.checked_add(256)),
        );
        Some(B2Origins {
            order,
            states,
            covered,
            _memory: memory,
        })
    }
}

#[cfg(test)]
#[test]
fn state_origins_compare_all_words_and_keep_widths_separate() {
    let state = |positions: [u32; 3], width| {
        let mut all = [0; 13];
        all[..3].copy_from_slice(&positions);
        State {
            positions: all,
            width,
        }
    };
    let memory = Reservation::analysis(Some(4096)).unwrap();
    let plan = B2Origins {
        order: vec![7, STRIDE + 7, STRIDE * 2 + 7, STRIDE * 3 + 7, u32::MAX],
        states: vec![
            state([0, 1, 2], 3),
            state([0, 1, 2], 3),
            state([0, 1, 3], 3),
            state([0, 1, 2], 2),
        ],
        covered: 4,
        _memory: memory,
    };
    let words = [
        [3, 0, 0, 0],
        [5, 0, 0, 0],
        [7, 0, 0, 0],
        [7, 0, 0, 1],
        [0; 4],
    ];
    let mut aliases = plan.aliases(&words).unwrap();
    *aliases.slot(0).unwrap() = 79;
    assert_eq!(aliases.slot(1).copied(), Some(79));
    assert_eq!(aliases.slot(2).copied(), Some(u32::MAX));
    assert_eq!(aliases.slot(3).copied(), Some(u32::MAX));
    assert!(aliases.slot(4).is_none());
}
