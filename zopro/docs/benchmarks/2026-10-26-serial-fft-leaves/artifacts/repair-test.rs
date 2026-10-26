use std::{fs,path::Path};
fn main(){let root=Path::new("/tmp/zopro-serial-fft-leaves");let p=root.join("overlay-v2/crates/zopro-core/src/fft/pair/serial_leaf.rs");let old=fs::read_to_string(&p).unwrap();let empty="plan.roots.columns.clear();plan.inverse_roots.columns.clear();";let fresh=r#"let make_roots=|base:Fr| {
                        let mut value=Fr::ONE;
                        Twiddles::new((0..n/2).map(|_|{let out=value;value*=base;out}).collect())
                    };
                    // Replace the whole object so its allocation and admission
                    // token are both released. Clearing only columns kept credit.
                    plan.roots=make_roots(root);
                    plan.inverse_roots=make_roots(root.inverse().unwrap());"#;
assert!(old.contains(empty));let old_cache=r#"plan.roots.prepare_columns();plan.inverse_roots.prepare_columns();
                        assert!(!plan.roots.columns.is_empty()&&!plan.inverse_roots.columns.is_empty());
                        check::<PackedPair>(&plan,root,false,61);"#;let new_cache=r#"// Each builder's peak may consume the entire existing optional
                        // budget. Exercise admitted directions one at a time.
                        plan.roots.prepare_columns();
                        assert!(!plan.roots.columns.is_empty());
                        check::<PackedPair>(&plan,root,false,61);
                        plan.roots=make_roots(root);
                        plan.inverse_roots.prepare_columns();
                        assert!(!plan.inverse_roots.columns.is_empty());
                        check::<PackedPair>(&plan,root,false,79);"#;assert!(old.contains(old_cache));let new=old.replace(empty,fresh).replace(old_cache,new_cache);assert_eq!(old.split("#[cfg(test)]").next(),new.split("#[cfg(test)]").next());fs::write(p,new).unwrap();println!("Only cfg(test) cache fixtures changed; runtime module prefix identical");}
