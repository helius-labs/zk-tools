($parent[0]) as $p |
($accepted[0]) as $a |
{
  version:1,
  blocks:7,
  profile_blocks:3,
  profile_cases:["1x2-w32","merge-54x1-w32"],
  command:$p.command,
  variants:[
    {name:"accepted",binary:$a.native_binary},
    {name:"off",binary:$off[0].binary},
    {name:"on",binary:$on[0].binary}
  ],
  cases:([$p.cases[]|.cpu_affinity="0-31"] + [($p.cases[0] | .name="1x2-w16" | .workers=16 | .cpu_affinity="0-15" | .reference=null)]),
  pins:($pins[0] + {
    "/tmp/zopro-complex-shared-product/sdk-experiment.json":"5ed669a57310f120b9ce6ad7077051a59b0c6cd8e0136405e69060e3a349d0c3",
    "/tmp/zopro-complex-shared-product/component-result.json":"b718314daac48a7fc14878d28deb0dd00d9c729a8e0a7094b4e3252e80e11391"
  }),
  verifier:$p.verifier,
  environment:$a.environment,
  profile_environment:$p.profile_environment,
  checks:[],
  perf_prefix:$p.perf_prefix,
  child_prefix:$p.child_prefix,
  symbol_categories:[
    {name:"grouping",contains:["scalar_groups","hashbrown"]},
    {name:"G2_arithmetic",contains:["g2_prefix","stream_g2","g2_phi","g2_horner","G2Config","complex_product_shared"]},
    {name:"G1_arithmetic",contains:["stream_g1","fused_kh","G1Config"]},
    {name:"inverse",contains:["zopro_bn254::inverse"]},
    {name:"epoch",contains:["crossbeam_epoch"]},
    {name:"scheduler",contains:["rayon_core","crossbeam_deque"]},
    {name:"fft",contains:["zopro_core::fft"]},
    {name:"solver",contains:["zopro_core::solver"]}
  ],
  prior_proof_sha256:([$p.prior_proof_sha256[],($old[0]|to_entries[]|select(.key|endswith(".proof"))|.value),($compat[0]|to_entries[]|select(.key|endswith(".proof"))|.value),($preflight[0]|to_entries[]|select(.key|endswith(".proof"))|.value)]|unique),
  timeout_seconds:180,
  frequency_hz:999,
  perf:$p.perf
}
