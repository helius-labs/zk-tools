"""Count samples at the two indexed-collector notification sites by request and time."""
import argparse
from decimal import Decimal
import gzip,hashlib,json
from pathlib import Path

def digest(path):
    with path.open('rb') as file:
        return hashlib.file_digest(file,'sha256').hexdigest()

def read(path):
    return json.loads(path.read_bytes())

def collect(capture,docs):
    prior=docs/'2026-11-19-c7a-previous-proof574/artifacts'
    counts=json.load(gzip.open(prior/'collector-pcs.json.gz'))
    pins=read(docs/'2026-11-18-c7a-worker-attribution381/artifacts/inputs.json')
    assert digest(capture/'bundle/bin/sdk')==counts['elf']
    sites={0x4c27d7:8,0x4c1a47:9}
    for address,closure in sites.items():
        pc=counts['pcs'][hex(address)]
        assert pc['assembly']==['test','rax,rcx'] and f'solve_inner::{{closure#{closure}}}' in pc['frames'][0]
    records=[]
    for process in counts['rows']:
        name=process['proc']
        trace=capture/'remote/output/stages'/(name+'.trace')
        assert digest(trace)==pins[name]['trace']
        slide=int(counts['maps'][name]['page_base'],16)
        start=None
        index=0
        block,width,shape,_=name.split('-')
        for line in trace.open():
            if 'zp363stages:interp_leave:' in line:
                assert start is None
                start=Decimal(line.split()[1][:-1])
                times=[]
            elif start is not None and 'cpu-clock/' in line:
                fields=line.split(None,4)
                address=int(fields[3],16)-slide
                if address in sites:
                    times.append((Decimal(fields[1][:-1]),address))
            elif 'zp363stages:fft_entry:' in line:
                assert start is not None
                end=Decimal(line.split()[1][:-1])
                previous=process['rows'][index]
                assert previous['i']==index
                for address,closure in sites.items():
                    elapsed = [float(1000*(ts-start)) for ts,pc in times if pc==address]
                    thirds=[0,0,0]
                    for ts,pc in times:
                        if pc==address:
                            assert start<=ts<=end
                            thirds[min(2,int(3*(ts-start)/(end-start)))]+=1
                    assert sum(thirds)==previous['pcs'].get(str(address),0)
                    records.append(dict(shape=shape,width=int(width),block=int(block),request=index,closure=closure,thirds=thirds,first_ms=min(elapsed,default=None),last_ms=max(elapsed,default=None)))
                index+=1
                start=None
        assert index==11 and start is None
    assert len(records)==264
    return records

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ['capture','docs','output']:parser.add_argument(name,type=Path)
    args=parser.parse_args()
    args.output.write_bytes(gzip.compress(json.dumps(collect(args.capture,args.docs),separators=(',',':')).encode(),mtime=0))
