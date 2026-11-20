from zp_report_common import *
import os
d=base/'compressed-final-exponent867'
files=['crates/zopro-bn254/src/'+name for name in ['pairing.rs','pairing_compressed_trial.rs','tests.rs']]
source=[]
for file in files:
    old=repo/file
    source.extend(difflib.unified_diff(old.read_text().splitlines(keepends=True) if old.exists() else [],
                                      (d/file).read_text().splitlines(keepends=True),
                                      'a/'+file if old.exists() else '/dev/null','b/'+file))
(d/'source.patch').write_text(''.join(source))
env=os.environ|rd(base/'varied-column-center427/build-plan.json')['environment']|{'CARGO_TARGET_DIR':str(base/'grouped-b2-layout767/target')}
cargo=['cargo','--offline']
args=['--locked','--release','--target','x86_64-unknown-linux-gnu']
with (d/'tests.log').open('w') as out:
    subprocess.run([cargo[0],'test',cargo[1],*args,'-p','zopro-bn254','--lib'],cwd=d,env=env,stdout=out,stderr=subprocess.STDOUT,check=True)
with (d/'build.log').open('w') as out:
    subprocess.run([cargo[0],'build',cargo[1],*args,'-p','zopro-bench','--example','cpu-sdk'],cwd=d,env=env,stdout=out,stderr=subprocess.STDOUT,check=True)
shutil.copy2(Path(env['CARGO_TARGET_DIR'])/'x86_64-unknown-linux-gnu/release/examples/cpu-sdk',d/'cpu-sdk')
save(d/'built.json',dict(binary=sha(d/'cpu-sdk'),source_patch=sha(d/'source.patch'),
                        tests=sha(d/'tests.log'),environment={k:env[k] for k in ('RUSTFLAGS','RUSTUP_TOOLCHAIN')}))
