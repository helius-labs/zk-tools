//! Native preparation of the quotient query in the evaluation basis.
//! Cached data contains only proving-key constants, never solved values.
use crate::{fft, Fr, G1Affine, G1Projective, Store};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{fs, io::{Read, Write}, path::Path};

const MAGIC: &[u8; 16] = b"zopro-qap-v1\0\0\0\0";

pub(super) struct Queries {
    pub product: Store<G1Affine>,
    pub product_scale: Fr,
    pub basis: Option<[u8;32]>,
    pub fused_product: bool,
    pub euler: bool,
    pub constraints: Vec<G1Affine>,
    pub folded: Option<Folded>,
    reservation: Option<crate::preparation::Reservation>,
}

pub(super) struct Folded {
    pub private: Store<G1Affine>,
    pub slots: Vec<usize>,
    public: Store<G1Affine>,
    public_window: Option<crate::groth16::FixedBase<zopro_bn254::G1Config>>,
    windows: Vec<crate::groth16::FixedBase<zopro_bn254::G1Config>>,
    reservation: Option<crate::preparation::Reservation>,
}
impl Folded {
    #[cfg(zopro_live_preparation_charge)]
    fn retained_bytes(&self) -> Option<usize> {
        use crate::preparation::allocation_bytes;
        let bytes = self.private.owned_bytes()?
            .checked_add(allocation_bytes(&self.slots)?)?
            .checked_add(self.public.owned_bytes()?)?
            .checked_add(allocation_bytes(&self.windows)?)?;
        let bytes = self.windows.iter().try_fold(bytes, |n, w| n.checked_add(w.allocation_bytes()?))?;
        bytes.checked_add(match &self.public_window { Some(w) => w.allocation_bytes()?, None => 0 })
    }

    fn prepare(c: crate::solver::LinearC, k: &[G1Affine], query: &[G1Affine]) -> Self {
        assert_eq!(k.len(), c.n_wires - c.n_public);
        let mut starts = vec![0usize; c.n_values + 1];
        for term in &c.terms {
            starts[term.wire + 1] += 1;
        }
        for i in 0..c.n_values {
            starts[i + 1] += starts[i];
        }
        let mut positions = starts[..c.n_values].to_vec();
        let mut order = vec![0usize; c.terms.len()];
        for (i, term) in c.terms.iter().enumerate() {
            order[positions[term.wire]] = i;
            positions[term.wire] += 1;
        }
        let slots: Vec<_> = (c.n_wires..c.n_values)
            .filter(|&i| starts[i] != starts[i + 1])
            .collect();
        let targets: Vec<_> = (0..c.n_wires).chain(slots.iter().copied()).collect();
        let points: Vec<_> = targets
            .par_iter()
            .map(|&wire| {
                let mut point = if (c.n_public..c.n_wires).contains(&wire) {
                    k[wire - c.n_public].to_projective()
                } else {
                    G1Projective::IDENTITY
                };
                for &i in &order[starts[wire]..starts[wire + 1]] {
                    let term = &c.terms[i];
                    let base = query[term.row];
                    if term.coefficient == Fr::ONE {
                        point -= base;
                    } else if term.coefficient == -Fr::ONE {
                        point += base;
                    } else {
                        point -= base * term.coefficient;
                    }
                }
                point
            })
            .collect();
        let mut private = G1Projective::batch_to_affine(&points);
        let public: Vec<_> = private.drain(..c.n_public).collect();
        let windows = public
            .iter()
            .map(|&point| crate::groth16::FixedBase::new(point))
            .collect();
        log::info!(
            "ZOPRO_TRACE lagrange-fold-size terms={} slots={} private={}",
            c.terms.len(),
            slots.len(),
            private.len()
        );
        Self {
            reservation: None,
            private: Store::Owned(private),
            slots,
            public: Store::Owned(public),
            public_window: None,
            windows,
        }
    }
    fn prepare_public_window(&mut self) {
        // One optional table for the common constant-plus-one-input case.
        // Arbitrary public-input counts retain their mapped representation.
        if self.public.len() != 2 { return; }
        type Window = crate::groth16::FixedBase<zopro_bn254::G1Config>;
        #[cfg(zopro_resource_schedule)]
        let reservation = crate::preparation::Reservation::analysis(Window::preparation_bytes());
        #[cfg(not(zopro_resource_schedule))]
        let reservation = crate::preparation::Reservation::acquire(self.public.len(), Window::preparation_bytes());
        let Some(reservation) = reservation else { return; };
        let window = Window::new(self.public[1]);
        // Construction scratch has dropped before releasing its credit.
        #[cfg(zopro_live_preparation_charge)]
        let reservation = reservation.retain(window.allocation_bytes());
        self.public_window = Some(window);
        self.reservation = Some(reservation);
    }
    pub(super) fn public_term(&self, scalars: &[Fr]) -> G1Projective {
        assert_eq!(self.public.len(), scalars.len());
        if let Some(window) = &self.public_window {
            let first = if scalars[0] == Fr::ONE { self.public[0].to_projective() }
                else { self.public[0] * scalars[0] };
            return first + window.mul(scalars[1]);
        }
        if self.windows.is_empty() {
            return crate::msm_g1(&self.public, scalars);
        }
        assert_eq!(self.windows.len(), self.public.len());
        let mut result = G1Projective::IDENTITY;
        for ((base, window), &scalar) in self.public.iter().zip(&self.windows).zip(scalars) {
            if scalar == Fr::ONE {
                result += *base;
            } else {
                result += window.mul(scalar);
            }
        }
        result
    }
}

