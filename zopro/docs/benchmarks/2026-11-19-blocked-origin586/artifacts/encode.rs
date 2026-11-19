use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter,Write};
use std::path::Path;
use zopro_bn254::Fr;

type Pair = [u8;64];
fn main() {
    let args:Vec<_>=std::env::args_os().skip(1).collect();
    let [input,output,decoded]=&args[..] else {panic!("input, output, decoded")};
    let input=fs::read(input).unwrap();
    let rows=u64::from_le_bytes(input[..8].try_into().unwrap());
    assert_eq!(input.len() as u64,8+64*rows);
    let mut ids=HashMap::<Pair,u32>::new();
    let mut pairs=vec![[0u8;64]];
    ids.insert(pairs[0],0);
    let mut descriptors=Vec::new();
    let mut exceptions=Vec::new();
    let mut uniform=0;let mut untouched=0;let mut common_zero=0;
    for block in input[8..].chunks(64*64) {
        let mut lanes=[0;64];
        // At most 64 distinct IDs. The first occurrence wins a frequency tie.
        let mut counts=[(0u32,0u8);64];let mut used=0;
        for (j,bytes) in block.chunks_exact(64).enumerate(){
            let pair:Pair=bytes.try_into().unwrap();
            let id=*ids.entry(pair).or_insert_with(||{let i=u32::try_from(pairs.len()).unwrap();pairs.push(pair);i});
            lanes[j]=id;
            let pos=counts[..used].iter().position(|c|c.0==id).unwrap_or_else(||{counts[used]=(id,0);used+=1;used-1});
            counts[pos].1+=1;
        }
        let n=block.len()/64;
        let mut common=counts[0];
        for &entry in &counts[1..used]{ if entry.1>common.1{common=entry;} }
        let start=exceptions.len() as u64;
        let mut mask=0u64;
        for (j,&id) in lanes[..n].iter().enumerate(){
            if id!=common.0{mask|=1u64<<j;exceptions.push(id);}
        }
        descriptors.push((start,mask,common.0));
        if mask==0 {uniform+=n; if common.0==0{untouched+=n;} }
        if common.0==0 {common_zero+=n;}
    }
    let width=if pairs.len()<=256 {1usize} else if pairs.len()<=65536 {2}else{4};
    let mut file=BufWriter::new(File::create(output).unwrap());
    file.write_all(b"EUB1").unwrap();file.write_all(&[width as u8,0,0,0]).unwrap();file.write_all(&rows.to_le_bytes()).unwrap();
    file.write_all(&u32::try_from(pairs.len()).unwrap().to_le_bytes()).unwrap();file.write_all(&64u32.to_le_bytes()).unwrap();
    for p in &pairs {for s in p.chunks_exact(32){
        let value=Fr::from_be_bytes(s.try_into().unwrap()).expect("canonical field");
        for word in value.0{file.write_all(&word.to_le_bytes()).unwrap();}
    }}
    for (start,mask,default) in &descriptors{
        file.write_all(&start.to_le_bytes()).unwrap();file.write_all(&mask.to_le_bytes()).unwrap();
        file.write_all(&default.to_le_bytes()).unwrap();file.write_all(&[0;4]).unwrap();
    }
    for &id in &exceptions{file.write_all(&id.to_le_bytes()[..width]).unwrap();}
    file.flush().unwrap();drop(file);

    let coded=fs::read(output).unwrap();let pairs_end=24+64*pairs.len();let exceptions_at=pairs_end+24*descriptors.len();
    let u64_at=|i|u64::from_le_bytes(coded[i..i+8].try_into().unwrap());
    let mut rebuilt=BufWriter::new(File::create(Path::new(decoded)).unwrap());rebuilt.write_all(&rows.to_le_bytes()).unwrap();
    let mut cursor=0;
    for i in 0..usize::try_from(rows).unwrap(){
        let base=pairs_end+24*(i/64);let lane=i%64;let mask=u64_at(base+8);let bit=1u64<<lane;
        if lane==0{assert_eq!(u64_at(base),cursor);}
        let id=if mask&bit==0{u32::from_le_bytes(coded[base+16..base+20].try_into().unwrap())}
        else{
            let pos=u64_at(base)+u64::from((mask&(bit-1)).count_ones());assert_eq!(pos,cursor);cursor+=1;
            let offset=exceptions_at+width*(pos as usize);let mut id=[0;4];id[..width].copy_from_slice(&coded[offset..offset+width]);u32::from_le_bytes(id)
        };
        let start=24+id as usize*64;
        for j in [0,32] {
            let words=std::array::from_fn(|n|u64_at(start+j+8*n));
            let bytes=Fr::from_montgomery(words).to_be_bytes();
            assert_eq!(bytes.as_slice(),&input[8+64*i+j..8+64*i+j+32]);rebuilt.write_all(&bytes).unwrap();
        }
    }
    assert_eq!(coded.len(),exceptions_at+width*(cursor as usize));rebuilt.flush().unwrap();
    println!("{rows} {} {width} {} {} {uniform} {untouched} {common_zero} {}",
             pairs.len(),descriptors.len(),exceptions.len(),coded.len());
}
