//! Proof-local borrowed pending operands for ordinary G2 phi windows.
use super::{batch_size, digit};
use crate::msm::BatchAdd;
use zopro_bn254::{Field, Fq, G2Affine, G2Config, G2Projective};
const BETA: Fq = Fq::from_decimal(
    "21888242871839275220042445260109153167277707414472061641714758635765020556616",
);
#[inline]
fn component(s: &[u64; 4], high: bool, w: usize, bits: usize) -> i16 {
    let i = usize::from(high) * 2;
    let d = digit(&[s[i], s[i + 1] & 0x7fffffffffffffff, 0, 0], w, bits);
    if s[i + 1] >> 63 != 0 {
        -d
    } else {
        d
    }
}
struct Window<'a> {
    affine: Vec<G2Affine>,
    projective: Vec<G2Projective>,
    projective_indices: Vec<u32>,
    projective_buckets: Vec<usize>,
    busy: Vec<u64>,
    batch: Vec<(u32, &'a G2Affine)>,
    queue: Vec<(u32, &'a G2Affine)>,
    packed: BatchAdd<G2Config>,
}
impl<'a> Window<'a> {
    fn new(half: usize, size: usize, packed: BatchAdd<G2Config>) -> Self {
        Self {
            affine: vec![G2Affine::IDENTITY; half],
            projective: Vec::new(),
            projective_indices: vec![u32::MAX; half],
            projective_buckets: Vec::new(),
            busy: vec![0; half.div_ceil(64)],
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            packed,
        }
    }
    #[inline(always)]
    fn busy(&self, b: usize) -> bool {
        self.busy[b / 64] >> (b % 64) & 1 == 1
    }
    fn projective(&mut self, b: usize, p: G2Affine) {
        let i = self.projective_indices[b];
        if i == u32::MAX {
            self.projective_indices[b] = self.projective.len() as u32;
            self.projective_buckets.push(b);
            self.projective.push(p.to_projective());
        } else {
            self.projective[i as usize] += p;
        }
    }
    #[inline(always)]
    fn add(&mut self, encoded: u32, p: &'a G2Affine) {
        let b = (encoded & 0x7fffffff) as usize;
        let neg = encoded >> 31 != 0;
        let a = &mut self.affine[b];
        if a.is_identity() {
            *a = if neg { -*p } else { *p };
            return;
        }
        if a.x == p.x {
            let signed = if neg { -*p } else { *p };
            if a.y == signed.y {
                self.projective(b, signed)
            } else {
                *a = G2Affine::IDENTITY
            }
            return;
        }
        self.busy[b / 64] |= 1 << (b % 64);
        self.batch.push((encoded, p));
    }
    fn execute(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        self.packed
            .apply_signed_borrowed_g2(&mut self.affine, &self.batch);
        #[cfg(zopro_dense_busy_reset)]
        if self.busy.len() <= self.batch.len() {
            self.busy.fill(0);
        } else {
        for (i, _) in &self.batch {
            self.busy[(i & 0x7fffffff) as usize / 64] = 0;
        }
        }
        #[cfg(not(zopro_dense_busy_reset))]
        for (i, _) in &self.batch {
            self.busy[(i & 0x7fffffff) as usize / 64] = 0;
        }
        self.batch.clear();
    }
    fn take_from_queue(&mut self) {
        while let Some(&(encoded, p)) = self.queue.last() {
            if self.busy((encoded & 0x7fffffff) as usize) {
                return;
            }
            self.queue.pop();
            self.add(encoded, p);
        }
    }
    fn flush_queue(&mut self) {
        for i in 0..self.queue.len() {
            let (encoded, p) = self.queue[i];
            self.projective(
                (encoded & 0x7fffffff) as usize,
                if encoded >> 31 != 0 { -*p } else { *p },
            );
        }
        self.queue.clear();
    }
    fn feed(
        &mut self,
        bases: &'a [G2Affine],
        short: &[[u64; 4]],
        high: bool,
        w: usize,
        bits: usize,
        size: usize,
    ) {
        for (p, s) in bases.iter().zip(short) {
            let d = component(s, high, w, bits);
            if d == 0 || p.is_identity() {
                continue;
            }
            let b = d.unsigned_abs() as usize - 1;
            let encoded = b as u32 | if d < 0 { 1 << 31 } else { 0 };
            if self.busy(b) {
                self.queue.push((encoded, p));
                if self.queue.len() == size - 1 {
                    self.flush_queue();
                }
                continue;
            }
            self.add(encoded, p);
            if self.batch.len() == size {
                self.execute();
                self.take_from_queue();
            }
        }
        self.execute();
        self.flush_queue();
    }
    fn finish(mut self) -> Vec<G2Affine> {
        let combined: Vec<_> = self
            .projective
            .iter()
            .zip(&self.projective_buckets)
            .map(|(p, &i)| *p + self.affine[i])
            .collect();
        for (p, &i) in G2Projective::batch_to_affine(&combined)
            .into_iter()
            .zip(&self.projective_buckets)
        {
            self.affine[i] = p;
        }
        self.affine
    }
}
pub(super) fn try_window(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    w: usize,
    bits: usize,
) -> Option<Vec<G2Affine>> {
    let packed = BatchAdd::default();
    if !packed.borrowed_g2_enabled() {
        return None;
    }
    let size = batch_size(bits).max(32);
    let mut s = Window::new(1 << (bits - 1), size, packed);
    s.feed(bases, short, true, w, bits, size);
    debug_assert!(s.batch.is_empty() && s.queue.is_empty());
    for p in &mut s.affine {
        if !p.is_identity() {
            p.x = p.x.mul_by_fp(&BETA);
        }
    }
    for p in &mut s.projective {
        p.x = p.x.mul_by_fp(&BETA);
    }
    s.feed(bases, short, false, w, bits, size);
    Some(s.finish())
}
#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::Fr;
    fn random(seed: &mut u64) -> Fr {
        let mut b = [0; 32];
        for c in b.chunks_mut(8) {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            c.copy_from_slice(&seed.to_le_bytes());
        }
        Fr::from_le_bytes_mod_order(&b)
    }
    fn check(n: usize, adversarial: bool) {
        let g = G2Projective::generator();
        let mut seed = 0x6a793d249f1b582c;
        let coefficients: Vec<_> = (0..n)
            .map(|i| {
                if adversarial {
                    match i % 7 {
                        0 => Fr::ZERO,
                        1..=3 => Fr::ONE,
                        4 => -Fr::ONE,
                        _ => Fr::from((i + 7) as u64),
                    }
                } else {
                    Fr::from((i + 1234567) as u64)
                }
            })
            .collect();
        let bases =
            G2Projective::batch_to_affine(&coefficients.iter().map(|s| g * *s).collect::<Vec<_>>());
        let scalars: Vec<_> = (0..n)
            .map(|i| {
                if adversarial {
                    match i % 9 {
                        0 => Fr::ZERO,
                        1 => Fr::ONE,
                        2 => -Fr::ONE,
                        3 => Fr::from(2u64),
                        4 => -Fr::from(2u64),
                        _ => random(&mut seed),
                    }
                } else {
                    random(&mut seed)
                }
            })
            .collect();
        let expected = g * scalars
            .iter()
            .zip(&coefficients)
            .fold(Fr::ZERO, |a, (s, c)| a + *s * *c);
        let mut words: Vec<_> = scalars.iter().map(|s| s.to_canonical()).collect();
        let actual = super::super::g2_phi::raw_exclusive(&bases, &mut words);
        assert_eq!(actual.to_affine(), expected.to_affine());
    }
    #[test]
    fn full_width_phi_msm_matches_generator_sum() {
        for n in [1024, 4096, 8192] {
            check(n, false)
        }
    }
    #[test]
    fn exceptional_keys_and_scalar_edges_match_generator_sum() {
        for n in [1024, 2048] {
            check(n, true)
        }
    }
}

