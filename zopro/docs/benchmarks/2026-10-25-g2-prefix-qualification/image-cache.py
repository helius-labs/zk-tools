import ctypes,os
libc=ctypes.CDLL(None,use_errno=True)
libc.mmap.restype=ctypes.c_void_p
libc.mmap.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_long]
libc.mincore.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.POINTER(ctypes.c_ubyte)]
libc.mincore.restype=ctypes.c_int
libc.munmap.argtypes=[ctypes.c_void_p,ctypes.c_size_t]
libc.munmap.restype=ctypes.c_int
def resident(fd,size):
 pages=(size+os.sysconf('SC_PAGE_SIZE')-1)//os.sysconf('SC_PAGE_SIZE')
 vector=(ctypes.c_ubyte*pages)()
 # A PROT_NONE mapping provides residency information without reading pages.
 address=libc.mmap(None,size,0,1,fd,0)
 assert address!=ctypes.c_void_p(-1).value,os.strerror(ctypes.get_errno())
 try:
  assert libc.mincore(address,size,vector)==0,os.strerror(ctypes.get_errno())
  return dict(pages=pages,resident_pages=sum(v&1 for v in bytes(vector)),page_size=os.sysconf('SC_PAGE_SIZE'))
 finally:assert libc.munmap(address,size)==0
