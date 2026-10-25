from pathlib import Path
import sys,os,subprocess,json,shutil,time
out=Path(sys.argv[1]);source=Path(sys.argv[2]);binary=sys.argv[3];out.mkdir();inputs=out/'input';inputs.mkdir();target=inputs/source.name
with source.open('rb')as a,target.open('wb')as b:shutil.copyfileobj(a,b,1<<20)
ballast=bytearray(32<<20);group=Path('/sys/fs/cgroup')/Path('/proc/self/cgroup').read_text().strip().split('::',1)[1].lstrip('/');env=dict(os.environ,ZOPRO_BASIS_KEY_DIR=str(inputs));cmd=[binary,'1x2','51','1','4',str(out/'proofs')]
with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:
 p=subprocess.Popen(cmd,env=env,stdout=fo,stderr=fe);timeout=False
 try:code=p.wait(timeout=90)
 except subprocess.TimeoutExpired:timeout=True;p.kill();code=p.wait(timeout=10)
 metrics={n:(group/n).read_text()for n in ['memory.current','memory.peak','memory.high','memory.max','memory.swap.max','memory.events','memory.pressure','memory.stat','cpu.stat']};(out/'receipt.json').write_text(json.dumps(dict(time=time.time(),returncode=code,timeout=timeout,ballast_bytes=len(ballast),metrics=metrics),indent=2)+'\n')
target.unlink();inputs.rmdir();raise SystemExit(124 if timeout else code)
