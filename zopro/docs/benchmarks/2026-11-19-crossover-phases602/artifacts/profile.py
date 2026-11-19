"""Profile the two equal-domain crossover shapes and the two M5 controls on the same host."""
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import pwd
import random
import shutil
from statistics import mean, median
import subprocess
import tempfile

SHAPES = ("5x16", "6x2", "4x2", "8x2")
SYMBOLS = {
    "sdk": "<zopro_zolana::Prover>::prove",
    "solve": "<zopro_core::solver::Plan>::solve_in_with_slots",
    "fft": "<zopro_core::lagrange::Queries>::product_values",
    "core": "<zopro_core::Key>::solve_canonical_wires",
    "queries": "<zopro_core::Key>::cpu_msms",
}

def sha(path):
    with path.open("rb") as f:
        return hashlib.file_digest(f, "sha256").hexdigest()

def save(path, obj):
    path.write_text(json.dumps(obj, indent=2) + "\n")

def read(path):
    return json.loads(path.read_bytes())

def sites(binary):
    names = subprocess.check_output(["nm", "-nC", binary], text=True).splitlines()
    loads = [line.split() for line in subprocess.check_output(["readelf", "-lW", binary], text=True).splitlines() if line.lstrip().startswith("LOAD ")]
    offsets = {}
    for name, symbol in SYMBOLS.items():
        address, = [int(line.split(" ", 1)[0],16) for line in names if line.endswith(" t " + symbol)]
        offset, = [int(seg[1],16) + address-int(seg[2],16) for seg in loads if "E" in seg and int(seg[2],16)<=address<int(seg[2],16)+int(seg[4],16)]
        offsets[name] = hex(offset)
    return offsets

def register(event):
    subprocess.run(["sudo","-n","bash","-c",'echo "$1" >> /sys/kernel/tracing/uprobe_events',"register",event],check=True)

def intervals(data, count):
    lines = subprocess.check_output(["sudo","-n","perf","script","--ns","-i",data,"-F","tid,time,event"],text=True)
    active = {}
    finished = {key:[] for key in SYMBOLS}
    for line in lines.splitlines():
        if not line.strip(): continue
        tid, timestamp, event = line.split()[:3]
        name, edge = event.rstrip(":").split(":")[1].rsplit("_", 1)
        stamp = int(timestamp.rstrip(":").replace(".", ""))
        key = (tid,name)
        if edge == "entry":
            assert key not in active
            active[key] = stamp
        else:
            assert edge == "leave"
            finished[name].append((active.pop(key),stamp))
    assert not active and all(len(finished[key])==count for key in ("sdk","solve","fft"))
    rows = []
    for first, last in sorted(finished["sdk"]):
        nested = {key:[(a,b) for a,b in values if first<=a<=b<=last] for key,values in finished.items()}
        (s0,s1), = nested["solve"]
        (f0,f1), = nested["fft"]
        assert first<=s0<=s1<=f0<=f1<=last
        times = {"entry_to_solve":s0-first,"solve":s1-s0,"solve_to_fft":f0-s1,"fft":f1-f0,"after_fft":last-f1}
        assert sum(times.values()) == last-first
        rows.append({"partition_ns":times,"queries_ns":[[a-first,b-first] for a,b in nested["queries"]],
                     "core_ns":[[a-first,b-first] for a,b in nested["core"]],"total_ns":last-first})
    return rows

