use std::fs;
fn main() {
    let old = "/tmp/zopro-g2-tree-sdk";
    let new = "/tmp/zopro-current-c7a-20261027";
    for file in ["cloud.rs", "transfer.rs", "sigv4.rs"] {
        let source = fs::read_to_string(format!("{old}/{file}")).unwrap();
        let mut s = source
            .replace(old, new)
            .replace("zopro-g2-tree-sdk", "zopro-current-c7a");
        if file == "cloud.rs" {
            s = s
                .replace(
                    "im[\"local_gnark_proofs\"],41",
                    "im[\"local_gnark_proofs\"],27",
                )
                .replace(
                    "im[\"local_preflight_sdk\"],164",
                    "im[\"local_preflight_sdk\"],108",
                )
                .replace("local41Gnark/164SDK", "local27Gnark/108SDK")
                .replace("/usr/local/bin/gs-", "/usr/local/bin/cr-")
                .replace(
                    "--memory 30g --pids-limit",
                    "--memory 30g --memory-swap 30g --pids-limit",
                );
        }
        fs::write(format!("{new}/{file}"), s).unwrap();
    }
    let source = fs::read_to_string(format!("{old}/finalize-image.rs")).unwrap();
    let mut s = source
        .replace(old, new)
        .replace("zopro-g2-tree-sdk:preflight", "zopro-current-c7a:preflight")
        .replace("gs-c7a-", "cr-c7a-")
        .replace("/gs/preflight-complete.json", "/cr/preflight-complete.json")
        .replace("41", "27")
        .replace("164", "108")
        .replace("inventory.as_object().unwrap().len(),82", "inventory.as_object().unwrap().len(),54")
        .replace("route_checks_passed", "all_fixture_checks_passed");
    let oldfield =
        "\"route_analysis_sha256\":hash(r.join(\"local-output/preflight/route-analysis.json\")),";
    assert_eq!(s.matches(oldfield).count(), 1);
    s = s.replace(oldfield, "");
    fs::write(format!("{new}/finalize-image.rs"), s).unwrap();
    println!("Prepared inherited owned AWS/transport lifecycle for current comparison;no publication or provisioning");
}
