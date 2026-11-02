use std::{fs,path::Path};
fn main(){let r=Path::new("/home/ubuntu/src/zopro-perf-work/paired-fq2-pack176");
    let s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175/native.rs").unwrap().replace("/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175",r.to_str().unwrap()).replace("zopro_r256_truncated","zopro_fq2_pair_pack");
    assert!(!r.join("native.rs").exists());fs::write(r.join("native.rs"),s).unwrap();
}
