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
    pub fn run(&mut self,rounds:usize,two_chain:bool,borrowed:bool){
        if borrowed {
            // The production borrowed caller already owns its pending refs.
            // Build those once per driver call, outside the per-batch loop.
            let mut storage=[(0,&self.source[0]);512];
            for (slot,(&(i,_),p)) in storage.iter_mut().zip(self.owned.iter().zip(&self.incoming)){*slot=(i,p);}
            let refs=&storage[..self.owned.len()];
            for _ in 0..rounds {
                #[cfg(zopro_two_chain_stream)] if two_chain {apply::<G2Config,true>(&mut self.buckets,refs,&mut self.scratch);continue;}
                let _=two_chain;super::g2_borrowed::apply::<G2Config,true>(&mut self.buckets,refs,&mut self.scratch);
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
