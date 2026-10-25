($old[0].pins | with_entries(select(
    (.key|startswith("/tmp/zopro-canonical-grain-adoption/images/")) or
    (.key|startswith("/tmp/zopro-grouping-module/target-native/")) or
    (.key == "/tmp/zopro-grouping-module/native/candidate-sdk") or
    (.key == "/tmp/zopro-gnark-verify/verify") or
    (.key|contains("/tools/zolana/"))
))) as $inherited |
$source[0] + $native[0].input_pins + $inherited +
{($native[0].binary): $native[0].binary_sha256}
