"""Compare two pinned merge key images on the same verified requests."""
import argparse,hashlib,json,os,random,subprocess
from pathlib import Path
root=Path(__file__).resolve().parent
def read(p):return json.loads(p.read_bytes())
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def write(p,r):
    tmp=p.with_suffix('.tmp');tmp.write_text(json.dumps(r,indent=2)+'\n');tmp.replace(p)
parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('output',type=Path);parser.add_argument('--ryzen',action='store_true');args=parser.parse_args()
out=args.output.resolve();out.mkdir(parents=True,exist_ok=True);cfg=read(root/'config.json');widths=[16,32] if args.ryzen else [64,96];blocks=3 if args.ryzen else 5
assert set(range(max(widths))).issubset(os.sched_getaffinity(0))
cp=Path('/sys/devices/system/cpu')
top=[[(cp/f'cpu{i}/topology'/k).read_text().strip() for k in ['physical_package_id','core_id','thread_siblings_list']] for i in range(max(widths))]
if not args.ryzen:assert len({tuple(l[:2]) for l in top})==96
assert all(sha(root/k)==v for k,v in cfg['hashes'].items())
host=dict(cpu=subprocess.check_output(['lscpu'],text=True),topology=top,hashes=cfg['hashes'],widths=widths)
if (out/'host.json').exists():assert read(out/'host.json')==host
else:write(out/'host.json',host)
rows=read(out/'rows.json') if (out/'rows.json').exists() else []
for k in range(blocks):
    todo=[(w,m) for w in widths for m in ['ordinary','mapped']];random.Random(577+k).shuffle(todo)
    for w,m in todo:
        q=out/f'{k}-{w}-{m}';q.mkdir(exist_ok=True);allrows=[r for r in rows if (r['block'],r['w'],r['mode'])==(k,w,m)]
        names=list(cfg['requests']);random.Random(1577+k).shuffle(names)
        if allrows:
            assert len(allrows)==len(names) and [r['key'] for r in allrows]==names
            for i,r in enumerate(allrows):assert sha(q/f'{i}.proof')==r['proof'] and sha(q/f'{i}.public')==cfg['requests'][r['key']]['public']
            continue
        job=q/'manifest.json';write(job,[str(root/f'requests/{name}.json') for name in names])
        env={k:v for k,v in os.environ.items() if not k.startswith(('ZOPRO_','RAYON_','MALLOC_'))}
        env.update(cfg['environment'],RAYON_NUM_THREADS=str(w),ZOPRO_BASIS_KEY_DIR=str(root/'images'),ZOPRO_CPU_LAGRANGE_DIR=str(out/'cache'))
        key=root/f'images/{m}.image'
        with (q/'sdk.log').open('w') as f:subprocess.run(['taskset','-c',f'0-{w-1}',root/'bin/sdk',key,job,'11',q],env=env,stdout=f,stderr=subprocess.STDOUT,check=True,timeout=600)
        ls=(q/'sdk.log').read_text().splitlines()
        assert sum(l.startswith('ORIGIN_MAPPED ') for l in ls)==int(m=='mapped') and not any(l.startswith('ORIGIN_KEY ') for l in ls)
        assert [json.loads(l[10:]) for l in ls if l.startswith('MAIN_CPUS ')]==[list(range(w))]
        su=[json.loads(l[8:]) for l in ls if l.startswith('SUMMARY ')];xs=[json.loads(l[7:]) for l in ls if l.startswith('SAMPLE ')];assert len(su)==len(names)
        current=[]
        for i,name in enumerate(names):
            p=q/f'{i}.proof';pub=q/f'{i}.public';expected=cfg['requests'][name]['public'];assert sha(pub)==expected;digest=sha(p)
            msg=subprocess.check_output([root/'bin/verify',root/'images/ordinary.image',cfg['hashes']['images/ordinary.image'],p,pub,expected],text=True)
            assert 'VERIFIED proof_sha256='+digest in msg;(q/f'{i}.verify').write_text(msg)
            data=[x['ms'] for x in xs if x['input']==i];assert [x['round'] for x in xs if x['input']==i]==list(range(11)) and su[i]['input']==i and su[i]['workers']==w and su[i]['priming']==3
            current.append(dict(block=k,w=w,mode=m,key=name,proof=digest,samples=data,summary=su[i]))
        rows.extend(current);write(out/'rows.json',rows)
assert len({r['proof'] for r in rows})==len(rows)==blocks*len(widths)*2*len(cfg['requests'])
write(out/'complete.json',dict(verified=len(rows),timed=len(rows)*11))
