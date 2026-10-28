use std::fs;
fn main() {
    let p = "/home/ubuntu/src/zopro-perf-work/scaled-r260-native/native.rs";
    let s = fs::read_to_string(p).unwrap();
    let t = s[s.find("fn freeze(").unwrap()..]
        .replace(
            "/home/ubuntu/src/zopro-perf-work/scaled-r260-native",
            "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue",
        )
        .replace(
            "[\"off\", \"on\"].contains(&label)",
            "[\"frozen\", \"off\", \"on\"].contains(&label)",
        )
        .replace("zopro_g2_scaled_r260", "zopro_g2_borrow_queue");
    fs::write(
        "native.rs",
        format!("{}\n{t}", fs::read_to_string("native-prefix.rs").unwrap()),
    )
    .unwrap();
    println!("GeneratedimmutablefullMSMlibraryfreezer;frozen/OFF/ONseparatetargets");
}
