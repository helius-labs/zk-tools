import argparse
from collections import Counter
import gzip
import hashlib
import json
import mmap
from pathlib import Path
import struct
import sys

def sha(path):
    with path.open("rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()

parser = argparse.ArgumentParser(description="Count FFT tiles available after each solver band")
parser.add_argument("workflow", type=Path)
parser.add_argument("pins", type=Path)
args = parser.parse_args()
assert sys.byteorder == "little"
pinned = json.loads(args.pins.read_text())
workflow = json.loads(gzip.decompress(args.workflow.read_bytes()))
jobs = [row for row in workflow["steps"]
        if row["name"].startswith("route-b0-") and row["name"].endswith("catalog-ordinary")]

def row_depths(image):
    with image.open("rb") as fp, mmap.mmap(fp.fileno(), 0, access=mmap.ACCESS_READ) as mm:
        start = mm.rfind(b"ZSP1")
        while start >= 8 and (struct.unpack_from("<Q", mm, start - 8)[0] != len(mm) - start
                              or struct.unpack_from("<I", mm, start + 4)[0] != 2):
            start = mm.rfind(b"ZSP1", 0, start)
        assert start >= 8
        data = memoryview(mm)[start:].cast("I")
        magic,version,nw,ni,np,nc,na,nb,ncoef,nt,ns,nslot,nops = data[:13]
        assert magic==0x3150535a and version==2
        term = 13 + na + nb + 8*ncoef
        sums = term + 2*nt
        pos = sums + 3*ns
        producer = [-1] * (nw + nslot)
        row_depth = [-1] * nc
        levels = []
        def term_depth(lo,hi):
            return max((producer[data[term + 2*j + 1]] for j in range(lo,hi)),default=-1)
        for _ in range(nops):
            if data[pos]:
                flag,kind,lo,hi,first,last = data[pos:pos+6]
                assert flag == 1
                depth = 1 + term_depth(lo,hi)
                producer[first:last] = [depth] * (last-first)
                pos += 6
            else:
                flag,row,lo,hi,slot = data[pos:pos+5]
                writes = []
                depth = -1
                for j in range(lo,hi):
                    prev, coefficient, wire = data[sums+3*j:sums+3*j+3]
                    depth = max(depth,producer[wire])
                    if (prev & 0x7fffffff) < 0x7ffffffe:
                        depth = max(depth,producer[prev & 0x7fffffff])
                    if prev >> 31:
                        writes.append(nw+slot)
                        slot += 1
                for i in (5,8,11):
                    tag,a,b = data[pos+i:pos+i+3]
                    if tag == 1:
                        depth = max(depth,producer[a])
                    elif tag in (3,4):
                        depth = max(depth,term_depth(a,(a+1 if tag==3 else b)))
                    else:
                        assert tag in (0,2)
                assigned = data[pos+14]
                assert assigned in (0,1)
                if assigned:
                    writes.append(data[pos+16])
                assert row_depth[row]==-1
                depth += 1
                row_depth[row] = depth
                for wire in writes:
                    producer[wire] = depth
                pos += 26 if assigned else 15
            levels.append(depth)
        assert pos==len(data) and -1 not in row_depth
        data.release()
        return row_depth,levels

results={}
for job in jobs:
    assert job["environment"]["ZOPRO_CPU_BOUNDED_SCHEDULE"] == "1"
    image=Path(job["argv"][4])
    expected=pinned[str(image)]
    assert sha(image)==expected
    row_depths_result,depth_levels=row_depths(image)
    n=len(row_depths_result)
    domain=1<<(n-1).bit_length()
    levels=max(depth_levels)+1
    epochs=int(job["environment"]["ZOPRO_SOLVE_EPOCHS"])
    packed=job["environment"]["ZOPRO_SOLVE_PACK_TASKS"]=="1"
    hist=Counter(depth_levels)
    depth_band=[]
    seen=0
    for level in range(levels):
        count=hist[level]
        band=epochs * (2 * len(depth_levels) * level + (2*seen+count) * levels)//(4*len(depth_levels)*levels) if packed else level*epochs//levels
        depth_band.append(min(epochs-1,band))
        seen+=count
    row_band=[depth_band[d] for d in row_depths_result]+[-1]*(domain-n)
    stats={}
    for radix in (2,4,8,16,32):
        columns=domain//(radix*4)
        ready=[max(row_band[4*(stripe*columns+col)+lane] for stripe in range(radix) for lane in range(4))
               for col in range(columns)]
        stats[radix]={k: dict(tiles=len(ready)//k, ready_at_or_before=[sum(max(ready[i:i+k])<=band for i in range(0,columns,k)) for band in range(epochs)])
                      for k in (1,16,64)}
    results[job["name"].split("-")[2]]=dict(image_sha256=expected,domain=domain,rows=n,depth_levels=levels,
                                           base_epochs=epochs,packed_tasks=packed,radix=stats)
assert set(results) == {"1x2", "2x2", "4x2"}
print(json.dumps(results, indent=2))
