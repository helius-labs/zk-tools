from zp_report_common import *
from concurrent.futures import ThreadPoolExecutor
import time

d=base/'compressed-final-exponent-ryzen867'
build=base/'compressed-final-exponent867'
prior=base/'borrowed-g2-group-c7a866'
doc=docs/'2026-11-20-compressed-final-exponent-ryzen867'
a=doc/'artifacts'
o=d/'results'
for _ in range(600):
    result=o/'result.json'
    if result.exists():
        status=rd(result)['status']
        if status=='complete': break
        if status in ('failed','error'): raise RuntimeError('local run '+status)
    time.sleep(1)
else:
    raise RuntimeError('local run timed out')
pin=rd(build/'built.json')
assert pin['source_patch']==sha(build/'source.patch')
assert pin['tests']==sha(build/'tests.log')
assert '40 passed; 0 failed' in (build/'tests.log').read_text()
modes=('release','ordinary','signed','compressed')

def audit(job):
    name=job['name']
    args=job['verify']
    proof=sha(Path(args[3]))
    run=o/name
    token=f'VERIFIED proof_sha256={proof} public_sha256={args[-1]}'
    assert token in (o/(name+'-verify')/'stdout.log').read_text()
    assert token in subprocess.check_output(args,text=True)
    logs=(run/'stdout.log').read_text().splitlines()
    assert [json.loads(v[10:]) for v in logs if v.startswith('MAIN_CPUS ')]==[list(range(24))]
    assert [json.loads(v[11:]) for v in logs if v.startswith('FRONT_CPUS ')]==[list(range(24,32))]
    samples=[json.loads(v[7:]) for v in logs if v.startswith('SAMPLE ')]
    summary,=[json.loads(v[8:]) for v in logs if v.startswith('SUMMARY ')]
    assert [p['round'] for p in samples]==list(range(job['rounds'])) and summary['priming']==3 and summary['workers']==24
    clocks=[list(map(int,line.split()[1:])) for line in (run/'stderr.log').read_text().splitlines()
            if line.startswith('FINAL_EXP_TRIAL ')]
    if job['panel']=='clock':
        mode=modes.index(job['m'])-1
        assert all(m==mode and n>0 for m,n in clocks),(name,clocks)
        assert len(clocks)==1+summary['priming']+len(samples),(name,clocks)
    else:
        assert not clocks
    return job|dict(proof=proof,ms=mean(v['ms'] for v in samples),clocks=clocks,
                    rss=summary['whole_process_maxrss_kib']/1024,summary=summary,samples=samples)

with ThreadPoolExecutor(max_workers=4) as pool:
    rows=list(pool.map(audit,rd(d/'jobs.json')))
assert len(rows)==222
save(d/'rows.json',rows)
table=[]
clock_table=[]
selection=[]
summary={}
for shape in ('1x2','2x2','4x2'):
    name=shape.replace('x','×')
    summary[shape]={}
    for h in ('catalog','heldout'):
        cells=[r for r in rows if (r['shape'],r['h'],r['panel'])==(shape,h,'timing')]
        groups={m:[r for r in cells if r['m']==m] for m in modes}
        assert all(len(g)==7 for g in groups.values())
        for m,g in groups.items():
            record=dict(ms=med(g),ordinary=pair(g,groups['ordinary']),release=pair(g,groups['release']),rss=peak(g))
            summary[shape].setdefault(h,{})[m]=record
            x,y=record['ordinary'],record['release']
            table.append(f"| {name} | {h} | {m} | {record['ms']:.3f} | {x[0]:+.2f}% ({x[1]}/7) | {y[0]:+.2f}% ({y[1]}/7) | {record['rss']:.1f} |")
        cells=[r for r in rows if (r['shape'],r['h'],r['panel'])==(shape,h,'clock')]
        stats={}
        for m in modes[1:]:
            g=[r for r in cells if r['m']==m]
            assert len(g)==3
            total=[mean(n/1_000_000 for _,n in r['clocks'][-r['rounds']:]) for r in g]
            clock_delta=pair(g,[r for r in groups[m] if r['b']<3])
            stats[m]=median(total)
            summary[shape][h][m].update(exponent_ms=stats[m],clock_effect=clock_delta)
            clock_table.append(f"| {name} | {h} | {m} | {stats[m]:.3f} | {clock_delta[0]:+.2f}% ({clock_delta[1]}/3) |")
    for m in modes[2:]:
        if all(g[m][c][0]<0 and g[m][c][1]>=5 for g in summary[shape].values() for c in ('ordinary','release')):
            selection.append(f'{name} {m}')
