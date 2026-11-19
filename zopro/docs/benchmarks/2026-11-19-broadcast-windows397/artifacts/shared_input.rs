use super::*;
use crate::msm::broadcast_update::Packet;
use std::{array, time::Instant};
use zopro_bn254::Fr;

fn flush_packets(s: &mut Window<'_>, pending: &mut Vec<Packet<'_>>) {
    if pending.is_empty() {
        return;
    }
    assert!(s.batch.is_empty() && s.queue.is_empty());
    s.packed.update_packets(&mut s.affine, pending);
    for packet in pending.drain(..) {
        for i in packet.indices {
            let i = i as usize;
            s.busy[i / 64] &= !(1 << (i % 64));
        }
    }
}
fn fallback<'a>(
    state: &mut Window<'a>,
    d: i16,
    lane: usize,
    half: usize,
    p: &'a G2Affine,
    size: usize,
) {
    if d == 0 {
        return;
    }
    let b = lane * half + d.unsigned_abs() as usize - 1;
    let encoded = b as u32 | (u32::from(d < 0) << 31);
    if state.busy(b) {
        state.queue.push((encoded, p));
        if state.queue.len() == size - 1 {
            state.flush_queue();
        }
    } else {
        state.add(encoded, p);
        if state.batch.len() == size {
            state.execute();
            state.take_from_queue();
        }
    }
}

fn group(
    bases: &[G2Affine],
    words: &[[u64; 4]],
    bits: usize,
    capacity: usize,
    broadcast: bool,
) -> (Vec<G2Affine>, usize) {
    let half = 1 << (bits - 1);
    let size = batch_size(bits).max(32);
    let mut s = Window::new(half * 8, size, BatchAdd::default());
    assert!(s.packed.borrowed_g2_enabled());
    let mut pending = Vec::<Packet>::with_capacity(capacity);
    let mut packets = 0;
    for high in [true, false] {
        for (p, scalar) in bases.iter().zip(words) {
            if p.is_identity() {
                continue;
            }
            let digits: [i16; 8] = array::from_fn(|i| component(scalar, high, i, bits));
            let nonzero = digits.iter().all(|&d| d != 0);
            if nonzero {
                let ids: [u32; 8] =
                    array::from_fn(|i| (i * half + digits[i].unsigned_abs() as usize - 1) as u32);
                if !pending.is_empty() && ids.iter().any(|&i| s.busy(i as usize)) {
                    flush_packets(&mut s, &mut pending);
                }
                let eligible = ids.iter().all(|&i| {
                    let b = i as usize;
                    !s.busy(b) && !s.affine[b].is_identity() && s.affine[b].x != p.x
                });
                if broadcast && eligible {
                    if pending.is_empty() {
                        s.execute();
                        s.flush_queue();
                    }
                    let mut negative = 0u8;
                    for (lane, &index) in ids.iter().enumerate() {
                        let i = index as usize;
                        s.busy[i / 64] |= 1 << (i % 64);
                        negative |= u8::from(digits[lane] < 0) << lane;
                    }
                    pending.push(Packet {
                        indices: ids,
                        negative,
                        point: p,
                    });
                    packets += 1;
                    if pending.len() == capacity {
                        flush_packets(&mut s, &mut pending);
                    }
                    continue;
                }
            }
            flush_packets(&mut s, &mut pending);
            for (lane, &d) in digits.iter().enumerate() {
                fallback(&mut s, d, lane, half, p, size);
            }
        }
        flush_packets(&mut s, &mut pending);
        s.execute();
        s.flush_queue();
        if high {
            for b in &mut s.affine {
                if !b.is_identity() {
                    b.x = b.x.mul_by_fp(&BETA);
                }
            }
            for b in &mut s.projective {
                b.x = b.x.mul_by_fp(&BETA);
            }
        }
    }
    (s.finish(), packets)
}