fn point_bytes(points: &[G1Affine]) -> &[u8] {
    // SAFETY: this module is little-endian only. G1Affine is repr(C),
    // comprising two repr(transparent) four-u64 fields and a zero-sized tag.
    // It has no padding and every byte of each coordinate is initialized.
    unsafe { std::slice::from_raw_parts(points.as_ptr().cast(), points.len() * 64) }
}

fn field_bytes(values: &[Fr]) -> &[u8] {
    // SAFETY: Fr is repr(transparent) over four initialized u64 limbs.
    unsafe { std::slice::from_raw_parts(values.as_ptr().cast(), values.len() * 32) }
}

impl Queries {
    pub(super) fn fold(&mut self, plan: &crate::solver::Plan, k: &[G1Affine]) {
        let started = crate::time::Instant::now();
        let Some(reservation) = crate::preparation::Reservation::acquire(self.product.len(), plan.preparation_peak()) else { return };
        let mut folded = Folded::prepare(plan.linear_c(), k, &self.constraints);
        // Folded::prepare has returned: its LinearC and temporary point/index
        // vectors have dropped before any process-wide credit is released.
        #[cfg(zopro_live_preparation_charge)]
        let reservation = reservation.retain(folded.retained_bytes());
        folded.reservation = Some(reservation);
        self.folded = Some(folded);
        #[cfg(all(zopro_live_preparation_charge, zopro_folded_query_reclaim))]
        {
            // Folded private/public queries already contain this correction.
            // Generic calls without public values use the original quotient.
            self.constraints = Vec::new();
            if let Some(reservation) = self.reservation.take() {
                self.reservation = Some(reservation.retain(
                    self.product.owned_bytes(),
                ));
            }
        }
        crate::trace("lagrange-fold", started);
    }
    pub(super) fn load_or_prepare(fft: &fft::Plan, bases: &[G1Affine]) -> Option<Self> {
        // Includes both projective transforms, affine outputs, expanded weights,
        // decoding/permutation copies, and batch-normalization scratch.
        let reservation = crate::preparation::Reservation::acquire(fft.size(), fft.size().checked_mul(1024))?;
        let mut queries = Self::load_or_prepare_natural(fft, bases);
        if fft.fused_product_output() {
            let n = queries.product.len();
            queries.product = Store::Owned((0..n)
                .into_par_iter()
                .map(|i| queries.product[fft::product_output_index(n, i)])
                .collect());
            queries.fused_product = true;
        }
        // Weight/transform/decoding scratch and the old permutation input
        // have dropped. Both retained point-vector capacities remain charged.
        #[cfg(zopro_live_preparation_charge)]
        let reservation = reservation.retain(
            queries.product.owned_bytes()
                .and_then(|n| n.checked_add(crate::preparation::allocation_bytes(&queries.constraints)?)),
        );
        queries.reservation = Some(reservation);
        Some(queries)
    }

