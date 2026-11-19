//! Component study for packets already accepted by a cross-window scheduler.
use super::*;
use std::{array, sync::Barrier, time::Instant};
use zopro_bn254::{Fr, G2Affine, G2Config, G2Projective};

// Each lane owns a distinct bucket in a different window. The dispatcher must
// handle first writes, zero digits, equal/opposite points and pending conflicts.
struct Packet<'a> {
    indices: [u32; LANES],
    negative: u8,
    point: &'a G2Affine,
}
fn broadcast(value: Fq2) -> [FqBatch; 2] {
    [
        FqBatch::new(&[value.c0; LANES]),
        FqBatch::new(&[value.c1; LANES]),
    ]
}

#[inline(never)]
fn add_packets(buckets: &mut [G2Affine], packets: &[Packet<'_>], scratch: &mut Fq2AddScratch) {
    if packets.is_empty() {
        return;
    }
    let mut chains = FqBatch::one();
    scratch.prefix_records.clear();
    scratch.prefix_records.reserve(packets.len());
    for packet in packets {
        let ax = fq2_pack(packet.indices.map(|i| buckets[i as usize].x));
        let bx = broadcast(packet.point.x);
        let denominator = array::from_fn(|i| canonical_sub::<true>(&bx[i], &ax[i]));
        let norm = prefix_norm(&denominator);
        scratch.prefix_records.push([chains, norm]);
        chains = prefix_mul(&chains, &norm);
    }
    let mut inv = prefix_inverse(&chains, true, scratch.vector_inverse).unwrap_or_else(|| {
        let words = invert_lanes(chains.get(), scratch.vector_inverse);
        prefix_tail_inverse(FqBatch::new(&words), packets.len() * LANES)
    });
    for (packet, record) in packets.iter().zip(&scratch.prefix_records).rev() {
        let a = packet.indices.map(|i| buckets[i as usize]);
        let ax = fq2_pack(a.map(|v| v.x));
        let ay = fq2_pack(a.map(|v| v.y));
        let bx = broadcast(packet.point.x);
        let by = broadcast(packet.point.y);
        let dy = array::from_fn(|i| by[i].signed_difference_canonical(&ay[i], packet.negative));
        let denominator = array::from_fn(|i| canonical_sub::<true>(&bx[i], &ax[i]));
        let factor = prefix_mul(&record[0], &inv);
        inv = prefix_mul(&inv, &record[1]);
        let slope = prefix_numerator(&dy, &denominator, &factor);
        let square = Fq2::lanes_square(&slope);
        let x = array::from_fn(|i| {
            square[i].subtract_uncarried_xsum(&FqBatch::uncarried_xsum(&ax[i], &bx[i]))
        });
        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax, &x)), &ay);
        let (x, y) = (fq2_unpack(x), fq2_unpack(y));
        for (lane, index) in packet.indices.iter().enumerate() {
            buckets[*index as usize] = G2Affine::new(x[lane], y[lane]);
        }
    }
}

fn multiples(first: u64, n: usize) -> Vec<G2Affine> {
    let generator = G2Projective::generator();
    let mut current = generator * Fr::from(first);
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        points.push(current);
        current += generator;
    }
    G2Projective::batch_to_affine(&points)
}
fn fixture(incoming: &[G2Affine], mask: u8) -> Vec<Packet<'_>> {
    let count = incoming.len();
    incoming
        .iter()
        .enumerate()
        .map(|(i, p)| Packet {
            indices: array::from_fn(|lane| (lane * count + i) as u32),
            negative: mask.rotate_left((i % 8) as u32),
            point: p,
        })
        .collect()
}
fn expanded<'a>(packets: &[Packet<'a>]) -> Vec<(u32, &'a G2Affine)> {
    packets
        .iter()
        .flat_map(|p| {
            p.indices
                .iter()
                .enumerate()
                .map(|(lane, &id)| (id | (u32::from((p.negative >> lane) & 1) << 31), p.point))
        })
        .collect()
}
fn scratch() -> Fq2AddScratch {
    Fq2AddScratch {
        canonical: true,
        vector_inverse: true,
        ..Default::default()
    }
}

