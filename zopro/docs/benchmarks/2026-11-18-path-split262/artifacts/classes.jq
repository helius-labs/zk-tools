def avg: if length==0 then null else add/length end;
def median: sort | .[(length/2|floor)];
def fclass: if .<10 then "fast" elif .<11 then "middle" else "slow" end;
def records($row):
  [range($row.job.n) as $i | $row.series | with_entries(.value=.value[$i])
    | . + {class:(.full|fclass),outside_sdk:(if has("sdk") then .full-.sdk else null end)}];
def times: ["outside_sdk","to_solver","solve","prepare","fft_check","to_g2","to_fill","fill_span","fold_tail","after_g2","sdk_tail","fft","check","check_after_fft_start","check_end_minus_fft","raw","a","a_exit_minus_raw","last_fill","full"];
def means($rows): reduce times[] as $key ({}; .[$key]=([$rows[]|.[$key]]|avg));
def counts($r): reduce ["fast","middle","slow"][] as $key ({};.[$key]=([$r[]|select(.class==$key)]|length));
.rows as $rows |
[range(3) as $b | [32,96][] as $w |
  ([$rows[]|select(.job.pre==false and .job.block==$b and .job.w==$w)]|INDEX(.job.arm)) as $arms |
  records($arms.trace) as $r | records($arms.plain) as $p |
  ($r|map(select(.class=="fast"))) as $fast | ($r|map(select(.class=="slow"))) as $slow |
  means($fast) as $fm | means($slow) as $sm |
  {w:$w,block:$b,ratio:($arms.trace.means.full/$arms.plain.means.full),trace_ms:$arms.trace.means.full,
   plain_ms:$arms.plain.means.full,aeglos_ms:$arms.aeglos.means.full,trace_counts:counts($r),plain_counts:counts($p),
   fast:$fm,slow:$sm,delta:(reduce times[] as $k ({}; .[$k]=(if $fm[$k]!=null and $sm[$k]!=null then $sm[$k]-$fm[$k] else null end))),
   routes:([$r[]|{class,fft_cpu,check_cpu,raw_cpu,a_cpu,last_fill_cpu,solver_cpu,sdk_cpu}]),
   fast_samples:($fast|length),slow_samples:($slow|length)}]
