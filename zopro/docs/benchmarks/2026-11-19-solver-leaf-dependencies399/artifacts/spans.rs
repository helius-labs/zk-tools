// Replay the three unlimited-worker spans from the diagnostic's LEAF output.
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufRead};

#[derive(Default)]
struct Parent {
    wave_costs: BTreeMap<usize, u64>,
    dependencies: BTreeSet<(usize, usize)>,
}

#[derive(Default)]
struct Model {
    leaves: Vec<((usize, usize), u64)>,
    parents: BTreeMap<(usize, usize), Parent>,
}

impl Model {
    fn add(&mut self, id: usize, owner: (usize, usize), wave: usize, cost: u64, inputs: &[usize]) {
        assert_eq!(id, self.leaves.len());
        let parent = self.parents.entry(owner).or_default();
        let wave_cost = parent.wave_costs.entry(wave).or_default();
        *wave_cost = (*wave_cost).max(cost);
        let mut start = 0;
        for &input in inputs {
            let (other, finish) = self.leaves[input];
            start = start.max(finish);
            if other != owner {
                assert!(other.0 < owner.0);
                parent.dependencies.insert(other);
            }
        }
        self.leaves.push((owner, start + cost));
    }

    fn spans(self) -> (u64, u64, u64) {
        let mut epochs = BTreeMap::new();
        let mut finishes = BTreeMap::new();
        for (owner, parent) in self.parents {
            let span = parent.wave_costs.values().sum::<u64>();
            let longest = epochs.entry(owner.0).or_insert(0u64);
            *longest = (*longest).max(span);
            let start = parent
                .dependencies
                .iter()
                .map(|p| finishes[p])
                .max()
                .unwrap_or(0);
            finishes.insert(owner, start + span);
        }
        (
            epochs.values().sum(),
            finishes.values().copied().max().unwrap_or(0),
            self.leaves.iter().map(|&(_, t)| t).max().unwrap_or(0),
        )
    }
}

fn main() {
    let mut model = Model::default();
    for line in io::stdin().lock().lines() {
        let line = line.expect("read diagnostic");
        let Some(row) = line.strip_prefix("LEAF ") else {
            continue;
        };
        let (fields, parents) = row.split_once('{').expect("edge set");
        let values: Vec<usize> = fields
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let [id, epoch, parent, wave, cost] = values[..] else {
            panic!("invalid task header")
        };
        let inputs: Vec<usize> = parents
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        model.add(id, (epoch, parent), wave, cost as u64, &inputs);
    }
    assert!(!model.leaves.is_empty());
    let (current, parent, leaf) = model.spans();
    println!("{current} {parent} {leaf}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn joining_all_parents_delays_the_longer_successor() {
        let mut m = Model::default();
        m.add(0, (0, 0), 0, 10, &[]);
        m.add(1, (0, 1), 0, 2, &[]);
        m.add(2, (1, 0), 0, 10, &[1]);
        m.add(3, (1, 1), 0, 1, &[0]);
        assert_eq!(m.spans(), (20, 12, 12));
    }
    #[test]
    fn distinct_inner_dependencies_release_one_leaf_early() {
        let mut m = Model::default();
        m.add(0, (0, 0), 0, 10, &[]);
        m.add(1, (0, 0), 0, 2, &[]);
        m.add(2, (0, 0), 1, 9, &[1]);
        m.add(3, (0, 1), 0, 4, &[]);
        m.add(4, (1, 0), 0, 5, &[2, 3]);
        assert_eq!(m.spans(), (24, 24, 16));
    }
}
