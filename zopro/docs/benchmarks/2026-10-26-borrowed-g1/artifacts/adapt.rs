use std::{fs,path::Path};
const R:&str="/tmp/zopro-borrowed-g1/candidate/crates/zopro-core/src";
fn main(){let r=Path::new(R);let p=r.join("msm_affine/borrowed_pending.rs");let old=fs::read_to_string(&p).unwrap();let at=old.find("#[cfg(test)] mod tests").unwrap();
 let head=old[..at].replace("use zopro_bn254::{Field, Fr, G1Affine, G1Config, G1Projective};","use zopro_bn254::{Affine, CurveConfig, Field, Fq, Fr, Projective};")
 .replace("struct Window<'a>","struct Window<'a, C: CurveConfig<Base=Fq>>")
 .replace("impl<'a> Window<'a>","impl<'a, C: CurveConfig<Base=Fq>> Window<'a,C>")
 .replace("G1Affine","Affine<C>").replace("G1Projective","Projective<C>").replace("G1Config","C")
 .replace("Affine<C>::","Affine::<C>::").replace("Projective<C>::","Projective::<C>::")
 .replace("fn fused(","fn fused<C: CurveConfig<Base=Fq>>(");
 let tests=old[at..].replace("    use super::*;","    use super::*;\n    use zopro_bn254::{G1Affine,G1Projective};");
 fs::write(p,format!("{head}\n{}\n{tests}",fs::read_to_string("/tmp/zopro-borrowed-g1/ordinary.rs").unwrap())).unwrap();
 let p=r.join("msm_affine.rs");let old=fs::read_to_string(&p).unwrap();let old=old.replace("mod borrowed_pending;","pub(super) mod borrowed_pending;");
 let marker="where C::Base:BatchInverse {\n    let part = bases.len().div_ceil(parts);";
 assert_eq!(old.matches(marker).count(),1);
 let new=old.replace(marker,"where C::Base:BatchInverse {\n    if let Some(result)=C::Base::borrowed_msm(bases,scalars,parts,width){return result;}\n    merge_split_width_owned(bases,scalars,parts,width)\n}\nfn merge_split_width_owned<C:CurveConfig>(bases:&[Affine<C>],scalars:&[[u64;4]],parts:usize,width:usize)->Projective<C>\nwhere C::Base:BatchInverse {\n    let part = bases.len().div_ceil(parts);");fs::write(p,new).unwrap();
 let p=r.join("msm.rs");let old=fs::read_to_string(&p).unwrap();
 let old=old.replace("    use std::borrow::Borrow;\n","");
 let old=old.replace("pub(super) trait BatchInverse: Field {","pub(super) trait BatchInverse: Field {\n    fn borrowed_msm<C:CurveConfig<Base=Self>>(_bases:&[Affine<C>],_scalars:&[[u64;4]],_parts:usize,_width:usize)->Option<Projective<C>>{None}");
 let marker="impl BatchInverse for Fq {";assert_eq!(old.matches(marker).count(),1);
 let new=old.replace(marker,"impl BatchInverse for Fq {\n    #[cfg(all(feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\n    fn borrowed_msm<C:CurveConfig<Base=Self>>(bases:&[Affine<C>],scalars:&[[u64;4]],parts:usize,width:usize)->Option<Projective<C>>{\n        if crate::msm_affine::borrowed_pending::ordinary_enabled() && (1024..65536).contains(&bases.len()) && (1..=2).contains(&parts) && (10..=14).contains(&width) {\n            Some(crate::msm_affine::borrowed_pending::msm(bases,scalars,parts,width))\n        }else{None}\n    }");fs::write(p,new).unwrap();
}
