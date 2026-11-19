"""Measure repeatable placement of sampled G2 work on a pinned worker pool."""
import argparse
from bisect import bisect_left
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path
from statistics import median
import subprocess


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def timestamp(field):
    seconds, fraction = field.rstrip(":").split(".")
    return int(seconds) * 10**9 + int(fraction.ljust(9, "0"))


def read_process(path, pin, width):
    trace, log = path.with_suffix(".trace"), path.with_suffix(".log")
    assert digest(trace) == pin["trace"] and digest(log) == pin["placement_log"]
    summary, = [json.loads(line[8:]) for line in log.read_text().splitlines()
                if line.startswith("SUMMARY ")]
    placement = summary["placement_before"]
    assert placement == summary["placement_after"]
    worker = {row["tid"]: row["index"] for row in placement["worker_rows"]}
    assert set(worker.values()) == set(range(width))
    assert all(row["allowed"] == [row["index"]] for row in placement["worker_rows"])
    events = defaultdict(list)
    samples = []
    all_ticks = []
    for line in trace.read_text().splitlines():
        fields = line.split()
        if len(fields) < 3:
            continue
        event = fields[2].rstrip(":").split(":")[-1]
        t = timestamp(fields[1])
        if event.startswith("cpu-clock"):
            all_ticks.append(t)
            symbol = " ".join(fields[4:])
            if symbol == "zopro_core::msm::g2_prefix::apply::<zopro_bn254::curve::G2Config, true>":
                samples.append((t, worker.get(int(fields[0]), -1)))
        else:
            events[event].append(t)
    boundaries = [events[name] for name in ("msms_entry", "denseh_leave", "msms_leave")]
    assert list(map(len, boundaries)) == [11] * 3
    assert samples == sorted(samples, key=lambda pair: pair[0])
    ticks = [t for t, _ in samples]
    assert all_ticks == sorted(all_ticks)
    assert all(len(events[k]) == 11 for k in ("sdk_entry","sdk_leave","denseh_entry"))
    result = []
    for i, (start, split, end) in enumerate(zip(*boundaries)):
        assert start <= split <= end
        for phase, begin, finish in [("before H returns", start, split),
                                     ("after H returns", split, end)]:
            first, last = bisect_left(ticks, begin), bisect_left(ticks, finish)
            counts = Counter(index for _, index in samples[first:last])
            sdk_ns = events["sdk_leave"][i] - events["sdk_entry"][i]
            h_start = events["denseh_entry"][i]
            assert events["sdk_entry"][i] <= start <= h_start <= split <= end <= events["sdk_leave"][i]
            before_h_entry = bisect_left(ticks, h_start)-bisect_left(ticks,start)
            result.append(dict(round=i, phase=phase, duration_ns=finish-begin, sdk_ns=sdk_ns,
                               query_ns=end-start, h_ns=split-h_start,
                               pre_h_entry=before_h_entry, all_samples=bisect_left(all_ticks, finish)-bisect_left(all_ticks, begin),
                               outside=counts.pop(-1, 0), counts=dict(counts)))
    return result


def pool(rows):
    hist = Counter()
    for row in rows:
        hist.update(row["counts"])
    return hist


def hottest(training, width):
    return set(sorted(range(width), key=lambda idx: (-training[idx], idx))[:width//4])


def fraction(hist, subset):
    return sum(hist[idx] for idx in subset) / sum(hist.values())


def evaluate(processes, width, phase):
    results = []
    for k, process in enumerate(processes):
        rows = [r for r in process if r["phase"] == phase]
        train = pool(rows[:5])
        test = pool(rows[5:])
        across = pool(r for j, other in enumerate(processes) if j != k
                      for r in other if r["phase"] == phase)
        assert across
        results.append(dict(block=k, early=sum(train.values()), late=sum(test.values()),
                            outside_late=sum(r["outside"] for r in rows[5:]),
                            within=fraction(test, hottest(train, width)) if train and test else None,
                            transferred=fraction(test, hottest(across, width)) if test else None,
                            training_covered=len(train), late_covered=len(test)))
    return results


def main():
    p = argparse.ArgumentParser()
    for arg in ["stages", "pins", "config", "bundle", "output"]:
        p.add_argument(arg, type=Path)
    args = p.parse_args()
    pins = json.loads(args.pins.read_bytes())
    config = json.loads(args.config.read_bytes())["cases"]
    args.output.mkdir(parents=True, exist_ok=True)
    all_rows, comparisons, proofs = {}, [], []
    for shape, tag in [("16x2", "16"), ("merge-24x1", "merge")]:
        c = config[shape]
        for width in [64, 96]:
            processes = []
            for k in range(3):
                q = args.stages / f"{k}-{width}-{tag}-trace"
                pin = pins[q.name]
                proof, public = q / "0.proof", q / "0.public"
                assert digest(proof) == pin["proof"] and digest(public) == c["public_sha256"]
                out = subprocess.check_output([args.bundle / "bin/verify", args.bundle / c["image"],
                    c["image_sha256"], proof, public, c["public_sha256"]], text=True)
                assert "VERIFIED proof_sha256=" + pin["proof"] in out
                proofs.append(pin["proof"])
                rows = read_process(q, pin, width)
                processes.append(rows)
                all_rows[q.name] = rows
            for phase in ["before H returns", "after H returns"]:
                comparisons.append(dict(shape=shape, width=width, phase=phase,
                    blocks=evaluate(processes, width, phase)))
    assert len(set(proofs)) == 12
    def write(name, content):
        (args.output / name).write_text(json.dumps(content, indent=2) + "\n")
    write("comparison.json", comparisons)
    write("verification.json", dict(reverified_proofs=proofs, input_pins_sha256=digest(args.pins)))
    (args.output / "workers.json.gz").write_bytes(gzip.compress(
        json.dumps(all_rows, separators=(",", ":")).encode(), mtime=0))
    for r in comparisons:
        blocks = r["blocks"]
        print(r["shape"], r["width"], r["phase"], [(b["early"],b["late"],
              round(100*b["transferred"], 1) if b["transferred"] is not None else None) for b in blocks])
    modes = []
    for name, rows in all_rows.items():
        for early, late in zip(rows[::2], rows[1::2]):
            before, after = [sum(r["counts"].values())+r["outside"] for r in (early,late)]
            share = after/(before+after) if before+after else None
            modes.append(dict(process=name, round=early["round"], before=before, after=after,
                pre_h_entry=late["pre_h_entry"], during_h=before-late["pre_h_entry"], share=share, tail_ms=late["duration_ns"]/1e6, query_ms=late["query_ns"]/1e6,
                sdk_ms=late["sdk_ns"]/1e6, h_ms=late["h_ns"]/1e6,
                tail_samples=late["all_samples"]))
    write("requests.json", modes)
    for tag in ["16","merge"]:
        for width in [64,96]:
            rows=[r for r in modes if f"-{width}-{tag}-" in r["process"]]
            print(tag,width,sorted(round(100*r["share"]) if r["share"] is not None else -1 for r in rows))
            for label,group in [("low",[r for r in rows if r["share"] is not None and r["share"] < .1]),
                    ("mid",[r for r in rows if r["share"] is not None and .1 <= r["share"] < .5]),
                    ("high",[r for r in rows if r["share"] is not None and .5 <= r["share"]])]:
                if group:print(label,len(group),[round(median(r[k] for r in group),3) for k in ["tail_ms","query_ms","sdk_ms","h_ms"]])


if __name__ == "__main__":
    main()
