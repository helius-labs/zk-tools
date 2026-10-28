//! Diagnostic category masks before the existing Euler output is overwritten.
use std::{collections::BTreeMap,sync::{atomic::{AtomicBool,Ordering},Mutex}};
use zopro_bn254::Fr;
static ACTIVE:AtomicBool=AtomicBool::new(false);
static TOTAL:Mutex<BTreeMap<String,u64>>=Mutex::new(BTreeMap::new());
fn add(m:&mut BTreeMap<String,u64>,k:impl Into<String>,v:u64){*m.entry(k.into()).or_default()+=v;}
pub fn begin(){assert!(!ACTIVE.load(Ordering::Acquire));TOTAL.lock().unwrap().clear();ACTIVE.store(true,Ordering::Release);}
pub fn end()->BTreeMap<String,u64>{assert!(ACTIVE.swap(false,Ordering::AcqRel));TOTAL.lock().unwrap().clone()}
fn census(a:&[Fr],b:&[Fr])->BTreeMap<String,u64>{
    let mut m=BTreeMap::new();let mut eqrun=0u64;let mut maxeqrun=0u64;
    for(first,out)in a.chunks(8).enumerate(){let mut equal=0u8;let mut zero=0u8;let mut single=0u8;let mut opposite=0u8;let valid=((1u16<<out.len())-1)as u8;
        for(j,&aa)in out.iter().enumerate(){let bb=b.get(8*first+j).copied().unwrap_or(Fr::ZERO);let eq=aa==bb;let az=aa==Fr::ZERO;let bz=bb==Fr::ZERO;
            let z=az&&bz;let one=eq||az||bz;equal|=u8::from(eq)<<j;zero|=u8::from(z)<<j;single|=u8::from(one)<<j;opposite|=u8::from(aa==-bb)<<j;
            add(&mut m,"rows",1);add(&mut m,"equal",u64::from(eq));add(&mut m,"both_zero",u64::from(z));add(&mut m,"one_product_nonzero",u64::from(one&&!z));add(&mut m,"two_products",u64::from(!one));
            if eq{eqrun+=1;maxeqrun=maxeqrun.max(eqrun);}else{eqrun=0;}
        }
        add(&mut m,"blocks",1);add(&mut m,"pure_equal_blocks",u64::from(equal==valid));add(&mut m,"pure_zero_blocks",u64::from(zero==valid));add(&mut m,"pure_single_blocks",u64::from(single==valid));
        for(name,mask)in[("equal",equal),("zero",zero),("single",single),("opposite",opposite)]{add(&mut m,format!("mask/{name}/valid={valid:02x}/value={mask:02x}"),1);}
    }
    m.insert("max_equal_run".into(),maxeqrun);m
}
pub(crate)fn record(a:&[Fr],b:&[Fr]){if !ACTIVE.load(Ordering::Acquire){return;}let local=census(a,b);let mut out=TOTAL.lock().unwrap();add(&mut out,"calls",1);for(k,v)in local{add(&mut out,k,v);}}
#[cfg(test)]mod tests{
    use super::*;
    fn factor(a:Fr,b:Fr,da:Fr,db:Fr)->Fr{if a==Fr::ZERO{da*b}else if b==Fr::ZERO{a*db}else if a==b{a*(da+db)}else{da*b+a*db}}
    #[test]fn all_equality_masks_preserve_unequal_derivatives(){
        for mask in 0u16..256{for mode in 0..4{let mut a=[Fr::ZERO;8];let mut b=[Fr::ZERO;8];
            for i in 0..8{a[i]=Fr::from((11+i)as u64);b[i]=if mask>>i&1!=0{a[i]}else{-a[i]};if mode==1&&i%3==0{a[i]=Fr::ZERO;}if mode==2&&i%3==1{b[i]=Fr::ZERO;}if mode==3&&i%3==2{a[i]=Fr::ZERO;b[i]=Fr::ZERO;}
                let da=Fr::from((3+i*7)as u64);let db=-Fr::from((5+i*19)as u64);assert_ne!(da,db);assert_eq!(factor(a[i],b[i],da,db),da*b[i]+a[i]*db);}
            let m=census(&a,&b);assert_eq!(m["rows"],8);assert_eq!(m["both_zero"]+m["one_product_nonzero"]+m["two_products"],8);if mode==0{assert_eq!(m["equal"],mask.count_ones()as u64);}
        }}
    }
    #[test]fn tails_gating_and_derivative_counterexample(){
        for n in 0..=25{for nb in[0,n/2,n]{let a:Vec<_>=(0..n).map(|i|if i%3==0{Fr::ZERO}else{Fr::from(i as u64+1)}).collect();let b:Vec<_>=(0..nb).map(|i|if i%2==0{a[i]}else{-a[i]}).collect();let m=census(&a,&b);assert_eq!(m.get("rows").copied().unwrap_or(0),n as u64);assert_eq!(m.get("blocks").copied().unwrap_or(0),n.div_ceil(8)as u64);}}
        // A(X)=1+X, B(X)=2X at X=1: values equal2, Euler derivatives1and2differ.
        let(a,b,da,db)=(Fr::from(2u64),Fr::from(2u64),Fr::ONE,Fr::from(2u64));assert_eq!(factor(a,b,da,db),Fr::from(6u64));assert_ne!(factor(a,b,da,db),Fr::from(2u64)*a*da);
        record(&[a],&[b]);assert!(TOTAL.lock().unwrap().is_empty());begin();record(&[a],&[b]);let m=end();assert_eq!(m["calls"],1);assert_eq!(m["equal"],1);TOTAL.lock().unwrap().clear();
    }
}