assert (prior/'published.json').is_file()
a.mkdir(parents=True,exist_ok=True)
pack(a/'rows.json.gz',rows)
pack(a/'workflow.json.gz',rd(d/'workflow.json'))
save(a/'pins.json',rd(d/'pins.json'))
for f in ('source.patch','built.json'):
    shutil.copyfile(build/f,a/f)
for f in ('zp_build867.py','zp_run867.py','zp_report867.py'):
    shutil.copyfile(Path('/tmp')/f,a/f)
shutil.copyfile(build/'tests.log',a/'field-tests.log')
v=extend(d,prior/'ledger.json',[r['proof'] for r in rows])
save(a/'verification.json',v)
save(a/'summary.json',summary)
decision=('The following profiles met the local screen against both controls on both inputs: '+', '.join(selection)+'.'
          if selection else 'Neither power algorithm improved both controls consistently across both inputs of any tested shape.')
(doc/'README.md').write_text(f"""# Signed and compressed BN254 final exponentiation on Ryzen

The signed arm uses 24 signed nonzero digits of the BN254 parameter. The compressed arm obtains the same exponent from a four-coordinate cyclotomic squaring ladder and reconstructs 23 powers with a single batch inverse. Its arrays use 7,360 bytes of fixed stack payload. If any reconstruction denominator is zero, the complete-coordinate signed method handles that element. All modes use the existing Miller loop and complete final exponentiation. [Karabina's squaring formulas](https://eprint.iacr.org/2010/542)

The native field suite passed 40 tests. Independent Arkworks comparisons cover signed and compressed powers, zero and identity inputs, and the complete final exponent. All {v['new_exports']} SDK exports independently verified.

The run uses 24 main and eight frontend Ryzen logical CPUs. Seven matched process blocks per input each prime three proofs and time thirteen. A profile must have a lower paired median and at least five wins against each control on both inputs to advance. Separate clocked processes measure the whole final exponent after key setup and priming. Peak RSS is the highest process reading in each group.

{decision} The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
{chr(10).join(table)}

| Shape | Input | Mode | Final exponent ms | Full-proof clock effect |
|---|---|---|---:|---:|
{chr(10).join(clock_table)}

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
""")
branch='sergei/zopro-parallel-prototype'
url='https://github.com/helius-labs/zk-tools.git'
auth=['-c','credential.helper=','-c','credential.helper=!gh auth git-credential']
rel=str(doc.relative_to(repo.parent))

def git(*args,capture=False):
    return subprocess.run(['git',*args],cwd=repo.parent,check=True,text=True,stdout=subprocess.PIPE if capture else None).stdout

assert git('branch','--show-current',capture=True).strip()==branch
git('add','--',rel)
git('diff','--cached','--check','--',rel,':!*.patch',':!*.log')
git('commit','--only','-m','Measure signed and compressed BN254 final exponentiation','--',rel)
commit=git('rev-parse','HEAD',capture=True).strip()
git(*auth,'push',url,'HEAD:refs/heads/'+branch)
assert git(*auth,'ls-remote',url,'refs/heads/'+branch,capture=True).split()[0]==commit
save(d/'published.json',dict(commit=commit,verified=v,selected=selection))
note=(f"867 pushed at {commit}. {decision} {v['new_exports']} independently verified exports. "
      f"Ledger {d}/ledger.json, {v['cumulative']} distinct proofs, SHA256 {v['ledger_sha256']}. Report {rel}/README.md.")
for f in ('zopro-cpu-perf-handoff.md','zopro-next-agent-prompt.md'):
    p=repo.parent.parent/f
    p.write_text('# Signed and compressed BN254 final exponentiation on Ryzen\n\n'+note+'\n\n'+p.read_text())
