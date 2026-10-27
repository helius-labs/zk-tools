#[path="model130.rs"]
pub mod model;
pub mod constants;
#[cfg(all(target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512dq"))]
pub mod native;
#[cfg(all(target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512dq"))]
pub mod experiment;
