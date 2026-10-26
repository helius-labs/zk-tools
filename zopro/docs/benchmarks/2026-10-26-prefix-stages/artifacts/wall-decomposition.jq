def med: sort | length as $n | if $n%2==1 then .[($n/2|floor)] else (.[($n/2)-1]+.[($n/2)])/2 end;
map(.events | map({key:.label,value:.}) | from_entries |
  . as $e |
  (["core.solve_wires","core.public_correction","core.canonical_wires","core.gather_queries","core.product_fft"] | map(($e[.].wall1-$e[.].wall0)/1000000) | add) as $work |
  (($e["core.query_join"].wall0-$e["core.external_entry"].wall0)/1000000) as $prefix |
  {external_to_queries_ms:$prefix, recorded_preparation_work_ms:$work, other_before_queries_ms:($prefix-$work), query_join_ms:(($e["core.query_join"].wall1-$e["core.query_join"].wall0)/1000000), after_queries_ms:(($e["core.external_entry"].wall1-$e["core.query_join"].wall1)/1000000)}) as $rows |
{file:input_filename,rounds:($rows|length),process_medians:($rows[0]|keys|map(. as $k|{key:$k,value:($rows|map(.[$k])|med)})|from_entries),rounds_decomposition:$rows}
