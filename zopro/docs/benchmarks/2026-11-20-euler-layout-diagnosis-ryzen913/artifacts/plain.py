from zp_report_common import *
import random

d=base/'euler-layout-diagnosis913'
d.mkdir(exist_ok=True)
prior=base/'euler-fused-ends-ryzen912'
workflow=rd(prior/'workflow.json')
steps={s['name']:s for s in workflow['steps']}
pins=rd(prior/'pins.json')
old=base/'euler-derivative-bridge911'
new=base/'euler-fused-ends912'
records=rd(d/'results.json')
assert len(records)==18
for location,digest in pins.items():
    assert sha(Path(location))==digest
for s in ('1x2','2x2','4x2'):
    a,b=[steps[f'timing-b0-{s}-catalog-{m}'] for m in ('prior','bridge')]
    assert a['environment']==b['environment']
    assert [k for k,(x,y) in enumerate(zip(a['argv'],b['argv'])) if x!=y]==[3,7]
assert rd(old/'built.json')['environment']==rd(new/'built.json')['environment']

def cell(panel,k,shape,arm,rounds):
    name=f'{panel}{rounds}-b{k}-{shape}-{arm}'
    dest=d/name
    prefix=f'timing-b0-{shape}-catalog-{arm}'
    test,check=steps[prefix],steps[prefix+'-verify']
    environment=test['environment']
    argv=test['argv'][:-2]+[str(rounds),str(dest/'proofs')]
    verify=check['argv'][:3]+[str(dest/'proofs/0.proof'),str(dest/'proofs/0.public')]+check['argv'][5:]
    assert sha(Path(argv[3]))==pins[argv[3]]
    assert sha(Path(verify[0]))==pins[verify[0]] and sha(Path(verify[1]))==verify[2]
    dest.mkdir()
    import os
    with (dest/'stdout.log').open('w') as out,(dest/'stderr.log').open('w') as err:
        subprocess.run(argv,env=os.environ|environment,stdout=out,stderr=err,check=True,timeout=90)
    proof,pub=sha(Path(verify[3])),sha(Path(verify[4]))
    token=f'VERIFIED proof_sha256={proof} public_sha256={pub}'
    receipt=subprocess.check_output(verify,text=True)
    assert token in receipt and pub==verify[-1]
    (dest/'verify.log').write_text(receipt)
    lines=(dest/'stdout.log').read_text().splitlines()
    summary,=[json.loads(x[8:]) for x in lines if x.startswith('SUMMARY ')]
    samples=[json.loads(x[7:]) for x in lines if x.startswith('SAMPLE ')]
    assert len(samples)==rounds and summary['workers']==24 and summary['priming']==3
    assert [json.loads(x[10:]) for x in lines if x.startswith('MAIN_CPUS ')]==[list(range(24))]
    assert [json.loads(x[11:]) for x in lines if x.startswith('FRONT_CPUS ')]==[list(range(24,32))]
    row=dict(panel=panel,block=k,shape=shape,arm=arm,proof=proof,public=pub,ms=mean(x['ms'] for x in samples),
             argv=argv,environment=environment,rounds=rounds,verify=verify,summary=summary,samples=samples)
    records.append(row)
    save(d/'combined.json',records)
    print(name,round(row['ms'],3),flush=True)


for k in range(3):
    matrix=[(s,rounds) for s in ('1x2','2x2','4x2') for rounds in (19,151)]
    random.Random(91310+k//2).shuffle(matrix)
    if k%2:
        matrix.reverse()
    for shape,rounds in matrix:
        for arm in (('prior','bridge') if k%2==0 else ('bridge','prior')):
            cell('plain',k,shape,arm,rounds)
assert len(records)==54
