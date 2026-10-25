$old[0] |
.blocks=3 | .profile_blocks=3 | .checks=[] | .frequency_hz=999 |
.variants=[
  {name:"accepted",binary:"/tmp/zopro-grouping-module/native/candidate-sdk"},
  {name:"diagnostic-off",binary:"/tmp/zopro-phase-cpu/native/diagnostic-sdk",environment:{ZOPRO_BENCH_PHASE_CPU:"0"}},
  {name:"diagnostic-on",binary:"/tmp/zopro-phase-cpu/native/diagnostic-sdk",environment:{ZOPRO_BENCH_PHASE_CPU:"1"}}
] |
.cases=[.cases[]|select(.name=="1x2-w16" or .name=="1x2-w32" or .name=="merge-54x1-w32")|
  .rounds=(if .shape=="merge-54x1" then 9 else 17 end)] |
.profile_cases=[.cases[].name] | .pins=$pins[0] |
.prior_proof_sha256=((.prior_proof_sha256+
  [$closed[0],$preflight[0],$our_preflight[0]|to_entries[]|select(.key|endswith(".proof"))|.value])|unique)
