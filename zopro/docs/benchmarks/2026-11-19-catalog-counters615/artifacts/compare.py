"""Compare archived 32/96-core measurements within each circuit, engine, and block."""
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path
from statistics import mean, median

REPORT = Path(__file__).resolve().parents[1]
SOURCE = REPORT.parent / "2026-11-18-full-catalog-c7a244/artifacts/processes.json.gz"

def metrics(row):
    n = row["rounds"]
    assert n == 7 and len(row["round_times_ms"]) == n
    assert abs(mean(row["round_times_ms"]) - row["mean_ms"]) < 1e-7
    return dict(ms=row["mean_ms"], cpu=row["CPU_ms_per_request"],
                faults=row["reported_loop_minor_faults"] / n,
                voluntary=row["voluntary_switches"] / n,
                rss=row["max_RSS_MiB"])

def compare():
    with gzip.open(SOURCE, "rt") as stream:
        data = [row for row in json.load(stream) if not row["preflight"]]
    assert len(data) == 1476
    with gzip.open(REPORT / "artifacts/identities-and-switches.json.gz", "rt") as stream:
        details = {row["name"]:row for row in json.load(stream)}
    assert len(details) == len(data)
    group = defaultdict(dict)
    for row in data:
        key = row["arm"], row["shape"], row["block"]
        assert row["workers"] not in group[key] and row["workers"] in (32, 96)
        group[key][row["workers"]] = row | details[row["name"]]
    pairs = defaultdict(list)
    for (arm, shape, block), cells in sorted(group.items()):
        left, right = cells[32], cells[96]
        assert all(left[k] == right[k] for k in ["image_sha256", "public_sha256", "aeglos_key_sha256"])
        lo, hi = metrics(left), metrics(right)
        delta = {k:hi[k]-v for k,v in lo.items()}
        pairs[arm,shape].append(dict(block=block, low=lo, high=hi, delta=delta))
    assert len(pairs) == 82 and all(len(rows)==9 for rows in pairs.values())
    result = []
    for (arm,shape), rows in pairs.items():
        assert sorted(r["block"] for r in rows)==list(range(9))
        summary = {k:{field:median(r[k][field] for r in rows) for field in rows[0][k]}
                   for k in ["low","high","delta"]}
        result.append(dict(arm=arm,shape=shape,summary=summary,
                           slower_blocks=sum(r["delta"]["ms"]>0 for r in rows),
                           fewer_fault_blocks=sum(r["delta"]["faults"]<0 for r in rows),
                           pairs=rows))
    return result

if __name__=="__main__":
    result=compare()
    out=REPORT / "artifacts/results.json"
    out.write_text(json.dumps({"archive_sha256":hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
                               "cells":result},separators=(",",":"))+"\n")
    for arm in ["ON","AEGLOS"]:
        cells=[r for r in result if r["arm"]==arm]
        print(arm,Counter((r["summary"]["delta"]["ms"]>0,r["summary"]["delta"]["faults"]>0) for r in cells))
