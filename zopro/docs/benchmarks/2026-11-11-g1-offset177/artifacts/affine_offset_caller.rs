use super::*;use zopro_bn254::{G1Affine,G1Config,G1Projective,G2Affine,G2Config,G2Projective};
pub struct G1Caller {
    source:Vec<G1Affine>, buckets:Vec<G1Affine>, incoming:Vec<G1Affine>,
    owned:Vec<(u32,G1Affine)>, unsigned:Vec<(u32,G1Affine)>, scratch:FqAddScratch, size:usize, seed:u64,
}
impl G1Caller {
    pub fn new(n:usize,seed:u64)->Self {
        assert!(n<=512);
        let size=n.max(1).next_power_of_two();
        let source=G1Projective::batch_to_affine(&(0..size).map(|i|G1Affine::GENERATOR*Fr::from(Self::base(i,seed))).collect::<Vec<_>>());
        let incoming=G1Projective::batch_to_affine(&(0..n).map(|i|G1Affine::GENERATOR*Fr::from(Self::step(i,seed))).collect::<Vec<_>>());
        let owned:Vec<_>=incoming.iter().enumerate().map(|(i,&p)|((((i*37+seed as usize)%size)as u32)|if i%3!=1{1<<31}else{0},p)).collect();
        let unsigned=owned.iter().map(|&(i,p)|((i&0x7fffffff),if i>>31!=0{-p}else{p})).collect();
        Self{buckets:source.clone(),source,incoming,owned,unsigned,size,seed,scratch:FqAddScratch{write_groups:true,reserve_groups:true,vector_inverse:true,canonical:true,..Default::default()}}
    }
    fn base(i:usize,seed:u64)->u64 {(1u64<<48)*(i as u64+1)+(seed+1)*100000003}
    fn step(i:usize,seed:u64)->u64 {11111+(seed+1)*1337+i as u64*101}
    pub fn reset(&mut self){self.buckets.copy_from_slice(&self.source);}
    #[inline(never)]
    pub fn run(&mut self,rounds:usize,_tree:bool,borrowed:bool){
        if borrowed {
            // The production borrowed caller already owns its pending refs.
            // Build those once per driver call, outside the per-batch loop.
            let mut storage=[(0,&self.source[0]);512];
            for (slot,(&(i,_),p)) in storage.iter_mut().zip(self.owned.iter().zip(&self.incoming)){*slot=(i,p);}
            let refs=&storage[..self.owned.len()];
            for _ in 0..rounds{stream_g1_impl::<G1Config,true,true>(&mut self.buckets,refs,&mut self.scratch);}
        }else{
            for _ in 0..rounds{stream_g1_impl::<G1Config,true,false>(&mut self.buckets,&self.unsigned,&mut self.scratch);}
        }
    }
    pub fn check(&self,rounds:usize){
        let mut coeff=(0..self.size).map(|i|Fr::from(Self::base(i,self.seed))).collect::<Vec<_>>();
        for (j,&(index,_)) in self.owned.iter().enumerate(){let s=Fr::from(Self::step(j,self.seed))*Fr::from(rounds as u64);let k=(index&0x7fffffff)as usize;coeff[k]+=if index>>31!=0{-s}else{s};}
        let expected=G1Projective::batch_to_affine(&coeff.iter().map(|&c|G1Affine::GENERATOR*c).collect::<Vec<_>>());assert_eq!(self.buckets,expected);
    }
    pub fn capacities(&self)->[usize;2]{[self.scratch.groups.capacity()*core::mem::size_of::<FqAddGroup>(),self.scratch.tail.capacity()*core::mem::size_of::<[Fq;5]>()]}
    pub fn digest(&self)->Vec<u8>{let mut out=vec![];for p in &self.buckets{for f in[p.x,p.y]{for w in f.to_canonical(){out.extend_from_slice(&w.to_le_bytes());}}}out}
}

