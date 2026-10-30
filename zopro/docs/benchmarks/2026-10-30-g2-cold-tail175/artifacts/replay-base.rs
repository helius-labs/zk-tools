//! Private diagnostic: bounded production operands and arithmetic-only replay.
use super::*;
use std::collections::BTreeSet;
use zopro_bn254::G2Config;

const MAGIC: &[u8; 8] = b"ZOG2R001";
const MAX_PAIRS: usize = 512;
type Point = Affine<G2Config>;
type Row = (u32, Fq2, Fq2, Fq2, Fq2);
struct Record {borrowed:bool,buckets:usize,rows:Vec<Row>}
fn write_point(bytes:&mut Vec<u8>,x:Fq2,y:Fq2){for f in [x.c0,x.c1,y.c0,y.c1]{for word in f.0 {bytes.extend_from_slice(&word.to_le_bytes());}}}
fn encode<C:CurveConfig<Base=Fq2>>(record:&Record,after:&[Affine<C>])->Vec<u8>{
    assert!(record.rows.len()<=MAX_PAIRS);
    let mut bytes=Vec::with_capacity(32+388*record.rows.len());bytes.extend_from_slice(MAGIC);
    for n in [record.borrowed as u64,record.buckets as u64,record.rows.len()as u64]{bytes.extend_from_slice(&n.to_le_bytes());}
    for &(id,ax,ay,bx,by)in &record.rows{let out=after[(id&0x7fff_ffff)as usize];assert!(out.is_on_curve());bytes.extend_from_slice(&id.to_le_bytes());write_point(&mut bytes,ax,ay);write_point(&mut bytes,bx,by);write_point(&mut bytes,out.x,out.y);}
    assert_eq!(bytes.len(),32+388*record.rows.len());bytes
}
struct Reader<'a>{bytes:&'a[u8],at:usize}
impl Reader<'_>{
    fn take<const N:usize>(&mut self)->Result<[u8;N],String>{let end=self.at.checked_add(N).ok_or("length overflow")?;let slice=self.bytes.get(self.at..end).ok_or("truncated fixture")?;self.at=end;Ok(slice.try_into().unwrap())}
    fn word(&mut self)->Result<u64,String>{Ok(u64::from_le_bytes(self.take()?))}
    fn field(&mut self)->Result<Fq,String>{let a=[self.word()?,self.word()?,self.word()?,self.word()?];if a.iter().rev().cmp(Fq::MODULUS.iter().rev())!=core::cmp::Ordering::Less{return Err("noncanonical Montgomery field".into())}Ok(Fq::from_montgomery(a))}
    fn point(&mut self)->Result<Point,String>{let p=Point::new(Fq2::new(self.field()?,self.field()?),Fq2::new(self.field()?,self.field()?));if !p.is_on_curve(){return Err("off-curve point".into())}Ok(p)}
}
/// Arithmetic operands are real; compact bucket/key locality is synthetic.
pub struct Fixture {
    initial:Vec<Point>, buckets:Vec<Point>, batch:Vec<(u32,Point)>, expected:Vec<Point>,
    scratch:Fq2AddScratch,borrowed:bool,original_bucket_count:usize,
}
impl Fixture{
    pub fn decode(bytes:&[u8])->Result<Self,String>{
        let mut r=Reader{bytes,at:0};if &r.take::<8>()?!=MAGIC{return Err("bad fixture magic".into())}
        let mode=r.word()?;if mode>1{return Err("bad caller mode".into())}
        let original_bucket_count=usize::try_from(r.word()?).map_err(|_|"bucket count overflow")?;
        let n=usize::try_from(r.word()?).map_err(|_|"pair count overflow")?;
        if n<8||n>MAX_PAIRS||original_bucket_count==0{return Err("unsupported fixture size".into())}
        if bytes.len()!=32+388*n{return Err("wrong fixture length".into())}
        let(mut initial,mut batch,mut expected)=(Vec::with_capacity(n),Vec::with_capacity(n),Vec::with_capacity(n));let mut seen=BTreeSet::new();
        for i in 0..n {let id=u32::from_le_bytes(r.take()?);let index=(id&0x7fff_ffff)as usize;if index>=original_bucket_count||!seen.insert(index)||mode==0&&id>>31!=0{return Err("invalid or repeated destination".into())}
            let a=r.point()?;let b=r.point()?;let c=r.point()?;
            if a.is_identity()||b.is_identity()||a.x==b.x{return Err("exceptional affine input".into())}
            initial.push(a);batch.push((i as u32|(id&0x8000_0000),b));expected.push(c);
        }
        assert_eq!(r.at,bytes.len());let scratch=BatchAdd::<G2Config>::default().fq2;
        Ok(Self{buckets:initial.clone(),initial,batch,expected,scratch,borrowed:mode!=0,original_bucket_count})
    }
    pub fn len(&self)->usize{self.batch.len()}
    pub fn borrowed(&self)->bool{self.borrowed}
    pub fn original_bucket_count(&self)->usize{self.original_bucket_count}
    pub fn negative(&self)->usize{self.batch.iter().filter(|(id,_)|id>>31!=0).count()}
    pub fn runner(&mut self)->Runner<'_>{Runner{initial:&self.initial,buckets:&mut self.buckets,batch:&self.batch,references:self.batch.iter().map(|(i,p)|(*i,p)).collect(),expected:&self.expected,scratch:&mut self.scratch,borrowed:self.borrowed}}
    pub fn with_runner<T>(&mut self,f:impl FnOnce(&mut Runner<'_>)->T)->T{f(&mut self.runner())}
}
pub struct Runner<'a>{initial:&'a[Point],buckets:&'a mut[Point],batch:&'a[(u32,Point)],references:Vec<(u32,&'a Point)>,expected:&'a[Point],scratch:&'a mut Fq2AddScratch,borrowed:bool}
impl Runner<'_>{
    /// Reset copying is included; reference construction is outside this loop.
    pub fn run(&mut self,rounds:usize){for _ in 0..rounds{self.buckets.copy_from_slice(self.initial);if self.borrowed{super::g2_borrowed::apply::<G2Config,true>(self.buckets,&self.references,self.scratch)}else{super::g2_prefix::apply::<G2Config,true>(self.buckets,self.batch,self.scratch)}}}
    pub fn check(&self){assert_eq!(self.buckets,self.expected)}
    pub fn capacity_bytes(&self)->usize{self.scratch.prefix_records.capacity()*core::mem::size_of::<[FqBatch;2]>()+self.scratch.tail.capacity()*core::mem::size_of::<[Fq2;5]>()+self.references.capacity()*core::mem::size_of::<(u32,&Point)>()}
    pub fn bytes(&self)->Vec<u8>{let mut out=Vec::with_capacity(self.buckets.len()*128);for p in self.buckets.iter(){write_point(&mut out,p.x,p.y);}out}
}

#[cfg(test)]mod tests{
    use super::*;
    fn sample(n:usize,borrowed:bool)->Vec<u8>{
        let g=Projective::<G2Config>::generator();let mut before=vec![Point::IDENTITY;n*2];let mut after=before.clone();let mut rows=vec![];
        for i in 0..n{let a=(g*Fr::from((i+100)as u64)).to_affine();let b=(g*Fr::from((i+10007)as u64)).to_affine();let id=(i*2)as u32|if borrowed&&i%3==0{1<<31}else{0};before[i*2]=a;after[i*2]=(a+if id>>31!=0{-b}else{b}).to_affine();rows.push((id,a.x,a.y,b.x,b.y));}
        encode(&Record{borrowed,buckets:before.len(),rows},&after)
    }
    #[test]fn replay_codec_signs_tails_and_reuse(){
        for n in [8,9,31,32,127,128,199,200,255,256,351,352,511,512]{for borrowed in[false,true]{let bytes=sample(n,borrowed);let mut fixture=Fixture::decode(&bytes).unwrap();assert_eq!(fixture.len(),n);assert_eq!(fixture.borrowed(),borrowed);assert_eq!(fixture.original_bucket_count(),n*2);fixture.with_runner(|r|{r.run(1);r.check();r.run(3);r.check();});}}
    }
    #[test]fn replay_rejects_malformed_and_ambiguous_records(){
        let bytes=sample(9,true);
        for length in [0,7,31,32,bytes.len()-1]{assert!(Fixture::decode(&bytes[..length]).is_err());}
        for (offset,patch)in[(0,vec![0]),(8,2u64.to_le_bytes().to_vec()),(24,513u64.to_le_bytes().to_vec()),(32+388,0u32.to_le_bytes().to_vec()),(36,vec![255;32])]{let mut bad=bytes.clone();bad[offset..offset+patch.len()].copy_from_slice(&patch);assert!(Fixture::decode(&bad).is_err(),"offset {offset}");}
        let mut extra=bytes;extra.push(0);assert!(Fixture::decode(&extra).is_err());
    }

}
