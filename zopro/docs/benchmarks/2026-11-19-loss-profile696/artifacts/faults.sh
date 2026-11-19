#!/usr/bin/env bash
# The engine opens both FIFOs and enables the group after warmup.
set -euo pipefail

engine=$1
output=$2
shift 2
mkdir -p "$output"
case "$engine" in
    aeglos) prefix=CROSS_PERF_ ;;
    zopro) prefix=ZOPRO_BENCH_PERF_ ;;
    *) exit 2 ;;
esac

fifo_dir=$(mktemp -d)
trap 'rm -rf -- "$fifo_dir"' EXIT
mkfifo -m 600 "$fifo_dir/ctl" "$fifo_dir/ack"
export "${prefix}CTL=$fifo_dir/ctl" "${prefix}ACK=$fifo_dir/ack"

perf=(/usr/bin/perf)
prover=("$@")
# A restricted host may allow privileged perf. Keep the prover at its original UID.
if (( EUID != 0 )) && [[ $(< /proc/sys/kernel/perf_event_paranoid) -gt 2 ]]; then
    perf=(/usr/bin/sudo -n /usr/bin/perf)
    engine_env=(PATH=/usr/bin:/bin)
    while IFS= read -r name; do
        case "$name" in
            AEGLOS_*|CROSS_PERF_*|GOMAXPROCS|LD_LIBRARY_PATH|MALLOC_*|RAYON_NUM_THREADS|ZOPRO_*)
                engine_env+=("$name=${!name}") ;;
        esac
    done < <(compgen -e)
    prover=(/usr/sbin/runuser -u "$(id -un)" -- /usr/bin/env -i "${engine_env[@]}" "$@")
fi

LC_ALL=C "${perf[@]}" record -q --no-buildid-cache --clockid mono -D -1 \
    --control "fifo:$fifo_dir/ctl,$fifo_dir/ack" -e minor-faults:u -c 11 \
    -o "$output/cycles.data" -- "${prover[@]}"
"${perf[@]}" script -i "$output/cycles.data" -F pid,period,sym,dso > "$output/symbols.txt"
"${perf[@]}" script -D -i "$output/cycles.data" | awk '/PERF_RECORD_LOST/ {lost++} END {print lost+0; exit (lost!=0)}' > "$output/lost.txt"