#[test]
fn broadcast_matches_complete_addition_at_every_sign_mask() {
    for count in [0, 1, 2, 7, 25, 44] {
        let initial = multiples(900_000, count * LANES);
        let incoming = multiples(712, count);
        let plus = G2Projective::batch_to_affine(
            &initial
                .iter()
                .enumerate()
                .map(|(i, p)| p.to_projective().add_affine(&incoming[i % count]))
                .collect::<Vec<_>>(),
        );
        let minus = G2Projective::batch_to_affine(
            &initial
                .iter()
                .enumerate()
                .map(|(i, p)| p.to_projective().add_affine(&(-incoming[i % count])))
                .collect::<Vec<_>>(),
        );
        let mut s = scratch();
        for mask in 0..=255u8 {
            let packets = fixture(&incoming, mask);
            let mut got = initial.clone();
            add_packets(&mut got, &packets, &mut s);
            for p in packets {
                for (lane, &i) in p.indices.iter().enumerate() {
                    assert_eq!(
                        got[i as usize],
                        if p.negative >> lane & 1 != 0 {
                            minus[i as usize]
                        } else {
                            plus[i as usize]
                        }
                    );
                }
            }
        }
    }
}

#[test]
fn full_bucket_caller() {
    let Ok(mode) = std::env::var("ZOPRO_TEST_PACKET_MODE") else {
        return;
    };
    assert!(mode == "original" || mode == "broadcast");
    let read = |k| std::env::var(k).unwrap().parse::<usize>().unwrap();
    let workers = read("ZOPRO_TEST_PACKET_WORKERS");
    let count = read("ZOPRO_TEST_PACKET_COUNT");
    let iterations = read("ZOPRO_TEST_PACKET_ITERATIONS");
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .start_handler(|id| {
            // SAFETY: CPU_ZERO initializes the set and the supported masks are 0..32.
            unsafe {
                let mut set: libc::cpu_set_t = std::mem::zeroed();
                libc::CPU_ZERO(&mut set);
                libc::CPU_SET(id, &mut set);
                assert_eq!(
                    libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set),
                    0
                );
            }
        })
        .build()
        .unwrap();
    let initial = multiples(9_000_000, count * LANES);
    let incoming = multiples(701, count);
    let packets = fixture(&incoming, 0b1011_0100);
    let entries = expanded(&packets);
    let steps = (iterations + 4) as u64;
    let reference = G2Projective::batch_to_affine(
        &(0..initial.len())
            .map(|i| {
                let lane = i / count;
                let k = i % count;
                let delta = steps * (701 + k as u64);
                let weight = 9_000_000 + i as u64;
                G2Projective::generator()
                    * Fr::from(if packets[k].negative >> lane & 1 != 0 {
                        weight - delta
                    } else {
                        weight + delta
                    })
            })
            .collect::<Vec<_>>(),
    );
    let barrier = Barrier::new(workers);
    let samples = pool.broadcast(|ctx| {
        assert_eq!(unsafe { libc::sched_getcpu() }, ctx.index() as i32);
        let mut b = initial.clone();
        let mut s = scratch();
        let mut update = || {
            if mode == "broadcast" {
                add_packets(&mut b, &packets, &mut s)
            } else {
                apply::<G2Config, true>(&mut b, &entries, &mut s);
            }
            std::hint::black_box(&b);
        };
        for _ in 0..4 {
            update();
        }
        barrier.wait();
        let start = Instant::now();
        for _ in 0..iterations {
            update();
        }
        let elapsed = start.elapsed().as_nanos();
        barrier.wait();
        assert_eq!(b, reference);
        elapsed
    });
    let nanos = samples.iter().max().unwrap();
    println!(
        "PACKET_RESULT {mode} {workers} {count} {iterations} {nanos} {}",
        std::mem::size_of::<Packet<'_>>()
    );
}
