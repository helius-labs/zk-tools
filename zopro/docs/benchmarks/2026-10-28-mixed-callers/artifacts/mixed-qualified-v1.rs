//! Mixed signed G1 flush diagnostic. Production source is unchanged.
use super::*;
use std::{borrow::Borrow, hint::black_box};
use zopro_bn254::{G1Affine,G1Config,G1Projective};
type Entry<P>=(u32,P);
#[derive(Clone,Copy)]
struct TripleRecord {prefix:FqBatch,denominator:FqBatch,first:FqBatch,second:FqBatch,ax:FqBatch,ay:FqBatch,xsum:FqBatch,rx:FqBatch}
#[derive(Default)]
struct MixedScratch {
    slots:Vec<[u32;3]>,selected:Vec<u8>,ordinary:Vec<usize>,triples:Vec<[usize;2]>,
    ordinary_records:Vec<FqAddGroup>,triple_records:Vec<TripleRecord>,
}
fn key(encoded:u32)->u32 {encoded&0x7fffffff}
fn signed<P:Borrow<G1Affine>>(e:&Entry<P>)->G1Affine {let p=*e.1.borrow();if e.0>>31!=0 {-p}else{p}}
impl MixedScratch {
    fn classify<P:Copy>(&mut self,q:&[Entry<P>],queue:&[Entry<P>]) {
        self.ordinary.clear();self.triples.clear();
        self.selected.clear();self.selected.resize(queue.len(),0);
        if q.is_empty() {return;}
        let size=(2*q.len()).next_power_of_two();self.slots.resize(size,[0;3]);self.slots.fill([0;3]);
        let mask=size-1;
        for (i,e) in q.iter().enumerate() {
            let id=key(e.0)+1;let mut slot=(id.wrapping_mul(0x9e3779b9)as usize)&mask;
            while self.slots[slot][0]!=0 {assert_ne!(self.slots[slot][0],id,"distinct pending destinations");slot=(slot+1)&mask;}
            self.slots[slot]=[id,i as u32+1,0];
        }
        for (j,e) in queue.iter().enumerate().rev() {
            let id=key(e.0)+1;let mut slot=(id.wrapping_mul(0x9e3779b9)as usize)&mask;
            while self.slots[slot][0]!=0 && self.slots[slot][0]!=id {slot=(slot+1)&mask;}
            if self.slots[slot][0]==id && self.slots[slot][2]==0 {self.slots[slot][2]=j as u32+1;self.selected[j]=1;}
        }
        // Restore pending order without sorting or a per-bucket array.
        self.ordinary.resize(q.len(),usize::MAX);
        for slot in &self.slots {if slot[0]!=0 {self.ordinary[slot[1]as usize-1]=slot[2]as usize;}}
        for i in 0..q.len() {let j=self.ordinary[i];if j>0 {self.triples.push([i,j-1]);}}
        let mut count=0;for i in 0..q.len() {if self.ordinary[i]==0 {self.ordinary[count]=i;count+=1;}}
        self.ordinary.truncate(count);
    }
    fn compact<P:Copy>(&self,queue:&mut[Entry<P>])->usize {
        let mut out=0;for i in 0..queue.len() {if self.selected[i]==0 {queue[out]=queue[i];out+=1;}}out
    }
    fn bytes(&self)->usize {
        self.slots.capacity()*std::mem::size_of::<[u32;3]>()+self.selected.capacity()+self.ordinary.capacity()*std::mem::size_of::<usize>()
            +self.triples.capacity()*std::mem::size_of::<[usize;2]>()+self.ordinary_records.capacity()*std::mem::size_of::<FqAddGroup>()
            +self.triple_records.capacity()*std::mem::size_of::<TripleRecord>()
    }
}
fn complete<P:Copy+Borrow<G1Affine>>(b:&mut[G1Affine],q:&[Entry<P>],queue:&[Entry<P>],s:&MixedScratch) {
    for e in q {let i=key(e.0)as usize;b[i]=(b[i]+signed(e)).to_affine();}
    for &[_,j] in &s.triples {let e=&queue[j];let i=key(e.0)as usize;b[i]=(b[i]+signed(e)).to_affine();}
}
fn mixed<P:Copy+Borrow<G1Affine>>(b:&mut[G1Affine],q:&[Entry<P>],queue:&[Entry<P>],s:&mut MixedScratch) {
    if q.is_empty() {return;}
    if q.iter().any(|e| b[key(e.0)as usize].is_identity() || e.1.borrow().is_identity() || b[key(e.0)as usize].x==e.1.borrow().x)
        || s.triples.iter().any(|r|queue[r[1]].1.borrow().is_identity()) {complete(b,q,queue,s);return;}
    s.ordinary_records.clear();s.triple_records.clear();
    s.ordinary_records.reserve(s.ordinary.len().div_ceil(LANES));s.triple_records.reserve(s.triples.len().div_ceil(LANES));
    let mut chains=FqBatch::one();
    for row in s.ordinary.chunks(LANES) {
        let a:[G1Affine;LANES]=core::array::from_fn(|i|if i<row.len(){b[key(q[row[i]].0)as usize]}else{G1Affine::IDENTITY});
        let c:[G1Affine;LANES]=core::array::from_fn(|i|if i<row.len(){*q[row[i]].1.borrow()}else{G1Affine::new(Fq::ONE,Fq::ZERO)});
        let negative=(0..row.len()).fold(0u8,|mask,i|mask|(((q[row[i]].0>>31)as u8)<<i));
        // Same field spelling and write order as stream_g1_impl's ordinary records.
        unsafe {
            let slot=s.ordinary_records.as_mut_ptr().add(s.ordinary_records.len());
            let ax=FqBatch::new(&a.map(|p|p.x));core::ptr::addr_of_mut!((*slot).ax).write(ax);
            let bx=FqBatch::new(&c.map(|p|p.x));let denominator=canonical_sub::<true>(&bx,&ax);
            core::ptr::addr_of_mut!((*slot).xsum).write(canonical_add::<true>(&ax,&bx));
            core::ptr::addr_of_mut!((*slot).denominator).write(denominator);
            let ay=FqBatch::new(&a.map(|p|p.y));core::ptr::addr_of_mut!((*slot).ay).write(ay);
            let by=FqBatch::new(&c.map(|p|p.y));let numerator=scaled_signed_dy::<true,true>(&by,&ay,&chains,negative);
            core::ptr::addr_of_mut!((*slot).numerator).write(numerator);chains=chains.mul(&denominator);
            s.ordinary_records.set_len(s.ordinary_records.len()+1);
        }
    }
    for row in s.triples.chunks(LANES) {
        let a:[G1Affine;LANES]=core::array::from_fn(|i|if i<row.len(){b[key(q[row[i][0]].0)as usize]}else{G1Affine::IDENTITY});
        let c:[G1Affine;LANES]=core::array::from_fn(|i|if i<row.len(){signed(&q[row[i][0]])}else{G1Affine::new(Fq::ONE,Fq::ZERO)});
        let r:[G1Affine;LANES]=core::array::from_fn(|i|if i<row.len(){signed(&queue[row[i][1]])}else{G1Affine::new(Fq::ONE,Fq::ZERO)});
        let ax=FqBatch::new(&a.map(|p|p.x));let ay=FqBatch::new(&a.map(|p|p.y));
        let bx=FqBatch::new(&c.map(|p|p.x));let by=FqBatch::new(&c.map(|p|p.y));
        let rx=FqBatch::new(&r.map(|p|p.x));let ry=FqBatch::new(&r.map(|p|p.y));
        let d=canonical_sub::<true>(&bx,&ax);let n=canonical_sub::<true>(&by,&ay);let d2=d.mul(&d);
        let xsum=canonical_add::<true>(&ax,&bx);let h=d2.difference2(&xsum.add(&rx),&n,&n);
        let u=d.sum2(&ay.add(&ry),&n,&rx.sub(&ax));let denominator=d.mul(&h);
        // No output has been overwritten. Zero includes either denominator exception.
        if denominator.get().iter().any(Field::is_zero) {complete(b,q,queue,s);return;}
        let first=n.mul(&h);let second=d2.mul(&u);
        s.triple_records.push(TripleRecord{prefix:chains,denominator,first,second,ax,ay,xsum,rx});chains=chains.mul(&denominator);
    }
    let mut inverse=chains.inverse_nonzero();
    for (j,g) in s.triple_records.iter().enumerate().rev() {
        let factor=g.prefix.mul(&inverse);inverse=inverse.mul(&g.denominator);
        let l=g.first.mul(&factor);let w=g.second.mul(&factor);let k=w.sub(&l);
        let xs=l.mul(&l).sub(&g.xsum);let x=k.mul(&k).sub(&xs).sub(&g.rx);
        let y=w.sum2(&xs.sub(&x),&l,&x.sub(&g.ax)).add(&g.ay);let(x,y)=(x.get(),y.get());
        for i in 0..LANES.min(s.triples.len()-j*LANES) {b[key(q[s.triples[j*LANES+i][0]].0)as usize]=G1Affine::new(x[i],y[i]);}
    }
    for (j,g) in s.ordinary_records.iter().enumerate().rev() {
        let slope=g.numerator.mul(&inverse);inverse=inverse.mul(&g.denominator);
        let x=slope.mul(&slope).sub(&g.xsum);let y=slope.mul(&g.ax.sub(&x)).sub(&g.ay);let(x,y)=(x.get(),y.get());
        for i in 0..LANES.min(s.ordinary.len()-j*LANES) {b[key(q[s.ordinary[j*LANES+i]].0)as usize]=G1Affine::new(x[i],y[i]);}
    }
}
fn baseline<P:Copy+Borrow<G1Affine>>(b:&mut[G1Affine],q:&[Entry<P>],r:&[Entry<P>],s:&mut BatchAdd<G1Config>) {
    if q.is_empty(){return;}
    if q.iter().any(|e|b[key(e.0)as usize].is_identity()||e.1.borrow().is_identity()||b[key(e.0)as usize].x==e.1.borrow().x) {
        for e in q.iter().chain(r) {let i=key(e.0)as usize;b[i]=(b[i]+signed(e)).to_affine();}return;
    }
    stream_g1_impl::<G1Config,true,true>(b,q,&mut s.fq);
    if r.is_empty(){return;}
    if r.iter().any(|e|b[key(e.0)as usize].is_identity()||e.1.borrow().is_identity()||b[key(e.0)as usize].x==e.1.borrow().x) {
        for e in r {let i=key(e.0)as usize;b[i]=(b[i]+signed(e)).to_affine();}return;
    }
    stream_g1_impl::<G1Config,true,true>(b,r,&mut s.fq);
}

