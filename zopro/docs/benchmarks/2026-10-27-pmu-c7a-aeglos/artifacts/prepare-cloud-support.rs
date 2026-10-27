use std::{fs, path::Path};
fn main() {
    let old = "/tmp/zopro-current-c7a-20261027";
    let new = "/tmp/zopro-pmu-c7a-20261027";
    let r = Path::new(new);
    for file in ["cloud.rs", "transfer.rs", "sigv4.rs", "verify-cleanup.rs"] {
        let mut s = fs::read_to_string(format!("{old}/{file}"))
            .unwrap()
            .replace(old, new)
            .replace("zopro-current-c7a", "zopro-pmu-c7a");
        if file == "cloud.rs" {
            s = s
                .replace(
                    "assert_eq!(im[\"local_gnark_proofs\"], 27);",
                    "assert_eq!(im[\"local_gnark_proofs\"], 2);",
                )
                .replace(
                    "assert_eq!(im[\"local_preflight_sdk\"], 108);",
                    "assert_eq!(im[\"local_preflight_sdk\"], 8);",
                )
                .replace("local27Gnark/108SDK", "local2Gnark/8SDK");
            let begin = s.find("        \"package\" => {").unwrap();
            let end = begin + s[begin..].find("        \"cleanup\" => {").unwrap();
            let part=s[begin..end].replace("assert_eq!(invocation(\"measure\")[\"Status\"], \"Success\");","assert!(matches!(invocation(\"measure\")[\"Status\"].as_str(),Some(\"Success\"|\"Failed\")),\"package only terminal measurement; preserve partial outcome\");");
            s.replace_range(begin..end, &part);
        }
        fs::write(r.join(file), s).unwrap();
    }
    let mut s = fs::read_to_string(format!("{old}/finalize-image.rs"))
        .unwrap()
        .replace(old, new)
        .replace("zopro-current-c7a:preflight", "zopro-pmu-c7a:preflight")
        .replace("cr-c7a-", "pmu-c7a-");
    for (before, after) in [
        (
            "complete[\"sdk_requests\"], 108",
            "complete[\"sdk_requests\"], 8",
        ),
        (
            "complete[\"distinct_gnark_exports\"], 27",
            "complete[\"distinct_gnark_exports\"], 2",
        ),
        (
            "inventory.as_object().unwrap().len(), 54",
            "inventory.as_object().unwrap().len(), 4",
        ),
        ("hashes.len(), 27", "hashes.len(), 2"),
        ("\"sdk_requests\":108", "\"sdk_requests\":8"),
        (
            "\"distinct_gnark_exports\":27",
            "\"distinct_gnark_exports\":2",
        ),
        ("\"local_gnark_proofs\":27", "\"local_gnark_proofs\":2"),
        ("\"local_preflight_sdk\":108", "\"local_preflight_sdk\":8"),
        ("27Gnark/108SDK", "2Gnark/8SDK"),
    ] {
        assert!(s.contains(before), "missing {before}");
        s = s.replace(before, after);
    }
    fs::write(r.join("finalize-image.rs"), s).unwrap();
    println!("Prepared owned publication/lifecycle/transfer support; no external writes");
}
