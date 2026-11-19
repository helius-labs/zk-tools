from zp_report_common import *
import tempfile,os,random,time
d=base/'image-pages723';b=base/'aws-wide663/bundle';xtask=b/'bin/xtask';src=rd(base/'reference-panel653/workflow.json');spec={s['name']:s for s in src['steps']};meta=rd(base/'catalog-holdout700/inputs.json');shapes=['12x2','16x2','merge-24x1'];refs={'12x2':(284,101),'16x2':(335,124),'merge-24x1':(507,175)};prior=base/'dedup-scope722/ledger.json'
while not (d/'built.json').exists():time.sleep(2)
sdk=d/'target/x86_64-unknown-linux-gnu/release/examples/cpu-sdk';obs=[];frozen={}
for sh in shapes:
 template=spec['b0-'+sh+'-profile'];image=Path(template['argv'][4]);ident=sha(image);assert ident==rd(base/'reference-panel653/pins.json')[str(image)]
 with tempfile.TemporaryDirectory(prefix='zopro-image-pages-',dir='/dev/shm') as work:
  backing={}
  for m in ['large','base']:
   f=Path(work)/m;shutil.copyfile(image,f);f.chmod(0o400);assert sha(f)==ident;backing[m]=f
  for stage,blocks in [('screen',5),('confirm',7)]:
   if stage=='confirm' and not frozen[sh]:continue
   jobs=[];steps=[];pins=rd(base/'reference-panel653/pins.json');pins[str(sdk)]=sha(sdk)
   for block in range(blocks):
    hs=['catalog','second'];random.Random(72300+block+100*(stage=='confirm')).shuffle(hs)
    for h in hs:
     modes=['frozen','ordinary','base','large'];random.Random(7230+shapes.index(sh)*4+int(h=='second')+block//2+100*(stage=='confirm')).shuffle(modes)
     if block%2:modes.reverse()
     for mode in modes:
      ref='b0-'+sh+'-profile';s,v=[json.loads(json.dumps(spec[ref+k])) for k in ['', '-verify']];n=f'{stage}-b{block}-{sh}-{h}-{mode}';q=d/'proofs'/n;s['name']=n;v['name']=n+'-verify';s['argv'][-2:]=['11',str(q)];v['argv'][3:5]=[str(q/'0.proof'),str(q/'0.public')]
      if mode!='frozen':s['argv'][3]=str(sdk)
      if mode in backing:s['argv'][4]=str(backing[mode]);pins[str(backing[mode])]=ident;s['environment']['ZOPRO_TRIAL_IMAGE_PAGES']=mode
      if h=='second':
       req=base/'catalog-holdout700/inputs'/sh/'request.json';s['argv'][5]=str(req);pins[str(req)]=meta[sh]['request'];v['argv'][5]=meta[sh]['public']
      steps.extend([s,v]);jobs.append(dict(name=n,sh=sh,h=h,m=mode,b=block,stage=stage,q=str(q),pub=v['argv'][5]))
   key=stage+'-'+sh;pp=d/(key+'-pins.json');save(pp,pins);wf=dict(version=1,environment=src['environment'],source_pins=str(pp),steps=steps);save(d/(key+'.json'),wf);out=d/(key+'-results');subprocess.run([xtask,'workflow',d/(key+'.json'),out],check=True)
   for j in jobs:
    q=Path(j['q']);proof=sha(q/'0.proof');assert sha(q/'0.public')==j['pub'] and f"VERIFIED proof_sha256={proof} public_sha256={j['pub']}" in (out/(j['name']+'-verify')/'stdout.log').read_text()
    ls=(out/j['name']/'stdout.log').read_text().splitlines();ss=[json.loads(x[7:]) for x in ls if x.startswith('SAMPLE ')];su,=[json.loads(x[8:]) for x in ls if x.startswith('SUMMARY ')];assert len(ss)==su['rounds']==11 and su['priming']==3;marks=[x.split()[1:] for x in ls if x.startswith('IMAGE_PMD_KIB ')]
    expected=image.stat().st_size//(2<<20)*2048 if j['m']=='large' else 0
    assert marks==([[str(expected),str(image.stat().st_size)]] if j['m'] in backing else [])
    obs.append(j|dict(proof=proof,ms=mean(x['ms'] for x in ss),rss=su['whole_process_maxrss_kib']/1024,pmd_kib=expected if marks else None,samples=ss,summary=su))
   if stage=='screen':
    def rs(h,m):return [r for r in obs if (r['sh'],r['stage'],r['h'],r['m'])==(sh,stage,h,m)]
    frozen[sh]=all(pair(rs(h,'large'),rs(h,m))[0]<0 and pair(rs(h,'large'),rs(h,m))[1]>=3 for h in ['catalog','second'] for m in ['base','frozen'])
    save(d/'selection.json',frozen)
   save(d/'rows.json',obs)
a=docs/'2026-11-19-image-pages723/artifacts';a.mkdir(parents=True,exist_ok=True);stats={};tab=[]
for sh in shapes:
 for stage in ['screen','confirm']:
  for h in ['catalog','second']:
   get=lambda m:[r for r in obs if (r['sh'],r['stage'],r['h'],r['m'])==(sh,stage,h,m)];large=get('large')
   if not large:continue
   base4,orig,old=map(get,['base','ordinary','frozen']);p,w=pair(large,base4);v,k=pair(large,old);rc,rm=refs[sh];t=med(large);n=len(large);stats[sh+'-'+stage+'-'+h]=dict(ms=[med(rs) for rs in [large,base4,orig,old]],vs_base=[p,w],vs_frozen=[v,k],peak_mib=[peak(rs) for rs in [large,base4,orig,old]],m5_speedup=[rc/t,rm/t] if h=='catalog' else None,passes=sum(r['ms']*10<=rc for r in large) if h=='catalog' else None);ref=f'{rc/t:.2f}× / {rm/t:.2f}×' if h=='catalog' else '—';tab.append(f"| {sh} | {stage} | {h} | "+' / '.join(f'{med(rs):.3f}' for rs in [large,base4,orig,old])+f" | {p:+.2f}% ({w}/{n}) | {v:+.2f}% ({k}/{n}) | {large[0]['pmd_kib']/1024:.0f} | {peak(large):.1f} / {peak(base4):.1f} | {ref} |")
save(a/'summary.json',stats);save(a/'selection.json',frozen);save(a/'verification.json',extend(d,prior,[r['proof'] for r in obs]));pack(a/'observations.json.gz',obs);shutil.copyfile(d/'source.patch',a/'source.patch');save(a/'binaries.json',rd(d/'built.json'));shutil.copyfile('/tmp/zp_run723.py',a/'measure.py')
(a.parent/'README.md').write_text('''# Page size of immutable proving-key images on Ryzen

This experiment changes the page tables used for the existing proving key. An aligned read-only shared mapping of one tmpfs copy uses 2 MiB pages. A separate byte-identical tmpfs copy uses base pages at the same alignment. Both are populated before setup. The benchmark checks the kernel’s ShmemPmdMapped count for the exact mapping before priming and requires full coverage of the aligned interior for the large-page arm and zero coverage for the base-page arm. A same-binary ordinary file mapping and the earlier experimental executable supply full-proof controls. All remaining key material uses its usual backing. Temporary copies are removed after each shape. Peak RSS is per process and excludes the other fixture’s tmpfs pages. Copying and page preparation are outside warm latency.

Five paired blocks per transaction screen three reference keys. Seven fresh paired blocks are scheduled on a key only when its large-page arm beats both base pages and the earlier executable on paired median and in at least three of five blocks on each transaction. Each process measures eleven verified requests after three warmups. The M5 CPU and Metal references apply to the catalog inputs.

| Shape | Stage | Input | Large / base / ordinary / earlier ms | vs base | vs earlier | Large-page MiB | Peak MiB large / base | vs M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|
'''+ '\n'.join(tab)+'\n\n[Observations](artifacts/observations.json.gz) · [Mapping source](artifacts/source.patch) · [Verification](artifacts/verification.json)\n');save(d/'audited.json',dict(qualified=frozen,exports=len(obs)))
