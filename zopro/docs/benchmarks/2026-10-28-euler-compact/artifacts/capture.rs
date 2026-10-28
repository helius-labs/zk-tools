//! Trusted local-only representation capture for this component study.
use std::{
    fs,
    io::Write,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};
use zopro_bn254::{ifma8::PackedIfma8, Fr, FrConfig};
static PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
static DONE: AtomicBool = AtomicBool::new(false);
pub fn begin(path: PathBuf) {
    let mut slot = PATH.lock().unwrap();
    assert!(slot.is_none());
    assert!(!path.exists());
    DONE.store(false, Ordering::Release);
    *slot = Some(path);
}
pub fn captured() -> bool {
    DONE.load(Ordering::Acquire)
}
pub(crate) fn capture(a: &[Fr], b: &[Fr], values: &[PackedIfma8<FrConfig>]) {
    let Some(path) = PATH.lock().unwrap().take() else {
        return;
    };
    assert_eq!(core::mem::size_of::<PackedIfma8<FrConfig>>(), 256);
    let mut out = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .unwrap();
    out.write_all(b"zopro-euler-in1\0").unwrap();
    for n in [a.len(), b.len(), values.len(), 256] {
        out.write_all(&(n as u64).to_le_bytes()).unwrap();
    }
    for v in a.iter().chain(b) {
        for word in v.0 {
            out.write_all(&word.to_le_bytes()).unwrap();
        }
    }
    // SAFETY: PackedIfma8 is four fully initialized SIMD vectors plus
    // PhantomData; its checked256-byte size contains no padding bytes.
    let raw =
        unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<u8>(), values.len() * 256) };
    out.write_all(raw).unwrap();
    out.flush().unwrap();
    DONE.store(true, Ordering::Release);
}
