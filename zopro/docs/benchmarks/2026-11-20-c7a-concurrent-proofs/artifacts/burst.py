"""Coordinate warmed processes using the existing prover perf-control FIFOs."""
import hashlib
import json
import os
from pathlib import Path
import selectors
import subprocess
import time

root=Path.cwd()
protocol=json.loads((root/'protocol.json').read_text())
assert root==Path(protocol['root'])
assert set(os.sched_getaffinity(0))==set(range(protocol['physical_cores']))
for file,digest in json.loads((root/'pins.json').read_text()).items():
    with open(file,'rb') as handle:
        assert hashlib.file_digest(handle,'sha256').hexdigest()==digest, file

def remap(profile,case,worker,directory):
    n,total,engine=case['workers'],case['total'],case['engine']
    capacity=96//n
    first=capacity*worker
    width=capacity if n>1 else 96
    argv,env=profile['argv'].copy(),profile['env'].copy()
    argv[2]=f'{first}-{first+width-1}'
    rounds=total//n
    assert rounds*n==total
    dest=root/'proofs'/directory.parent.name/directory.name
    front=[]
    if engine=='aeglos':
        env.update(GOMAXPROCS=str(width),AEGLOS_CPU_THREADS=str(width),ZOPRO_BENCH_PROOFS=str(dest))
        env.update(CROSS_PERF_CTL=str(directory/'ctl'),CROSS_PERF_ACK=str(directory/'ack'))
        argv[-1]=str(rounds)
        dest/=Path(argv[5]).stem
        main=width
    else:
        argv[-2:]=[str(rounds),str(dest)]
        old_main=int(env['RAYON_NUM_THREADS'])
        old_front=env.get('ZOPRO_BENCH_FRONT_CPUS','').split(',') if env.get('ZOPRO_BENCH_FRONT_CPUS') else []
        if n==1:
            main=old_main
            front=list(map(int,old_front))
        else:
            front_width=max(1,round(width*len(old_front)/(old_main+len(old_front)))) if old_front else 0
            main=width-front_width
            front=list(range(first+main,first+width))
            if front:
                env['ZOPRO_BENCH_FRONT_CPUS']=','.join(map(str,front))
            if env.get('ZOPRO_BENCH_SOLVER_CPUS'):
                old_solvers=len(env['ZOPRO_BENCH_SOLVER_CPUS'].split(','))
                solver_width=max(1,min(front_width-1,round(front_width*old_solvers/len(old_front))))
                assert 0<solver_width<front_width
                env['ZOPRO_BENCH_SOLVER_CPUS']=','.join(map(str,front[:solver_width]))
        env.update(RAYON_NUM_THREADS=str(main), ZOPRO_CPU_LAGRANGE_DIR=str(root/'cache'/case['shape']),
                   ZOPRO_BENCH_PERF_CTL=str(directory/'ctl'),ZOPRO_BENCH_PERF_ACK=str(directory/'ack'))
    verify=profile['verify'].copy()
    verify[3:5]=[str(dest/'0.proof'),str(dest/'0.public')]
    return argv,env,verify,main,front

def rss_mib(children):
    total=0
    for child in children:
        try:
            total+=int(Path(f'/proc/{child["process"].pid}/statm').read_text().split()[1])
        except FileNotFoundError:
            continue
    return total*os.sysconf('SC_PAGE_SIZE')/2**20

def file_records(source,key):
    return [json.loads(l.removeprefix(key)) for l in source.splitlines() if l.startswith(key)]

