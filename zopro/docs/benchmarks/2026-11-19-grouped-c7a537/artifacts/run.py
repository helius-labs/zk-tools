"""Compare 24×2 grouped proofs and Aeglos on original and held-out requests."""
from pathlib import Path
from statistics import median, mean
import argparse
import hashlib
import json
import os
import random
import subprocess

def load(p):
    return json.loads(p.read_bytes())

def digest(p):
    with p.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

root = Path(__file__).resolve().parent
cfg = load(root/'config.json')
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('output', type=Path)
parser.add_argument('--smoke', action='store_true')
parser.add_argument('--shapes', nargs='+', choices=list(cfg['shapes']), default=list(cfg['shapes']))
args = parser.parse_args()
widths = (16, 32) if args.smoke else (32, 64, 96)
blocks = 1 if args.smoke else 5
for name, expected in cfg['hashes'].items():
    assert digest(root/name) == expected, name
assert set(range(max(widths))) <= os.sched_getaffinity(0)
topology = [{key:(Path(f'/sys/devices/system/cpu/cpu{i}/topology')/key).read_text().strip()
             for key in ('core_id','physical_package_id','thread_siblings_list')} for i in range(max(widths))]
if not args.smoke:
    assert len({(p['physical_package_id'],p['core_id']) for p in topology}) == 96
out=args.output.resolve()
out.mkdir(parents=True,exist_ok=False)
(out/'host.json').write_text(json.dumps(dict(topology=topology,lscpu=subprocess.check_output(['lscpu'],text=True),smoke=args.smoke,hashes=cfg['hashes']),indent=2))
modes=['release','ordinary','outer','group','group-outer','aeglos']
rows=[]
for k in range(blocks):
    jobs=[(sh,w,m) for sh in args.shapes for w in widths for m in modes]
    random.Random(537+k).shuffle(jobs)
    for sh,w,m in jobs:
        c=cfg['shapes'][sh]
        front=min(c['front'],w//2)
        n=1 if args.smoke else 11
        dest=out/f'{k}-{sh}-{w}-{m}'
        e={key:val for key,val in os.environ.items() if not key.startswith(('ZOPRO_','AEGLOS_','MALLOC_','RAYON_','GOMAXPROCS'))}
        e.update(cfg['base_env'],RAYON_NUM_THREADS=str(w),ZOPRO_BASIS_KEY_DIR=str(root/'images'),ZOPRO_CPU_LAGRANGE_DIR=str(out/'cache'))
        e['LD_LIBRARY_PATH']=str(root/'lib')+((':'+e['LD_LIBRARY_PATH']) if e.get('LD_LIBRARY_PATH') else '')
        prefix=['taskset','-c',f'0-{w-1}']
        if m=='aeglos':
            e.update(AEGLOS_CPU_THREADS=str(w),GOMAXPROCS=str(w),AEGLOS_BENCH_DISABLE_SHIFTED='0',AEGLOS_BENCH_MSM_PHASES='0',ZOPRO_BENCH_PROOFS=str(dest))
            cmd=[root/'bin/aeglos','aeglos',root/c['aeglos_key'],root/c['request'],str(n)]
            proofdir=dest/c['id']
        else:
            e.update(c['env'])
            if c['wire']:e['ZOPRO_BENCH_WIRE_FILE']=str(root/c['wire'])
            if front:e['ZOPRO_BENCH_FRONT_CPUS']=','.join(map(str,range(front)))
            if m!='release':e['ZOPRO_REPORT_IDLE_ROUNDS']='1'
            if m in ('outer','group-outer'):
                e.update(ZOPRO_IDLE_CONTEXT='outer',ZOPRO_IDLE_PROBES='128')
            if m in ('group','group-outer'):
                e.update(ZOPRO_TRIAL_GROUPS=str(root/'plans/groups.bin'),ZOPRO_LINEAR_GROUPS=str(root/'plans'),ZOPRO_ADAPTIVE_CORRECTIONS='1',ZOPRO_GROUP_SAMPLE='1')
            cmd=[root/'bin'/('release' if m=='release' else 'candidate'),root/c['image'],root/c['request'],str(n),dest]
            proofdir=dest
        log=dest.with_suffix('.log')
        with log.open('w') as stream:subprocess.run(prefix+cmd,env=e,stdout=stream,stderr=subprocess.STDOUT,check=True,timeout=240)
        proof=digest(proofdir/'0.proof')
        assert digest(proofdir/'0.public')==c['public_sha256']
        checked=subprocess.check_output([root/'bin/verify',root/c['image'],c['image_sha256'],proofdir/'0.proof',proofdir/'0.public',c['public_sha256']],text=True)
        assert 'VERIFIED proof_sha256='+proof in checked
        dest.with_suffix('.verify').write_text(checked)
        ls=log.read_text().splitlines()
        def tagged(prefix): return [line[len(prefix):] for line in ls if line.startswith(prefix)]
        su,=map(json.loads,tagged('SUMMARY '))
        xs=list(map(json.loads,tagged('SAMPLE ')))
        assert su['priming']==3 and su['rounds']==n and [x['round'] for x in xs]==list(range(n))
        if m=='aeglos':
            assert su['threads']==w and all(x['cache_hit'] for x in xs)
            key='verified_ms'
        else:
            key='ms'
            assert su['workers']==w and list(map(json.loads,tagged('MAIN_CPUS ')))==[list(range(w))]
            assert list(map(json.loads,tagged('FRONT_CPUS ')))==([list(range(front))] if front else [])
            assert bool(tagged('WIRE_FILE true'))==bool(c['wire'])
            for tag in ('GROUP_FILE true', 'LINEAR_FILE true'):
                assert bool(tagged(tag)) == (m in ('group','group-outer'))
            contexts=[tuple(map(int,line.split())) for line in tagged('IDLE_CONTEXT ')]
            assert bool(contexts) == (m != 'release')
            if contexts:
                expected = min(32,(128+w-2)//(w-1)) if m in ('outer','group-outer') else 32
                assert (w,32,expected) in contexts
        rows.append(dict(sh=sh,w=w,m=m,b=k,ms=mean(x[key] for x in xs),peak_mib=su['whole_process_maxrss_kib']/1024,proof=proof,summary=su,samples=xs))
        (out/'rows.json').write_text(json.dumps(rows))
assert len(rows)==len(set(r['proof'] for r in rows))==blocks*len(args.shapes)*len(widths)*len(modes)
table=[]
for sh in args.shapes:
    for w in widths:
        for m in modes:
            arm=[r for r in rows if (r['sh'],r['w'],r['m'])==(sh,w,m)]
            ms=median(r['ms'] for r in arm)
            table.append(dict(shape='24x2',request=sh,width=w,mode=m,ms=ms,max_rss_mib=max(r['peak_mib'] for r in arm),m5_cpu=528/ms if sh=='original' else None,m5_metal=176/ms if sh=='original' else None))
(out/'complete.json').write_text(json.dumps(dict(smoke=args.smoke,processes=len(rows),summary=table),indent=2))