def run(bundle, out, widths, smoke=False):
    cfg = read(bundle/"config/config.json")
    locks = read(bundle/"payload-pins.json")
    for file in ("bin/sdk","bin/aeglos","bin/verify",*(cfg["cases"][s][k] for s in SHAPES for k in ("image","request","aeglos_key"))):
        assert sha(bundle/file)==locks[file]
    out.mkdir(parents=True,exist_ok=True)
    probe = out/"profile-sdk"
    shutil.copy2(bundle/"bin/sdk",probe)
    assert sha(probe)==cfg["sdk_sha256"]
    offsets=sites(probe)
    save(out/"pins.json",dict(sdk=sha(probe),aeglos=sha(bundle/"bin/aeglos"),sites=offsets,shapes={s:cfg["cases"][s] for s in SHAPES}))
    assert all(set(range(w))<=os.sched_getaffinity(0) for w in widths)
    group=f"zp602_{os.getpid()}"
    attached=[]
    env={k:v for k,v in os.environ.items() if not k.startswith(("ZOPRO_","RAYON_","AEGLOS_","MALLOC_","GOMAXPROCS"))}
    rows=read(out/"rows.json") if (out/"rows.json").is_file() else []
    jobs=[]
    for block in range(5):
        order=[(block,w,s,m) for w in widths for s in SHAPES for m in ("zopro","aeglos","trace")]
        random.Random(602035+block).shuffle(order)
        jobs.extend(order)
    if smoke:
        jobs=[(0,widths[0],"5x16","trace")]
    save(out/"schedule.json", jobs)
    try:
        for name,offset in offsets.items():
            for edge,kind in [("entry","p"),("leave","r")]:
                tag=f"{name}_{edge}"
                register(f"{kind}:{group}/{tag} {probe}:{offset}")
                attached.append(tag)
        for block,w,s,m in jobs:
            if any((r["block"],r["workers"],r["shape"],r["mode"])==(block,w,s,m) for r in rows):continue
            c=cfg["cases"][s]; n=2 if smoke else 11
            q=out/f"{block}-{w}-{s}-{m}";q.mkdir(exist_ok=True)
            e=env|cfg["environment"]|dict(RAYON_NUM_THREADS=str(w),AEGLOS_CPU_THREADS=str(w),GOMAXPROCS=str(w),
                ZOPRO_BASIS_KEY_DIR=str(bundle/"images"),ZOPRO_CPU_LAGRANGE_DIR=str(out/"cache"),ZOPRO_BENCH_PROOFS=str(q),
                AEGLOS_BENCH_DISABLE_SHIFTED="0",AEGLOS_BENCH_MSM_PHASES="0",ZOPRO_BENCH_MSM_PHASES="0",LD_LIBRARY_PATH=str(bundle/"lib"))
            data=q/"perf.data"
            with tempfile.TemporaryDirectory(dir=out) as tmp:
                cmd=[]
                if m=="trace":
                    ctl=Path(tmp)/"ctl";ack=Path(tmp)/"ack";os.mkfifo(ctl);os.mkfifo(ack)
                    e.update(ZOPRO_BENCH_PERF_CTL=str(ctl),ZOPRO_BENCH_PERF_ACK=str(ack))
                    cmd=["sudo","-n","perf","record","--no-buildid-cache","--clockid","mono","-o",data,"-e",group+":*","--delay=-1","--control",f"fifo:{ctl},{ack}","--","runuser","-u",pwd.getpwuid(os.getuid()).pw_name,"--"]
                allowed={k:v for k,v in e.items() if k.startswith(("ZOPRO_","RAYON_","AEGLOS_","MALLOC_","GOMAXPROCS")) or k in ("LD_LIBRARY_PATH","PATH","HOME","LANG")}
                cmd+=["env",*(f"{k}={v}" for k,v in allowed.items()),"taskset","-c",f"0-{w-1}"]
                if m=="aeglos":
                    cmd += [bundle/"bin/aeglos","aeglos",bundle/c["aeglos_key"],bundle/c["request"],str(n)]
                    exports=q/c["id"]
                else:
                    cmd += [probe if m=="trace" else bundle/"bin/sdk",bundle/c["image"],bundle/c["request"],str(n),q]
                    exports=q
                with (q/"sdk.log").open("w") as log:
                    subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=240)
            p=exports/"0.proof";u=exports/"0.public"
            assert sha(u)==c["public_sha256"]
            digest=sha(p)
            receipt=subprocess.check_output([bundle/"bin/verify",bundle/c["image"],c["image_sha256"],p,u,c["public_sha256"]],text=True)
            assert "VERIFIED proof_sha256="+digest in receipt
            (q/"receipt").write_text(receipt)
            lines=(q/"sdk.log").read_text().splitlines()
            samples=[json.loads(l[7:]) for l in lines if l.startswith("SAMPLE ")]
            summary,=[json.loads(l[8:]) for l in lines if l.startswith("SUMMARY ")]
            assert [r["round"] for r in samples]==list(range(n)) and summary["priming"]==3
            if m=="aeglos":assert summary["threads"]==w and all(r["cache_hit"] for r in samples)
            else: assert summary["workers"]==w
            spans=intervals(data,n) if m=="trace" else []
            rows.append(dict(block=block,workers=w,shape=s,mode=m,public=sha(u),proof=digest,
                ms=mean(r["verified_ms" if m=="aeglos" else "ms"] for r in samples),rss=summary["whole_process_maxrss_kib"]/1024,
                samples=samples,spans=spans))
            save(out/"rows.json",rows)
    finally:
        for tag in reversed(attached):register(f"-:{group}/{tag}")
    assert len(rows)==len(jobs) and len({r["proof"] for r in rows})==len(rows)
    with gzip.open(out/"results.json.gz","wt") as f:json.dump(rows,f,separators=(",",":"))
    save(out/"complete.json",{"exports":len(rows),"widths":widths})

if __name__ == "__main__":
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("bundle",type=Path)
    parser.add_argument("out",type=Path)
    parser.add_argument("--workers",type=int,nargs="+",default=[32])
    parser.add_argument("--smoke",action="store_true")
    args=parser.parse_args()
    run(args.bundle.resolve(),args.out.resolve(),args.workers,args.smoke)
