from pathlib import Path
import hashlib,json,time,shutil
r=Path(__file__).resolve().parent;prior=Path('/tmp/zopro-small-core-entry-native-qualification')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert json.loads((r/'functional-complete.json').read_text())['passed']
source=(prior/'run.py').read_text();(r/'prior-resource-controller.py').write_text(source)
source=source.replace("p=json.load(open(r/'protocol.json'))","p=json.load(open(r/'resource-protocol.json'))")
source=source.replace("requestp=json.load(open('/tmp/zopro-worker-groups-request-entry/protocol.json'));",'')
start=source.index("for base in ['/tmp/zopro-small-core-entry'");end=source.index('base={k:v',start)
source=source[:start]+'''old=set(json.load(open('/tmp/zopro-operation-model/prior-inventory.json'))['sha256s'])
for root in [Path('/tmp/zopro-operation-model'),parent]:
 for n,h in json.load(open(root/'proof-inventory.json')).items():
  if n.endswith('.proof'):old.add(h)
for n,h in json.load(open(r/'functional-proof-inventory.json')).items():
 f=r/n
 if n.endswith('.proof'):old.add(h)
 else:refs.setdefault(f.parent.name,f.read_bytes())
assert len(old)==1726
'''+source[end:]
source=source.replace("requestp['images']","a['basis_images']")
source=source.replace(",'ZOPRO_SMALL_CORE_ENTRY_TRACE':'1','ZOPRO_CPU_WORKSPACE_TRACE':'1'",'')
source=source.replace("'ZOPRO_CPU_SMALL_CORE_ENTRY'","'ZOPRO_CPU_G2_PREFIX_RECORDS'")
lines=[]
for line in source.splitlines():
 if line.startswith(' entered=len('):continue
 if ';entered=len('in line:line=line.split(';entered=len(')[0]
 if 'if passed:verify(out,1);assert entered=='in line:line=' if passed:verify(out,1)'
 line=line.replace(',entry_hits=entered','')
 lines.append(line)
source='\n'.join(lines)+'\n'
source=source.replace('zopro-small-core-entry-q-','zopro-g2-prefix-q-').replace("str(r/'child.py')","str(r/'pressure-child.py')")
source=source.replace("*[k+'='+v for k,v in env.items()]","*[k+'='+v for k,v in env.items()if k.startswith(('ZOPRO_','RAYON_','MALLOC_'))]")
source=source.replace("r/'started.json'","r/'resource-started.json'").replace("r/'proof-inventory.json'","r/'resource-proof-inventory.json'").replace("r/'complete.json'","r/'resource-complete.json'")
source=source.replace('SMALL_CORE_ENTRY_RESOURCE_QUALIFICATION_COMPLETE','G2_PREFIX_RESOURCE_QUALIFICATION_COMPLETE')
assert 'requestp'not in source and 'entered='not in source and 'entry_hits'not in source
compile(source,str(r/'resources.py'),'exec');(r/'resources.py').write_text(source)
child=(r/'child.py').read_text();assert'ballast=bytearray(32<<20);'in child
child=child.replace("ballast=bytearray(32<<20);(out/'ballast-ready').write_text('ready')","ballast=bytearray(32<<20)\n  for offset in range(0,len(ballast),4096):ballast[offset]=1\n  (out/'ballast-ready').write_text('ready')")
compile(child,str(r/'pressure-child.py'),'exec');(r/'pressure-child.py').write_text(child)
p=json.loads((r/'protocol.json').read_text());p.update(declared_resources=time.time(),build_manifest_sha256=p['parent_build_sha256'],ordinary_concurrency=[dict(label='candidate',**c)for c in p['ordinary_concurrency']],consumer_sources=p['consumer_sources']|{'pressure-child.py':sha(r/'pressure-child.py'),'resources.py':sha(r/'resources.py')},functional_result_sha256=sha(r/'functional-complete.json'),gate='All4 concurrency cases32SDK32exports; all9 warm-pressure cases45SDK9exports. Warm512MiB then128soft/192hard, swap0, explicitly page-touched32MiB ballast and2postproofs. No max/oom/oom_kill events or timeout. Preserve all failures without unchanged retry.',prior_controller_sha256=sha(r/'prior-resource-controller.py'))
(r/'resource-protocol.json').write_text(json.dumps(p,indent=2)+'\n');print('RESOURCE_CONTROLLER_PREPARED')