fn reference(bases: &[G2Affine], words: &[[u64; 4]], bits: usize) -> Vec<G2Affine> {
    use rayon::prelude::*;
    (0..8)
        .into_par_iter()
        .map(|w| try_window(bases, words, w, bits).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect()
}

#[test]
fn signed_windows_match_complete_projective_sums() {
    let bits = 6;
    let half = 1 << (bits - 1);
    let mut z = 1234567u64;
    let mut rand = || {
        z ^= z << 13;
        z ^= z >> 7;
        z ^= z << 17;
        z
    };
    let mut points = Vec::new();
    let mut words = Vec::new();
    let generator = G2Projective::generator();
    let choices = G2Projective::batch_to_affine(
        &(1..21)
            .map(|n| generator * Fr::from(n as u64))
            .collect::<Vec<_>>(),
    );
    for i in 0..600 {
        let p = choices[i % choices.len()];
        points.push(match i % 9 {
            0 => G2Affine::IDENTITY,
            1..=4 => -p,
            _ => p,
        });
        words.push(if i % 6 == 0 {
            [0; 4]
        } else {
            array::from_fn(|_| rand())
        });
    }
    for n in [0, 1, 31, 600] {
        let mut expected = vec![G2Projective::IDENTITY; 8 * half];
        for high in [true, false] {
            for (p, word) in points[..n].iter().zip(&words[..n]) {
                for w in 0..8 {
                    let d = component(word, high, w, bits);
                    if d != 0 {
                        expected[w * half + d.unsigned_abs() as usize - 1] +=
                            if d < 0 { -*p } else { *p };
                    }
                }
            }
            if high {
                for v in &mut expected {
                    v.x = v.x.mul_by_fp(&BETA);
                }
            }
        }
        let expected = G2Projective::batch_to_affine(&expected);
        for enabled in [false, true] {
            assert_eq!(
                group(&points[..n], &words[..n], bits, 8, enabled).0,
                expected
            );
        }
        assert_eq!(reference(&points[..n], &words[..n], bits), expected);
    }
}

#[test]
fn primary_window_panel() {
    use rayon::prelude::*;
    let Ok(mode) = std::env::var("ZOPRO_TEST_WINDOW_PANEL") else {
        return;
    };
    assert!(matches!(
        mode.as_str(),
        "reference" | "grouped" | "broadcast"
    ));
    let read = |key| std::env::var(key).unwrap().parse::<usize>().unwrap();
    let workers = read("ZOPRO_TEST_WINDOW_WORKERS");
    let parts = read("ZOPRO_TEST_WINDOW_PARTS");
    let bits = if parts == 2 { 12 } else { 11 };
    let data = std::fs::read(std::env::var_os("ZOPRO_TEST_WINDOW_FIXTURE").unwrap()).unwrap();
    assert_eq!(&data[..8], b"B2SCAN01");
    let n = u64::from_le_bytes(data[8..16].try_into().unwrap()) as usize;
    assert_eq!(16 + n * 160, data.len());
    let points: Vec<_> = data[16..16 + n * 128]
        .chunks_exact(128)
        .map(|b| {
            let q = |k: usize| {
                Fq::from_canonical(array::from_fn(|i| {
                    u64::from_be_bytes(
                        b[k * 32 + (3 - i) * 8..k * 32 + (4 - i) * 8]
                            .try_into()
                            .unwrap(),
                    )
                }))
                .unwrap()
            };
            G2Affine::new(
                zopro_bn254::Fq2::new(q(1), q(0)),
                zopro_bn254::Fq2::new(q(3), q(2)),
            )
        })
        .collect();
    assert!(points.iter().all(|p| p.is_on_curve()));
    let words: Vec<_> = data[16 + n * 128..]
        .chunks_exact(32)
        .map(|b| {
            super::super::g2_phi::phi_split(array::from_fn(|i| {
                u64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap())
            }))
        })
        .collect();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .start_handler(|id| unsafe {
            let mut set: libc::cpu_set_t = std::mem::zeroed();
            libc::CPU_ZERO(&mut set);
            libc::CPU_SET(id, &mut set);
            assert_eq!(
                libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set),
                0
            );
        })
        .build()
        .unwrap();
    let run = |m: &str| {
        pool.install(|| {
            points
                .par_chunks(n.div_ceil(parts))
                .zip(words.par_chunks(n.div_ceil(parts)))
                .map(|(p, s)| {
                    if m == "reference" {
                        (reference(p, s, bits), 0)
                    } else {
                        group(p, s, bits, 16, m == "broadcast")
                    }
                })
                .collect::<Vec<_>>()
        })
    };
    let expected = run("reference");
    let warm = run(&mode);
    for (a, b) in warm.iter().zip(&expected) {
        assert_eq!(a.0, b.0);
    }
    for _ in 0..7 {
        let now = Instant::now();
        let got = run(&mode);
        let ns = now.elapsed().as_nanos();
        assert_eq!(got.len(), expected.len());
        let mut packets = 0;
        for (a, b) in got.iter().zip(&expected) {
            assert_eq!(a.0, b.0);
            packets += a.1;
        }
        println!("WINDOW_PANEL {mode} {workers} {parts} {ns} {packets}");
    }
}