pub struct G2Caller {
    source:Vec<G2Affine>, buckets:Vec<G2Affine>, incoming:Vec<G2Affine>,
    owned:Vec<(u32,G2Affine)>, unsigned:Vec<(u32,G2Affine)>, scratch:Fq2AddScratch, size:usize, seed:u64,
}
impl G2Caller {
    pub fn new(n:usize,seed:u64)->Self {
        assert!(n<=512);
        let size=n.max(1).next_power_of_two();
        let source=G2Projective::batch_to_affine(&(0..size).map(|i|G2Affine::GENERATOR*Fr::from(Self::base(i,seed))).collect::<Vec<_>>());
        let incoming=G2Projective::batch_to_affine(&(0..n).map(|i|G2Affine::GENERATOR*Fr::from(Self::step(i,seed))).collect::<Vec<_>>());
        let owned:Vec<_>=incoming.iter().enumerate().map(|(i,&p)|((((i*37+seed as usize)%size)as u32)|if i%3!=1{1<<31}else{0},p)).collect();
        let unsigned=owned.iter().map(|&(i,p)|((i&0x7fffffff),if i>>31!=0{-p}else{p})).collect();
        Self{buckets:source.clone(),source,incoming,owned,unsigned,size,seed,scratch:Fq2AddScratch{vector_inverse:true,canonical:true,..Default::default()}}
    }
    fn base(i:usize,seed:u64)->u64 {(1u64<<48)*(i as u64+1)+(seed+1)*100000003}
    fn step(i:usize,seed:u64)->u64 {11111+(seed+1)*1337+i as u64*101}
    pub fn reset(&mut self){self.buckets.copy_from_slice(&self.source);}
    #[inline(never)]
    pub fn run(&mut self,rounds:usize,tree:bool,borrowed:bool){
        if borrowed {
            // The production borrowed caller already owns its pending refs.
            // Build those once per driver call, outside the per-batch loop.
            let mut storage=[(0,&self.source[0]);512];
            for (slot,(&(i,_),p)) in storage.iter_mut().zip(self.owned.iter().zip(&self.incoming)){*slot=(i,p);}
            let refs=&storage[..self.owned.len()];
            for _ in 0..rounds {
                let _=tree;super::g2_borrowed::apply::<G2Config,true>(&mut self.buckets,refs,&mut self.scratch);
            }
        }else{panic!("G2 caller is borrowed");
        }
    }
    pub fn check(&self,rounds:usize){
        let mut coeff=(0..self.size).map(|i|Fr::from(Self::base(i,self.seed))).collect::<Vec<_>>();
        for (j,&(index,_)) in self.owned.iter().enumerate(){let s=Fr::from(Self::step(j,self.seed))*Fr::from(rounds as u64);let k=(index&0x7fffffff)as usize;coeff[k]+=if index>>31!=0{-s}else{s};}
        let expected=G2Projective::batch_to_affine(&coeff.iter().map(|&c|G2Affine::GENERATOR*c).collect::<Vec<_>>());assert_eq!(self.buckets,expected);
    }
    pub fn capacities(&self)->[usize;2]{[self.scratch.prefix_records.capacity()*core::mem::size_of::<[FqBatch;2]>(),self.scratch.tail.capacity()*core::mem::size_of::<[Fq2;5]>()]}
    pub fn digest(&self)->Vec<u8>{let mut out=vec![];for p in &self.buckets{for f in[p.x.c0,p.x.c1,p.y.c0,p.y.c1]{for w in f.to_canonical(){out.extend_from_slice(&w.to_le_bytes());}}}out}
}


