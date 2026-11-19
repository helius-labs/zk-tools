set pagination off
set confirm off
set print thread-events off
set disable-randomization off
starti
python
import gdb,os
from pathlib import Path
class Capture(gdb.Breakpoint):
    def stop(self):
        n=int(gdb.parse_and_eval('$rdx'))
        assert n==int(gdb.parse_and_eval('$r8'))==int(gdb.parse_and_eval('$r9'))
        root=Path(os.environ['CLASS_OUTPUT'])
        assert n in {int(s) for s in os.environ['CLASS_LENGTHS'].split(',')}
        proc=gdb.selected_inferior()
        for reg,width,part in [('rsi',32,'values'),('rcx',8,'counts')]:
            val=bytes(proc.read_memory(int(gdb.parse_and_eval('$'+reg)),width*n))
            p=root/(str(n)+'.'+part)
            if p.exists():
                assert p.read_bytes()==val
            else:
                p.write_bytes(val)
        with (root/'captures').open('a') as log: log.write(str(n)+'\n')
        return False
Capture("*'_RNvNtNtCs90rqOEZpJ68_10zopro_core10msm_affine13split_classes5build'")
end
continue
