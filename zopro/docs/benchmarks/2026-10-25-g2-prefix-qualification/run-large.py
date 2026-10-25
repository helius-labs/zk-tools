from pathlib import Path
import os,sys,subprocess,time,json,uuid,threading
r=Path('/tmp/zopro-g2-prefix-qualification');label=sys.argv[1];cmd=sys.argv[2:];unit='zopro-g2-prefix-large-'+label+'-'+uuid.uuid4().hex[:8];out=r/'runs'/label;out.mkdir(exist_ok=False)
env=dict(os.environ,GOMEMLIMIT='96GiB',GOMAXPROCS=os.environ.get('FORESTER_WORKERS','32'),RAYON_NUM_THREADS=os.environ.get('FORESTER_WORKERS','32'))
prefix=['systemd-run','--user','--unit='+unit,'--service-type=exec','--wait','--pipe','--property=RemainAfterExit=yes','--property=MemoryAccounting=yes','--property=MemoryMax='+str(int(os.environ['FORESTER_MEMORY_GIB'])*2**30),'--property=MemorySwapMax=0','--property=OOMPolicy=stop']
for k,v in env.items():
 if k.startswith(('ZOPRO_','MALLOC_','GOMEMLIMIT','GOMAXPROCS','RAYON_NUM_THREADS','AEGLOS_BENCH_')):prefix+=['--setenv='+k+'='+v]
(out/'command.json').write_text(json.dumps(dict(command=cmd,unit=unit,launcher=prefix,environment={k:v for k,v in env.items() if k.startswith(('ZOPRO_','MALLOC_','GOMEMLIMIT','GOMAXPROCS','RAYON_NUM_THREADS','AEGLOS_BENCH_'))}),indent=2)+'\n')
t=time.time();snap={};peak=0
with (out/'stdout.log').open('w') as log,(out/'memory.jsonl').open('w') as mem:
 p=subprocess.Popen(prefix+cmd,stdout=log,stderr=subprocess.STDOUT,env=env)
 cg=None
 while p.poll() is None:
  if cg is None:
   path=subprocess.run(['systemctl','--user','show',unit,'--property=ControlGroup','--value'],capture_output=True,text=True).stdout.strip()
   if path:cg=Path('/sys/fs/cgroup')/path.lstrip('/')
  if cg is not None:
   try:
    snap={name:(cg/name).read_text().strip() for name in ['memory.current','memory.peak','memory.swap.peak','memory.events','cpu.stat','memory.stat']};peak=max(peak,int(snap['memory.peak']));mem.write(json.dumps(dict(seconds=time.time()-t,**snap))+'\n');mem.flush()
   except FileNotFoundError:pass
  state=subprocess.run(['systemctl','--user','show',unit,'--property=SubState','--value'],capture_output=True,text=True).stdout.strip()
  if state in ['exited','failed']:break
  time.sleep(1)
 properties=subprocess.run(['systemctl','--user','show',unit,'--property=Result,ExecMainCode,ExecMainStatus,MemoryPeak,MemorySwapPeak,CPUUsageNSec,ActiveState,SubState'],capture_output=True,text=True).stdout
 subprocess.run(['systemctl','--user','stop',unit],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
 p.wait(timeout=30)
 fields=dict(line.split('=',1)for line in properties.splitlines()if '='in line)
 normal_exit=fields.get('Result')=='success'and fields.get('ExecMainCode')=='1'and fields.get('ExecMainStatus')=='0'
 returncode=p.returncode if p.returncode else (0 if normal_exit else 125)
 result=dict(label=label,returncode=returncode,launcher_returncode=p.returncode,normal_main_exit_observed=normal_exit,seconds=time.time()-t,last_cgroup=snap,sampled_peak_bytes=peak,systemd_properties=properties)
 (out/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result),flush=True)
 subprocess.run(['systemctl','--user','stop',unit],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
 subprocess.run(['systemctl','--user','reset-failed',unit],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
 sys.exit(returncode)
