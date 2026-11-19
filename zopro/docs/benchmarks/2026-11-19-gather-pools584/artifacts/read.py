"""Locate sampled Rayon gather notifications by pool and part of the handoff."""
import argparse
from collections import Counter
from decimal import Decimal
import gzip
import hashlib
import json
from pathlib import Path

def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()

def read(path):
    return json.loads(path.read_bytes())

def run(capture, docs, out):
    parent = docs / "2026-11-19-c7a-previous-proof574/artifacts"
    pins = read(docs / "2026-11-18-c7a-worker-attribution381/artifacts/inputs.json")
    with gzip.open(parent / "collector-pcs.json.gz") as data:
        relocated = json.load(data)
    original_counts = {(r["proc"],r["i"]):sum(r["counts"].values()) for r in read(parent / "gap-samples.json")}
    phases = {(r["proc"],r["i"]):r["solver_to_fft"] for r in read(parent / "requests.json")}
    locations = {int(pc,16) for pc,record in relocated["pcs"].items()
                 if pc in {"0x4c1a47","0x4c27d7"} and record["assembly"]==["test","rax,rcx"]}
    assert len(locations)==2
    results = []
    stages = capture / "remote/output/stages"
    assert digest(capture/"bundle/bin/sdk")==relocated["elf"]
    for name,pin in pins.items():
        trace = stages/(name+".trace")
        log = stages/(name+".log")
        proof = stages/name/"0.proof"
        assert digest(trace)==pin["trace"] and digest(log)==pin["placement_log"] and digest(proof)==pin["proof"]
        assert "VERIFIED proof_sha256="+pin["proof"] in (stages/(name+".verify")).read_text()
        block, width, shape, _ = name.split("-")
        width = int(width)
        info, = [json.loads(line[8:]) for line in log.read_text().splitlines() if line.startswith("SUMMARY ")]
        before,after = info["placement_before"],info["placement_after"]
        assert before["worker_rows"]==after["worker_rows"]
        cpus = {r["tid"]:r["allowed"][0] for r in before["worker_rows"] if r["allowed"]==[r["index"]]}
        assert sorted(cpus.values())==list(range(width))
        slide = int(relocated["maps"][name]["page_base"],16)
        start = None
        caller = None
        index = 0
        for line in trace.open():
            if "zp363stages:sdk_entry:" in line:
                tid=int(line.split()[0])
                assert caller in (None,tid)
                caller=tid
                assert tid not in cpus
            elif "zp363stages:interp_leave:" in line:
                assert start is None
                start=Decimal(line.split()[1][:-1])
                samples=[]
            elif start is not None and "cpu-clock/" in line:
                tid,ts,_,ip,symbol=line.split(None,4)
                tid=int(tid)
                site=int(ip,16)-slide
                kind = ("job notification" if site in locations else
                        "Crossbeam" if "crossbeam_" in symbol else "other")
                if kind=="job notification":
                    assert "IterProducer<u32>" in symbol and "CollectConsumer" in symbol
                group = f"main {cpus[tid]//32}" if tid in cpus else ("caller" if tid==caller else "other threads")
                samples.append((Decimal(ts[:-1]),group,kind))
            elif "zp363stages:fft_entry:" in line:
                assert start is not None
                end=Decimal(line.split()[1][:-1])
                assert abs(float(1000*(end-start))-phases[name,index])<1e-8
                assert len(samples)==original_counts[name,index]
                counts=Counter()
                for ts,group,kind in samples:
                    assert start<=ts<=end
                    third=min(2,int(3*(ts-start)/(end-start)))
                    counts[group,third,kind]+=1
                results.append(dict(process=name,shape=shape,width=width,block=int(block),round=index,
                                    observations=[[*k,n] for k,n in sorted(counts.items())]))
                index+=1
                start=None
        assert index==11 and start is None
    assert len(results)==132
    for shape, targets in [("16",[245,568]),("merge",[308,666])]:
        for width,expected in zip([64,96],targets):
            assert sum(n for r in results if (r["shape"],r["width"])==(shape,width)
                       for _,_,kind,n in r["observations"] if kind=="job notification")==expected
    out.write_bytes(gzip.compress(json.dumps(results,separators=(",",":")).encode(),mtime=0))

if __name__=="__main__":
    parser=argparse.ArgumentParser(description=__doc__)
    for key in ["capture","docs","output"]:
        parser.add_argument(key,type=Path)
    opts=parser.parse_args()
    run(opts.capture,opts.docs,opts.output)
