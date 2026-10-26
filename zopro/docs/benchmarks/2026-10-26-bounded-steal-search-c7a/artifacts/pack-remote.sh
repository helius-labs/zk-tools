set -eu
test -f /var/tmp/zopro-bounded-search-c7a/output/review/result.json
test -f /var/tmp/zopro-bounded-search-c7a/output/comparison-stat/result.json
tar -C /var/tmp/zopro-bounded-search-c7a --exclude='*/ctl' --exclude='*/ack' -czf /var/tmp/zopro-bounded-search-c7a/review-results.tar.gz output hardware.txt bench.log container-exit.txt container-resources.json host-stat-before.txt host-stat-after.txt host-cpu-pressure-before.txt host-cpu-pressure-after.txt host-memory-pressure-before.txt host-memory-pressure-after.txt perf-probe.log record-probe.log
sha256sum /var/tmp/zopro-bounded-search-c7a/review-results.tar.gz
stat -c '%s' /var/tmp/zopro-bounded-search-c7a/review-results.tar.gz
docker cp zopro-bounded-search-c7a:/experiment/serve-results /var/tmp/zopro-bounded-search-c7a/serve-results
systemd-run --unit=zopro-bounded-search-transfer --collect --property=RuntimeMaxSec=180 /var/tmp/zopro-bounded-search-c7a/serve-results /var/tmp/zopro-bounded-search-c7a/review-results.tar.gz
systemctl is-active zopro-bounded-search-transfer.service
systemctl is-active zopro-bounded-search-ttl.timer
