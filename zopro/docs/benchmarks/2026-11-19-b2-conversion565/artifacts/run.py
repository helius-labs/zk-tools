"""Measure when each curve query begins and when grouped B2 work can start."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import subprocess

root = Path(__file__).resolve().parent
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("output", type=Path)
parser.add_argument("--ryzen", action="store_true")
args = parser.parse_args()
cfg = json.loads((root / "config.json").read_bytes())
for name, digest in cfg["hashes"].items():
    with (root / name).open("rb") as stream:
        assert hashlib.file_digest(stream, "sha256").hexdigest() == digest, name
widths = (16, 32) if args.ryzen else (64, 96)
assert set(range(max(widths))).issubset(os.sched_getaffinity(0))
cpu = Path("/sys/devices/system/cpu")
topology = [
    {key: (cpu / f"cpu{i}/topology" / key).read_text().strip()
     for key in ("physical_package_id", "core_id", "thread_siblings_list")}
    for i in range(max(widths))
]
if not args.ryzen:
    assert len({(r["physical_package_id"], r["core_id"]) for r in topology}) == 96
output = args.output.resolve()
output.mkdir(parents=True, exist_ok=True)
host = {
    "topology": topology, "lscpu": subprocess.check_output(["lscpu"], text=True),
    "ryzen": args.ryzen, "hashes": cfg["hashes"],
}
if (output / "host.json").exists():
    assert json.loads((output / "host.json").read_bytes())["hashes"] == cfg["hashes"]
    assert json.loads((output / "host.json").read_bytes())["topology"] == topology
else:
    (output / "host.json").write_text(json.dumps(host, indent=2))
rounds = 11
rows_file = output / "rows.json"
rows = json.loads(rows_file.read_bytes()) if rows_file.exists() else []
done = {(r["block"],r["shape"],r["workers"],r["mode"]) for r in rows}
assert len(done) == len(rows)
for r in rows:
    shape = cfg["shapes"][r["shape"]]
    arm = output / f"{r['block']}-{r['shape']}-{r['workers']}-{r['mode']}"
    with (arm / "0.proof").open("rb") as stream:
        assert hashlib.file_digest(stream,"sha256").hexdigest() == r["proof"]
    msg = subprocess.check_output([root / "bin/verify", root / shape["image"], shape["image_sha256"],
        arm / "0.proof", arm / "0.public", shape["public_sha256"]], text=True)
    assert "VERIFIED proof_sha256=" + r["proof"] in msg
for block in range(5 if args.ryzen else 7):
    jobs = [(sh, width, mode) for sh in cfg["shapes"] for width in widths
            for mode in ("release", "control", "serial", "trace-control", "trace-serial")]
    random.Random(565 + block).shuffle(jobs)
    for sh, width, mode in jobs:
        if (block,sh,width,mode) in done:
            continue
        shape = cfg["shapes"][sh]
        merger = sh == "merge-24x1"
        arm = output / f"{block}-{sh}-{width}-{mode}"
        env = {k: v for k, v in os.environ.items()
               if not k.startswith(("ZOPRO_", "RAYON_", "MALLOC_"))}
        env.update(cfg["env"], RAYON_NUM_THREADS=str(width),
                   ZOPRO_BASIS_KEY_DIR=str(root / "images"),
                   ZOPRO_CPU_LAGRANGE_DIR=str(output / "cache"),
                   ZOPRO_SOLVE_EPOCHS="8" if merger else "6",
                   ZOPRO_CPU_FFT_LOCAL_EULER=str(int(merger)),
                   ZOPRO_CPU_POSEIDON_EMIT=str(int(not merger)))
        env.update(ZOPRO_TRIAL_B2_DIRECT="0", ZOPRO_TRIAL_SERIAL_B2_WORDS=str(int(mode.endswith("serial"))),
                   ZOPRO_TRIAL_B2_TIMES=str(int(mode.startswith("trace-"))))
        if not merger:
            env["ZOPRO_BENCH_WIRE_FILE"] = str(root / "public16.wo")
        with arm.with_suffix(".log").open("w") as stream:
            subprocess.run(["taskset", "-c", f"0-{width-1}", root / "bin" / ("release" if mode == "release" else "candidate"),
                            root / shape["image"], root / shape["request"], str(rounds), arm],
                           env=env, stdout=stream, stderr=subprocess.STDOUT, check=True, timeout=180)
        with (arm / "0.proof").open("rb") as stream:
            digest = hashlib.file_digest(stream, "sha256").hexdigest()
        verified = subprocess.check_output([root / "bin/verify", root / shape["image"],
                    shape["image_sha256"], arm / "0.proof", arm / "0.public", shape["public_sha256"]], text=True)
        assert "VERIFIED proof_sha256=" + digest in verified
        arm.with_suffix(".verify").write_text(verified)
        lines = arm.with_suffix(".log").read_text().splitlines()
        def tagged(prefix):
            return [line.removeprefix(prefix) for line in lines if line.startswith(prefix)]
        assert [json.loads(s) for s in tagged("MAIN_CPUS ")] == [list(range(width))]
        assert not tagged("FRONT_CPUS ")
        assert bool(tagged("WIRE_FILE true")) == (not merger)
        summary, = (json.loads(s) for s in tagged("SUMMARY "))
        samples = [json.loads(s) for s in tagged("SAMPLE ")]
        assert summary["workers"] == width and summary["priming"] == 3
        assert [s["round"] for s in samples] == list(range(rounds))
        ticks = []
        for data in tagged("B2_READY "):
            vector, joined = data.rsplit(" ", 1)
            times = json.loads(vector) + [int(joined)]
            assert len(times) == 11 and min(times) > 0
            assert all(times[i] <= times[j] <= times[10] for i,j in
                       ((0,1),(2,3),(4,5),(6,7),(7,8),(8,9)))
            ticks.append(times)
        assert len(ticks) == (rounds + 3 if mode.startswith("trace-") else 0)
        stages = [list(map(int, text.split())) for text in tagged("B2_GROUP ")]
        assert len(stages) == len(ticks) and all(len(row)==4 and row[0]>0 and 0<=row[1]<=row[0] and row[2]>0 and row[3]>=0 for row in stages)
        rows.append(dict(group_stages=stages[3:],shape=sh, workers=width, mode=mode, block=block, proof=digest,
                         samples=samples, summary=summary, returns=ticks[3:]))
        rows_file.with_suffix(".tmp").write_text(json.dumps(rows))
        rows_file.with_suffix(".tmp").replace(rows_file)
assert len({row["proof"] for row in rows}) == len(rows)
(output / "complete.json").write_text(json.dumps({"processes": len(rows), "timed": rounds * len(rows)}))
