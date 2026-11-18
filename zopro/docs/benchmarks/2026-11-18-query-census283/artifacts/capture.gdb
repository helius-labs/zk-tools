set pagination off
set confirm off
set print thread-events off
set debuginfod enabled off
set language c
starti
python
import gdb,os,json
seen={}
class Probe(gdb.Breakpoint):
 def stop(self):
  n=int(gdb.parse_and_eval('$rdx'))
  assert n==int(gdb.parse_and_eval('$r8')) and 0<n<4000000
  reg=lambda r:int(gdb.parse_and_eval('$'+r))
  # One complete census per distinct raw input length. No scalar or point values are written.
  if n in seen:return False
  inf=gdb.selected_inferior(); bases=bytes(inf.read_memory(reg('rsi'),64*n)); words=bytes(inf.read_memory(reg('rcx'),32*n)); counts={}; unique=set(); eff=0
  for i in range(n):
   scalar=words[32*i:32*i+32]; val=int.from_bytes(scalar,'little');point=bases[i*64:i*64+64]; identity=point==bytes(64)
   if not identity and val:eff+=1
   for name,flag in [('identity',identity),('zero',val==0),('one',val==1),('fits64',val.bit_length()<=64)]:
    if flag:counts[name]=counts.get(name,0)+1
   unique.add(scalar)
  seen[n]=dict(n=n,nontrivial=eff,distinct=len(unique),counts=counts,stack=gdb.execute('bt 10',to_string=True).splitlines()[:10]);open(os.environ['CENSUS'],'w').write(json.dumps(list(seen.values()),indent=2))
  return False
bp=Probe('*_RINvNtCs90rqOEZpJ68_10zopro_core10msm_affine7msm_rawNtNtCs6pTJ8pTmkOJ_11zopro_bn2545curve8G1ConfigEB4_')
end
continue
