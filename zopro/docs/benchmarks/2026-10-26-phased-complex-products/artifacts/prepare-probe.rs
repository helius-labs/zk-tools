use std::{fs,path::Path};
fn main(){let r=Path::new("/tmp/zopro-phased-complex-products");let mut s=fs::read_to_string("/tmp/zopro-complex-shared-product/component.rs").unwrap();
let st=s.find("#[inline(always)]\nfn multiply<const SHARED").unwrap();let en=s[st..].find("\nfn vector(").unwrap()+st;
s.replace_range(st..en,"#[inline(always)]\nfn multiply<const STRATEGY: u8>(a: &Complex, b: &Complex) -> Complex {\n    #[cfg(zopro_phased_complex_products)]\n    {\n      if STRATEGY == 1 {return V::complex_product_shared(&a[0],&a[1],&b[0],&b[1]);}\n      if STRATEGY == 2 {return V::complex_product_phased(&a[0],&a[1],&b[0],&b[1]);}\n    }\n    ordinary(a,b)\n}\n");
s=s.replace("#[cfg(shared_complex)]","#[cfg(zopro_phased_complex_products)]");s=s.replace("fn run<const SHARED: bool>","fn run<const STRATEGY: u8>").replace("multiply::<SHARED>","multiply::<STRATEGY>");
let code="\n#[cfg(zopro_phased_complex_products)]\n#[no_mangle]\n#[inline(never)]\npub fn complex_phased_codegen(a:&Complex,b:&Complex)->Complex {V::complex_product_phased(&a[0],&a[1],&b[0],&b[1])}\n";
let at=s.find("#[inline(never)]\nfn run").unwrap();s.insert_str(at,code);
s=s.replace("black_box(complex_shared_codegen(&a, &b));","{black_box(complex_shared_codegen(&a, &b));black_box(complex_phased_codegen(&a,&b));}");
s=s.replace("component old|shared PATTERN","component old|shared|phased PATTERN");
let st=s.find("    let shared = match").unwrap();let en=s[st..].find("    let pattern =").unwrap()+st;
s.replace_range(st..en,"    let strategy:u8=match args[1].as_str(){\n      \"old\"=>0,\"shared\" if cfg!(zopro_phased_complex_products)=>1,\"phased\" if cfg!(zopro_phased_complex_products)=>2,_=>return Err(\"unsupported mode\".into())\n    };\n");
s=s.replace("if shared { run::<true>(pattern, 2000) } else { run::<false>(pattern, 2000) }","match strategy {0=>run::<0>(pattern,2000),1=>run::<1>(pattern,2000),2=>run::<2>(pattern,2000),_=>unreachable!()}");
s=s.replace("if shared { run::<true>(pattern, iterations) } else { run::<false>(pattern, iterations) }","match strategy {0=>run::<0>(pattern,iterations),1=>run::<1>(pattern,iterations),2=>run::<2>(pattern,iterations),_=>unreachable!()}");
s=s.replace("\\\"shared\\\":{shared}","\\\"strategy\\\":{strategy}");
fs::write(r.join("probe.rs"),s).unwrap();
fs::create_dir(r.join("oracle")).unwrap();let mut cargo=fs::read_to_string("/tmp/zopro-deferred-dif32/oracle/Cargo.toml").unwrap();cargo=cargo.replace("deferred-dif32-probe","phased-complex-probe");fs::write(r.join("oracle/Cargo.toml"),cargo).unwrap();let lock=fs::read_to_string("/tmp/zopro-deferred-dif32/oracle/Cargo.lock").unwrap().replace("deferred-dif32-probe","phased-complex-probe");fs::write(r.join("oracle/Cargo.lock"),lock).unwrap();}
