from pathlib import Path
import sys,os,subprocess,time,json,shutil
out=Path(sys.argv[1]);source=Path(sys.argv[2]);binary=sys.argv[3];out.mkdir();inputs=out/'input';inputs.mkdir();target=inputs/source.name
with source.open('rb')as a,target.open('wb')as b:shutil.copyfileobj(a,b,1<<20)
g=Path('/sys/fs/cgroup')/Path('/proc/self/cgroup').read_text().strip().split('::',1)[1].lstrip('/');(out/'group').write_text(str(g))
for n in ['ctl','ack']:(out/n).touch()
e=dict(os.environ,ZOPRO_BASIS_KEY_DIR=str(inputs),ZOPRO_PRESSURE_PAUSE=str(out));cmd=[binary,'1x2','2',str(out/'proofs'),str(out/'ctl'),str(out/'ack')];timeout=False;ballast=None
with(out/'stdout.log').open('w')as fo,(out/'stderr.log').open('w')as fe:
 p=subprocess.Popen(cmd,env=e,stdout=fo,stderr=fe);(out/'sdk-pid').write_text(str(p.pid));start=time.monotonic()
 try:
  while not(out/'ready').exists():
   if p.poll()is not None:raise RuntimeError('ended before barrier')
   if time.monotonic()-start>25:raise TimeoutError('warm barrier')
   time.sleep(.02)
  ballast=bytearray(32<<20)
  for offset in range(0,len(ballast),4096):ballast[offset]=1
  (out/'ballast-ready').write_text('ready')
  try:code=p.wait(timeout=50)
  except subprocess.TimeoutExpired:timeout=True;p.kill();code=p.wait(timeout=10)
 finally:
  if p.poll()is None:p.kill();p.wait(timeout=10)
 metrics={n:(g/n).read_text()for n in ['memory.current','memory.peak','memory.high','memory.max','memory.swap.max','memory.events','memory.pressure','memory.stat','cpu.stat']}
 (out/'receipt.json').write_text(json.dumps(dict(time=time.time(),returncode=code,timeout=timeout,ballast_bytes=len(ballast),metrics=metrics),indent=2)+'\n')
target.unlink();inputs.rmdir();raise SystemExit(124 if timeout else code)
