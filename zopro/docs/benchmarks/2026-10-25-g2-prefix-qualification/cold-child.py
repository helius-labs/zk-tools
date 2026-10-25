from pathlib import Path
import json,os,shutil,subprocess,sys,time
out=Path(sys.argv[1]);source=Path(sys.argv[2]);binary=sys.argv[3];inputs=out/'input';inputs.mkdir()
with source.open('rb')as src,(inputs/source.name).open('wb')as dst:shutil.copyfileobj(src,dst,1<<20)
group=Path('/sys/fs/cgroup')/Path('/proc/self/cgroup').read_text().strip().split('::',1)[1].lstrip('/')
env=dict(os.environ,ZOPRO_BASIS_KEY_DIR=str(inputs));cmd=[binary,'1x2','1','1','4',str(out/'proofs')]
with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:p=subprocess.run(cmd,env=env,stdout=fo,stderr=fe,timeout=60)
metrics={n:(group/n).read_text()for n in ['memory.peak','memory.max','memory.high','memory.swap.max','memory.events','memory.stat','cpu.stat']}
(out/'receipt.json').write_text(json.dumps(dict(completed=time.time(),returncode=p.returncode,metrics=metrics),indent=2)+'\n')
(inputs/source.name).unlink();inputs.rmdir();raise SystemExit(p.returncode)
