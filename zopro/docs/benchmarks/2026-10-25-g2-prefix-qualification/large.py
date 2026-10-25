from pathlib import Path
import hashlib,json,os,time
ROOT=Path('/tmp/zopro-g2-prefix-qualification')
assert json.loads((ROOT/'complete.json').read_text())['passed'] and json.loads((ROOT/'cold-complete.json').read_text())['passed'] and json.loads((ROOT/'functional-complete.json').read_text())['passed']
(ROOT/'runs').mkdir(exist_ok=False)
exec((ROOT/'large-helpers.py').read_text())
exec((ROOT/'image-cache.py').read_text())
expected={250:'44340a201fb843267ec048967779f1128f8ac7ebfe80ea4af0662bee4a1bf91b',4999:'92d986baaa9d270c9a31e03a0cef78a002a3920eafac668edfdd91ca5904e8e0'}
attempts=[]
for batch in [250,4999]:
    image=Path('/tmp/zopro-original-constants')/f'batch-{batch}.image'
    with image.open('rb')as f:digest=hashlib.file_digest(f,'sha256').hexdigest()
    assert digest==expected[batch]
    size=image.stat().st_size;fd=os.open(image,os.O_RDONLY)
    try:
        before=resident(fd,size);os.fsync(fd);os.posix_fadvise(fd,0,0,os.POSIX_FADV_DONTNEED);after=resident(fd,size)
    finally:os.close(fd)
    (ROOT/f'cold-cache-{batch}-95.json').write_text(json.dumps(dict(created=time.time(),image=str(image),bytes=size,sha256=digest,before=before,after=after,cap_GiB=95,method='fsync/POSIX_FADV_DONTNEED on owned accepted image; PROT_NONE mincore'),indent=2)+'\n')
    assert after['resident_pages']==0
    print('EMPTY_IMAGE_CACHE',batch,flush=True)
    success=prove(batch,image,95);attempts.append(dict(batch=batch,cap_GiB=95,success=success))
    (ROOT/'large-progress.json').write_text(json.dumps(dict(completed=time.time(),attempts=attempts),indent=2)+'\n')
    assert success, 'Preserve failure; no undeclared fallback or unchanged retry.'
(ROOT/'large-complete.json').write_text(json.dumps(dict(completed=time.time(),attempts=attempts),indent=2)+'\n')
print('LARGE_QUALIFICATION_COMPLETE',flush=True)
