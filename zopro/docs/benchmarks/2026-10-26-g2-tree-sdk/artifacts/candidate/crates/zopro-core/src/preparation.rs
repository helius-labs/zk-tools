//! Process-wide admission for optional CPU preparation, including scratch.
//! Required key data and proof workspace are outside this optional budget.
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) const MAX_DOMAIN: usize = 1 << 20;
// Legacy table builders share this process-wide ceiling. Audited solver
// indices and temporary analysis use the resource policy below.
const DEFAULT_LIMIT: usize = 64 << 20;
static RESERVED: AtomicUsize = AtomicUsize::new(0);

/// Holds the peak charge through construction. After all temporary builders
/// have returned, selected objects can release credit for freed scratch.
#[derive(Debug)]
pub(crate) struct Reservation(usize, usize);
// Forecast future proof allocations for all schedules that have opted in.
static WORKSPACES: AtomicUsize = AtomicUsize::new(0);
// Forecast updates and budget snapshots must commit together. Legacy table
// admission shares the same optional counter and respects active forecasts.
// Drops and credit release never take this brief admission lock.
static ADMISSION: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
pub(crate) fn charges_for_test()->(usize,usize){
    (RESERVED.load(Ordering::Acquire),WORKSPACES.load(Ordering::Acquire))
}

impl Reservation {
    /// A schedule's audited index buffers use actual costs, not the legacy
    /// table-builder ceiling. Keep expected proof allocations out of its budget.
    #[cfg(zopro_resource_schedule)]
    pub(crate) fn schedule(
        _domain: usize,
        peak: Option<usize>,
        workspace: Option<usize>,
    ) -> Option<Self> {
        let peak = peak?;
        let concurrency = match std::env::var("ZOPRO_CPU_PROOF_CONCURRENCY") {
            Ok(s) => s.parse::<usize>().ok().filter(|&n| n > 0)?,
            Err(std::env::VarError::NotPresent) => 1,
            Err(_) => return None,
        };
        let workspace = workspace?.checked_mul(concurrency)?;
        if workspace == 0 {
            return None;
        }
        let _admission = ADMISSION.lock().ok()?;
        WORKSPACES
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                n.checked_add(workspace)
            })
            .ok()?;
        // This token releases the forecast even if a later OS query or
        // admission fails. Growth charges only new index/output allocations.
        let mut owner = Self(0, workspace);
        if !reserve(&RESERVED, peak, resource_limit_now()?) {
            return None;
        }
        owner.0 = peak;
        Some(owner)
    }
    /// Admit optional scratch with checked capacity costs and active forecasts.
    /// The token covers construction and every consumer until it is dropped.
    pub(crate) fn analysis(peak: Option<usize>) -> Option<Self> {
        let peak = peak?;
        let _admission = ADMISSION.lock().ok()?;
        reserve(&RESERVED, peak, resource_limit_now()?).then(|| Self(peak, 0))
    }
    #[cfg(all(
        zopro_bounded_schedule,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    pub(crate) fn bytes(&self) -> usize {
        self.0
    }
    // Extend the original charge through a second allocation phase. The
    // selected policy and shared process-wide charges are checked before growth.
    #[cfg(all(
        zopro_bounded_schedule,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    pub(crate) fn increase(&mut self, domain: usize, extra: usize) -> bool {
        if extra == 0 {
            return true;
        }
        let Some(total) = self.0.checked_add(extra) else {
            return false;
        };
        #[cfg(zopro_resource_schedule)]
        if self.1 != 0 {
            let Ok(_admission) = ADMISSION.lock() else {
                return false;
            };
            let Some(limit) = resource_limit_now() else {
                return false;
            };
            if !reserve(&RESERVED, extra, limit) {
                return false;
            }
            self.0 = total;
            return true;
        }
        let Some(added) = Self::acquire(domain, Some(extra)) else {
            return false;
        };
        self.0 = total;
        std::mem::forget(added);
        true
    }
    /// Call only after builder temporaries have dropped, with all retained
    /// allocation capacities counted. Invalid estimates keep the peak charge.
    #[cfg(zopro_live_preparation_charge)]
    pub(crate) fn retain(mut self, bytes: Option<usize>) -> Self {
        retain_charge(&RESERVED, &mut self.0, bytes);
        self
    }

    pub(crate) fn acquire(domain: usize, peak: Option<usize>) -> Option<Self> {
        if domain > MAX_DOMAIN {
            return None;
        }
        let peak = peak?;
        let limit = match std::env::var("ZOPRO_CPU_PREPARE_MAX_BYTES") {
            Ok(value) => value.parse().ok()?,
            Err(std::env::VarError::NotPresent) => DEFAULT_LIMIT,
            Err(_) => return None,
        };
        // Do not commit optional preparation against more than a quarter of
        // current host/container headroom, leaving space for actual proofs.
        let limit = limit.min(DEFAULT_LIMIT);
        // An impossible request needs no OS queries or temporary parsing
        // allocations. This preserves admission while keeping the fallback cheap.
        if peak > limit {
            return None;
        }
        #[cfg(zopro_resource_schedule)]
        let _admission = ADMISSION.lock().ok()?;
        let limit = limit.min(headroom()? / 4);
        #[cfg(zopro_resource_schedule)]
        let limit = if WORKSPACES.load(Ordering::Acquire) == 0 {
            limit
        } else {
            limit.min(resource_limit_now()?)
        };
        reserve(&RESERVED, peak, limit).then(|| Self(peak, 0))
    }
}

