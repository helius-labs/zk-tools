"""Count actual grouped-curve calls in the archived catalog SDK at the transfer crossover."""
import argparse
from collections import Counter
import importlib.util
import json
import os
from pathlib import Path
import pwd
import re
import struct
import shutil
import subprocess
import tempfile

profile_path = Path(__file__).resolve().parents[2] / "2026-11-19-crossover-phases602/artifacts/profile.py"
spec = importlib.util.spec_from_file_location("phase_profile", profile_path)
prof = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prof)

def capture(bundle, out, workers, cardinality):
    out.mkdir(parents=True,exist_ok=True)
    config = prof.read(bundle/"config/config.json")
    pins = prof.read(bundle/"payload-pins.json")
    for f in ("bin/sdk","bin/verify"):
        assert prof.sha(bundle/f)==pins[f]
    group = f"zp603_{os.getpid()}"
    probe = out/"routes-sdk"
    shutil.copy2(bundle/"bin/sdk",probe)
    prof.SYMBOLS = {
        "sdk":"<zopro_zolana::Prover>::prove",
        "group_witness":"zopro_core::msm_affine::grouping::group_witness",
        "scalar_groups":"zopro_core::msm_affine::grouping::scalar_groups",
    }
    for curve in ("G1","G2"):
        for stage, prefix in [("grouped","zopro_core::msm_affine::msm_grouped"),("combine","zopro_core::msm::group_affine")]:
            prof.SYMBOLS[f"{stage}_{curve.lower()}"]=f"{prefix}::<zopro_bn254::curve::{curve}Config>"
    prof.SYMBOLS["prepared_g1"]="zopro_core::msm_affine::msm_grouped_prepared::<zopro_bn254::curve::G1Config>"
    offsets = prof.sites(probe)
    prof.save(out/"pins.json",dict(sdk=prof.sha(probe),symbols=prof.SYMBOLS,offsets=offsets))
    attached=[]
    rows=[]
    try:
        for name,offset in offsets.items():
            for edge,kind in ([("entry","p"),("leave","r")] if name=="sdk" else [("entry","p")]):
                tag=f"{name}_{edge}"
                args = ""
                if cardinality and name in ("grouped_g1","grouped_g2"):
                    # These slice lengths come from msm_grouped in this pinned ELF.
                    # Capture dimensions only. No witness values or pointers are recorded.
                    args = " bases=%dx:u64 distinct=+16(%cx):u64 nonzero=+40(%cx):u64 starts=+64(%cx):u64 classes=+88(%cx):u64 original=+96(%cx):u64"
                prof.register(f"{kind}:{group}/{tag} {probe}:{offset}{args}")
                attached.append(tag)
        for shape in ("5x16","6x2","4x2","8x2"):
            c=config["cases"][shape]
            assert prof.sha(bundle/c["request"])==c["request_sha256"]
            for mode in ("default","disabled"):
                q=out/f"{shape}-{mode}";q.mkdir(exist_ok=True)
                e=config["environment"]|dict(RAYON_NUM_THREADS=str(workers),ZOPRO_BASIS_KEY_DIR=str(bundle/"images"),
                  ZOPRO_CPU_LAGRANGE_DIR=str(out/"cache"),ZOPRO_BENCH_MSM_PHASES="0",LD_LIBRARY_PATH=str(bundle/"lib"))
                assert "ZOPRO_CPU_DEDUP" not in e
                if mode=="disabled":e["ZOPRO_CPU_DEDUP"]="0"
                data=q/"events"
                with tempfile.TemporaryDirectory(dir=out) as temp:
                    ctl=Path(temp)/"ctl";ack=Path(temp)/"ack";os.mkfifo(ctl);os.mkfifo(ack)
                    e.update(ZOPRO_BENCH_PERF_CTL=str(ctl),ZOPRO_BENCH_PERF_ACK=str(ack))
                    cmd=["sudo","-n","perf","record","--no-buildid-cache","--clockid","mono","-o",data,
                         "-e",group+":*","--delay=-1","--control",f"fifo:{ctl},{ack}","--","runuser","-u",pwd.getpwuid(os.getuid()).pw_name,
                         "--","env","-i","PATH="+os.environ["PATH"],"HOME="+os.environ["HOME"],
                         *(f"{k}={v}" for k,v in e.items()),"taskset","-c",f"0-{workers-1}",probe,bundle/c["image"],bundle/c["request"],"3",q]
                    with (q/"sdk.log").open("w") as log:subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=150)
                tr=subprocess.check_output(["sudo","-n","perf","script","--ns","-i",data,"-F","time,event,trace" if cardinality else "time,event"],text=True)
                calls=None;observations=[];sizes=[]
                with (bundle/c["image"]).open("rb") as image:
                    header = struct.unpack_from("<9I", image.read(76), 40)
                for line in tr.splitlines():
                    if not line.strip():continue
                    name=line.split()[1].rstrip(":").split(":")[1]
                    if name=="sdk_entry":
                        assert calls is None
                        calls=Counter()
                    elif name=="sdk_leave":
                        assert calls is not None
                        observations.append(dict(calls));calls=None
                    else:
                        assert calls is not None
                        site=name.removesuffix("_entry")
                        calls[site]+=1
                        if cardinality and site in ("grouped_g1","grouped_g2"):
                            fields={k:int(v,0) for k,v in re.findall(r"(bases|distinct|nonzero|starts|classes|original)=(0x[0-9a-fA-F]+|[0-9]+)",line)}
                            assert len(fields)==6 and fields["bases"]==fields["original"]
                            assert fields["distinct"]==fields["classes"]==fields["starts"]-1
                            assert 0 < fields["distinct"] <= fields["nonzero"] <= fields["bases"]
                            if site=="grouped_g2":assert fields["bases"]==header[8]
                            sizes.append(fields|{"site":site,"round":len(observations)})
                assert calls is None and len(observations)==3 and len({tuple(sorted(o.items())) for o in observations})==1
                p=q/"0.proof";u=q/"0.public";assert prof.sha(u)==c["public_sha256"]
                receipt=subprocess.check_output([bundle/"bin/verify",bundle/c["image"],c["image_sha256"],p,u,c["public_sha256"]],text=True)
                digest=prof.sha(p);assert "VERIFIED proof_sha256="+digest in receipt
                (q/"receipt").write_text(receipt)
                rows.append(dict(shape=shape,mode=mode,calls=observations,sizes=sizes,proof=digest,public=c["public_sha256"]))
                prof.save(out/"rows.json",rows)
    finally:
        for tag in reversed(attached):prof.register(f"-:{group}/{tag}")
    assert len({r["proof"] for r in rows})==len(rows)==8
    prof.save(out/"complete.json", dict(exports=len(rows),workers=workers))

if __name__=="__main__":
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("bundle",type=Path)
    parser.add_argument("out",type=Path)
    parser.add_argument("--workers",type=int,default=32)
    parser.add_argument("--cardinality",action="store_true")
    args=parser.parse_args()
    capture(args.bundle.resolve(),args.out.resolve(),args.workers,args.cardinality)
