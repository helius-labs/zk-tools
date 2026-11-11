use crate::msm::BatchAdd;
use zopro_bn254::{Affine,CurveConfig,Field,Fq,Projective,G1Config,G1Affine};
struct Window<'a, C: CurveConfig<Base = Fq>> {
    half: usize,
    affine: Vec<Affine<C>>,
    projective: Vec<Projective<C>>,
    projective_indices: Vec<u32>,
    projective_buckets: Vec<usize>,
    busy: Vec<u64>,
    batch: Vec<(u32, &'a Affine<C>)>,
    queue: Vec<(u32, &'a Affine<C>)>,
    packed: BatchAdd<C>,
}
impl<'a, C: CurveConfig<Base = Fq>> Window<'a, C> {
    fn new(half: usize, size: usize) -> Self {
        Self {
            half,
            affine: vec![Affine::<C>::IDENTITY; 2*half],
            projective: Vec::new(),
            projective_indices: vec![u32::MAX; half],
            projective_buckets: Vec::new(),
            busy: vec![0; (2*half).div_ceil(64)],
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            packed: BatchAdd::default(),
        }
    }
    #[inline(always)]
    fn busy(&self, bucket: usize) -> bool {
        self.busy[bucket / 64] >> (bucket % 64) & 1 == 1
    }
    fn projective(&mut self, bucket: usize, point: Affine<C>) {
        let bucket=bucket%self.half;
        let index = self.projective_indices[bucket];
        if index == u32::MAX {
            self.projective_indices[bucket] = self.projective.len() as u32;
            self.projective_buckets.push(bucket);
            self.projective.push(point.to_projective());
        } else {
            self.projective[index as usize] += point;
        }
    }
    #[inline(always)]
    fn add(&mut self, encoded: u32, point: &'a Affine<C>) {
        let bucket = (encoded & 0x7fffffff) as usize;
        let negative = encoded >> 31 != 0;
        let current = &mut self.affine[bucket];
        if current.is_identity() {
            *current = if negative { -*point } else { *point };
            return;
        }
        if current.x == point.x {
            let signed = if negative { -*point } else { *point };
            if current.y == signed.y {
                self.projective(bucket, signed);
            } else {
                *current = Affine::<C>::IDENTITY;
            }
            return;
        }
        self.busy[bucket / 64] |= 1 << (bucket % 64);
        self.batch.push((encoded, point));
    }
    fn execute(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        self.packed
            .apply_signed_borrowed(&mut self.affine, &self.batch);
        #[cfg(zopro_dense_busy_reset)]
        if self.busy.len() <= self.batch.len() {
            self.busy.fill(0);
        } else {
        for (index, _) in &self.batch {
            self.busy[(index & 0x7fffffff) as usize / 64] = 0;
        }
        }
        #[cfg(not(zopro_dense_busy_reset))]
        for (index, _) in &self.batch {
            self.busy[(index & 0x7fffffff) as usize / 64] = 0;
        }
        self.batch.clear();
    }
    fn select(&self,b:usize)->Option<usize>{if !self.busy(b){Some(b)}else if !self.busy(b+self.half){Some(b+self.half)}else{None}}
    fn take_from_queue(&mut self) {
        while let Some(&(encoded, point)) = self.queue.last() {
            let Some(physical)=self.select((encoded&0x7fffffff)as usize) else {return};
            self.queue.pop();
            self.add(physical as u32|(encoded&0x80000000), point);
        }
    }
    fn flush_queue(&mut self) {
        for i in 0..self.queue.len() {
            let (encoded, point) = self.queue[i];
            self.projective(
                (encoded & 0x7fffffff) as usize,
                if encoded >> 31 != 0 { -*point } else { *point },
            );
        }
        self.queue.clear();
    }
    fn finish(mut self) -> Vec<Affine<C>> {
        self.execute();
        self.flush_queue();
        let combined: Vec<_> = self
            .projective
            .iter()
            .zip(&self.projective_buckets)
            .map(|(p, &i)| *p + self.affine[i])
            .collect();
        for (p, &i) in Projective::<C>::batch_to_affine(&combined)
            .into_iter()
            .zip(&self.projective_buckets)
        {
            self.affine[i] = p;
        }
        {
            let (left,right)=self.affine.split_at_mut(self.half);
            let mut pairs=Vec::with_capacity(self.half);
            for (i,(a,b)) in left.iter_mut().zip(right.iter()).enumerate(){
                if a.is_identity(){*a=*b;}else if b.is_identity(){}else if a.x==b.x{*a=(*a+*b).to_affine();}else{pairs.push((i as u32,b));}
            }
            self.packed.apply_signed_borrowed(left,&pairs);
        }
        self.affine.truncate(self.half);
        self.affine
    }
}


pub(crate) fn probe(points:&[G1Affine],indices:&[u32],half:usize,size:usize)->(Vec<G1Affine>,[u64;6]){
 assert_eq!(points.len(),indices.len());assert!(half>0&&half<=4096&&size>=2);
 let mut s=Window::<G1Config>::new(half,size);let mut counts=[0u64;6];
 for (p,&encoded)in points.iter().zip(indices){let b=(encoded&0x7fffffff)as usize;assert!(b<half);if p.is_identity(){counts[5]+=1;continue}
  if let Some(i)=s.select(b){counts[0]+=u64::from(i>=half);let a=s.affine[i];if !a.is_identity()&&a.x==p.x{counts[3]+=1;let signed=if encoded>>31!=0{-*p}else{*p};counts[4]+=u64::from(a.y!=signed.y);}s.add(i as u32|(encoded&0x80000000),p);if s.batch.len()==size{s.execute();s.take_from_queue();}}
  else{s.queue.push((encoded,p));counts[1]+=1;if s.queue.len()==size-1{s.flush_queue();counts[2]+=1;}}
 }
 (s.finish(),counts)
}
