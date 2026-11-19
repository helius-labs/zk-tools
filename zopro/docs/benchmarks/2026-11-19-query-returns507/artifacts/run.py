"""Measure the end of A, K, H, and B2 against the plain prover on one host."""
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
parser.add_argument("--smoke", action="store_true")
args = parser.parse_args()
cfg = json.loads((root / "config.json").read_bytes())
for name, digest in cfg["hashes"].items():
    with (root / name).open("rb") as stream:
        assert hashlib.file_digest(stream, "sha256").hexdigest() == digest, name
widths = (16, 32) if args.smoke else (64, 96)
assert set(range(max(widths))).issubset(os.sched_getaffinity(0))
cpu = Path("/sys/devices/system/cpu")
topology = [
    {key: (cpu / f"cpu{i}/topology" / key).read_text().strip()
     for key in ("physical_package_id", "core_id", "thread_siblings_list")}
    for i in range(max(widths))
]
if not args.smoke:
    assert len({(r["physical_package_id"], r["core_id"]) for r in topology}) == 96
output = args.output.resolve()
output.mkdir(parents=True, exist_ok=False)
(output / "host.json").write_text(json.dumps({
    "topology": topology, "lscpu": subprocess.check_output(["lscpu"], text=True),
    "smoke": args.smoke, "hashes": cfg["hashes"],
}, indent=2))
rounds = 1 if args.smoke else 11
rows = []
for block in range(1 if args.smoke else 7):
    jobs = [(sh, width, mode) for sh in cfg["shapes"] for width in widths
            for mode in ("release", "trace")]
    random.Random(507 + block).shuffle(jobs)
    for sh, width, mode in jobs:
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
        if not merger:
            env["ZOPRO_BENCH_WIRE_FILE"] = str(root / "public16.wo")
        with arm.with_suffix(".log").open("w") as stream:
            subprocess.run(["taskset", "-c", f"0-{width-1}", root / "bin" / mode,
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
        for data in tagged("QUERY_RETURNS "):
            route, omit, *stamp = data.split()
            times = list(map(int, stamp))
            assert omit == "true" and len(times) == 5 and 0 < min(times[:4]) <= max(times[:4]) <= times[4]
            ticks.append({"route": route, "ns": times})
        assert len(ticks) == (rounds + 3 if mode == "trace" else 0)
        rows.append(dict(shape=sh, workers=width, mode=mode, block=block, proof=digest,
                         samples=samples, summary=summary, returns=ticks[3:]))
        (output / "rows.json").write_text(json.dumps(rows))
assert len({row["proof"] for row in rows}) == len(rows)
(output / "complete.json").write_text(json.dumps({"processes": len(rows), "timed": rounds * len(rows)}))
