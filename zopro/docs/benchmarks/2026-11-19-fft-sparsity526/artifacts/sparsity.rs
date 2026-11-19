//! Count whole-vector simplifications in the first five inverse layers.
//! The observation runs once per process and never emits field values.
use super::{Fr, Plan};

#[derive(Copy, Clone, Default)]
struct Flags {
    left_zero: bool,
    right_zero: bool,
    equal: bool,
}

impl Flags {
    fn examine(left: &[Fr], right: &[Fr]) -> Self {
        Self {
            left_zero: left.iter().all(|x| *x == Fr::ZERO),
            right_zero: right.iter().all(|x| *x == Fr::ZERO),
            equal: left == right,
        }
    }

    fn intersect(self, other: Self) -> Self {
        Self {
            left_zero: self.left_zero && other.left_zero,
            right_zero: self.right_zero && other.right_zero,
            equal: self.equal && other.equal,
        }
    }

    fn record(self, counters: &mut [u64; 5]) {
        counters[0] += 1;
        counters[1] += u64::from(self.left_zero && self.right_zero);
        counters[2] += u64::from(self.left_zero ^ self.right_zero);
        counters[3] += u64::from(self.equal);
        counters[4] += u64::from(self.equal && !self.left_zero);
    }
}

pub(super) fn observe(plan: &Plan, a: &[Fr], b: &[Fr]) {
    let span = plan.n / 32;
    assert_eq!(span % 8, 0);
    let mut counts = [[[0u64; 5]; 4]; 5];
    for start in (0..span).step_by(8) {
        let read = |v: &[Fr]| -> [[Fr; 8]; 32] {
            core::array::from_fn(|row| core::array::from_fn(|lane|
                v.get(row * span + start + lane).copied().unwrap_or(Fr::ZERO)))
        };
        let mut aa = read(a);
        let mut bb = read(b);
        for (stage, results) in counts.iter_mut().enumerate() {
            let gap = 16 >> stage;
            for block in (0..32).step_by(2 * gap) {
                for j in 0..gap {
                    let lo = block + j;
                    let hi = lo + gap;
                    let (al, ah, bl, bh) = (aa[lo], aa[hi], bb[lo], bb[hi]);
                    for offset in [0, 4] {
                        let r = offset..offset + 4;
                        Flags::examine(&al[r.clone()], &ah[r.clone()])
                            .intersect(Flags::examine(&bl[r.clone()], &bh[r]))
                            .record(&mut results[0]);
                    }
                    Flags::examine(&al, &ah).record(&mut results[1]);
                    Flags::examine(&bl, &bh).record(&mut results[2]);
                    let dl: [Fr; 8] = core::array::from_fn(|i| bl[i] - al[i]);
                    let dh: [Fr; 8] = core::array::from_fn(|i| bh[i] - ah[i]);
                    Flags::examine(&dl, &dh).record(&mut results[3]);
                    if stage == 4 { continue; }
                    for h in 0..2 {
                        let root = plan.inverse_roots.layers[stage][(j * span + start) / 4 + h].get();
                        for (k, w) in root.into_iter().enumerate() {
                            let i = 4 * h + k;
                            aa[lo][i] = al[i] + ah[i];
                            bb[lo][i] = bl[i] + bh[i];
                            aa[hi][i] = (al[i] - ah[i]) * w;
                            bb[hi][i] = (bl[i] - bh[i]) * w;
                        }
                    }
                }
            }
        }
    }
    for (stage, row) in counts.iter().enumerate() {
        for (layout, values) in row.iter().enumerate() {
            eprintln!("FFT_COUNTS {} {} {} {} {} {} {}",stage,layout,values[0],values[1],values[2],values[3],values[4]);
        }
    }
}
