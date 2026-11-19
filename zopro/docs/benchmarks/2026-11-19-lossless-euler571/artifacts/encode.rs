use std::collections::HashMap;
use std::io::{BufReader,BufWriter,Read,Write};
use std::fs::File;
use std::path::Path;
type Word = [u8;32];

fn for_pairs(path:&Path,mut visit:impl FnMut(Word,Word)) -> u64 {
    let mut f=BufReader::new(File::open(path).unwrap());
    let mut word=[0;8];f.read_exact(&mut word).unwrap();
    let n=u64::from_le_bytes(word);
    assert_eq!(f.get_ref().metadata().unwrap().len(),8+64*n);
    for _ in 0..n {
        let mut a=[0;32];let mut b=[0;32];
        f.read_exact(&mut a).unwrap();f.read_exact(&mut b).unwrap();visit(a,b);
    }
    n
}
fn intern<T:Copy+Eq+std::hash::Hash>(v:T, ids:&mut HashMap<T,u32>, values:&mut Vec<T>)->u32{
    *ids.entry(v).or_insert_with(|| {let id=u32::try_from(values.len()).unwrap();values.push(v);id})
}
fn width(len:usize)->usize{if len<=256 {1} else if len<=65536{2}else{4}}
fn main(){
    let args:Vec<_>=std::env::args_os().skip(1).collect();assert_eq!(args.len(),2);
    let raw=Path::new(&args[0]);let coded=Path::new(&args[1]);
    let (mut word_ids,mut words)=(HashMap::new(),Vec::new());
    let (mut pair_ids,mut pairs)=(HashMap::new(),Vec::new());
    intern([0;32],&mut word_ids,&mut words);intern((0,0),&mut pair_ids,&mut pairs);
    let n=for_pairs(raw,|a,b|{
        let i=intern(a,&mut word_ids,&mut words);let j=intern(b,&mut word_ids,&mut words);
        intern((i,j),&mut pair_ids,&mut pairs);
    });
    let fw=width(words.len());let pw=width(pairs.len());
    let mut out=BufWriter::new(File::create(coded).unwrap());
    out.write_all(b"EUL2").unwrap();out.write_all(&[fw as u8,pw as u8,0,0]).unwrap();out.write_all(&n.to_le_bytes()).unwrap();
    for size in [words.len(),pairs.len()] {out.write_all(&u32::try_from(size).unwrap().to_le_bytes()).unwrap();}
    for field in &words {out.write_all(field).unwrap();}
    for &(i,j) in &pairs {for k in [i,j]{out.write_all(&k.to_le_bytes()[..fw]).unwrap();}}
    for_pairs(raw,|a,b|{let id=pair_ids[&(word_ids[&a],word_ids[&b])];out.write_all(&id.to_le_bytes()[..pw]).unwrap();});
    out.flush().unwrap();
    println!("{n} {} {} {fw} {pw}",words.len(),pairs.len());
}