impl Drop for Reservation {
    fn drop(&mut self) {
        RESERVED.fetch_sub(self.0, Ordering::AcqRel);
        #[cfg(zopro_resource_schedule)]
        WORKSPACES.fetch_sub(self.1, Ordering::AcqRel);
    }
}

#[cfg(zopro_live_preparation_charge)]
fn retain_charge(counter: &AtomicUsize, charge: &mut usize, bytes: Option<usize>) {
    if let Some(bytes) = bytes.filter(|&bytes| bytes <= *charge) {
        let released = *charge - bytes;
        *charge = bytes;
        counter.fetch_sub(released, Ordering::AcqRel);
    }
}

// Account allocation capacities, with a small per-allocation margin. Required
// key/proof workspace and allocator-wide overhead remain outside this budget.
#[cfg(zopro_live_preparation_charge)]
pub(crate) fn allocation_bytes<T>(values: &Vec<T>) -> Option<usize> {
    let bytes = values.capacity().checked_mul(core::mem::size_of::<T>())?;
    bytes.checked_add(if bytes == 0 { 0 } else { 64 })
}

fn reserve(counter: &AtomicUsize, peak: usize, limit: usize) -> bool {
    counter
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
            used.checked_add(peak).filter(|&total| total <= limit)
        })
        .is_ok()
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub(crate) fn headroom() -> Option<usize> {
    // Without an OS headroom estimate, retain the allocation-free fallback.
    Some(0)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) fn headroom() -> Option<usize> {
    let meminfo = std::fs::read_to_string("/proc/meminfo").ok()?;
    let host = meminfo.lines().find_map(|line| {
        line.strip_prefix("MemAvailable:")?
            .split_whitespace()
            .next()?
            .parse::<usize>()
            .ok()?
            .checked_mul(1024)
    })?;
    let groups = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    let mounts = std::fs::read_to_string("/proc/self/mountinfo").ok()?;
    Some(host.min(cgroup_headroom(&groups, &mounts).unwrap_or(usize::MAX)))
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn cgroup_headroom(groups: &str, mounts: &str) -> Option<usize> {
    use std::path::{Path, PathBuf};
    let mut available = None;
    for line in mounts.lines() {
        let Some((before, after)) = line.split_once(" - ") else {
            continue;
        };
        let parts: Vec<_> = before.split_whitespace().collect();
        let info: Vec<_> = after.split_whitespace().collect();
        if parts.len() < 5 || info.len() < 3 {
            continue;
        }
        let (v2, memory) = (
            info[0] == "cgroup2",
            info[2].split(',').any(|s| s == "memory"),
        );
        if !v2 && !(info[0] == "cgroup" && memory) {
            continue;
        }
        let root = unescape(parts[3]);
        let mount = PathBuf::from(unescape(parts[4]));
        for group in groups.lines() {
            let mut fields = group.splitn(3, ':');
            let (Some(_), Some(controllers), Some(path)) =
                (fields.next(), fields.next(), fields.next())
            else {
                continue;
            };
            if !(if v2 {
                controllers.is_empty()
            } else {
                controllers.split(',').any(|s| s == "memory")
            }) {
                continue;
            }
            let Ok(relative) = Path::new(path).strip_prefix(&root) else {
                continue;
            };
            let mut directory = mount.join(relative);
            while directory.starts_with(&mount) {
                let (limit_file, current_file) = if v2 {
                    ("memory.max", "memory.current")
                } else {
                    ("memory.limit_in_bytes", "memory.usage_in_bytes")
                };
                if let (Ok(limit), Ok(current)) = (
                    std::fs::read_to_string(directory.join(limit_file)),
                    std::fs::read_to_string(directory.join(current_file)),
                ) {
                    let limit = limit.trim().parse::<usize>().ok();
                    let high = v2
                        .then(|| {
                            std::fs::read_to_string(directory.join("memory.high"))
                                .ok()
                                .and_then(|s| s.trim().parse::<usize>().ok())
                        })
                        .flatten();
                    if let (Some(limit), Ok(current)) = (
                        limit.into_iter().chain(high).min(),
                        current.trim().parse::<usize>(),
                    ) {
                        let remaining = limit.saturating_sub(current);
                        available = Some(available.map_or(remaining, |n: usize| n.min(remaining)));
                    }
                }
                if directory == mount || !directory.pop() {
                    break;
                }
            }
        }
    }
    available
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn unescape(path: &str) -> String {
    path.replace("\\040", " ")
        .replace("\\011", "\t")
        .replace("\\012", "\n")
        .replace("\\134", "\\")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reservations_are_atomic_and_overflow_safe() {
        let used = AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for _ in 0..32 {
                scope.spawn(|| {
                    reserve(&used, 7, 100);
                });
            }
        });
        assert_eq!(used.load(Ordering::Acquire), 98);
        assert!(!reserve(&used, usize::MAX, usize::MAX));
    }
    #[test]
    #[cfg(zopro_live_preparation_charge)]
    fn freed_scratch_credit_is_reusable_without_losing_live_charge() {
        let used = AtomicUsize::new(0);
        assert!(reserve(&used, 80, 100));
        let mut charge = 80;
        retain_charge(&used, &mut charge, Some(20));
        assert_eq!(used.load(Ordering::Acquire), 20);
        assert!(reserve(&used, 80, 100));
        assert!(!reserve(&used, 1, 100));
        retain_charge(&used, &mut charge, None);
        retain_charge(&used, &mut charge, Some(usize::MAX));
        assert_eq!(charge, 20);
        assert_eq!(used.load(Ordering::Acquire), 100);
        used.fetch_sub(charge, Ordering::AcqRel);
        assert_eq!(used.load(Ordering::Acquire), 80);
        used.fetch_sub(80, Ordering::AcqRel);
        assert_eq!(used.load(Ordering::Acquire), 0);
        let mut v = Vec::<u64>::with_capacity(20);
        v.push(1);
        assert_eq!(allocation_bytes(&v), Some(v.capacity() * 8 + 64));
        assert_eq!(allocation_bytes(&Vec::<u64>::new()), Some(0));
    }
    #[test]
    fn huge_circuits_never_prepare_optional_tables() {
        assert!(Reservation::acquire(1 << 28, Some(1)).is_none());
        assert!(Reservation::acquire(8, None).is_none());
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn cgroup_parent_limit_and_namespace_mount_are_respected() {
        let root =
            std::env::temp_dir().join(format!("zopro-cgroup-test-{} memory", std::process::id()));
        std::fs::create_dir_all(root.join("child")).unwrap();
        for (directory, limit, used) in [(&root, "1000", "900"), (&root.join("child"), "max", "10")]
        {
            std::fs::write(directory.join("memory.max"), limit).unwrap();
            std::fs::write(directory.join("memory.current"), used).unwrap();
        }
        let mounts = format!(
            "1 0 0:1 /parent {} rw - cgroup2 cgroup rw\n",
            root.display().to_string().replace(" ", "\\040")
        );
        assert_eq!(cgroup_headroom("0::/parent/child\n", &mounts), Some(100));
        std::fs::write(root.join("memory.current"), "1001").unwrap();
        assert_eq!(cgroup_headroom("0::/parent/child\n", &mounts), Some(0));
        std::fs::write(root.join("memory.current"), "900").unwrap();
        std::fs::write(root.join("memory.high"), "950").unwrap();
        assert_eq!(cgroup_headroom("0::/parent/child\n", &mounts), Some(50));
        std::fs::write(root.join("memory.max"), "max").unwrap();
        assert_eq!(cgroup_headroom("0::/parent/child\n", &mounts), Some(50));
        for (directory, limit, used) in
            [(&root, "1000", "900"), (&root.join("child"), "1024", "10")]
        {
            std::fs::write(directory.join("memory.limit_in_bytes"), limit).unwrap();
            std::fs::write(directory.join("memory.usage_in_bytes"), used).unwrap();
        }
        let mounts = format!(
            "1 0 0:1 /parent {} rw - cgroup cgroup rw,memory\n",
            root.display().to_string().replace(" ", "\\040")
        );
        assert_eq!(
            cgroup_headroom("7:cpu,memory:/parent/child\n", &mounts),
            Some(100)
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}

// The resident key, prepared tables and existing live proof allocations are
// already reflected in OS headroom. Forecasts deliberately remain reserved
// as well: this errs toward fallback when several loaded keys may prove.
fn resource_limit(available: usize, workspace: usize, ceiling: usize) -> usize {
    available
        .saturating_sub(workspace)
        .checked_div(4)
        .unwrap()
        .min(ceiling)
}
fn resource_limit_now() -> Option<usize> {
    let ceiling = match std::env::var("ZOPRO_CPU_PREPARE_MAX_BYTES") {
        Ok(s) => s.parse().ok()?,
        Err(std::env::VarError::NotPresent) => usize::MAX,
        Err(_) => return None,
    };
    let mut available = headroom()?;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        available = available.min(address_headroom()?);
    }
    Some(resource_limit(
        available,
        WORKSPACES.load(Ordering::Acquire),
        ceiling,
    ))
}
#[cfg(any(target_os = "linux", target_os = "android"))]
fn address_headroom() -> Option<usize> {
    let limits = std::fs::read_to_string("/proc/self/limits").ok()?;
    let limit = limits
        .lines()
        .find_map(|s| s.strip_prefix("Max address space"))?
        .split_whitespace()
        .next()?;
    if limit == "unlimited" {
        return Some(usize::MAX);
    }
    let limit = limit.parse::<usize>().ok()?;
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let used = status
        .lines()
        .find_map(|s| s.strip_prefix("VmSize:"))?
        .split_whitespace()
        .next()?
        .parse::<usize>()
        .ok()?
        .checked_mul(1024)?;
    Some(limit.saturating_sub(used))
}
#[cfg(all(test, zopro_resource_schedule))]
mod resource_tests {
    use super::*;
    #[test]
    fn schedule_budget_tracks_headroom_workspace_concurrency_and_explicit_ceiling() {
        let mib = 1 << 20;
        assert_eq!(resource_limit(256 * mib, 192 * mib, usize::MAX), 16 * mib);
        assert_eq!(resource_limit(256 * mib, 96 * mib, usize::MAX), 40 * mib);
        assert_eq!(
            resource_limit(8 * 1024 * mib, 1024 * mib, usize::MAX),
            1792 * mib
        );
        assert_eq!(
            resource_limit(8 * 1024 * mib, 1024 * mib, 64 * mib),
            64 * mib
        );
        assert_eq!(resource_limit(256 * mib, 512 * mib, usize::MAX), 0);
        assert_eq!(resource_limit(usize::MAX, usize::MAX, usize::MAX), 0);
        assert_eq!(resource_limit(usize::MAX, 0, 0), 0);
        let used = AtomicUsize::new(0);
        assert!(reserve(
            &used,
            80 * mib,
            resource_limit(1024 * mib, 128 * mib, usize::MAX)
        ));
        // A second key or larger concurrency forecast reduces admission.
        assert!(!reserve(
            &used,
            80 * mib,
            resource_limit(1024 * mib, 768 * mib, usize::MAX)
        ));
    }
}
