#!/bin/sh
set -eu
mode=${1:?preflight or bench required}
case "$mode" in
  preflight) exec /experiment/zopro-xtask workflow /experiment/preflight.json /output/preflight ;;
  bench)
    mkdir /output/hardware
    lscpu > /output/hardware/lscpu.txt
    uname -a > /output/hardware/uname.txt
    cat /proc/meminfo > /output/hardware/meminfo.txt
    for resource in cpu.stat cpu.max memory.events memory.max memory.swap.current; do
      if test -f "/sys/fs/cgroup/$resource"; then
        cat "/sys/fs/cgroup/$resource" > "/output/hardware/before-$resource"
      fi
    done
    record_after() {
      for resource in cpu.stat cpu.max memory.events memory.max memory.swap.current; do
        if test -f "/sys/fs/cgroup/$resource"; then
          cat "/sys/fs/cgroup/$resource" > "/output/hardware/after-$resource"
        fi
      done
    }
    trap record_after EXIT
    /experiment/zopro-xtask perf /experiment/perf.json /output/perf
    /experiment/zopro-xtask workflow /experiment/comparison.json /output/comparison
    ;;
  *) exit 2 ;;
esac
