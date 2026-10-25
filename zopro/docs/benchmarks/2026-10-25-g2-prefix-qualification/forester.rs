use std::{fs,io::Write,path::Path,sync::Arc,time::Instant};
use zopro_bn254::Fr;
use zopro_prover::Prover;
#[global_allocator]static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn event(stage:&str,mut v:serde_json::Value){let mut u=unsafe{std::mem::zeroed::<libc::rusage>()};assert_eq!(unsafe{libc::getrusage(libc::RUSAGE_SELF,&mut u)},0);v["stage"]=stage.into();v["process_peak_rss_kib"]=u.ru_maxrss.into();println!("EVENT {}",v);}
fn map(path:&str)->memmap2::Mmap{let f=fs::File::open(path).unwrap();unsafe{memmap2::Mmap::map(&f)}.unwrap()}
struct Logger;
impl log::Log for Logger {fn enabled(&self,_:&log::Metadata)->bool{true} fn log(&self,r:&log::Record){eprintln!("{}",r.args());}fn flush(&self){}}
static LOGGER:Logger=Logger;
fn main(){log::set_logger(&LOGGER).unwrap();log::set_max_level(log::LevelFilter::Info);let a:Vec<_>=std::env::args().collect();let start=Instant::now();
 if a[1]=="dump-image" {
  event("image_begin",serde_json::json!({"source":"mapped dump", "directory":a[2]}));
  let dir=Path::new(&a[2]);
  let head=zopro_prover::zopro_core::dump::Head::read(dir).unwrap();
  let maps:Vec<_>=["a","b","k","z","b2","program","coefficients"].iter().map(|name|map(dir.join(name).to_str().unwrap())).collect();
  let names=|name:&str|->Vec<String>{fs::read_to_string(dir.join(name)).unwrap().lines().map(str::to_owned).collect()};
  let public_names=names("public-names");let secret_names=names("secret-names");let metadata=fs::read(dir.join("metadata")).unwrap();
  let parts=zopro_prover::zopro_core::artifact::Parts{
   name:&a[3],metadata:&metadata,public:head.public,secret:head.secret,constraints:head.constraints,
   fixed:&head.fixed,domain:&head.domain,verifying_key:&head.verifying_key,public_names:&public_names,secret_names:&secret_names,
   a:&maps[0],b:&maps[1],k:&maps[2],z:&maps[3],b2:&maps[4],program:&maps[5],coefficients:&maps[6],
  };
  let path=Path::new(&a[4]);assert!(!path.exists());let temporary=path.with_extension("partial");
  let file=fs::OpenOptions::new().write(true).create_new(true).open(&temporary).unwrap();
  let mut out=std::io::BufWriter::with_capacity(1<<20,file);
  Prover::write_dump_image(&parts,&mut out).unwrap();out.flush().unwrap();out.get_ref().sync_all().unwrap();
  fs::hard_link(&temporary,path).unwrap();fs::remove_file(&temporary).unwrap();
  event("image_written",serde_json::json!({"seconds":start.elapsed().as_secs_f64(),"bytes":fs::metadata(path).unwrap().len()}));return;
 }
 if a[1]=="pack"{event("pack_begin",serde_json::json!({}));let dump=zopro_prover::zopro_core::dump::Dump::read(Path::new(&a[2])).unwrap();let bytes=zopro_prover::zopro_core::artifact::pack(&dump.parts(&a[3])).unwrap();fs::write(&a[4],bytes).unwrap();event("packed",serde_json::json!({"seconds":start.elapsed().as_secs_f64()}));return;}
 if a[1]=="image"{event("image_begin",serde_json::json!({}));let bytes=map(&a[2]);let mut out=std::io::BufWriter::new(fs::File::create(&a[3]).unwrap());Prover::write_image(&bytes,&mut out).unwrap();out.flush().unwrap();out.get_ref().sync_all().unwrap();event("image_written",serde_json::json!({"seconds":start.elapsed().as_secs_f64()}));return;}
 assert_eq!(a[1],"prove");let rounds:usize=a[5].parse().unwrap();let out=Path::new(&a[4]);fs::create_dir_all(out).unwrap();event("load_begin",serde_json::json!({"path":a[2]}));let mut p=Prover::load_image(Arc::new(map(&a[2]))).unwrap();event("loaded",serde_json::json!({"seconds":start.elapsed().as_secs_f64(),"public":p.public_inputs(),"inputs":p.names().len()}));
 let inputs:Vec<_>=(0..2).map(|i|fs::read(Path::new(&a[3]).join(format!("{i}.witness"))).unwrap()).collect();
 for round in -1..rounds as i32{let variant=((round+1)%2)as usize;event("proof_begin",serde_json::json!({"round":round,"variant":variant}));let t=Instant::now();let b=&inputs[variant];let word=|i|u32::from_be_bytes(b[i..i+4].try_into().unwrap())as usize;assert_eq!(word(0),p.public_inputs());assert_eq!(word(0)+word(4),word(8));assert_eq!(word(8),p.names().len());assert_eq!(b.len(),12+32*word(8));let w:Vec<Fr>=b[12..].chunks_exact(32).map(|v|Fr::from_be_bytes(v.try_into().unwrap()).unwrap()).collect();let decode=t.elapsed().as_secs_f64();let proved=p.prove(&w).unwrap();let proof=proved.proof.to_gnark();let public=proved.public_witness();let seconds=t.elapsed().as_secs_f64();event("proof",serde_json::json!({"round":round,"variant":variant,"verified_seconds":seconds,"decode_seconds":decode,"prove_ms":proved.prove_ms,"verify_ms":proved.verify_ms}));fs::write(out.join(format!("{round}.proof")),proof).unwrap();fs::write(out.join(format!("{round}.public")),public).unwrap();}
 event("complete",serde_json::json!({"rounds":rounds,"prime":1}));
}