def run(case):
    name=f'{case["shape"]}-n{case["workers"]}-b{case["block"]}-{case["engine"]}'
    directory=root/'results'/name
    directory.mkdir()
    scope=Path('/sys/fs/cgroup')/f'zopro-load1031-{name}'
    scope.mkdir()
    children=[]
    profile=protocol['profiles'][case['shape']][case['engine']]
    try:
        (scope/'memory.max').write_text(str(protocol['memory_limit_bytes']))
        (scope/'memory.swap.max').write_text('0')
        with selectors.DefaultSelector() as sel:
            for i in range(case['workers']):
                worker=directory/str(i)
                worker.mkdir()
                for filename in ('ctl','ack'):
                    os.mkfifo(worker/filename,0o600)
                ctl=os.open(worker/'ctl',os.O_RDWR|os.O_NONBLOCK)
                ack=os.open(worker/'ack',os.O_RDWR|os.O_NONBLOCK)
                argv,env,verify,main,front=remap(profile,case,i,worker)
                def join_scope():
                    (scope/'cgroup.procs').write_text(str(os.getpid()))
                with (worker/'stdout.log').open('wb') as out, (worker/'stderr.log').open('wb') as err:
                    process=subprocess.Popen(argv,env=os.environ|env,stdout=out,stderr=err,preexec_fn=join_scope)
                child=dict(ctl=ctl,ack=ack,process=process,verify=verify,worker=worker,main=main,front=front,
                           argv=argv,ready=False,done=False)
                children.append(child)
                sel.register(ctl,selectors.EVENT_READ,child)

            deadline=time.monotonic()+180
            start=0
            rss_max=rss_ready=0
            next_sample=time.monotonic()
            while not all(c['done'] for c in children):
                assert time.monotonic()<deadline, f'gate timed out for {name}'
                for event,_ in sel.select(timeout=0.005):
                    c=event.data
                    line=os.read(c['ctl'],64)
                    if line==b'enable\n':
                        assert not c['ready'] and not start
                        c['ready']=True
                        c['ready_at']=time.monotonic()
                    else:
                        assert line==b'disable\n' and c['ready'] and start and not c['done'], line
                        c['done']=True
                        c['done_at']=time.monotonic_ns()
                        os.write(c['ack'],b'ack\n')
                        sel.unregister(c['ctl'])
                for c in children:
                    if not c['done'] and c['process'].poll() is not None:
                        raise RuntimeError(f'{c["argv"]} exited before its timing gate: {(c["worker"]/"stderr.log").read_text()}')
                now=time.monotonic()
                if not start and all(c['ready'] for c in children):
                    rss_ready=rss_mib(children)
                    rss_max=max(rss_max,rss_ready)
                    start=time.monotonic_ns()
                    for c in children:
                        c['released_at']=time.monotonic_ns()
                        os.write(c['ack'],b'ack\n')
                if now>=next_sample:
                    rss_max=max(rss_max,rss_mib(children))
                    next_sample=now+0.02

        for c in children:
            assert c['process'].wait(timeout=30)==0
        peak_bytes=int((scope/'memory.peak').read_text())
        events={k:int(v) for k,v in (line.split() for line in (scope/'memory.events').read_text().splitlines())}
        assert not events['oom'] and not events['oom_kill']
        rows=[]
        for i,c in enumerate(children):
            stdout=(c['worker']/'stdout.log').read_text()
            summary,=file_records(stdout,'SUMMARY ')
            samples=file_records(stdout,'SAMPLE ')
            rounds=case['total']//case['workers']
            assert summary['rounds']==rounds and summary['priming']==3
            assert int(summary.get('threads',summary.get('workers')))==c['main']
            assert [s['round'] for s in samples]==list(range(rounds))
            if case['engine']=='aeglos':
                assert all(s['cache_hit'] for s in samples)
            elif c['front']:
                assert file_records(stdout,'FRONT_CPUS ')==[c['front']]
                main,=file_records(stdout,'MAIN_CPUS ')
                assert len(main)==c['main'] and not set(main)&set(c['front'])
            vals=[s['ms'] if 'ms' in s else s['verified_ms'] for s in samples]
            first=(c['released_at']-start)/1e6
            last=(c['done_at']-start)/1e6
            output=subprocess.check_output(c['verify'],text=True,timeout=180)
            proof,=(line for line in output.splitlines() if line.startswith('VERIFIED '))
            rows.append(dict(worker=i,first_ms=first,last_ms=last,verified=proof,verify=c['verify'],
                             main=c['main'],frontend=c['front'],proof_ms=vals,summary=summary,argv=c['argv']))
        wall=max(r['last_ms'] for r in rows)
        result=case|dict(name=name,burst_ms=wall,proofs_per_second=case['total']*1000/wall,
                         sampled_max_rss_mib=rss_max,warm_rss_mib=rss_ready,group_peak_mib=peak_bytes/2**20,
                         memory_events=events,workers_data=rows)
        (directory/'result.json').write_text(json.dumps(result))
        print(f'DONE {name} {wall:.3f} ms {peak_bytes/2**20:.2f} MiB',flush=True)
    finally:
        for c in children:
            if c['process'].poll() is None:
                c['process'].kill()
            c['process'].wait()
            for k in ('ctl','ack'):
                os.close(c[k])
            for k in ('ctl','ack'):
                (c['worker']/k).unlink()
        scope.rmdir()

for case in protocol['cases']:
    run(case)
