set -eu
systemctl is-active zopro-bounded-search-ttl.timer
cat /proc/stat > /var/tmp/zopro-bounded-search-c7a/host-stat-before.txt
cat /proc/pressure/cpu > /var/tmp/zopro-bounded-search-c7a/host-cpu-pressure-before.txt
cat /proc/pressure/memory > /var/tmp/zopro-bounded-search-c7a/host-memory-pressure-before.txt
finish() {
 bench_rc=$?
 trap - EXIT
 docker inspect zopro-bounded-search-c7a --format '{{.State.ExitCode}} {{.State.OOMKilled}}' > /var/tmp/zopro-bounded-search-c7a/container-exit.txt || true
 docker inspect zopro-bounded-search-c7a --format '{"memory":{{.HostConfig.Memory}},"memory_swap":{{.HostConfig.MemorySwap}},"cpu_quota":{{.HostConfig.CpuQuota}},"cpu_period":{{.HostConfig.CpuPeriod}},"nano_cpus":{{.HostConfig.NanoCpus}},"cpuset_cpus":{{json .HostConfig.CpusetCpus}},"pids_limit":{{.HostConfig.PidsLimit}},"ulimits":{{json .HostConfig.Ulimits}}}' > /var/tmp/zopro-bounded-search-c7a/container-resources.json || true
 cat /proc/stat > /var/tmp/zopro-bounded-search-c7a/host-stat-after.txt
 cat /proc/pressure/cpu > /var/tmp/zopro-bounded-search-c7a/host-cpu-pressure-after.txt
 cat /proc/pressure/memory > /var/tmp/zopro-bounded-search-c7a/host-memory-pressure-after.txt
 tail -n 24 /var/tmp/zopro-bounded-search-c7a/bench.log
 exit "$bench_rc"
}
trap finish EXIT
docker run --name zopro-bounded-search-c7a --network none --cpuset-cpus 0-95 --memory 30g --memory-swap 30g --pids-limit 1024 --ulimit memlock=268435456:268435456 --cap-drop ALL --cap-add PERFMON --security-opt seccomp=unconfined --security-opt no-new-privileges -v /var/tmp/zopro-bounded-search-c7a/output:/output 558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover@sha256:11d0347ca3aae898c6631354a32ccca98ce5616869080356879336a40767c9a5 bench > /var/tmp/zopro-bounded-search-c7a/bench.log 2>&1
test -f /var/tmp/zopro-bounded-search-c7a/output/perf/result.json
test -f /var/tmp/zopro-bounded-search-c7a/output/comparison-latency/result.json
test -f /var/tmp/zopro-bounded-search-c7a/output/comparison-stat/result.json
test -f /var/tmp/zopro-bounded-search-c7a/output/review/result.json
