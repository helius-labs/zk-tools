//! The capture-helper prepare mode performs the complete preparation and pin review.
//! Run only after all native, portable, source-review and publication helpers exist.
fn main(){let r="/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175";assert!(std::process::Command::new(format!("{r}/capture-helper")).arg("prepare").status().unwrap().success());}
