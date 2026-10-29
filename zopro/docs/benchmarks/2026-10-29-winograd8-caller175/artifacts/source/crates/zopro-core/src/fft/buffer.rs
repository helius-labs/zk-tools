//! Expanded SIMD quads borrow storage from the input allocation. Its Vec<Fr>
//! remains the owner, so deallocation uses the original size/alignment.
use super::{quads, scalars, Fr, Quad};
use rayon::prelude::*;
use std::{
    mem::{align_of, size_of, MaybeUninit},
    ops::{Deref, DerefMut},
};

const INPUT: usize = 4 * size_of::<Fr>();
const OUTPUT: usize = size_of::<Quad>();
const SERIAL: usize = 2048;

enum Data {
    InPlace {
        owner: Vec<Fr>,
        offset: usize,
        len: usize,
    },
    Separate(Vec<Quad>),
}
pub(crate) struct Values(Data);
impl From<Vec<Quad>> for Values {
    fn from(values: Vec<Quad>) -> Self {
        Self(Data::Separate(values))
    }
}
impl Deref for Values {
    type Target = [Quad];
    fn deref(&self) -> &[Quad] {
        match &self.0 {
            Data::Separate(values) => values,
            Data::InPlace { owner, offset, len } => {
                // SAFETY: pack initialized len aligned quads in this owned
                // allocation. Nothing exposes the overwritten Fr slice.
                unsafe {
                    std::slice::from_raw_parts(
                        owner.as_ptr().cast::<u8>().add(*offset).cast::<Quad>(),
                        *len,
                    )
                }
            }
        }
    }
}
impl DerefMut for Values {
    fn deref_mut(&mut self) -> &mut [Quad] {
        match &mut self.0 {
            Data::Separate(values) => values,
            Data::InPlace { owner, offset, len } => {
                // SAFETY: as Deref, with exclusive access to the owner.
                unsafe {
                    std::slice::from_raw_parts_mut(
                        owner.as_mut_ptr().cast::<u8>().add(*offset).cast::<Quad>(),
                        *len,
                    )
                }
            }
        }
    }
}
impl Values {
    pub(super) fn pack(mut owner: Vec<Fr>) -> Self {
        if owner.is_empty() || !owner.len().is_multiple_of(4) {
            return quads(&owner).into();
        }
        assert!(OUTPUT > INPUT);
        let len = owner.len() / 4;
        let needed = (len * OUTPUT + align_of::<Quad>() - 1).div_ceil(size_of::<Fr>());
        if owner.capacity() < needed {
            owner.reserve_exact(needed - owner.len());
        }
        let raw = owner.as_mut_ptr();
        let offset = raw.cast::<u8>().align_offset(align_of::<Quad>());
        assert!(offset < align_of::<Quad>());
        // SAFETY: the reserve covers all output bytes and the alignment
        // padding. The Fr array stays the allocation's sole owner.
        let packed = unsafe { raw.cast::<u8>().add(offset).cast::<MaybeUninit<Quad>>() };
        let mut end = len;
        while end > SERIAL {
            let start = (end * INPUT).div_ceil(OUTPUT);
            // SAFETY: OUTPUT*start >= INPUT*end, so these two ranges are
            // disjoint. Earlier waves wrote only above this source range;
            // later waves never read it. Each task writes a distinct quad.
            unsafe {
                let src = std::slice::from_raw_parts(raw.cast::<[Fr; 4]>().add(start), end - start);
                let dst = std::slice::from_raw_parts_mut(packed.add(start), end - start);
                dst.par_iter_mut().zip(src).for_each(|(dst, src)| {
                    dst.write(Quad::new(src));
                });
            }
            end = start;
        }
        for j in (0..end).rev() {
            // SAFETY: expansion goes backward; copy the entire source quad
            // before its overlapping destination is written.
            unsafe {
                let src = raw.cast::<[Fr; 4]>().add(j).read();
                packed.add(j).write(MaybeUninit::new(Quad::new(&src)));
            }
        }
        Self(Data::InPlace { owner, offset, len })
    }
    pub(super) fn unpack(self) -> Vec<Fr> {
        match self.0 {
            Data::Separate(values) => scalars(&values, values.len() * 4),
            Data::InPlace {
                mut owner,
                offset,
                len,
            } => {
                let raw = owner.as_mut_ptr();
                // SAFETY: pack established this aligned range of live quads.
                let packed = unsafe { raw.cast::<u8>().add(offset).cast::<Quad>() };
                let mut start = SERIAL.min(len);
                for j in 0..start {
                    // SAFETY: shrinking goes forward. Copy each quad before
                    // writing its Frs; no later packed quad is overwritten.
                    unsafe {
                        let src = packed.add(j).read();
                        raw.cast::<[Fr; 4]>().add(j).write(src.get());
                    }
                }
                while start < len {
                    let end = (start * OUTPUT / INPUT).min(len);
                    // SAFETY: INPUT*end <= OUTPUT*start, so this source and
                    // destination are disjoint (alignment padding only adds
                    // separation). Prior output lies below both ranges.
                    unsafe {
                        let src = std::slice::from_raw_parts(packed.add(start), end - start);
                        let dst = std::slice::from_raw_parts_mut(
                            raw.cast::<MaybeUninit<[Fr; 4]>>().add(start),
                            end - start,
                        );
                        dst.par_iter_mut().zip(src).for_each(|(dst, src)| {
                            dst.write(src.get());
                        });
                    }
                    start = end;
                }
                owner
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_storage_round_trips_sizes_capacities_and_alignment() {
        for n in [0, 1, 3, 4, 8, 1024, 8188, 8192, 8196, 16384, 65536, 262144] {
            for capacity in [n, n + 17, n * 2 + 4] {
                let mut input = Vec::with_capacity(capacity);
                let mut seed = 17u64;
                for _ in 0..n {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    input.push(Fr::from_montgomery([seed, seed.rotate_left(19), 0, 0]));
                }
                let mut expected = input.clone();
                expected.resize(n.next_multiple_of(4), Fr::ZERO);
                let packed = Values::pack(input);
                if n > 0 && n.is_multiple_of(4) {
                    assert!(matches!(packed.0, Data::InPlace { .. }));
                }
                assert_eq!(packed.unpack(), expected, "n={n}, capacity={capacity}");
            }
        }
    }
}
