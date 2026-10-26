#!/bin/sh
set -eu
case "${1:?preflight or bench required}" in
 preflight) exec /experiment/zopro-xtask workflow /experiment/preflight.json /output/preflight ;;
 bench)
  mkdir /output/hardware
  lscpu > /output/hardware/lscpu.txt
  lscpu -e=CPU,CORE,SOCKET,NODE > /output/hardware/cpu-core-map.txt
  uname -a > /output/hardware/uname.txt
  cat /proc/meminfo > /output/hardware/meminfo.txt
  snapshot() {
   prefix=$1
   for resource in cpu.stat cpu.max memory.events memory.max memory.swap.current; do
    if test -f "/sys/fs/cgroup/$resource"; then cat "/sys/fs/cgroup/$resource" > "/output/hardware/$prefix-$resource"; fi
   done
  }
  snapshot before
  trap 'snapshot after' EXIT
  /experiment/zopro-xtask workflow /experiment/comparison-latency.json /output/comparison-latency
  /experiment/zopro-xtask perf /experiment/perf.json /output/perf
  /experiment/zopro-xtask workflow /experiment/comparison-stat.json /output/comparison-stat
  /experiment/review all /output /output/review
  ;;
 *) exit 2 ;;
esac
