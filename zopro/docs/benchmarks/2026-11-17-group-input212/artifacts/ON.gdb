set pagination off
set confirm off
set print thread-events off
set debuginfod enabled off
set auto-load off
set language c
starti
set $hits = 0
break *'zopro_core::msm_affine::grouping::scalar_groups' if $rdx == 30089
commands
silent
set $hits = $hits + 1
set $begin = $rsi
set $end = $rsi + 32*$rdx
eval "dump binary memory /home/ubuntu/src/zopro-perf-work/group-input212/dumps/ON/scalar-%d.bin %p %p", $hits, $begin, $end
printf "INPUT_DUMP hit=%d n=%ld bytes=%ld alignment64=%ld\n", $hits, $rdx, $end-$begin, ((unsigned long)$begin)%64
continue
end
continue
printf "INPUT_DUMPS %d\n", $hits
if $hits != 4
quit 7
end
quit
