from zp_report_common import *
import os
import random

d = base/'euler-endpoints-recheck914'
upstream = base/'euler-fused-ends-ryzen912'
builders = {
    'reference': base/'seed-vector-b2-sdk851',
    'shift': base/'euler-ordinary-layout908',
    'prior': base/'euler-derivative-bridge911',
    'current': base/'euler-fused-ends912',
}
shapes = ('1x2', '2x2', '4x2')
inputs = ('catalog', 'heldout')
modes = ('reference', 'ordinary', 'prior', 'bridge', 'emit', 'ends')
blocks = 12
rounds = 19
assert not d.exists()
d.mkdir()
pins = rd(upstream/'pins.json')
for directory in builders.values():
    p = directory/'cpu-sdk'
    assert sha(p) == rd(directory/'built.json')['binary']
    pins[str(p)] = sha(p)
for path, wanted in pins.items():
    assert sha(Path(path)) == wanted
original_workflow = rd(upstream/'workflow.json')
source = {s['name']: s for s in original_workflow['steps']}
protocol = dict(
    shapes=shapes, inputs=inputs, modes=modes, blocks=blocks, timed_per_process=rounds, priming=3,
    reference={'1x2':'851 release','2x2':'908 shift','4x2':'851 seed4-emit'},
    calibration={'same_configuration_pairs':[['ordinary','reference'],['bridge','prior']],
                 'paired_median_max_absolute_pct':2.0, 'apply_per_shape_and_input':True},
    advancement={'min_wins':9, 'wins_denominator':12, 'require_negative_paired_median':True,
                 'controls':['reference','ordinary','prior'], 'apply_per_shape_and_input':True},
    schedule_seed=9141, arm_seed=91420, prior_workflow_sha256=sha(upstream/'workflow.json'),
    source_pins=sha(upstream/'pins.json'),
)
save(d/'protocol.json',protocol)

def job(k, shape, h, m):
    template_mode = 'ordinary' if m=='reference' else m
    stem = f'timing-b0-{shape}-{h}-{template_mode}'
    sample, verify = source[stem], source[stem+'-verify']
    destination = d/'proofs'/f'b{k}-{shape}-{h}-{m}'
    argv = sample['argv'].copy()
    env = sample['environment'].copy()
    if m == 'reference':
        argv[3] = str(builders['shift' if shape=='2x2' else 'reference']/'cpu-sdk')
    wanted = 'prior' if m=='prior' else ('shift' if shape=='2x2' else 'reference') if m=='reference' else 'current'
    assert argv[3] == str(builders[wanted]/'cpu-sdk') and argv[:3] == ['/usr/bin/taskset','-c','0-31']
    assert env['RAYON_NUM_THREADS'] == '24'
    assert 'ZOPRO_910_TOTAL_CLOCK' not in env and 'ZOPRO_910_TRACE' not in env
    env['ZOPRO_CPU_LAGRANGE_DIR'] = str(d/'cache')
    argv[-2:] = [str(rounds),str(destination)]
    check = verify['argv'].copy()
    assert check[2]==pins[check[1]] and argv[4]==check[1]
    check[3:5] = [str(destination/'0.proof'),str(destination/'0.public')]
    return dict(b=k,shape=shape,h=h,m=m,argv=argv,environment=env,verify=check)

jobs = []
for k in range(blocks):
    combinations = [(s,h) for s in shapes for h in inputs]
    random.Random(9141+k//2).shuffle(combinations)
    if k%2:
        combinations.reverse()
    for s,h in combinations:
        arms = list(modes)
        random.Random(91420+k//2+sum(map(ord,s+h))).shuffle(arms)
        if k%2:
            arms.reverse()
        jobs.extend(job(k,s,h,m) for m in arms)
assert len(jobs) == 432
save(d/'jobs.json',jobs)
save(d/'pins.json',pins)

for j in jobs:
    name = f"b{j['b']}-{j['shape']}-{j['h']}-{j['m']}"
    logs = d/'logs'/name
    logs.mkdir(parents=True,exist_ok=False)
    with (logs/'stdout').open('w') as out,(logs/'stderr').open('w') as err:
        subprocess.run(j['argv'],env=os.environ|j['environment'],stdout=out,stderr=err,check=True,timeout=90)
    output = (logs/'stdout').read_text().splitlines()
    summary, = [json.loads(x[8:]) for x in output if x.startswith('SUMMARY ')]
    samples = [json.loads(x[7:]) for x in output if x.startswith('SAMPLE ')]
    assert summary['priming']==3 and summary['workers']==24 and len(samples)==rounds
    assert [json.loads(x[10:]) for x in output if x.startswith('MAIN_CPUS ')]==[list(range(24))]
    assert [json.loads(x[11:]) for x in output if x.startswith('FRONT_CPUS ')]==[list(range(24,32))]
    assert not any('ZP910_' in x or 'ZP911_' in x or 'ZP912_' in x for x in (logs/'stderr').read_text().splitlines())
    save(logs/'record.json',dict(ms=mean(x['ms'] for x in samples),rss=summary['whole_process_maxrss_kib']/1024,
                                summary=summary,samples=samples))
    if name.endswith('-ends'):
        print(name,round(mean(x['ms'] for x in samples),3),flush=True)
save(d/'done.json',dict(count=len(jobs),protocol=sha(d/'protocol.json'),jobs=sha(d/'jobs.json')))