#[cfg(test)]
mod affine_offset_integration {
    use super::*;
    #[test]
    fn both_curves_signs_tails_storage_and_reuse() {
        assert_eq!(core::mem::size_of::<FqAddGroup>(), 1600);
        assert_eq!(core::mem::align_of::<FqAddGroup>(), 64);
        let mut points = 0;
        for n in [0,1,7,8,9,15,16,17,31,32,33,199,200,201,351,352,353,511] {
            for seed in [3,31] {
                for write in [false,true] {
                    let mut g1=G1Caller::new(n,seed);
                    let mut g2=G2Caller::new(n,seed);
                    g1.scratch.write_groups=write;
                    let inputs=(g1.incoming.clone(),g2.incoming.clone());
                    for rounds in [1,3,17] {
                        g1.reset();g2.reset();g1.run(rounds,false,true);g2.run(rounds,false,true);
                        g1.check(rounds);g2.check(rounds);
                        assert_eq!(g1.incoming,inputs.0);assert_eq!(g2.incoming,inputs.1);
                        let caps=(g1.capacities(),g2.capacities());
                        g1.reset();g2.reset();g1.run(rounds+1,false,true);g2.run(rounds+1,false,true);
                        g1.check(rounds+1);g2.check(rounds+1);
                        assert_eq!(caps,(g1.capacities(),g2.capacities()));points+=4*g1.size;
                    }
                }
            }
        }
        println!("AFFINE_OFFSET_COUPLED_OUTPUTS={points}");
    }
    #[test]
    fn owned_prefix_and_unsigned_g1_share_the_canonical_offset_contract() {
        let mut checked=0;
        for n in [0,1,7,8,9,17,199,200,201,351,352,353] {
            for fast in [false,true] {
                for write in [false,true] {
                    let mut g1=G1Caller::new(n,73);g1.scratch.write_groups=write;
                    let mut g2=G2Caller::new(n,73);
                    let originals=(g1.incoming.clone(),g2.incoming.clone());
                    for rounds in [1,3] {
                        g1.reset();g2.reset();
                        for _ in 0..rounds {
                            if fast {
                                stream_g1_impl::<G1Config,true,false>(&mut g1.buckets,&g1.unsigned,&mut g1.scratch);
                                crate::msm::g2_prefix::apply::<G2Config,true>(&mut g2.buckets,&g2.unsigned,&mut g2.scratch);
                            } else {
                                stream_g1_impl::<G1Config,false,false>(&mut g1.buckets,&g1.unsigned,&mut g1.scratch);
                                crate::msm::g2_prefix::apply::<G2Config,false>(&mut g2.buckets,&g2.unsigned,&mut g2.scratch);
                            }
                        }
                        g1.check(rounds);g2.check(rounds);checked+=2*g1.size;
                        assert_eq!(g2.scratch.groups.capacity(),0);
                    }
                    assert_eq!(g1.incoming,originals.0);assert_eq!(g2.incoming,originals.1);
                }
            }
        }
        println!("AFFINE_OFFSET_OWNED_OUTPUTS={checked}");
    }
    #[test]
    fn every_small_stack_worker_executes_complete_callers() {
        use std::sync::atomic::{AtomicUsize,Ordering};
        let visits=AtomicUsize::new(0);
        for workers in [1,4,16,32] {
            let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
            pool.broadcast(|context| {
                for n in [17,200,353] {
                    let mut g1=G1Caller::new(n,43+context.index() as u64);
                    let mut g2=G2Caller::new(n,43+context.index() as u64);
                    g1.run(3,false,true);g2.run(3,false,true);g1.check(3);g2.check(3);
                    let caps=(g1.capacities(),g2.capacities());
                    g1.reset();g2.reset();g1.run(5,false,true);g2.run(5,false,true);g1.check(5);g2.check(5);
                    assert_eq!(caps,(g1.capacities(),g2.capacities()));
                }
                visits.fetch_add(1,Ordering::SeqCst);
            });
        }
        assert_eq!(visits.load(Ordering::SeqCst),53);
        println!("AFFINE_OFFSET_ACTUAL_WORKERS=53 STACK_REQUEST=524288");
    }
}
