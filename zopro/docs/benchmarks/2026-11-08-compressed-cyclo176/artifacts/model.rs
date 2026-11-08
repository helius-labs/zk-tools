use ark_ff::{Field as ArkField, PrimeField};
use serde_json::{json, Value};
use std::fs;
use zopro_bn254::{Field, Fq, Fq2, Fq6, Fq12};

const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/compressed-cyclo176";
const X: u64 = 4_965_661_367_192_848_881;
include!("accepted-square.rs.inc");

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Counts { s2:u64,m2:u64,mq:u64,iq:u64,m12:u64,nr:u64,normal:u64,degenerate:u64,identity:u64 }
impl Counts {
    fn json(self)->Value {json!({"Fq2_squares":self.s2,"Fq2_multiplies":self.m2,"direct_Fq_multiplies":self.mq,"Fq_inverses":self.iq,"Fq12_multiplies":self.m12,"nonresidue_calls":self.nr+7*self.m12,"normal_recoveries":self.normal,"degenerate_recoveries":self.degenerate,"identity_recoveries":self.identity,"lowered_Fq_multiplies":2*self.s2+self.mq,"lowered_Fq_fused2":2*self.m2+36*self.m12})}
}
fn sq(x:Fq2,c:&mut Counts)->Fq2 {c.s2+=1;x.square()}
fn mul(x:Fq2,y:Fq2,c:&mut Counts)->Fq2 {c.m2+=1;x*y}
fn nr(x:Fq2,c:&mut Counts)->Fq2 {c.nr+=1;x.mul_by_nonresidue()}
fn three_minus(t:Fq2,x:Fq2)->Fq2 {(t-x).double()+t}
fn three_plus(t:Fq2,x:Fq2)->Fq2 {(t+x).double()+t}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Compressed {g1:Fq2,g2:Fq2,g3:Fq2,g5:Fq2}
const Z:Compressed=Compressed{g1:Fq2::ZERO,g2:Fq2::ZERO,g3:Fq2::ZERO,g5:Fq2::ZERO};
impl Compressed {
    fn from(x:Fq12)->Self {Self{g1:x.c0.c1,g2:x.c0.c2,g3:x.c1.c0,g5:x.c1.c2}}
    fn square(self,c:&mut Counts)->Self {
        let b0=sq(self.g3,c)+nr(sq(self.g2,c),c);
        let b1=mul(self.g3,self.g2,c).double();
        let c0=sq(self.g1,c)+nr(sq(self.g5,c),c);
        let c1=mul(self.g1,self.g5,c).double();
        Self{g1:three_minus(b0,self.g1),g2:three_minus(c0,self.g2),g3:three_plus(nr(c1,c),self.g3),g5:three_plus(b1,self.g5)}
    }
    fn fraction(self,c:&mut Counts)->Option<(Fq2,Fq2)> {
        if self.g3.is_zero() {
            if self.g2.is_zero() {assert_eq!(self,Z,"only valid identity tuples may enter this branch");c.identity+=1;None}
            else {c.degenerate+=1;Some((mul(self.g1,self.g5,c).double(),self.g2))}
        } else {
            c.normal+=1;
            let a=sq(self.g1,c);
            let b=nr(sq(self.g5,c),c);
            Some((b+(a-self.g2).double()+a,self.g3.double().double()))
        }
    }
    fn finish(self,g4:Fq2,c:&mut Counts)->Fq12 {
        let a=mul(self.g2,self.g1,c);
        let b=sq(g4,c);
        let t=(b-a).double()-a+mul(self.g3,self.g5,c);
        Fq12::new(Fq6::new(Fq2::ONE+nr(t,c),self.g1,self.g2),Fq6::new(self.g3,g4,self.g5))
    }
    fn debug_recover(self)->Fq12 {
        let mut c=Counts::default();
        match self.fraction(&mut c) {None=>Fq12::ONE,Some((a,b))=>self.finish(a*b.inverse().unwrap(),&mut c)}
    }
}
fn binary(f:Fq12,c:&mut Counts)->Fq12 {
    let mut r=Fq12::ONE;
    for bit in (0..64-X.leading_zeros()).rev() {
        r=accepted_square(&r);c.s2+=6;c.m2+=3;c.nr+=3;
        if X>>bit&1!=0 {r*=f;c.m12+=1;}
    }r
}
fn candidate(f:Fq12,c:&mut Counts)->(Fq12,usize) {
    assert_eq!(X&1,1);
    let mut p=Compressed::from(f);
    let mut selected=[Z;64];
    let mut n=0;
    for bit in 1..64-X.leading_zeros() {p=p.square(c);if X>>bit&1!=0{selected[n]=p;n+=1;}}
    let mut fractions=[(Fq2::ZERO,Fq2::ONE);64];
    let mut norms=[Fq::ONE;64];
    let mut prefixes=[Fq::ONE;64];
    let mut active=[false;64];
    let mut total=Fq::ONE;
    for i in 0..n {
        if let Some((a,b))=selected[i].fraction(c) {
            active[i]=true;fractions[i]=(a,b);
            norms[i]=b.c0.square()+b.c1.square();c.mq+=2;
            prefixes[i]=total;total*=norms[i];c.mq+=1;
        }
    }
    // The fixed recipe retains one inverse even for the all-identity input.
    let mut inv=total.inverse().unwrap();c.iq+=1;
    let mut r=f;
    for i in (0..n).rev() {
        if active[i] {
            let ni=prefixes[i]*inv;inv*=norms[i];c.mq+=2;
            let (a,b)=fractions[i];
            let bi=Fq2::new(b.c0*ni,-(b.c1*ni));c.mq+=2;
            assert_eq!(b*bi,Fq2::ONE);
            let g4=mul(a,bi,c);
            let value=selected[i].finish(g4,c);
            r*=value;c.m12+=1;
        }else{r*=Fq12::ONE;c.m12+=1;}
    }
    assert_eq!(inv,Fq::ONE,"reverse returns inverse initial unit");
    (r,n)
}
fn af(x:Fq)->ark_bn254::Fq {ark_bn254::Fq::from_be_bytes_mod_order(&x.to_be_bytes())}
fn a2(x:Fq2)->ark_bn254::Fq2 {ark_bn254::Fq2::new(af(x.c0),af(x.c1))}
fn a6(x:Fq6)->ark_bn254::Fq6 {ark_bn254::Fq6::new(a2(x.c0),a2(x.c1),a2(x.c2))}
fn a12(x:Fq12)->ark_bn254::Fq12 {ark_bn254::Fq12::new(a6(x.c0),a6(x.c1))}
fn rnd(s:&mut u64)->u64 {*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
fn random_fq(s:&mut u64)->Fq {let mut b=[0;64];for c in b.chunks_exact_mut(8){c.copy_from_slice(&rnd(s).to_le_bytes());}Fq::from_random_bytes(&b)}
fn random_12(s:&mut u64)->Fq12 {
    let mut f2=||Fq2::new(random_fq(s),random_fq(s));
    Fq12::new(Fq6::new(f2(),f2(),f2()),Fq6::new(f2(),f2(),f2()))
}
fn easy(x:Fq12)->Option<Fq12>{let t=x.conjugate()*x.inverse()?;Some(t.frobenius(2)*t)}
fn main(){
    assert_eq!(easy(Fq12::new(Fq6::ZERO,Fq6::ZERO)),None);
    let mut seed=0x4b41_5241_4249_4e41;
    let mut cases=Vec::new();let mut steps=0;let mut default_stats=None;
    for id in 0..257 {
        let raw=if id==0{Fq12::ONE}else{random_12(&mut seed)};
        let f=easy(raw).unwrap();let ar=a12(raw);
        let mut ae=ar;ae.conjugate_in_place();
        let t=ae*ar.inverse().unwrap();let mut fr=t;fr.frobenius_map_in_place(2);let ref_easy=fr*t;
        assert_eq!(a12(f),ref_easy);
        let mut p=Compressed::from(f);let mut whole=f;
        let mut check_counts=Counts::default();
        for bit in 1..64-X.leading_zeros(){p=p.square(&mut check_counts);whole=whole.square();let rec=p.debug_recover();assert_eq!(rec,whole);assert_eq!(a12(rec),ref_easy.pow([1u64<<bit]));steps+=1;}
        let mut old=Counts::default();let a=binary(f,&mut old);
        let mut new=Counts::default();let(b,n)=candidate(f,&mut new);
        assert_eq!(a,b);assert_eq!(a12(b),ref_easy.pow([X]));
        if id>0 {if let Some(prior)=default_stats {assert_eq!((old,new,n),prior);}else{default_stats=Some((old,new,n));}}
        cases.push(json!({"case":id,"identity_input":id==0,"old":old.json(),"candidate":new.json(),"selected_compressed_powers":n}));
    }
    // Invalid tuple test is a classification assertion, without executing an
    // invalid subgroup recovery path. No arbitrary tuple acceptance claim.
    let invalid=Compressed{g1:Fq2::ONE,..Z};
    assert!(invalid.g3.is_zero()&&invalid.g2.is_zero()&&invalid!=Z);
    assert_eq!(Z.debug_recover(),Fq12::ONE);
    let(old,new,n)=default_stats.unwrap();let o=old.json();let c=new.json();
    let dominated=["lowered_Fq_multiplies","lowered_Fq_fused2","Fq_inverses","nonresidue_calls"].iter().all(|k|c[k].as_u64().unwrap()>=o[k].as_u64().unwrap())&&["lowered_Fq_multiplies","lowered_Fq_fused2","Fq_inverses","nonresidue_calls"].iter().any(|k|c[k].as_u64().unwrap()>o[k].as_u64().unwrap());
    let out=json!({"passed_mathematical_checks":true,"input_cases":257,"fullwidth_random_cases":256,"consecutive_power_checks":steps,"identity_case":cases[0],"ordinary_counts":{"old":o,"candidate":c},"selected_powers":n,"parameter":X,"parameter_bitlen":64-X.leading_zeros(),"parameter_popcount":X.count_ones(),"count_dominated":dominated,"native_selection_requires_tradeoff_review":!dominated,"selected_native_backend":false,"degenerate_nonidentity_cases_observed":cases.iter().map(|x|x["candidate"]["degenerate_recoveries"].as_u64().unwrap()).sum::<u64>(),"invalid_tuple_classification_assertions":1,"zero_input_easy_map_rejected":true,"count_scope":"semantic source method entries; candidate debug/oracle calls excluded; no timing or emitted-instruction claim","fixed_payload_bytes":{"selected_compressed":std::mem::size_of::<[Compressed;64]>(),"fractions":std::mem::size_of::<[(Fq2,Fq2);64]>(),"norms":std::mem::size_of::<[Fq;64]>(),"prefixes":std::mem::size_of::<[Fq;64]>(),"active":std::mem::size_of::<[bool;64]>()},"cases":cases,"new_timings":0,"new_SDK_requests":0,"new_proofs":0,"canonical_unchanged":35171,"goal_complete":false});
    fs::write(format!("{ROOT}/model-result.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("PASS {} complete powers;{} consecutive square/recovery checks; selected{} powers; count dominated{}",257,steps,n,dominated);
}
