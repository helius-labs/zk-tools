// Separate negative-result checker. It does not import model.rs. Affine
// expressions use sorted vectors, never BTreeMap. After proving each
// assignment non-affine under this abstraction, its wire is a fresh atom.
use zopro_bn254::Fr;
use serde_json::{json,Value};
use std::{fs,path::Path};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/affine-closure176";
type Lc=Vec<(usize,Fr)>; // atom0 is constant one
fn normalize(mut v:Lc)->Lc{
    v.sort_unstable_by_key(|x|x.0);
    let mut out:Lc=vec![];
    for (w,c) in v {
        if let Some(last)=out.last_mut().filter(|last|last.0==w){last.1+=c;}
        else{out.push((w,c));}
    }
    out.retain(|x|x.1!=Fr::ZERO);out
}
fn add(mut a:Lc,b:Lc)->Lc{a.extend(b);normalize(a)}
fn scale(v:&Lc,c:Fr)->Lc{normalize(v.iter().map(|&(w,x)|(w,x*c)).collect())}
fn constant(v:&Lc)->Option<Fr>{if v.iter().any(|x|x.0!=0){None}else{Some(v.first().map_or(Fr::ZERO,|x|x.1))}}
struct Cursor<'a>{b:&'a[u8],at:usize}
impl Cursor<'_>{
    fn word(&mut self)->usize{let n=u32::from_le_bytes(self.b[self.at..self.at+4].try_into().unwrap());self.at+=4;n as usize}
    fn field(&mut self)->Fr{let w=std::array::from_fn(|_|{let lo=self.word() as u64;let hi=self.word() as u64;lo|(hi<<32)});assert!(w.iter().rev().cmp(Fr::MODULUS.iter().rev()).is_lt());Fr::from_montgomery(w)}
}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn verify(data:&[u8])->Value{
    let mut r=Cursor{b:data,at:0};assert_eq!(r.word(),0x3150535a);assert_eq!(r.word(),2);
    let [nw,ni,np,nc,na,nb,ncoef,nt,ns,nslots,nops]=std::array::from_fn(|_|r.word());
    let queries:[Vec<usize>;2]=[na,nb].map(|n|(0..n).map(|_|r.word()).collect());
    let coeff:Vec<_>=(0..ncoef).map(|_|r.field()).collect();
    assert_eq!(&coeff[..5],&[Fr::ZERO,Fr::ONE,Fr::from(2u64),-Fr::ONE,-Fr::from(2u64)]);
    let terms:Vec<_>=(0..nt).map(|_|(r.word(),r.word())).collect();
    let sums:Vec<_>=(0..ns).map(|_|(r.word(),r.word(),r.word())).collect();
    let mut values:Vec<Option<Lc>>=vec![None;nw+nslots];
    for i in 0..ni{values[i]=Some(vec![(i,Fr::ONE)]);}
    let term=|c:usize,w:usize,values:&Vec<Option<Lc>>|scale(values[w].as_ref().unwrap(),coeff[c]);
    let mut assignments=0;let mut hints=0;let mut constant_sides=0;let mut constraints=0;let mut stored=0;let mut sides=[0;3];
    let mut consumed_sums=vec![false;ns];let mut rows=vec![false;nc];
    for _ in 0..nops{
        if r.word()==1{
            let [kind,a,b,start,end]=std::array::from_fn(|_|r.word());assert!(kind<=1);
            let lc=terms[a..b].iter().fold(vec![],|v,&(c,w)|add(v,term(c,w,&values)));
            assert!(constant(&lc).is_none(),"known hint would contradict zero count");
            for w in start..end{assert!(values[w].is_none());values[w]=Some(vec![(w,Fr::ONE)]);}
            hints+=1;
        }else{
            let [row,a,b,first]=std::array::from_fn(|_|r.word());assert!(!rows[row]);rows[row]=true;
            let mut chain=vec![];let mut slot=nw+first;
            for j in a..b{
                assert!(!consumed_sums[j]);consumed_sums[j]=true;
                let(p,c,w)=sums[j];let prior=p&0x7fffffff;
                let value=term(c,w,&values);
                chain=match prior{0x7fffffff=>value,0x7ffffffe=>add(chain,value),w=>add(values[w].clone().unwrap(),value)};
                if p&0x80000000!=0{
                    assert!(constant(&chain).is_none(),"hidden constant stored slot");
                    assert!(values[slot].is_none());values[slot]=Some(chain.clone());slot+=1;stored+=1;
                }
            }
            let expr:[Lc;3]=std::array::from_fn(|_|{
                let[tag,a,b]=std::array::from_fn(|_|r.word());
                match tag{
                    0=>vec![],1=>values[a].clone().unwrap(),2=>normalize(vec![(0,coeff[a])]),
                    3=>{let(c,w)=terms[a];term(c,w,&values)},
                    4=>terms[a..b].iter().fold(vec![],|v,&(c,w)|add(v,term(c,w,&values))),
                    _=>panic!("expression tag")
                }
            });
            let cs=expr.each_ref().map(|v|constant(v));constant_sides+=cs.iter().filter(|x|x.is_some()).count();
            match r.word(){
                0=>{},1=>{
                    let [side,w,c]=std::array::from_fn(|_|r.word());let inv=r.field();assert_eq!(coeff[c]*inv,Fr::ONE);
                    if side==2{assert!(cs[0].is_none()&&cs[1].is_none(),"linear product assignment");}
                    else{assert!(cs[1-side].is_none(),"constant divisor");assert!(!(cs[side]==Some(Fr::ZERO)&&cs[2]==Some(Fr::ZERO)),"zero result for unknown divisor");}
                    assert!(values[w].is_none());values[w]=Some(vec![(w,Fr::ONE)]);assignments+=1;sides[side]+=1;
                },_=>panic!("assignment tag")
            }
            constraints+=1;
        }
    }
    assert_eq!(r.at,data.len());assert!(values.iter().all(Option::is_some));assert!(consumed_sums.iter().all(|x|*x));assert!(rows.iter().all(|x|*x));
    // Every noninput query wire is now certified as an opaque nonlinear
    // assignment or an unknown hint output under this exact abstraction.
    let query_counts=queries.map(|q|{assert!(q.iter().all(|&w|w<nw));q.len()});
    json!({"wires":nw,"inputs":ni,"public":np,"constraints":constraints,"hints":hints,"ops":nops,"sum_records":ns,"stored_slots":stored,"assignments":assignments,"assignment_sides":sides,"constant_expression_sides":constant_sides,"A_points":query_counts[0],"B_points":query_counts[1],"certified_no_affine_assignments":true,"certified_no_constant_hint_outputs":true,"certified_no_constant_stored_slots":true})
}
fn main(){
    let mut results=serde_json::Map::new();
    for shape in ["1x2","2x2"]{
        let input=read(Path::new(ROOT).join("inputs.json"));let file=input[shape]["file"].as_str().unwrap();
        let v=verify(&fs::read(file).unwrap());let original=read(Path::new(ROOT).join(format!("{shape}.json")));
        for k in ["wires","inputs","public","constraints","hints","ops","sum_records","stored_slots","assignments","assignment_sides","constant_expression_sides"]{assert_eq!(v[k],original[k],"{shape}/{k}");}
        assert_eq!(v["A_points"],original["queries"]["A"]["points"]);assert_eq!(v["B_points"],original["queries"]["B"]["points"]);
        assert_eq!(original["expanded_affine_assignments"],0);assert_eq!(original["expanded_constant_assignments"],0);assert_eq!(original["constant_hint_outputs"],0);assert_eq!(original["constant_stored_slots"],0);
        results.insert(shape.into(),v);
    }
    let out=json!({"complete":true,"shared_model_code":false,"representation":"sorted coefficient vectors with opaque ordinary wires after each certified nonlinear assignment","cases":results,"new_solver_exports":0,"new_real_witness_solves":0,"new_timing_samples":0,"new_proofs":0});
    fs::write(Path::new(ROOT).join("review.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("Independent negative-result certificate passed for both immutable programs");
}
