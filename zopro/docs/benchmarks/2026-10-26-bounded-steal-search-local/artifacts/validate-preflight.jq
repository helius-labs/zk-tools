$inventory[0] as $inv | $template[0] as $t |
[$inv|to_entries[]|select(.key|endswith(".proof"))|.value] as $proofs |
[$inv|to_entries[]|select(.key|endswith(".public"))|. as $entry |
 (.key|split("/")|.[-2]) as $stem |
 [$t.cases[]|select((.image|split("/")|last|rtrimstr(".image"))==$stem)|.public_sha256] | unique |
 if length!=1 then error("unknown or ambiguous public fixture")
 elif .[0]!=$entry.value then $entry else empty end] as $bad |
if ($inv|length)!=48 or ($proofs|length)!=24 or ($proofs|unique|length)!=24 or ($bad|length)!=0 or ([$proofs[]|. as $proof|select($t.prior_proof_sha256|index($proof))]|length)!=0
then error("preflight inventory or freshness check failed")
else {status:"passed",sdk_requests:96,distinct_gnark:24,proof_public_members:48,public_binding:true,unique_proofs:true,source_guards:true,performance_claim:false}
end
