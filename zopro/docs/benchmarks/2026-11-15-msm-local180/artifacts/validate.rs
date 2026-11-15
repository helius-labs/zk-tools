// Adapted from the qualified full-request preflight reader; transformations retained.
use super::*;
pub fn inspect(name: &str) -> Value {
    let jobs = read(format!("{R}/jobs.json"));
    let j = jobs
        .as_array()
        .unwrap()
        .iter()
        .find(|j| j["name"] == name)
        .unwrap();
    let run = format!("{R}/run/{name}");
    let exit = read(format!("{run}/exit.json"));
    assert_eq!(exit["success"], true);
    let stdout = fs::read_to_string(format!("{run}/stdout.log")).unwrap();
    let stderr = fs::read_to_string(format!("{run}/stderr.log")).unwrap();
    assert_eq!(
        hash(format!("{run}/stdout.log")),
        exit["stdout_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(format!("{run}/stderr.log")),
        exit["stderr_sha256"].as_str().unwrap()
    );
    let samples = stdout
        .lines()
        .filter_map(|s| s.strip_prefix("SAMPLE "))
        .map(|s| serde_json::from_str::<Value>(s).unwrap())
        .collect::<Vec<_>>();
    let summaries = stdout
        .lines()
        .filter_map(|s| s.strip_prefix("SUMMARY "))
        .map(|s| serde_json::from_str::<Value>(s).unwrap())
        .collect::<Vec<_>>();
    let rounds = j["rounds"].as_u64().unwrap() as usize;
    let workers = j["workers"].clone();
    assert_eq!(samples.len(), rounds);
    for (i, s) in samples.iter().enumerate() {
        assert_eq!(s["round"], i);
        let ms = if j["engine"] == "zopro" {
            s["ms"].as_f64().unwrap()
        } else {
            s["verified_ms"].as_f64().unwrap()
        };
        assert!(ms.is_finite() && ms > 0.0);
    }
    assert_eq!(summaries.len(), 1);
    assert_eq!(samples[0]["round"], 0);
    assert_eq!(summaries[0]["rounds"], rounds);
    assert_eq!(summaries[0]["priming"], 3);
    let z = j["engine"] == "zopro";
    if z {
        assert_eq!(summaries[0]["workers"], workers);
        assert_eq!(
            summaries[0]["placement_before"],
            summaries[0]["placement_after"]
        );
    } else {
        assert_eq!(summaries[0]["threads"], workers);
        assert_eq!(samples[0]["cache_hit"], true);
        assert_eq!(
            summaries[0]["key_sha256"],
            "7ef987966b0dfba5d86eab7c67068be00dbe581d823d45cdbbf6d338bfacb640"
        );
    }
    let prefix = if z {
        "ZOPRO_MSM_PHASES "
    } else {
        "MSM_PHASES "
    };
    let phases = stderr
        .lines()
        .filter_map(|s| s.strip_prefix(prefix))
        .map(|s| serde_json::from_str::<Value>(s).unwrap())
        .collect::<Vec<_>>();
    let on = j["arm"] == "ON";
    assert_eq!(
        phases.len(),
        if on { rounds + 3 } else { 0 },
        "observer count {name}"
    );
    for phase in &phases {
        assert_eq!(phase["workers"], workers);
        let total = phase["total_ns"].as_u64().unwrap();
        let rows = phase["rows"].as_array().unwrap();
        let names = rows
            .iter()
            .map(|v| v["phase"].as_str().unwrap())
            .collect::<Vec<_>>();
        if z {
            assert_eq!(phase["valid"], true);
            assert_eq!(phase["route"], 1);
            assert_eq!(
                names,
                ["solve", "prepare", "fft", "queries", "A", "KH", "G2", "output"]
            );
            for row in rows {
                assert_eq!(row["state"], 2);
            }
            let b = |i: usize| rows[i]["begin_ns"].as_u64().unwrap();
            let e = |i: usize| rows[i]["end_ns"].as_u64().unwrap();
            for pair in [(0, 1), (1, 2), (2, 3), (3, 7)] {
                assert!(e(pair.0) <= b(pair.1));
            }
            for child in 4..7 {
                assert!(b(child) >= b(3) && e(child) <= e(3));
            }
        } else {
            assert_eq!(phase["complete"], true);
            assert_eq!(phase["overflow"], false);
            assert_eq!(phase["malformed"], false);
            assert!(rows.len() <= 64);
            assert_eq!(
                &names[..6],
                &["begin", "recode", "count", "place", "scatter", "accumulate"]
            );
            let mut at = 6;
            while names[at] == "merge_plan" && names[at + 1] == "merge" {
                at += 2;
            }
            assert_eq!(&names[at..], &["merge_plan", "sums", "finish"]);
            let mut end = 0;
            for row in rows {
                let begin = row["begin_ns"].as_u64().unwrap();
                assert!(begin >= end);
                end = row["end_ns"].as_u64().unwrap();
                let items = row["job_items"].as_array().unwrap();
                if !items.is_empty() {
                    assert_eq!(items.len() as u64, phase["jobs"].as_u64().unwrap());
                    assert_eq!(
                        items.iter().map(|v| v.as_u64().unwrap()).sum::<u64>(),
                        row["items"].as_u64().unwrap()
                    );
                }
            }
        }
        for row in rows {
            let b = row["begin_ns"].as_u64().unwrap();
            let e = row["end_ns"].as_u64().unwrap();
            assert!(b <= e && e <= total);
        }
    }
    let proof = format!("{}/0.proof", j["proof_dir"].as_str().unwrap());
    let public = format!("{}/0.public", j["proof_dir"].as_str().unwrap());
    assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
    assert_eq!(hash(&public), j["public_sha256"].as_str().unwrap());
    let proofhash = hash(&proof);
    let vr = format!("{R}/run/verify-{name}");
    assert_eq!(read(format!("{vr}/exit.json"))["success"], true);
    let verified = fs::read_to_string(format!("{vr}/stdout.log")).unwrap();
    assert!(verified.contains(&format!("VERIFIED proof_sha256={proofhash} ")));
    assert!(verified.contains(&format!(
        " image_sha256={}",
        j["image_sha256"].as_str().unwrap()
    )));
    json!({"name":name,"passed":true,"proof_sha256":proofhash,"public_sha256":hash(public),"phases":phases,"summary":summaries[0],"samples":samples,"job":j})
}
