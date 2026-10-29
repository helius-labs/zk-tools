use num_bigint::{BigInt,BigUint};use num_traits::{One,Zero,ToPrimitive};use serde_json::json;
type U=BigUint;
fn digits(n:&U)->[i128;5]{core::array::from_fn(|i|((n>>(52*i))&U::from((1u64<<52)-1)).to_i128().unwrap())}
fn integer(t:&[i128])->BigInt{t.iter().enumerate().map(|(i,&v)|BigInt::from(v)<<(52*i)).sum()}
fn run(decimal:&str,bits:usize,conj:bool)->serde_json::Value{
    let p=U::parse_bytes(decimal.as_bytes(),10).unwrap();let r=U::one()<<bits;let twice=&p*2u32;assert!(&p*4u32<r);
    let mut inverse=U::one();for _ in 0..9{inverse=(&inverse*(&r+2u32-(&p*&inverse)%&r))%&r;}assert_eq!(&inverse*&p%&r,U::one());let ni=&r-inverse;
    let inv=(&ni&U::from((1u64<<52)-1)).to_i128().unwrap();let pd=digits(&p);let two=digits(&twice);let mask=(1i128<<52)-1;
    let edges=[U::zero(),U::one(),&p-1u32,p.clone(),&p+1u32,&twice-1u32,twice.clone()];
    let mut seed=0x53545245414d4e45u64;
    let mut next=||{let mut b=[0u8;64];for c in b.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;c.copy_from_slice(&seed.to_le_bytes());}U::from_bytes_le(&b)%(&twice+1u32)};
    let mut inputs=Vec::<[U;4]>::new();for i in 0..2401{inputs.push(core::array::from_fn(|j|edges[(i/7usize.pow(j as u32))%7].clone()));}for _ in 0..4096{inputs.push(core::array::from_fn(|_|next()));}
    for i in 0usize..5 {for v in [U::one()<<(52*i), (U::one()<<(52*i))-1u32]{if v<=twice {inputs.push([v.clone(),twice.clone(),v.clone(),v]);}}}
    let mut chain:[U;4]=core::array::from_fn(|_|next());let mut maximum=0u128;let mut count=0u64;
    for i in 0..inputs.len()+1024 {
        let a=if i<inputs.len(){inputs[i].clone()}else{chain.clone()};let ad=a.each_ref().map(digits);let negative=&twice-&a[3];let nd=digits(&negative);
        let wide=if conj{[&a[0]*&a[2]+&a[1]*&a[3],&a[1]*&a[2]+&a[0]*&negative]}else{[&a[0]*&a[2]+&a[1]*&negative,&a[0]*&a[3]+&a[1]*&a[2]]};
        assert!(wide.iter().all(|w|w<=&(&p*&p*8u32)));let mut t=[[0i128;6];2];let mut q=[U::zero(),U::zero()];let mut borrow=0;
        for row in 0..5 {
            let v=two[row]-ad[3][row]+borrow;borrow=v>>52;let n=v&mask;assert_eq!(n,nd[row]);
            for j in 0..5 {
                let products=if conj{[[ad[0][j]*ad[2][row],ad[1][j]*ad[3][row]],[ad[1][j]*ad[2][row],ad[0][j]*n]]}else{[[ad[0][j]*ad[2][row],ad[1][j]*n],[ad[0][j]*ad[3][row],ad[1][j]*ad[2][row]]]};
                for k in 0..2 {for v in products[k]{t[k][j]+=v&mask;t[k][j+1]+=v>>52;}}
            }
            let width=if row==4&&bits==256{48}else{52};let rm=(1i128<<width)-1;
            for k in 0..2 {
                let m=((t[k][0]&mask)*inv)&rm;q[k]+=U::from(m as u128)<<(52*row);
                let low=((t[k][0]+rm)>>width)+(((m<<(52-width))*pd[0])>>52);
                for j in 1..5{let v=m*pd[j];t[k][j]+=v&mask;t[k][j+1]+=v>>52;}
                for &v in &t[k]{assert!((0..1i128<<61).contains(&v));maximum=maximum.max(v as u128);}
                if width==52{t[k][1]+=low;t[k]=[t[k][1],t[k][2],t[k][3],t[k][4],t[k][5],0];}
                else{t[k]=[low+((t[k][1]&rm)<<4),(t[k][1]>>48)+((t[k][2]&rm)<<4),(t[k][2]>>48)+((t[k][3]&rm)<<4),(t[k][3]>>48)+((t[k][4]&rm)<<4),(t[k][4]>>48)+(t[k][5]<<4),0];}
            }
        }
        assert_eq!(borrow,0);
        let outputs:[U;2]=core::array::from_fn(|k|{
            assert_eq!(&q[k],&(&wide[k]*&ni%&r));let full=&wide[k]+&q[k]*&p;assert!((&full%&r).is_zero());let exact=&full>>bits;
            assert_eq!(integer(&t[k][..5]),BigInt::from(exact.clone()));assert!(exact<&p*3u32);
            for j in 0..4{t[k][j+1]+=t[k][j]>>52;t[k][j]&=mask;}assert_eq!(integer(&t[k][..5]),BigInt::from(exact.clone()));
            let out=if bits==256&&exact>=twice{exact-&twice}else{exact};assert!(out<twice);out
        });
        chain=[outputs[0].clone(),outputs[1].clone(),a[0].clone(),a[1].clone()];count+=1;
    }
    json!({"bits":bits,"conjugate":conj,"cases":count,"streamed_complement_exact":true,"inclusive2p":true,"max_unsigned_coefficient":maximum.to_string()})
}
fn main(){let mut rows=Vec::new();for p in ["21888242871839275222246405745257275088696311157297823662689037894645226208583","21888242871839275222246405745257275088548364400416034343698204186575808495617"]{for bits in[256,260]{for conj in[false,true]{rows.push(run(p,bits,conj));}}}let total:u64=rows.iter().map(|r|r["cases"].as_u64().unwrap()).sum();let v=json!({"status":"passed","exact_complex_CIOS_cases":total,"rows":rows,"SDK_requests":0,"new_proofs":0,"proof_ledger":22144});std::fs::write("/home/ubuntu/src/zopro-perf-work/interleaved-complex175/model.json",serde_json::to_vec_pretty(&v).unwrap()).unwrap();println!("{v}");}
