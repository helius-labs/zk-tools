.monotonic_clock=true | .blocks=3 | .profile_blocks=3 |
.variants=[
 {name:"accepted",binary:"/experiment/zopro-accepted"},
 {name:"diagnostic-off",binary:"/experiment/zopro-diagnostic",environment:{ZOPRO_BENCH_PHASE_CPU:"0"}},
 {name:"diagnostic-on",binary:"/experiment/zopro-diagnostic",environment:{ZOPRO_BENCH_PHASE_CPU:"1"}}
] |
.cases=[.cases[]|select(.name=="1x2-w32" or .name=="merge-54x1-w32")|
 . as $case | [32,64,96][] as $workers |
 $case | .name=(.shape+"-w"+($workers|tostring)) | .workers=$workers |
 .rounds=(if .shape=="1x2" then 31 else 9 end) | .cpu_affinity="0-95" |
 .image=(.image|sub("^/tmp/zopro-canonical-grain-adoption/images";"/basis-latest")) |
 .request=(.request|sub("^/home/ubuntu/src/zk-tools/zopro/tools/zolana/fixtures";"/fixtures"))] |
.profile_cases=[.cases[].name] | .pins=$pins[0] |
.environment.ZOPRO_BASIS_KEY_DIR="/basis-latest" |
.environment.ZOPRO_CPU_LAGRANGE_DIR="/output/lagrange-cache" |
.perf="/usr/lib/linux-tools/6.8.0-147-generic/perf" | .perf_prefix=[] | .child_prefix=[] |
.verifier=["/usr/local/bin/gnark-verify","/keys","{proof_dir}","/manifest/zopro-keys.lock"]
