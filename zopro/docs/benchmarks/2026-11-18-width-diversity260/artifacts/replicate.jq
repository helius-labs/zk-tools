[.[]|select(.shape=="1x2" and .workers==96 and .phase=="latency" and .arm=="ON")| . as $r | .round_times_ms as $x
 | if ($x|length)!=151 then error("incomplete block") else
   {block:$r.block,fast:([$x[]|select(.<10)]|length),middle:([$x[]|select(.>=10 and .<11)]|length),slow:([$x[]|select(.>=11)]|length),all_certified:$r.summary.early_certified,major_faults:$r.loop_major_faults} end]
 | if (map(.block)|sort)==[0,1,2] then . else error("unexpected blocks") end
