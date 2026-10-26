//! Host allocator with prompt release of large proof temporaries under pressure.
//! Linux uses the proof workspace forecast and current host/cgroup headroom.
//! Other targets retain their previous allocation policy.
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::Allocator;
#[cfg(target_os = "linux")]
pub(crate) use linux::ProofScope;
#[cfg(not(target_os = "linux"))]
mod system;
#[cfg(not(target_os = "linux"))]
pub use system::Allocator;
#[cfg(not(target_os = "linux"))]
pub(crate) struct ProofScope;
#[cfg(not(target_os = "linux"))]
impl ProofScope {
    pub(crate) fn enter(_: Option<usize>) -> Self { Self }
    pub(crate) fn release_unused<T>(&self, _: &mut Vec<T>) {}
}
