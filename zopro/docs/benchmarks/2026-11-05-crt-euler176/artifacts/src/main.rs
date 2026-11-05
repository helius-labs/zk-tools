use zopro_crt_euler_study::*;use zopro_bn254::Fr;use serde_json::json;
fn main(){
 let n=32768;let primes=choose_primes(n);let context=Context::new(n,&primes);
 let ac=vec![Fr::from(9u64),-Fr::from(5u64),Fr::from(123u64),Fr::ONE];
 let bc=vec![-Fr::from(37u64),Fr::from(29u64),-Fr::ONE];
 let(a,da)=polynomial_values(&ac,n);let(b,db)=polynomial_values(&bc,n);
 let expected=da.iter().zip(&db).zip(a.iter().zip(&b)).map(|((&da,&db),(&a,&b))|(da*b+a*db)*Fr::from(n as u64)).collect::<Vec<_>>();
 let actual=context.product(&a,&b);assert_eq!(actual,expected);assert_eq!(actual.len(),n);
 let(k,t)=context.known_payloads();
 let out=json!({"complete":true,"domain":n,"auxiliary_primes":primes,"prime_count":context.prime_count(),"modulus_product_bits":context.modulus_product_bits(),"signed_coefficient_bound_bits":coefficient_bound(n).bits(),"public_kernel_transform_u32_payload_bytes":k,"Fr_twist_and_restore_payload_bytes":t,"plan_root_tables_and_BigUint_allocations_not_in_these_payloads":true,"full_operator_coordinates_checked":n,"independent_reference":"low-degree coefficients differentiated then evaluated independently at every domain point","new_latency_claim":false,"new_SDK_requests":0,"new_proofs":0,"proof_ledger":29953,"production_changed":false,"adopted":false,"goal_achieved":false});
 std::fs::write("/home/ubuntu/src/zopro-perf-work/crt-euler176/algebra.json",serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("{out}");
}
