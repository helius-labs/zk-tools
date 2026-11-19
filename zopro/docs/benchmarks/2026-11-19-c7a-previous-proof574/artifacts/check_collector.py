"""Verify the archived C7a indexed-collection instruction samples."""
from collections import Counter, defaultdict
from pathlib import Path
import argparse
import hashlib
import json
import re
import subprocess

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument("elf",type=Path)
parser.add_argument("trace_directory",type=Path)
parser.add_argument("output",type=Path)
args=parser.parse_args()
sdk=args.elf
def sha(p):
    with p.open("rb") as stream:return hashlib.file_digest(stream,"sha256").hexdigest()
def rd(p):return json.loads(p.read_bytes())
def save(p,obj):p.write_text(json.dumps(obj,indent=2)+"\n")
assert sha(sdk)=="37caff895102f26611b0708654f505c4244c8d21892e5f0c58a9cfec0898a9d0"
symbols=defaultdict(list)
for line in subprocess.check_output(['nm','-SnC','--defined-only',sdk],text=True).splitlines():
    fields=line.split(None,3)
    if len(fields)==4 and fields[2] in 'TtWw':
        lo,n=int(fields[0],16),int(fields[1],16)
        if n:symbols[fields[3]].append((lo,lo+n))
root=args.trace_directory;ips=Counter();maps={};rows=[];report=rd(Path(__file__).with_name('gap-samples.json'))
for proc in sorted({r['proc'] for r in report}):
    tags=[];anchors=[];inside=False
    for line in (root/(proc+'.trace')).open():
        if 'zp363stages:interp_leave:' in line:inside=True;tags.append([])
        elif 'zp363stages:fft_entry:' in line:inside=False
        elif inside and 'cpu-clock' in line:
            _,_,_,ip,s=line.split(None,4);s=s.strip();addr=int(ip,16)
            if s in ['<crossbeam_epoch::internal::Global>::try_advance','<rayon_core::registry::WorkerThread>::wait_until_cold','<std::sys::sync::mutex::futex::Mutex>::lock_contended','<rayon_core::sleep::Sleep>::wake_specific_thread']:
                anchors.append((s,addr))
            if 'IterProducer<u32>' in s and 'zopro_core::Key>::solve_inner' in s and 'CollectConsumer<zopro_bn254::fp::Fp<zopro_bn254::FrConfig>>' in s:tags[-1].append((s,addr))
    assert len(tags)==11
    possible=None;used=set()
    for s,addr in anchors:
        if len(symbols[s])!=1:continue
        lo,hi=symbols[s][0];cand=set(range((addr-hi+4096)//4096*4096,(addr-lo)//4096*4096+1,4096));assert cand
        possible=cand if possible is None else possible&cand;used.add(s)
    assert len(possible)==1 and len(used)>=3,(proc,possible,used)
    slide=next(iter(possible));maps[proc]=dict(page_base=hex(slide),anchors=sorted(used));local=[]
    for i,each in enumerate(tags):
        assert len(each)==next(r for r in report if r['proc']==proc and r['i']==i)['counts'].get('indexed field collection',0)
        bins=Counter()
        for name,addr in each:
            rel=addr-slide;match=[v for v in symbols[name] if v[0]<=rel<v[1]]
            assert len(match)==1,(name,hex(rel),symbols[name][:1])
            ips[rel]+=1;bins[rel]+=1
        local.append(dict(i=i,pcs=dict(bins)))
    rows.append(dict(proc=proc,rows=local))
addrlist=sorted(ips);input_=''.join(hex(v)+'\n' for v in addrlist)
lines=subprocess.check_output(['addr2line','-afiC','-e',sdk],input=input_,text=True).splitlines()
by={};cur=None
for l in lines:
    if re.fullmatch(r'0x[0-9a-fA-F]+',l):
        cur=int(l,16);by[cur]=[]
    else:by[cur].append(l)
assert list(by)==addrlist
dis={};mn=re.compile(r'^\s*([a-f0-9]+):\s*([.a-zA-Z0-9_]+)\s*(.*)$')
# One disassembly across the sampled ranges keeps all control flow addresses numeric.
for l in subprocess.check_output(['objdump','-d','-M','intel','--no-show-raw-insn','--start-address='+hex(min(addrlist)),'--stop-address='+hex(max(addrlist)+20),sdk],text=True).splitlines():
    m=mn.match(l)
    if m:
        pc=int(m[1],16)
        if pc in ips:dis[pc]=(m[2],m[3])
assert len(dis)==len(ips)
save(args.output,dict(elf=sha(sdk),maps=maps,rows=rows,pcs={hex(k):dict(assembly=dis[k],frames=by[k],n=ips[k]) for k in ips}))