pub struct MixedCaller {
    source:Vec<G1Affine>,buckets:Vec<G1Affine>,initial:Vec<Fr>,pending:Vec<Entry<G1Affine>>,pending_values:Vec<Fr>,queue:Vec<Entry<G1Affine>>,queue_values:Vec<Fr>,scratch:MixedScratch,binary:BatchAdd<G1Config>,
}
impl MixedCaller {
    pub fn new(n:usize,m:usize)->Self {
        let words:Vec<_>=(0..n).map(|i|[Fr::from((1u64<<48)*(i as u64+1)+1800000054).to_canonical(),Fr::from(35177+i as u64*101).to_canonical(),Fr::from(83177+i as u64*137).to_canonical()]).collect();Self::from_words(&words,m)
    }
    pub fn from_words(words:&[[[u64;4];3]],m:usize)->Self {
        let n=words.len();assert!(m<=n && n<=512);let size=(n*2).max(1).next_power_of_two();let g=G1Projective::generator();
        let mut initial=vec![Fr::ONE;size];let mut pending=Vec::new();let mut pending_values=Vec::new();
        for (i,w) in words.iter().enumerate() {let slot=(i*37+17)%size;initial[slot]=Fr::from_canonical(w[0]).unwrap();let value=Fr::from_canonical(w[1]).unwrap();let flag=if i%3==0 {1<<31}else{0};pending.push((slot as u32|flag,(g*value).to_affine()));pending_values.push(if flag==0 {value}else{-value});}
        let mut queue=Vec::new();let mut queue_values=Vec::new();
        let mut push=|slot:u32,flag:u32,value:Fr| {queue.push((slot|flag,(g*value).to_affine()));queue_values.push(if flag==0 {value}else{-value});};
        for i in 0..m {let p=(i*37+19)%n;let slot=key(pending[p].0);let flag=if i%2==0 {1<<31}else{0};push(slot,flag,Fr::from_canonical(words[p][2]).unwrap());
            let free=((n+i)*37+17)%size;push(free as u32,0,Fr::from(900001+i as u64));
            if i%4==0 {push(slot,flag,Fr::from(910003+i as u64));}
        }
        let source=G1Projective::batch_to_affine(&initial.iter().map(|v|g* *v).collect::<Vec<_>>());
        Self {buckets:source.clone(),source,initial,pending,pending_values,queue,queue_values,scratch:MixedScratch::default(),binary:BatchAdd::default()}
    }
    pub fn inputs(&self)->(Vec<[u64;4]>,Vec<(u32,[u64;4])>,Vec<(u32,[u64;4])>) {
        (self.initial.iter().map(Fr::to_canonical).collect(),self.pending.iter().zip(&self.pending_values).map(|(e,v)|(key(e.0),v.to_canonical())).collect(),self.queue.iter().zip(&self.queue_values).map(|(e,v)|(key(e.0),v.to_canonical())).collect())
    }
    pub fn with_borrowed<T>(&mut self,f:impl FnOnce(&mut Borrowed<'_>)->T)->T {
        let pending:Vec<_>=self.pending.iter().map(|(i,p)|(*i,p)).collect();let template:Vec<_>=self.queue.iter().map(|(i,p)|(*i,p)).collect();let queue=template.clone();
        let mut selected=vec![false;self.queue.len()];let mut seen=vec![false;self.buckets.len()];
        for j in (0..self.queue.len()).rev() {let slot=key(self.queue[j].0)as usize;if !seen[slot]&&self.pending.iter().any(|e|key(e.0)as usize==slot){seen[slot]=true;selected[j]=true;}}
        let mut delta=vec![Fr::ZERO;self.buckets.len()];for (e,v) in self.pending.iter().zip(&self.pending_values){delta[key(e.0)as usize]+=*v;}for (j,(e,v)) in self.queue.iter().zip(&self.queue_values).enumerate(){if selected[j]{delta[key(e.0)as usize]+=*v;}}
        let expected_queue:Vec<_>=self.queue.iter().enumerate().filter(|(j,_)|!selected[*j]).map(|(_,e)|*e).collect();
        let mut view=Borrowed{source:&self.source,buckets:&mut self.buckets,pending,template,queue,queue_len:0,control_r:Vec::with_capacity(self.pending.len()),scratch:&mut self.scratch,binary:&mut self.binary,initial:&self.initial,delta,expected_queue};
        f(&mut view)
    }
}
pub struct Borrowed<'a> {
    source:&'a[G1Affine],buckets:&'a mut[G1Affine],pending:Vec<Entry<&'a G1Affine>>,template:Vec<Entry<&'a G1Affine>>,queue:Vec<Entry<&'a G1Affine>>,queue_len:usize,control_r:Vec<Entry<&'a G1Affine>>,scratch:&'a mut MixedScratch,binary:&'a mut BatchAdd<G1Config>,initial:&'a[Fr],delta:Vec<Fr>,expected_queue:Vec<Entry<G1Affine>>,
}
impl Borrowed<'_> {
    pub fn reset(&mut self){self.buckets.copy_from_slice(self.source);}
    pub fn run(&mut self,selected:bool,n:usize) {
        if selected {for _ in 0..n {let s=black_box(&mut *self);s.queue.copy_from_slice(&s.template);s.scratch.classify(&s.pending,&s.queue);
            if s.scratch.triples.is_empty(){baseline(s.buckets,&s.pending,&[],s.binary);}else{mixed(s.buckets,&s.pending,&s.queue,s.scratch);}
            s.queue_len=s.scratch.compact(&mut s.queue);
        }} else {for _ in 0..n {let s=black_box(&mut *self);s.queue.copy_from_slice(&s.template);s.scratch.classify(&s.pending,&s.queue);s.control_r.clear();for &[_,j] in &s.scratch.triples {s.control_r.push(s.queue[j]);}
            baseline(s.buckets,&s.pending,&s.control_r,s.binary);s.queue_len=s.scratch.compact(&mut s.queue);
        }}
    }
    pub fn check(&self,n:usize) {
        let g=G1Projective::generator();let expected=G1Projective::batch_to_affine(&self.initial.iter().zip(&self.delta).map(|(a,d)|g*(*a+Fr::from(n as u64)* *d)).collect::<Vec<_>>());assert_eq!(self.buckets,expected);
        let remaining:Vec<_>=self.queue[..self.queue_len].iter().map(|(i,p)|(*i,**p)).collect();assert_eq!(remaining,self.expected_queue);
    }
    pub fn coordinates(&self)->Vec<[[u64;4];2]> {self.buckets.iter().map(|p|[p.x.to_canonical(),p.y.to_canonical()]).collect()}
    pub fn scratch_bytes(&self)->(usize,usize) {(self.scratch.bytes(),self.binary.fq.groups.capacity()*std::mem::size_of::<FqAddGroup>()+self.binary.fq.tail.capacity()*std::mem::size_of::<[Fq;5]>()+self.control_r.capacity()*std::mem::size_of::<Entry<&G1Affine>>())}
    pub fn queue_sizes(&self)->(usize,usize,usize) {(self.template.len(),self.scratch.triples.len(),self.queue_len)}
}
