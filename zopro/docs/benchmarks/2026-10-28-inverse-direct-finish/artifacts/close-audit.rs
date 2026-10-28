use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn walk(p: &Path, root: &Path, m: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, root, m)
        } else if p.file_name().unwrap() != "SHA256SUMS" {
            m.insert(
                p.strip_prefix(root).unwrap().display().to_string(),
                hash(&p),
            );
        }
    }
}
fn main() {
    let d = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-inverse-direct-finish",
    );
    assert!(!Path::new("analysis-before-control-audit.json").exists());
    fs::copy("analysis.json", "analysis-before-control-audit.json").unwrap();
    let mut a: Value = serde_json::from_slice(&fs::read("analysis.json").unwrap()).unwrap();
    a["status"] = json!(
        "arithmetic checked; primitive timing comparison unqualified by duplicate ON control"
    );
    a["original_checks_before_identity_audit"] = a["failed_checks"].take();
    a["failed_checks"]=json!(["intended fresh OFF control actually linked ON library; identical fresh/candidate executables"]);
    a["timing_qualification_passed"] = json!(false);
    a["algorithm_speed_qualified"] = json!(false);
    a["algorithm_rejected_by_timing"] = json!(false);
    a["control_identity_audit"] = json!("control-identity-audit.json");
    a["successful_timing_repeated"] = json!(false);
    fs::write("analysis.json", serde_json::to_vec_pretty(&a).unwrap()).unwrap();
    let p = d.join("README.md");
    let old = fs::read_to_string(&p).unwrap();
    let mut s=old.replace("The candidate **rejected primitive screen**. These are complete inversion primitive measurements, not proving times. Production and the accepted SDK are unchanged.","The arithmetic checks pass, but the timing comparison is **unqualified**: the intended fresh OFF control actually linked the ON library. The candidate is not adopted, and the identical ON/ON comparison does not reject the arithmetic idea. Production and the accepted SDK are unchanged.");
    assert_ne!(s, old);
    let key = "The current binary-GCD inverse";
    let i = s.find(key).unwrap();
    s.insert_str(i,"The post-capture identity audit found that Cargo's primary-target `target/release/libzopro_bn254.rlib` path was overwritten by the later candidate build before the probes linked. Both fresh and candidate executables have SHA-256 `79343e35d9ae23a0f841965b0fb90d50ba775434261e7f602f97472576c8f955`. Their selected library hash is also identical. The capture pins correctly describe those bytes, but the harness did not enforce the intended different identities. This is a harness selection error, not a mathematical failure. The [control audit](artifacts/control-identity-audit.json) recovers distinct immutable OFF/ON libraries through the hashed `.rmeta` companion paths and checks their Cargo flags. A corrected OFF probe links successfully with a different hash; it was not executed. No completed test or timing was repeated. A subsequent actual-caller study must enforce immutable library paths and distinct OFF/ON hashes before capture.\n\n");
    s = s.replace(
        "accepted-library/fresh-OFF/selected-ON probes",
        "accepted-library, intended-fresh (actually ON), and selected-ON probes",
    );
    s = s.replace(
        "| Fresh ns/element |",
        "| Intended fresh, actually ON ns/element |",
    );
    s = s.replace(
        "| Paired ratio | Median |",
        "| Archived label ratio | Median |",
    );
    let i = s.find("The 10th–22nd ordered ratios").unwrap();
    s.insert_str(i,"Labels in the tables preserve the original process names: every `fresh` row is a second ON measurement. Candidate/fresh ratios therefore cannot estimate an ON/OFF effect. Candidate/accepted Fq paired median ratios are 0.989356 scalar and 0.988243 batch, but the missing OFF arm prevents attributing those differences to this arithmetic change. No gain is qualified. Fq process times also change sharply around block 15; that bimodal distribution makes ratios of independently displayed medians misleading. All process means and paired observations are retained, without assigning a cause or excluding blocks. The original statistical checks below remain as declared, not as valid ON/OFF conclusions.\n\n");
    s=s.replace("Passing only permits a separate caller/full-SDK qualification, not adoption.","The missing OFF arm prevents timing qualification. The next useful check is a separately declared actual-caller study with audited immutable OFF/ON libraries; this completed primitive schedule will not be rerun.");
    fs::write(&p, s).unwrap();
    for f in [
        "analysis.json",
        "analysis-before-control-audit.json",
        "control-identity-audit.json",
        "audit-controls.rs",
        "audit-controls-build.log",
        "audit-controls.log",
        "audit-controls-error.log",
        "corrected-fresh-link-command.json",
        "corrected-fresh-link-stdout.log",
        "corrected-fresh-link-stderr.log",
        "close-audit.rs",
    ] {
        fs::copy(f, d.join("artifacts").join(f)).unwrap();
    }
    let mut index = BTreeMap::new();
    walk(d, d, &mut index);
    fs::write(
        d.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!(
        "Closedcontrolidentityqualificationfailure;{}hashes;noSDK/noadoption;ledger12785",
        index.len()
    );
}
