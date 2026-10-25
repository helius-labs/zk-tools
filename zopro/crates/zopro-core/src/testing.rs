//! A seeded source of field elements for the tests (splitmix64).
use zopro_bn254::Fr;

pub(crate) struct Seeded(u64);

impl Seeded {
    pub(crate) fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn bytes(&mut self) -> [u8; 64] {
        let mut out = [0u8; 64];
        for chunk in out.chunks_mut(8) {
            chunk.copy_from_slice(&self.next_u64().to_le_bytes());
        }
        out
    }

    pub(crate) fn fr(&mut self) -> Fr {
        Fr::from_random_bytes(&self.bytes())
    }
}
#[test]
fn zeroed_field_storage_covers_parallel_boundaries_and_spare_capacity() {
    for len in [0, 1, 16383, 16384, 16385, 65537] {
        for capacity in [len, len + 17] {
            let mut values = crate::zero_frs(len, capacity);
            assert_eq!(values.len(), len);
            assert!(values.capacity() >= capacity);
            assert!(values.iter().all(|value| *value == zopro_bn254::Fr::ZERO));
            values.resize(len + 9, zopro_bn254::Fr::ONE);
            assert!(values[len..]
                .iter()
                .all(|value| *value == zopro_bn254::Fr::ONE));
        }
    }
}
