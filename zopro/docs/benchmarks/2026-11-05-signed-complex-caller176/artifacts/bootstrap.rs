use std::fs;
fn main(){let old="/home/ubuntu/src/zopro-perf-work/absolute-norm176";let new="/home/ubuntu/src/zopro-perf-work/signed-complex-caller176";let text=fs::read_to_string(format!("{old}/native.rs")).unwrap().replace(old,new).replace("zopro_absolute_norm","zopro_signed_complex_square");fs::write(format!("{new}/native.rs"),text).unwrap();}
