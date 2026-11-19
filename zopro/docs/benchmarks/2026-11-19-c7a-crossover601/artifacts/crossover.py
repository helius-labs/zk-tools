"""Reconstruct the C7a crossover from paired processes and original key headers."""
import argparse
from collections import defaultdict
import gzip
import hashlib
import json
from pathlib import Path
from statistics import mean, median
import struct

def load(path):
    return json.loads(path.read_bytes())

def analyze(bundle, docs):
    old = docs / "2026-11-18-full-catalog-c7a244/artifacts"
    comparisons = {(r["shape"], r["workers"]): r for r in load(old / "results.json")["all_comparisons"]}
    with gzip.open(old / "processes.json.gz") as stream:
        rows = [r for r in json.load(stream) if not r["preflight"]]
    with gzip.open(docs / "2026-11-18-c7a-gap250/artifacts/analysis.json.gz") as stream:
        phases = {r["shape"]: r for r in json.load(stream)["catalog"]["rows"]}
    samples = defaultdict(dict)
    for row in rows:
        identity = (row["shape"], row["workers"], row["arm"])
        block = row["block"]
        assert block not in samples[identity] and row["rounds"] == 7
        assert abs(mean(row["round_times_ms"]) - row["mean_ms"]) < 1e-8
        samples[identity][block] = row["mean_ms"]
    assert len(rows) == 41 * 4 * 9
    config = load(bundle / "config/config.json")["cases"]
    locks = load(bundle / "payload-pins.json")
    shapes = {}
    for name, case in config.items():
        path = bundle / case["image"]
        assert locks[case["image"]] == case["image_sha256"]
        with path.open("rb") as image:
            header = image.read(76)
        assert header[:16] == b"zopro-euler-v2\0\0" and header[24:40] == b"zopro-key".ljust(16, b"\0")
        version, _, _, constraints, g1a, g1b, g1k, hz, g2b = struct.unpack_from("<9I", header, 40)
        assert version == 3 and g1b == g2b and (hz+1).bit_count() == 1 and 0 < constraints <= hz+1
        family = "merge" if name.startswith("merge-") else "transfer"
        inputs, outputs = map(int, name.removeprefix("merge-").split("x"))
        result = dict(family=family, inputs=inputs, outputs=outputs, constraints=constraints,
                      fft_domain=hz+1, b2=g2b, image_sha256=case["image_sha256"],
                      header_sha256=hashlib.sha256(header).hexdigest())
        for width in (32, 96):
            z = samples[name, width, "ON"]
            a = samples[name, width, "AEGLOS"]
            assert sorted(z) == sorted(a) == list(range(9))
            ratios = sorted(z[k]/a[k] for k in z)
            published = comparisons[name, width]["Zopro_over_Aeglos_wall"]
            for value, label in [(ratios[1],"lower"),(ratios[4],"median"),(ratios[7],"upper")]:
                assert abs(value-published[label]) < 1e-9
            ae_mean = phases[name]["aeglos"]["verified"][f"{width}_ms"]
            assert abs(mean(a.values())-ae_mean) < 1e-8
            assert abs(mean(z.values())-phases[name][f"zopro_{width}_ms"]) < 1e-8
            result[str(width)] = dict(zopro_ms=median(z.values()), aeglos_ms=median(a.values()),
                                      low=ratios[1], ratio=ratios[4], high=ratios[7],
                                      verdict="Zopro" if ratios[7]<1 else "Aeglos" if ratios[1]>1 else "inconclusive")
        shapes[name] = result
    assert len(shapes)==41
    # Within a transfer input count, compare its minimum and maximum output widths.
    # Select the contrasts without reference to latency or performance direction.
    endpoints = {}
    for ni in sorted({r["inputs"] for r in shapes.values() if r["family"]=="transfer"}):
        group = sorted((r["outputs"], name) for name,r in shapes.items() if r["family"]=="transfer" and r["inputs"]==ni)
        if len(group)<2: continue
        first,last=group[0][1],group[-1][1]
        z0,z1=(samples[key,96,"ON"] for key in (first,last))
        a0,a1=(samples[key,96,"AEGLOS"] for key in (first,last))
        relative=[(z1[k]/z0[k])/(a1[k]/a0[k]) for k in z0]
        endpoints[ni]=dict(first=first,last=last,relative_change_pct=100*(median(relative)-1),
                           blocks_more_costly=sum(r>1 for r in relative))
    return dict(shapes=shapes,output_contrasts=endpoints)

if __name__=="__main__":
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("bundle",type=Path)
    parser.add_argument("docs",type=Path)
    parser.add_argument("output",type=Path)
    args=parser.parse_args()
    args.output.write_text(json.dumps(analyze(args.bundle,args.docs),indent=2)+"\n")
