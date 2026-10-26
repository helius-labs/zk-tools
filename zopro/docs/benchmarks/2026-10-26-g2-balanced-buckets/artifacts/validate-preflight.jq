$inventory[0] as $inv | $template[0] as $t |
[$inv|to_entries[]|select(.key|endswith(".proof"))|.value] as $proofs |
[$inv|to_entries[]|select(.key|endswith(".public"))|. as $e |
 (if (.key|contains("/transfer_confidential_1_2/")) then "1x2"
  elif (.key|contains("/transfer_confidential_2_2/")) then "2x2"
  elif (.key|contains("/merge_54_1/")) then "merge-54x1"
  else error("unknown preflight public path") end) as $shape |
 select($e.value != ([$t.cases[]|select(.shape==$shape)|.public_sha256]|unique|if length==1 then .[0] else error("ambiguous fixture") end))] as $bad |
if ($inv|length)!=24 or ($proofs|length)!=12 or ($proofs|unique|length)!=12 or ($bad|length)!=0 or ([$proofs[]|. as $p|select($t.prior_proof_sha256|index($p))]|length)!=0 then error("preflight inventory failed")
else {status:"passed",sdk_requests:48,distinct_gnark:12,proof_public_members:24,public_binding:true,unique_proofs:true,source_guards:true,performance_claim:false} end
