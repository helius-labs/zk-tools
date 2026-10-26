//! Experimental replacement CPU basis. The converter has one solver plan,
//! mapped original/cache points and one caller-sized projective point tile.
use crate::{Fr,G1Affine,G1Projective,Image,Points,Store,solver};
use sha2::{Digest,Sha256};
use std::{collections::BTreeSet,io::Write};
fn bytes(points:&[G1Affine])->&[u8]{unsafe{std::slice::from_raw_parts(points.as_ptr().cast(),points.len()*64)}}
fn field(h:&mut Sha256,f:Fr){for w in f.0{h.update(w.to_le_bytes());}}

pub(crate) fn cache_fingerprints(z:&[G1Affine],n:usize,generator:Fr,coset:Fr)->Result<[([u8;32],Fr);2],&'static str>{
    if n<8||!n.is_power_of_two()||z.len()+1!=n{return Err("invalid cache geometry");}
    let inv_n=Fr::from(n as u64).inverse().ok_or("domain inverse")?;
    let den=(coset.pow(&[n as u64])-Fr::ONE).inverse().ok_or("coset denominator")?;
    let base=coset.inverse().ok_or("coset inverse")?;
    let c_weight=inv_n*den;let extra=inv_n*inv_n;
    let mut h=Sha256::new();h.update(b"zopro-qap-v1\0\0\0\0");h.update((n as u64).to_le_bytes());h.update(bytes(z));
    let inverse=generator.inverse().ok_or("generator inverse")?;let mut v=Fr::ONE;
    for _ in 0..n/2{field(&mut h,v);v*=inverse;}
    let mut twisted=h.clone();
    let jumps:Vec<_>=(0..n.trailing_zeros()).map(|t|base.pow(&[(n>>(t+1))as u64])*coset.pow(&[(n-(n>>t))as u64])).collect();
    let mut power=Fr::ONE;
    for i in 0..n{let value=c_weight*power;field(&mut h,value);field(&mut twisted,value*extra);if i+1<n{power*=jumps[(i+1).trailing_zeros()as usize];}}
    field(&mut h,c_weight);field(&mut twisted,c_weight);
    Ok([(h.finalize().into(),Fr::ONE),(twisted.finalize().into(),extra)])
}


pub const EULER_CACHE_MAGIC:&[u8;16]=b"zopro-eul-v1\0\0\0\0";
pub fn euler_cache_fingerprint(z:&[G1Affine],n:usize,generator:Fr)->Result<[u8;32],&'static str>{
    if n<8||!n.is_power_of_two()||z.len()+1!=n{return Err("Euler cache geometry");}
    let inv=Fr::from(n as u64).inverse().ok_or("Euler domain inverse")?;
    let mut h=Sha256::new();h.update(EULER_CACHE_MAGIC);h.update((n as u64).to_le_bytes());h.update(bytes(z));
    field(&mut h,generator);field(&mut h,inv*inv*inv);field(&mut h,inv*inv);Ok(h.finalize().into())
}

pub struct CpuBasisBuilder {
    pub euler:bool,
    plan:solver::Plan,k:Store<G1Affine>,product:Store<G1Affine>,constraints:Store<G1Affine>,slots:Vec<usize>,
    pub scale:Fr,pub digest:[u8;32],pub tile_points:usize,
}
impl CpuBasisBuilder {
    pub fn new(points:Points,domain_params:&[u8],solver_image:&[u8],cache:&Image,tile_points:usize)->Result<Self,String>{
        let counts=points.counts()?;let n=counts[3].checked_add(1).ok_or("domain overflow")?;
        if domain_params.len()!=64||tile_points==0||tile_points>usize::MAX/256{return Err("invalid conversion parameters".into());}
        let params=crate::frs(domain_params);let z=points.g1(3)?;
        let raw=(**cache).as_ref();
        let expected=n.checked_mul(128).and_then(|v|v.checked_add(88)).ok_or("cache size overflow")?;
        if raw.len()!=expected||(&raw[..16]!=b"zopro-qap-v1\0\0\0\0" && &raw[..16]!=EULER_CACHE_MAGIC)||u64::from_le_bytes(raw[80..88].try_into().unwrap())!=n as u64{return Err("cache header mismatch".into());}
        let hash=Sha256::digest(&raw[88..]);if hash.as_slice()!=&raw[48..80]{return Err("cache checksum mismatch".into());}
        let euler=&raw[..16]==EULER_CACHE_MAGIC;
        let scale=if euler {
            if euler_cache_fingerprint(&z,n,params[0])?[..]!=raw[16..48]{return Err("Euler cache does not match original query".into());}Fr::ONE
        }else{cache_fingerprints(&z,n,params[0],params[1])?.into_iter().find(|(h,_)|h[..]==raw[16..48]).map(|(_,s)|s).ok_or("cache does not match original quotient query")?};
        drop(z);
        let plan=solver::Plan::from_image(solver_image)?;
        if !plan.matches(counts[0],counts[1],counts[2],n){return Err("solver/key mismatch".into());}
        let mut selected=BTreeSet::new();plan.for_each_c(|_,wire,_|{if wire>=plan.basis_wires(){selected.insert(wire);}});
        let slots=selected.into_iter().collect();let digest=plan.basis_c_digest();
        Ok(Self{euler,plan,k:points.g1(2)?,product:Store::mapped(cache,88,n*64)?,constraints:Store::mapped(cache,88+n*64,n*64)?,slots,scale,digest,tile_points})
    }
    pub fn slots(&self)->&[usize]{&self.slots}
    pub fn public_count(&self)->usize{self.plan.n_public}
    pub fn domain(&self)->usize{self.product.len()}
    pub fn write_private(&self,out:&mut impl Write)->Result<(),String>{
        let k=self.k.len();let count=k+self.slots.len();let nw=self.plan.basis_wires();let np=self.plan.n_public;
        for start in (0..count).step_by(self.tile_points){
            let end=(start+self.tile_points).min(count);
            let mut points:Vec<_>=(start..end).map(|i|if i<k{self.k[i].to_projective()}else{G1Projective::IDENTITY}).collect();
            self.plan.for_each_c(|row,wire,c|{
                let index=if (np..nw).contains(&wire){Some(wire-np)}else if wire>=nw{self.slots.binary_search(&wire).ok().map(|i|k+i)}else{None};
                if let Some(i)=index.filter(|&i|(start..end).contains(&i)){
                    let q=self.constraints[row];if c==Fr::ONE{points[i-start]-=q;}else if c==-Fr::ONE{points[i-start]+=q;}else{points[i-start]-=q*c;}
                }
            });
            let affine=G1Projective::batch_to_affine(&points);out.write_all(bytes(&affine)).map_err(|e|e.to_string())?;
        }Ok(())
    }
    pub fn write_product(&self,out:&mut impl Write)->Result<(),String>{out.write_all(bytes(&self.product)).map_err(|e|e.to_string())}
    pub fn write_public(&self,out:&mut impl Write)->Result<(),String>{
        // Public queries use the same tile bound as private ones.
        for start in (0..self.plan.n_public).step_by(self.tile_points){
            let end=(start+self.tile_points).min(self.plan.n_public);let mut points=vec![G1Projective::IDENTITY;end-start];
            self.plan.for_each_c(|row,wire,c|{if (start..end).contains(&wire){let q=self.constraints[row];if c==Fr::ONE{points[wire-start]-=q;}else if c==-Fr::ONE{points[wire-start]+=q;}else{points[wire-start]-=q*c;}}});
            out.write_all(bytes(&G1Projective::batch_to_affine(&points))).map_err(|e|e.to_string())?;
        }Ok(())
    }
}
