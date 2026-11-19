use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

type Pair = [u8; 64];

fn plurality(training: &[Pair]) -> Pair {
    // Ties take the earliest training observation, including a four-way tie.
    let mut best = (training[0], 0);
    for row in training {
        let votes = training.iter().filter(|other| *other == row).count();
        if votes > best.1 { best = (*row, votes); }
    }
    best.0
}

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let build = args[0] == "build";
    assert!(build || args[0] == "select");
    let files = if build { &args[2..] } else { &args[1..] };
    assert_eq!(files.len(), if build {4} else {7});
    let mut readers: Vec<_> = files.iter().map(|p| BufReader::new(File::open(p).unwrap())).collect();
    let mut n = None;
    for r in &mut readers {
        let mut head = [0; 8]; r.read_exact(&mut head).unwrap();
        let len = u64::from_le_bytes(head) as usize;
        assert_eq!(*n.get_or_insert(len), len);
    }
    let n = n.unwrap();
    assert!(n >= 64);
    let mut out = build.then(|| BufWriter::new(File::create_new(&args[1]).unwrap()));
    if let Some(f) = &mut out { f.write_all(&(n as u64).to_le_bytes()).unwrap(); }
    let mut rows = vec![[0u8; 64]; readers.len()];
    let mut sampled = [0;6];
    let mut whole = [0;6];
    let mut next = 0;
    for i in 0..n {
        for (row, file) in rows.iter_mut().zip(&mut readers) { file.read_exact(row).unwrap(); }
        if let Some(f) = &mut out { f.write_all(&plurality(&rows)).unwrap(); }
        else {
            let sample = next<64 && i==(2*next+1)*n/128;
            next += usize::from(sample);
            for (j, value) in rows[1..].iter().enumerate() {
                if value==&rows[0] { whole[j]+=1; sampled[j]+=usize::from(sample); }
            }
        }
    }
    for r in &mut readers { assert_eq!(r.read(&mut [0]).unwrap(),0); }
    if let Some(f) = &mut out { f.flush().unwrap(); }
    else {
        assert_eq!(next,64);
        for count in sampled.into_iter().chain(whole) {print!("{count} ");}
        println!();
    }
}

#[test]
fn exact_votes() {
    let a=[1;64]; let b=[2;64]; let c=[3;64]; let d=[4;64];
    assert_eq!(plurality(&[a,b,c,d]),a);
    assert_eq!(plurality(&[a,b,b,c]),b);
    assert_eq!(plurality(&[a,b,b,a]),a);
    assert_eq!(plurality(&[a,c,c,c]),c);
}
