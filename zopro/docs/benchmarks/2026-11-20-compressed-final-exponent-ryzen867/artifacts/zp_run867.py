from zp_report_common import *
import random,time
d=base/'compressed-final-exponent-ryzen867';d.mkdir(exist_ok=True);(d/'cache').mkdir(exist_ok=True)
build=base/'compressed-final-exponent867'
release=base/'seed-vector-b2-sdk851/cpu-sdk'
for _ in range(210):
    if (build/'built.json').exists():break
    time.sleep(2)
else:raise RuntimeError('candidate not built')
trial=build/'cpu-sdk'
pins={str(trial):sha(trial),str(release):sha(release)}
assert pins[str(trial)]==rd(build/'built.json')['binary']
assert pins[str(release)]==rd(base/'seed-vector-b2-sdk851/built.json')['binary']
sets={
    '1x2':('raw-a-api737','/opt/zopro-aapi737/', 'main-b0-{h}-80-api-bypass'),
    '2x2':('critical-path-c7a750','/opt/zopro-critical750/','timing-b0-2x2-{h}-split-band'),
    '4x2':('b2-origin-sdk-c7a786','/opt/zopro-origin786/','timing-b0-4x2-{h}-release'),
}
def sources(sh):
    name,root,patt=sets[sh];b=base/name/'bundle';old=rd(b/'pins.json')
    wf={s['name']:s for s in rd(b/'workflow.json')['steps']}
    def path(rel):
        if not rel.startswith(root):return rel
        p=str(b/rel.removeprefix(root))
        if rel in old:
            k=sha(Path(p));assert old[rel]==k
            pins[p]=k
        return p
    return path,wf,patt
steps=[];jobs=[]
for sh in sets:
    path,wf,patt=sources(sh)
    xtask=path(sets[sh][1]+'bin/xtask')
def job(b,sh,h,mode,panel,n):
    path,wf,patt=sources(sh)
    ref=wf[patt.format(h=h)];ver=wf[ref['name']+'-verify']
    name=f'{panel}-b{b}-{sh}-{h}-{mode}'
    env={k:path(v) for k,v in ref['environment'].items() if not k.startswith('ZOPRO_TRIAL_')}
    env.update(RAYON_NUM_THREADS='24',ZOPRO_BENCH_FRONT_CPUS=','.join(map(str,range(24,32))),
               ZOPRO_CPU_LAGRANGE_DIR=str(d/'cache'))
    if sh=='2x2':
        env['ZOPRO_BENCH_RAW_G1_A_FRONTEND']='1'
        env['ZOPRO_CPU_MEDIUM_KH']='1'
    if sh=='4x2':
        env.update(ZOPRO_CPU_SOLVER_FRONTEND='1',ZOPRO_SOLVE_EPOCHS='4',ZOPRO_CPU_UNINIT_SOLVER='1',
                   ZOPRO_SOLVE_TASK_CLAIMS='seed',ZOPRO_CPU_POSEIDON_EMIT='1')
    if mode in ('signed','compressed'):
        env['ZOPRO_TRIAL_FINAL_EXP']=mode
    if panel=='clock':
        env['ZOPRO_TRIAL_FINAL_EXP_CLOCK']='1'
    args=[path(x) for x in ref['argv']]
    args[2]='0-31';args[3]=str(release if mode=='release' else trial)
    args[6]=str(n);args[7]=str(d/'proofs'/name)
    verify=[path(x) for x in ver['argv']]
    verify[3]=args[7]+'/0.proof';verify[4]=args[7]+'/0.public'
    steps.extend([ref|dict(name=name,argv=args,environment=env),ver|dict(name=name+'-verify',argv=verify)])
    jobs.append(dict(name=name,b=b,h=h,shape=sh,m=mode,verify=verify,panel=panel,rounds=n))
for b in range(7):
    selected=list(sets)
    random.Random(8670+b).shuffle(selected)
    for sh in selected:
        inputs=['catalog','heldout']
        if b % 2: inputs.reverse()
        for h in inputs:
            arms=['release','ordinary','signed','compressed']
            random.Random(86700+b//2+sum(map(ord,sh+h))).shuffle(arms)
            if b%2:arms.reverse()
            for m in arms:job(b,sh,h,m,'timing',13)
            if b<3:
                for m in (arms if b%2==0 else list(reversed(arms))):
                    if m!='release':job(b,sh,h,m,'clock',13)
assert len(jobs)==222, len(jobs)
save(d/'jobs.json',jobs);save(d/'pins.json',pins)
save(d/'workflow.json',dict(version=1,environment={},source_pins=str(d/'pins.json'),steps=steps))
subprocess.run([xtask,'workflow',d/'workflow.json',d/'results'],check=True)
