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
records=[]
for location,digest in pins.items():
    assert sha(Path(location))==digest
for s in ('1x2','2x2','4x2'):
    a,b=[steps[f'timing-b0-{s}-catalog-{m}'] for m in ('prior','bridge')]
    assert a['environment']==b['environment']
    assert [k for k,(x,y) in enumerate(zip(a['argv'],b['argv'])) if x!=y]==[3,7]
assert rd(old/'built.json')['environment']==rd(new/'built.json')['environment']

def cell(panel,k,shape,arm):
    name=f'{panel}-b{k}-{shape}-{arm}'
    dest=d/name
    prefix=f'timing-b0-{shape}-catalog-{arm}'
    test,check=steps[prefix],steps[prefix+'-verify']
    environment=test['environment']
    argv=test['argv'][:-2]+['151',str(dest/'proofs')]
    verify=check['argv'][:3]+[str(dest/'proofs/0.proof'),str(dest/'proofs/0.public')]+check['argv'][5:]
    assert sha(Path(argv[3]))==pins[argv[3]]
    assert sha(Path(verify[0]))==pins[verify[0]] and sha(Path(verify[1]))==verify[2]
    dest.mkdir()
    if panel=='stat':
        flags=['stat','--no-big-num','-x','|','-e','task-clock,cycles:u,instructions:u,context-switches,minor-faults','-o',str(dest/'perf.stat')]
    else:
        flags=['record','-q','-e','cycles:u','-c','524287','-o',str(dest/'perf.data')]
    perf=['sudo','-n','perf',*flags,'--','/usr/bin/env',*[k+'='+v for k,v in environment.items()],*argv]
    with (dest/'stdout.log').open('w') as out,(dest/'stderr.log').open('w') as err:
        subprocess.run(perf,stdout=out,stderr=err,check=True,timeout=105)
    proof,pub=sha(Path(verify[3])),sha(Path(verify[4]))
    token=f'VERIFIED proof_sha256={proof} public_sha256={pub}'
    receipt=subprocess.check_output(verify,text=True)
    assert token in receipt and pub==verify[-1]
    (dest/'verify.log').write_text(receipt)
    lines=(dest/'stdout.log').read_text().splitlines()
    summary,=[json.loads(x[8:]) for x in lines if x.startswith('SUMMARY ')]
    samples=[json.loads(x[7:]) for x in lines if x.startswith('SAMPLE ')]
    assert len(samples)==151 and summary['workers']==24 and summary['priming']==3
    assert [json.loads(x[10:]) for x in lines if x.startswith('MAIN_CPUS ')]==[list(range(24))]
    assert [json.loads(x[11:]) for x in lines if x.startswith('FRONT_CPUS ')]==[list(range(24,32))]
    row=dict(panel=panel,block=k,shape=shape,arm=arm,proof=proof,public=pub,ms=mean(x['ms'] for x in samples),
             perf=perf,verify=verify,summary=summary,samples=samples)
    if panel=='stat':
        row['counts']={}
        for line in (dest/'perf.stat').read_text().splitlines():
            if not line or line.startswith('#'):
                continue
            pieces=line.split('|')
            assert float(pieces[4])>=99
            row['counts'][pieces[2]]=float(pieces[0])
        assert 'cycles:u' in row['counts'] and 'instructions:u' in row['counts']
    else:
        with (dest/'symbols.log').open('w') as out:
            subprocess.run(['sudo','-n','perf','script','-i',dest/'perf.data','-F','sym'],stdout=out,check=True)
        from collections import Counter
        symbol_lines=(dest/'symbols.log').read_text().splitlines()
        row['profile']=Counter(l.strip() for l in symbol_lines if l.strip()).most_common()
        assert sum(v for _,v in row['profile'])>100
    records.append(row)
    save(d/'results.json',records)
    print(name,round(row['ms'],3),flush=True)

for k in range(3):
    order=['1x2','2x2','4x2']
    random.Random(91300+k).shuffle(order)
    for shape in order:
        for mode in (('prior','bridge') if k%2==0 else ('bridge','prior')):
            cell('stat',k,shape,mode)
for k in range(2):
    for mode in (('prior','bridge') if k==0 else ('bridge','prior')):
        cell('profile',k,'1x2',mode)
assert len(records)==22
