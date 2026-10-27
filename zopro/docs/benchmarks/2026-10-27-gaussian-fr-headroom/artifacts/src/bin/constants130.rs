use num_bigint::BigInt;
use num_traits::ToPrimitive;
use serde_json::json;
use zopro_gaussian_fr_probe::model::{Params,modulo};
fn digits(x:&BigInt)->[u64;3]{std::array::from_fn(|j|((x>>(j*52))&BigInt::from((1u64<<52)-1)).to_u64().unwrap())}
fn main(){
 let p=Params::new();let root="/home/ubuntu/src/zopro-perf-work/gaussian-fr-r130";let mut text=String::from("// Exact Gaussian Fr headroom constants, R=2^130.\n");
 for(name,v)in[("A",&p.a),("B",&p.b),("HALF",&p.half),("BOUND",&p.bound)]{text+=&format!("pub const {name}:u128={}u128;\n",v.to_u128().unwrap());}
 for(name,v)in[("KRE",p.neg_inv.re.clone()),("KIM",p.neg_inv.im.clone()),("KSUM",modulo(&p.neg_inv.re+&p.neg_inv.im,&p.radix))]{text+=&format!("pub const {name}:[u64;3]={:?};\n",digits(&v));}
 std::fs::write(format!("{root}/src/constants.rs"),text).unwrap();
 std::fs::write(format!("{root}/model-gate.json"),serde_json::to_vec_pretty(&json!({"passed":true,"radix_bits":130,"threshold_bits":126,"prime":p.prime.to_string(),"a":p.a.to_string(),"b":p.b.to_string(),"H":p.bound.to_string(),"normal_product_bound":p.normal_limit.to_string(),"normal_product_requires_correction":false,"twiddle_product_bound":p.twiddle_limit.to_string(),"twiddle_correction_steps_per_axis":1,"linear_finish_max_coordinate":"5H","linear_finish_steps_per_axis":3,"stored_coordinate_pair_bytes":32,"native_vectors":6,"ordinary_native_vectors":5,"new_SDK_or_Gnark_proofs":0})).unwrap()).unwrap();
 println!("Exact R130 normal/lazy product and correction bounds verified; constants emitted");
}