#[cfg(all(test,zopro_dense_busy_reset))]
mod busy_reset_qualification {
    use super::*;
    use zopro_bn254::{Fr, Field};
    type Config = zopro_bn254::G2Config;
    type Point = zopro_bn254::Affine<Config>;
    type Proj = zopro_bn254::Projective<Config>;
    type State<'a> = Window<'a>;
    fn state<'a>(half: usize, size: usize) -> State<'a> { Window::new(half,size,BatchAdd::default()) }
    #[derive(Default, Debug)]
    struct Counts { cases: usize, checks: usize, dense: usize, sparse: usize, empty: usize, queued: usize, flushes: usize, equal: usize, opposite: usize, identities: usize }
    impl Counts {
        fn add(&mut self, b: Self) { self.cases+=b.cases;self.checks+=b.checks;self.dense+=b.dense;self.sparse+=b.sparse;self.empty+=b.empty;self.queued+=b.queued;self.flushes+=b.flushes;self.equal+=b.equal;self.opposite+=b.opposite;self.identities+=b.identities; }
    }
    fn check(s: &State<'_>, c: &mut Counts) {
        let mut want = vec![0u64; s.busy.len()];
        for &(encoded, _) in &s.batch {
            let b = (encoded & 0x7fffffff) as usize;
            assert_eq!(want[b/64] >> (b%64) & 1, 0);
            want[b/64] |= 1 << (b%64);
        }
        assert_eq!(s.busy, want); c.checks += 1;
    }
    fn execute(s: &mut State<'_>, c: &mut Counts) {
        check(s,c);
        let ptr = s.busy.as_ptr(); let cap = s.busy.capacity(); let len = s.busy.len();
        if s.batch.is_empty() { c.empty+=1; }
        else if s.busy.len() <= s.batch.len() { c.dense+=1; }
        else { c.sparse+=1; }
        s.execute();
        assert!(s.busy.iter().all(|v| *v==0)); assert!(s.batch.is_empty());
        assert_eq!(s.busy.as_ptr(),ptr); assert_eq!(s.busy.capacity(),cap); assert_eq!(s.busy.len(),len);
        check(s,c);
    }
    fn points() -> (Vec<Point>,Vec<Fr>) {
        let g=Proj::generator(); let mut w=Fr::from(0x123456789abcdefu64);
        let mut weights=Vec::new();
        for i in 0..64 {w=w.double()+Fr::from(0x9e3779b97f4a7c15u64);weights.push(match i%16 {0=>Fr::ZERO,1|2=>Fr::ONE,3=>-Fr::ONE,_=>w});}
        let points=Proj::batch_to_affine(&weights.iter().map(|w|g**w).collect::<Vec<_>>());
        (points,weights)
    }
    fn case(half:usize,size:usize,points:&[Point],weights:&[Fr]) -> Counts {
        let mut c=Counts::default(); let mut s=state(half,size);
        let n=half.min(size+7); let mut want=vec![Fr::ZERO;half];
        execute(&mut s,&mut c);
        for i in 0..(4*n+19) {
            let j=i%points.len();let p=&points[j];
            if p.is_identity() {c.identities+=1;continue;}
            let b=if i%9==0 {0} else {(i*37+i/7)%n};
            let neg=i%3==0;let encoded=b as u32|if neg{1<<31}else{0};
            want[b]+=if neg{-weights[j]}else{weights[j]};
            if s.busy(b) {
                s.queue.push((encoded,p));c.queued+=1;
                if s.queue.len()==size-1 {s.flush_queue();c.flushes+=1;}
            } else {
                if !s.affine[b].is_identity() && s.affine[b].x==p.x {
                    if s.affine[b].y==if neg{-p.y}else{p.y}{c.equal+=1;}else{c.opposite+=1;}
                }
                s.add(encoded,p);
                if s.batch.len()==size {execute(&mut s,&mut c);s.take_from_queue();}
            }
            check(&s,&mut c);
        }
        execute(&mut s,&mut c);
        s.take_from_queue();check(&s,&mut c);
        execute(&mut s,&mut c);
        s.flush_queue();
        assert!(s.batch.is_empty() && s.queue.is_empty());
        let actual=s.finish();let g=Proj::generator();
        for (i,(&w,p)) in want.iter().zip(&actual).enumerate() {assert_eq!(*p,(g*w).to_affine(),"bucket {i}/{half},capacity {size}");}
        c.cases+=1;c
    }
    #[test]
    fn exact_pending_set_complete_windows_and_workers() {
        let (p,w)=points();let original=p.clone();let mut c=Counts::default();
        for half in [1,63,64,65,127,128,2048,4096,16384] {for size in [8,32,200,352,512] {c.add(case(half,size,&p,&w));}}
        assert_eq!(c.cases,45);assert!(c.dense>0&&c.sparse>0&&c.empty>0&&c.queued>0&&c.flushes>0);
        let mut workers=0;
        for n in [1,4,16,32] {
            let pool=rayon::ThreadPoolBuilder::new().num_threads(n).stack_size(512*1024).build().unwrap();
            let outputs=pool.broadcast(|_|case(65,32,&p,&w));assert_eq!(outputs.len(),n);
            for o in outputs {c.add(o);}workers+=n;
        }
        let pool=rayon::ThreadPoolBuilder::new().num_threads(8).stack_size(512*1024).build().unwrap();
        let outputs=std::thread::scope(|scope|{
            let mut handles=Vec::new();for _ in 0..4 {let pool=&pool;let p=&p;let w=&w;handles.push(scope.spawn(move||{let mut out=Counts::default();for _ in 0..3 {out.add(pool.install(||case(65,32,p,w)));}out}));}
            handles.into_iter().map(|h|h.join().unwrap()).collect::<Vec<_>>()
        });
        for o in outputs {c.add(o);}
        assert_eq!(workers,53);assert_eq!(c.cases,110);assert_eq!(p,original);
        println!("BUSY_RESET G2 WORKERS={workers} CONCURRENT_CALLS=12 COUNTS={c:?}");
    }
}
