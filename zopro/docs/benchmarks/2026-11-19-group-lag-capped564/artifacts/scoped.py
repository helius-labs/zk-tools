import json
import os
import pathlib
import subprocess
import sys

job_path = pathlib.Path(sys.argv[1])
job = json.loads(job_path.read_text())
scope = pathlib.Path('/sys/fs/cgroup' + pathlib.Path('/proc/self/cgroup').read_text().strip().split('::', 1)[1])
assert int((scope / 'memory.max').read_text()) == job['limit_mib'] * 2**20
assert (scope / 'memory.swap.max').read_text().strip() == '0'
assert (scope / 'memory.high').read_text().strip() == 'max'

def counters():
    return {k: int(v) for k, v in (line.split() for line in (scope / 'memory.events.local').read_text().splitlines())}

def prefer_prover_for_scope_oom():
    pathlib.Path('/proc/self/oom_score_adj').write_text('500')

before = counters()
with open(job['log'], 'w') as log:
    p = subprocess.run(job['argv'], env=job['env'], stdout=log, stderr=subprocess.STDOUT,
                       preexec_fn=prefer_prover_for_scope_oom, timeout=150)
after = counters()
result = dict(returncode=p.returncode, events={k: v-before[k] for k, v in after.items()},
              charged_peak=int((scope / 'memory.peak').read_text()), limit_mib=job['limit_mib'])
job_path.with_suffix('.metrics').write_text(json.dumps(result))
