def med: sort | if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end;
def phases: ["outside_sdk","to_solver","solve","prepare","fft_check","to_g2","to_fill","fill_span","fold_tail","after_g2","sdk_tail"];
def records($row):
  [range($row.job.n) as $i | $row.series | with_entries(.value=.value[$i]) | . + {i:$i,outside_sdk:(.full-.sdk)}];
[.rows[]|select(.job.w==96 and .job.arm=="trace" and .job.pre==false)|
 .job.block as $b|records(.) as $r|[$r[]|select(.full<10)] as $fast|
 (reduce phases[] as $k ({};.[$k]=([$fast[]|.[$k]]|med))) as $base|
 [$r[]|. as $x| [phases[] as $k | {k:$k,excess:($x[$k]-$base[$k])}]|sort_by(-.excess)|
 {block:$b,i:$x.i,full:$x.full,class:(if $x.full<10 then "fast" elif $x.full>=11 then "slow" else "middle" end),largest:.[0],second:.[1],count_gt_1:map(select(.excess>1))|length,count_gt_1_5:map(select(.excess>1.5))|length,count_gt_2:map(select(.excess>2))|length}]]
|flatten|group_by(.class)|map({class:.[0].class,n:length,by_phase:(group_by(.largest.k)|map({phase:.[0].largest.k,n:length,excess:([.[].largest.excess]|med),second:([.[].second.excess]|med)})),
     exactly_one_above_1:map(select(.count_gt_1==1))|length,exactly_one_above_1_5:map(select(.count_gt_1_5==1))|length,
     exactly_one_above_2:map(select(.count_gt_2==1))|length,more_than_one_above_1:map(select(.count_gt_1>1))|length})
