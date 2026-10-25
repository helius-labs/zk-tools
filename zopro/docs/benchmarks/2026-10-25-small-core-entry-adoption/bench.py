from pathlib import Path
import json,os,subprocess,sys
p=Path(__file__).resolve().parent
m=json.load(open(p/"production-source-manifest.json"))
shape=sys.argv[1];rounds=sys.argv[2];out=sys.argv[3]
subprocess.run([m["native_binary"],shape,rounds,"1","4",out],env=os.environ|m["environment"],check=True)
