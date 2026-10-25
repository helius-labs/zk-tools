from pathlib import Path
import json,hashlib,time
r=Path(__file__).resolve().parent;prior=Path('/tmp/zopro-small-core-entry-native-qualification')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert json.loads((r/'complete.json').read_text())['passed']and json.loads((r/'cold-complete.json').read_text())['passed']
for name in ['run-large.py','large-helpers.py','large.py']:
    source=(prior/name).read_text().replace('/tmp/zopro-small-core-entry-native-qualification',str(r))
    source=source.replace('zopro-small-core-entry-large-','zopro-g2-prefix-large-')
    if name=='large-helpers.py':
        original="env=dict(os.environ,**json.loads((prior/'environment.json').read_text()),ZOPRO_CPU_WORKSPACE_TRACE='1',ZOPRO_CPU_GROUP_TRACE='1',ZOPRO_CPU_SMALL_CORE_ENTRY='1',ZOPRO_SMALL_CORE_ENTRY_TRACE='1')"
        replacement="parent=Path('/tmp/zopro-g2-prefix-sdk'); accepted=json.loads((parent/'accepted.json').read_text()); env={k:v for k,v in os.environ.items()if not k.startswith(('ZOPRO_','RAYON_','MALLOC_','CROSS_'))}|accepted['environment']|{'ZOPRO_CPU_G2_PREFIX_RECORDS':'1','ZOPRO_CPU_WORKSPACE_TRACE':'1'}"
        assert original in source;source=source.replace(original,replacement)
        source=source.replace("assert 'SMALL_CORE_ENTRY entered'not in(r/'runs'/label/'stdout.log').read_text(), 'large circuit must retain original caller entry'","assert 'G2_PREFIX_RECORDS'not in(r/'runs'/label/'stdout.log').read_text(), 'per-batch trace must remain disabled'")
        original="folder=Path('/tmp/zopro-canonical-grain-qualification')/f'proofs-prove-{batch_old}-95'\n  old.update(sha(f)for f in folder.glob('*.proof'))"
        replacement="for old_root in ['/tmp/zopro-canonical-grain-qualification','/tmp/zopro-small-core-entry-native-qualification']:\n   folder=Path(old_root)/f'proofs-prove-{batch_old}-95'\n   old.update(sha(f)for f in folder.glob('*.proof'))"
        assert original in source;source=source.replace(original,replacement)
    if name=='large.py':
        source=source.replace("assert json.loads((ROOT/'complete.json').read_text())['passed']","assert json.loads((ROOT/'complete.json').read_text())['passed'] and json.loads((ROOT/'cold-complete.json').read_text())['passed'] and json.loads((ROOT/'functional-complete.json').read_text())['passed']")
    compile(source,str(r/name),'exec');(r/name).write_text(source)
p=json.loads((r/'protocol.json').read_text());(r/'large-protocol.json').write_text(json.dumps(dict(declared=time.time(),cap_gib=95,swap=0,batches=[250,4999],proofs_each=2,source_hashes={n:sha(r/n)for n in ['large.py','large-helpers.py','run-large.py','image-cache.py']},native_environment=p['native_environment'],inputs=p['large']['image_identities'],purpose='Exact candidate native libraries and fatLTO forester consumer; cold original image cache, one cold and one warm independently verified proof each. No matched engine latency comparison. Any failure retained without undeclared fallback or repeat.'),indent=2)+'\n');print('LARGE_QUALIFICATION_PREPARED')