    fn load_or_prepare_natural(fft: &fft::Plan, bases: &[G1Affine]) -> Self {
        let (roots, weights, c_weight) = fft.quotient_query_weights();
        let mut hash = Sha256::new();
        hash.update(MAGIC);
        hash.update((fft.size() as u64).to_le_bytes());
        hash.update(point_bytes(bases));
        hash.update(field_bytes(&roots));
        hash.update(field_bytes(&weights));
        hash.update(field_bytes(&[c_weight]));
        let fingerprint: [u8; 32] = hash.finalize().into();
        let name: String = fingerprint.iter().map(|b| format!("{b:02x}")).collect();
        let directory = std::env::var_os("ZOPRO_CPU_LAGRANGE_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("zopro-lagrange-v1"));
        let path = directory.join(format!("{name}.bin"));
        if let Some(queries) = Self::load(&path, fingerprint, fft.size()) {
            return queries;
        }
        let started = crate::time::Instant::now();
        let queries = Self::prepare(bases, &roots, &weights, c_weight);
        crate::trace("lagrange-prepare", started);
        if let Err(error) =
            fs::create_dir_all(&directory).and_then(|_| queries.save(&path, fingerprint))
        {
            log::warn!("unable to save prepared quotient queries: {error}");
        }
        queries
    }

    fn prepare(bases: &[G1Affine], roots: &[Fr], weights: &[Fr], c_weight: Fr) -> Self {
        let n = weights.len();
        assert_eq!(bases.len() + 1, n);
        assert_eq!(roots.len(), n / 2);
        let (mut product, mut constraints): (Vec<_>, Vec<_>) = rayon::join(
            || {
                (0..n)
                    .into_par_iter()
                    .map(|i| {
                        bases
                            .get(i)
                            .map_or(G1Projective::IDENTITY, |p| *p * weights[i])
                    })
                    .collect()
            },
            || {
                (0..n)
                    .into_par_iter()
                    .map(|i| {
                        bases
                            .get(i)
                            .map_or(G1Projective::IDENTITY, |p| *p * c_weight)
                    })
                    .collect()
            },
        );
        rayon::join(
            || point_fft(&mut product, roots),
            || point_fft(&mut constraints, roots),
        );
        let (product, constraints) = rayon::join(
            || G1Projective::batch_to_affine(&product),
            || G1Projective::batch_to_affine(&constraints),
        );
        Self {
            reservation: None,
            euler: false, fused_product: false,
            product: Store::Owned(product),
            product_scale: Fr::ONE, basis: None,
            constraints,
            folded: None,
        }
    }

    fn load(path: &Path, fingerprint: [u8; 32], n: usize) -> Option<Self> {
        // Stream directly into the two decoded queries. Never retain a second
        // circuit-sized copy of the cache's byte representation.
        let expected = 88usize.checked_add(n.checked_mul(128)?)?;
        let mut file = fs::File::open(path).ok()?;
        if file.metadata().ok()?.len() != expected as u64 { return None; }
        let mut header = [0u8; 88];
        file.read_exact(&mut header).ok()?;
        if header.get(..16)? != MAGIC
            || header.get(16..48)? != fingerprint
            || u64::from_le_bytes(header.get(80..88)?.try_into().ok()?) != n as u64
        { return None; }
        let mut hash = Sha256::new();
        let mut buffer = vec![0u8; 64 * n.min(4096)];
        let mut read_query = || -> Option<Vec<G1Affine>> {
            let mut points = Vec::new();
            points.try_reserve_exact(n).ok()?;
            while points.len() < n {
                let count = (n - points.len()).min(buffer.len() / 64);
                let bytes = &mut buffer[..count * 64];
                file.read_exact(bytes).ok()?;
                hash.update(&*bytes);
                points.extend(crate::decode_g1(bytes));
            }
            Some(points)
        };
        let product = read_query()?;
        let constraints = read_query()?;
        if file.read(&mut [0]).ok()? != 0 || hash.finalize().as_slice() != &header[48..80] {
            return None;
        }
        Some(Self { reservation: None, euler: false, fused_product: false, folded: None, product: Store::Owned(product), product_scale:Fr::ONE, basis:None, constraints })
    }

    fn save(&self, path: &Path, fingerprint: [u8; 32]) -> std::io::Result<()> {
        debug_assert!(!self.fused_product, "cache stores natural query order");
        let mut hash = Sha256::new();
        hash.update(point_bytes(&self.product));
        hash.update(point_bytes(&self.constraints));
        let payload_hash = hash.finalize();
        let mut random = [0u8; 8];
        getrandom::getrandom(&mut random).expect("system randomness");
        let temporary = path.with_extension(format!(
            "{}.{}.tmp",
            std::process::id(),
            u64::from_le_bytes(random)
        ));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(MAGIC)?;
        file.write_all(&fingerprint)?;
        file.write_all(&payload_hash)?;
        file.write_all(&(self.product.len() as u64).to_le_bytes())?;
        file.write_all(point_bytes(&self.product))?;
        file.write_all(point_bytes(&self.constraints))?;
        drop(file);
        fs::rename(temporary, path)
    }
}

// Input queries are already bit-reversed, just like the quotient output.
// An unnormalized inverse DFT gives the transposed evaluation queries.
fn point_fft(values: &mut [G1Projective], roots: &[Fr]) {
    let n = values.len();
    let mut length = 2;
    while length <= n {
        let started = crate::time::Instant::now();
        let half = length / 2;
        let stride = n / length;
        values.par_chunks_mut(length).for_each(|chunk| {
            let (lo, hi) = chunk.split_at_mut(half);
            lo.par_iter_mut()
                .zip(hi.par_iter_mut())
                .enumerate()
                .for_each(|(j, (lo, hi))| {
                    let product = if j == 0 { *hi } else { *hi * roots[j * stride] };
                    let first = *lo;
                    *lo = first + product;
                    *hi = first - product;
                });
        });
        crate::trace(&format!("lagrange-fft-{length}"), started);
        length *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_queries_stream_across_chunks_and_reject_oversized_files() {
        let n = 4099;
        let queries = Queries {
            product: Store::Owned(vec![G1Affine::GENERATOR; n]), product_scale:Fr::ONE,basis:None,
            constraints: vec![G1Affine::IDENTITY; n],
            euler: false, fused_product: false, folded: None, reservation: None,
        };
        let fingerprint = [37; 32];
        let path = std::env::temp_dir().join(format!("zopro-cache-stream-{}.bin", std::process::id()));
        queries.save(&path, fingerprint).unwrap();
        let loaded = Queries::load(&path, fingerprint, n).unwrap();
        assert_eq!(&loaded.product[..], &queries.product[..]);
        assert_eq!(loaded.constraints, queries.constraints);
        let file = fs::OpenOptions::new().write(true).open(&path).unwrap();
        file.set_len((88 + n * 128 + 1) as u64).unwrap();
        assert!(Queries::load(&path, fingerprint, n).is_none());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn folded_c_queries_match_independent_c_msm() {
        use crate::solver::{LinearC, LinearCTerm};
        let k: Vec<_> = (0..5)
            .map(|i| (G1Affine::GENERATOR * Fr::from(i + 3)).to_affine())
            .collect();
        let q: Vec<_> = (0..4)
            .map(|i| (G1Affine::GENERATOR * Fr::from(i + 17)).to_affine())
            .collect();
        let entries = [
            (0, 2, 3i64),
            (0, 7, -1),
            (1, 0, 1),
            (2, 1, -1),
            (2, 3, 5),
            (3, 7, 1),
            (3, 8, 2),
        ];
        let terms: Vec<_> = entries
            .iter()
            .map(|&(row, wire, c)| LinearCTerm {
                row,
                wire,
                coefficient: if c < 0 {
                    -Fr::from((-c) as u64)
                } else {
                    Fr::from(c as u64)
                },
            })
            .collect();
        let folded = Folded::prepare(
            LinearC {
                n_wires: 7,
                n_public: 2,
                n_values: 9,
                terms,
            },
            &k,
            &q,
        );
        assert_eq!(folded.slots, [7, 8]);
        for seed in [0u64, 1, 7] {
            let mut w: Vec<_> = (0..9).map(|i| Fr::from(i + seed).pow(&[3])).collect();
            w[0] = Fr::ONE;
            let mut c = vec![Fr::ZERO; 4];
            for &(row, wire, coefficient) in &entries {
                let coefficient = if coefficient < 0 {
                    -Fr::from((-coefficient) as u64)
                } else {
                    Fr::from(coefficient as u64)
                };
                c[row] += w[wire] * coefficient;
            }
            let expected = crate::msm_g1(&k, &w[2..7]) - crate::msm_g1(&q, &c);
            let private: Vec<_> = w[2..7]
                .iter()
                .copied()
                .chain(folded.slots.iter().map(|&i| w[i]))
                .collect();
            let actual = crate::msm_g1(&folded.private, &private) + folded.public_term(&w[..2]);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn evaluation_queries_match_coefficient_queries() {
        for n in [8usize, 32, 128] {
            let generator = Fr::root_of_unity(n).unwrap();
            let coset = Fr::from(5u64);
            let plan = fft::Plan::new(n, generator, coset, fft::denominator(n, coset).unwrap());
            let bases: Vec<_> = (0..n - 1)
                .map(|i| (G1Affine::GENERATOR * Fr::from(i as u64 + 3)).to_affine())
                .collect();
            let (roots, weights, c_weight) = plan.quotient_query_weights();
            let queries = Queries::prepare(&bases, &roots, &weights, c_weight);
            for lengths in [[0, 0, 0], [n, n / 2, n - 1], [n - 3, n - 1, n]] {
                let [a, b, c]: [Vec<Fr>; 3] = lengths.map(|len| {
                    (0..len)
                        .map(|i| Fr::from(i as u64 + 11).pow(&[7]))
                        .collect()
                });
                let expected =
                    crate::msm_g1(&bases, &plan.quotient(a.clone(), b.clone(), c.clone()));
                let product = plan.coset_product(a, b);
                let actual = crate::msm_g1(&queries.product, &product)
                    - crate::msm_g1(&queries.constraints[..c.len()], &c);
                assert_eq!(actual, expected, "n={n}, lengths={lengths:?}");
            }
            let fingerprint = [n as u8; 32];
            let path = std::env::temp_dir()
                .join(format!("zopro-query-test-{}-{n}.bin", std::process::id()));
            queries.save(&path, fingerprint).unwrap();
            let loaded = Queries::load(&path, fingerprint, n).unwrap();
            assert_eq!(&loaded.product[..], &queries.product[..]);
            assert_eq!(loaded.constraints, queries.constraints);
            assert!(Queries::load(&path, [255; 32], n).is_none());
            let mut corrupted = fs::read(&path).unwrap();
            *corrupted.last_mut().unwrap() ^= 1;
            fs::write(&path, corrupted).unwrap();
            assert!(Queries::load(&path, fingerprint, n).is_none());
            fs::remove_file(path).unwrap();
        }
    }
}

impl Queries {
    pub(super) fn finish_product(&self, value:G1Projective)->G1Projective {
        if self.product_scale==Fr::ONE {value}else{value*self.product_scale}
    }
    pub(super) fn mapped_basis(image:&crate::Image,product:(usize,usize),private:(usize,usize),public:(usize,usize),slots:Vec<usize>,scale:Fr,digest:[u8;32])->Result<Self,&'static str>{
        let product=Store::mapped(image,product.0,product.1)?;
        let private=Store::mapped(image,private.0,private.1)?;
        let public=Store::<G1Affine>::mapped(image,public.0,public.1)?;
        let mut folded=Folded{private,slots,public,windows:Vec::new(),public_window:None,reservation:None};
        folded.prepare_public_window();
        Ok(Self{product,product_scale:scale,basis:Some(digest),euler: false, fused_product: false,constraints:Vec::new(),folded:Some(folded),reservation:None})
    }
}

#[cfg(test)]
mod replacement_basis_tests {
    use super::*;
    #[test]
    fn independent_parts_survive_folding_and_product_rescaling(){
        let mut cases=0;
        for n in [8usize,32,128] {
            let plan=fft::Plan::new(n,Fr::root_of_unity(n).unwrap(),Fr::from(5u64),fft::denominator(n,Fr::from(5u64)).unwrap());
            let z:Vec<_>=(0..n-1).map(|i|(G1Affine::GENERATOR*Fr::from(i as u64+3).pow(&[5])).to_affine()).collect();
            let k:Vec<_>=(0..5).map(|i|(G1Affine::GENERATOR*Fr::from(i+19)).to_affine()).collect();
            let (roots,weights,c_weight)=plan.quotient_query_weights();
            let original=Queries::prepare(&z,&roots,&weights,c_weight);
            let entries=[(0usize,0usize,1i64),(1,1,-1),(2,2,7),(3,3,-2),(4,4,1),(5,6,-1),(6,7,3),(7,8,-1),(1,2,0)];
            let terms=||entries.iter().filter(|x|x.2!=0).map(|&(row,wire,c)|crate::solver::LinearCTerm{row,wire,coefficient:if c<0{-Fr::from((-c)as u64)}else{Fr::from(c as u64)}}).collect();
            let folded=Folded::prepare(crate::solver::LinearC{n_wires:7,n_public:2,n_values:9,terms:terms()},&k,&original.constraints);
            for scale in [Fr::ONE,Fr::from(7u64),Fr::from(n as u64).pow(&[2])] {
                let scaled:Vec<_>=original.product.iter().map(|&p|(p*scale).to_affine()).collect();
                for seed in [0u64,1,37] {for lengths in [[0,0,0],[n,n/2,n-1],[n-3,n-1,n]] {
                    let [a,b,c]=lengths.map(|len|(0..len).map(|i|Fr::from(i as u64+seed).pow(&[7])).collect::<Vec<_>>());
                    let sk:Vec<_>=(0..5).map(|i|Fr::from(i+seed).pow(&[3])).collect();
                    let mut c_private=vec![Fr::ZERO;n];
                    for &(row,wire,coefficient)in &entries{if (2..7).contains(&wire){let co=if coefficient<0{-Fr::from((-coefficient)as u64)}else{Fr::from(coefficient as u64)};c_private[row]+=sk[wire-2]*co;}}
                    let correction=crate::msm_g1(&scaled,&plan.coset_product(c_private,vec![Fr::ONE;n]))*scale.inverse().unwrap();
                    let got_k=crate::msm_g1(&folded.private[..5],&sk)+correction;
                    assert_eq!(got_k,crate::msm_g1(&k,&sk));
                    let expected_h=crate::msm_g1(&z,&plan.quotient(a.clone(),b.clone(),c.clone()));
                    let mut product=plan.coset_product(a,b);let negative=plan.coset_product(c,vec![Fr::ONE;n]);
                    for (p,c)in product.iter_mut().zip(negative){*p-=c;}
                    let got_h=crate::msm_g1(&scaled,&product)*scale.inverse().unwrap();
                    assert_eq!(got_h,expected_h,"n={n} seed={seed} lengths={lengths:?}");cases+=1;
                }}
            }
            let fingerprints=crate::basis::cache_fingerprints(&z,n,Fr::root_of_unity(n).unwrap(),Fr::from(5u64)).unwrap();
            let mut h=Sha256::new();h.update(MAGIC);h.update((n as u64).to_le_bytes());h.update(point_bytes(&z));h.update(field_bytes(&roots));h.update(field_bytes(&weights));h.update(field_bytes(&[c_weight]));
            let want:[u8;32]=h.finalize().into();assert!(fingerprints.iter().any(|(h,_)|*h==want));
        }
        assert_eq!(cases,81);
    }
}

#[cfg(test)]
mod bounded_public_tests {
    use super::*;
    struct PointImage(Vec<G1Affine>);
    impl AsRef<[u8]> for PointImage {
        fn as_ref(&self) -> &[u8] { point_bytes(&self.0) }
    }
    #[test]
    fn mapped_public_queries_bound_tables_and_preserve_arbitrary_scalars() {
        let mut cases = 0;
        for n in [0usize, 1, 2, 3, 128] {
            let points: Vec<_> = (0..n).map(|i| {
                if i % 11 == 5 { G1Affine::IDENTITY }
                else { (G1Affine::GENERATOR * Fr::from(i as u64 + 3)).to_affine() }
            }).collect();
            let image: crate::Image = std::sync::Arc::new(PointImage(points.clone()));
            let mut queries = Queries::mapped_basis(&image,(0,0),(0,0),(0,n*64),Vec::new(),Fr::ONE,[0;32]).unwrap();
            let folded = queries.folded.as_mut().unwrap();
            assert!(matches!(&folded.public,Store::Mapped{..}));
            assert_eq!(folded.public.owned_bytes(),Some(0));
            assert!(folded.windows.is_empty());
            assert!(n == 2 || folded.public_window.is_none());
            if std::env::var("ZOPRO_CPU_PREPARE_MAX_BYTES").as_deref()==Ok("0") {
                assert!(folded.public_window.is_none());
                assert!(folded.reservation.is_none());
            }
            for fallback in [false,true] {
                if fallback { folded.public_window=None; folded.reservation=None; }
                for seed in [0u64,1,37] {
                    let scalars:Vec<_>=(0..n).map(|i|match seed {
                        0=>Fr::ZERO,1=>Fr::ONE,_=>Fr::from(i as u64+seed).pow(&[7])
                    }).collect();
                    let expected=points.iter().zip(&scalars).fold(G1Projective::IDENTITY,|sum,(&p,&s)|sum+p*s);
                    assert_eq!(folded.public_term(&scalars),expected,"n={n} fallback={fallback} seed={seed}");
                    cases+=1;
                }
            }
        }
        assert_eq!(cases,30);
    }
    #[test]
    fn single_public_window_peak_covers_retained_points() {
        type Window=crate::groth16::FixedBase<zopro_bn254::G1Config>;
        assert_eq!(Window::preparation_bytes(),Some(960*(96+32+64)+3*64));
        let window=Window::new(G1Affine::GENERATOR);
        for scalar in [Fr::ZERO,Fr::ONE,-Fr::ONE,Fr::from(123u64).pow(&[11])] {
            assert_eq!(window.mul(scalar),G1Affine::GENERATOR*scalar);
        }
        #[cfg(zopro_live_preparation_charge)]
        assert_eq!(window.allocation_bytes(),Some(960*64+64));
    }
}

impl Queries {
    pub(super) fn product_values(&self,plan:&fft::Plan,a:Vec<Fr>,b:Vec<Fr>)->Vec<Fr> {
        if self.euler {
            if std::env::var_os("ZOPRO_EULER_TRACE").is_some(){eprintln!("EULER_PRODUCT rows={} domain={}",a.len().max(b.len()),plan.size());}
            plan.euler_product_values(a,b)
        } else {plan.coset_product_ordered(a,b,self.fused_product)}
    }
}
