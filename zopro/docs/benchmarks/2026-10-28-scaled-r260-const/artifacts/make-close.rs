use std::fs;
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let old = "/home/ubuntu/src/zopro-perf-work/scaled-r260-native";
    let new = "/home/ubuntu/src/zopro-perf-work/scaled-r260-const";
    let mut s = fs::read_to_string(format!("{old}/close.rs"))
        .unwrap()
        .replace(old, new)
        .replace(
            "2026-10-28-scaled-r260-native",
            "2026-10-28-scaled-r260-const",
        );
    once(
        &mut s,
        "# Native coordinate-scaled R260 G2 batches",
        "# Compile-time modulus constants in scaled G2 batches",
    );
    once(&mut s,"Port the passed exact screen to actual AVX512 IFMA using","Require explicit const evaluation at three fixed-modulus sites in the validated parent native implementation. The arithmetic remains actual AVX512 IFMA using");
    once(&mut s,"Full core suites pass122 OFF and124 ON","Inherited unchanged OFF qualification has122 core tests; the changed ON build passes124 core tests");
    let start = s
        .find("Initial setup failed before builds because")
        .unwrap();
    let end = start
        + s[start..]
            .find("Successful tests and captures are not repeated for bookkeeping.")
            .unwrap();
    let end = end + "Successful tests and captures are not repeated for bookkeeping.".len();
    s.replace_range(start..end,"Initial generation fails before setup/build because a replacement matches both link and capture step declarations. Anchoring to the link variant loop repairs only generation. The original generator/partial outputs are preserved. Parent-only setup/numeric/preflight failures are documented in its report; they are not new failures here. The narrow20source-line/build-ID metadata allowance is declared before this follow-up, without changing performance gates. Native arithmetic is unchanged apart from compile-time evaluation. No successful tests or captures are repeated for bookkeeping.");
    s = s.replace(
        "Identical control code and constants exclude",
        "Identical control code and constants exclude",
    );
    s = s.replace("{R}/next-step.md", "{R}/followup-scope.md");
    s = s
        .replace(
            "Native scaledR260 caller gate passed",
            "Constant-only scaledR260 caller gate passed",
        )
        .replace(
            "Native scaledR260 caller gate failed",
            "Constant-only scaledR260 caller gate failed",
        );
    s=s.replace("Require const evaluation at three scaled modulus sites; profile showed runtime construction. Revalidate changed native code/callers; no unchanged arithmetic reroll or unmeasured speed claim","Use the actual constant-only profiles to select distinct operation-count work removal; no unchanged scaledR260 reroll");
    s = s.replace(
        "CURRENT COMPLETE/PUSHED — NATIVE COORDINATE-SCALED R260",
        "CURRENT COMPLETE/PUSHED — CONSTANT-ONLY COORDINATE-SCALED R260",
    );
    s = s.replace(
        "Nativecheckedfield33PASS,coreOFF122/ON124PASS",
        "Nativecheckedfield33PASS,inheritedOFF122/newON124PASS",
    );
    s=s.replace("Nativecoordinate(x/4,y/8)","Exactly3constcontexts replace runtimebuilder (parentprofile8.596%period); parentkernel/numericbytesunchanged. Nativecoordinate(x/4,y/8)");
    s = s.replace(
        "InitialsetupUTF8binaryauditrepair preserved;no arithmeticchange.",
        "Initialgeneration2-matchrepair preserved beforebuild;parentfailureshistorical only.",
    );
    fs::write("close.rs", s).unwrap();
    println!(
        "Generatedconstant-onlyreport helper;OFFqualificationinherited;parentfailuresdistinguished"
    );
}
