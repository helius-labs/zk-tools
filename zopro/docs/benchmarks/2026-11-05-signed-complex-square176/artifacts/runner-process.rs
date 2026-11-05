use crate::Result;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::Read,
    os::fd::AsRawFd,
    os::unix::process::CommandExt,
    path::Path,
    process::{Command, Stdio},
};

pub fn sha256(path: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub fn json(path: impl AsRef<Path>, value: &impl Serialize) -> Result<()> {
    std::fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}

pub fn check_pins(pins: &BTreeMap<String, String>) -> Result<()> {
    for (path, hash) in pins {
        if &sha256(path)? != hash {
            return Err(format!("artifact changed: {path}").into());
        }
    }
    Ok(())
}

#[derive(PartialEq, Eq)]
struct Identity {
    device: u64,
    inode: u64,
    length: u64,
    modified: (i64, i64),
    changed: (i64, i64),
}
fn identity(path: &str) -> Result<Identity> {
    use std::os::unix::fs::MetadataExt;
    let m = std::fs::metadata(path)?;
    Ok(Identity {
        device: m.dev(),
        inode: m.ino(),
        length: m.len(),
        modified: (m.mtime(), m.mtime_nsec()),
        changed: (m.ctime(), m.ctime_nsec()),
    })
}

/// Full content checks bracket the run; intervening checks detect replacement
/// and mutation without repeatedly streaming every immutable proving key.
pub struct PinGuard<'a> {
    pins: &'a BTreeMap<String, String>,
    identities: BTreeMap<String, Identity>,
}
impl<'a> PinGuard<'a> {
    pub fn new(pins: &'a BTreeMap<String, String>) -> Result<Self> {
        let mut identities = BTreeMap::new();
        for (path, hash) in pins {
            let before = identity(path)?;
            if &sha256(path)? != hash || identity(path)? != before {
                return Err(format!("artifact changed while hashing: {path}").into());
            }
            identities.insert(path.clone(), before);
        }
        Ok(Self { pins, identities })
    }
    pub fn check_identity(&self) -> Result<()> {
        for (path, before) in &self.identities {
            if &identity(path)? != before {
                return Err(format!("artifact identity changed: {path}").into());
            }
        }
        Ok(())
    }
    pub fn verify(&self) -> Result<()> {
        self.check_identity()?;
        check_pins(self.pins)?;
        self.check_identity()
    }
}

pub struct HostLock(File);
#[cfg(test)]
pub fn test_experiment_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().unwrap()
}
impl HostLock {
    pub fn acquire() -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(std::env::temp_dir().join("zopro-cpu-perf.lock"))?;
        // SAFETY: flock receives an open descriptor and does not retain pointers.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err("another CPU experiment holds the host lock".into());
        }
        Ok(Self(file))
    }
}
impl Drop for HostLock {
    fn drop(&mut self) {
        // SAFETY: self owns this valid descriptor through the end of Drop.
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}

pub fn run(
    argv: &[String],
    directory: &Path,
    environment: &BTreeMap<String, String>,
    timeout: u64,
    prefix: &[String],
) -> Result<()> {
    // A root timeout bounds root perf descendants as well as the SDK process.
    let mut command = prefix.to_vec();
    command.extend([
        "timeout".into(),
        "-k".into(),
        "5s".into(),
        format!("{timeout}s"),
    ]);
    command.extend_from_slice(argv);
    let receipt: BTreeMap<_, _> = environment
        .iter()
        .filter(|(key, _)| crate::config::managed(key))
        .collect();
    let expected = serde_json::json!({"argv":command,"environment":receipt});
    if directory.exists() {
        let recorded: serde_json::Value =
            serde_json::from_slice(&std::fs::read(directory.join("command.json"))?)?;
        let exit: serde_json::Value =
            serde_json::from_slice(&std::fs::read(directory.join("exit.json"))?)?;
        if recorded != expected
            || exit["success"] != true
            || exit["stdout_sha256"].as_str() != Some(&sha256(directory.join("stdout.log"))?)
            || exit["stderr_sha256"].as_str() != Some(&sha256(directory.join("stderr.log"))?)
        {
            return Err(format!(
                "no matching successful checkpoint: {}; existing process is not restarted",
                directory.display()
            )
            .into());
        }
        println!("REUSE {}", directory.display());
        return Ok(());
    }
    std::fs::create_dir(directory)?;
    json(directory.join("command.json"), &expected)?;
    let started = std::time::Instant::now();
    let status = Command::new(&command[0])
        .args(&command[1..])
        .env_clear()
        .envs(environment)
        .stdin(Stdio::null())
        .stdout(File::create(directory.join("stdout.log"))?)
        .stderr(File::create(directory.join("stderr.log"))?)
        .process_group(0)
        .status()?;
    json(
        directory.join("exit.json"),
        &serde_json::json!({"returncode":status.code(),"success":status.success(),
            "elapsed_ms":started.elapsed().as_secs_f64()*1000.,
            "stdout_sha256":sha256(directory.join("stdout.log"))?,"stderr_sha256":sha256(directory.join("stderr.log"))?}),
    )?;
    if !status.success() {
        return Err(format!(
            "command failed ({status}); see {}",
            directory.join("stderr.log").display()
        )
        .into());
    }
    Ok(())
}

pub fn fifo(path: &Path) -> Result<()> {
    use std::os::unix::ffi::OsStrExt;
    let path = std::ffi::CString::new(path.as_os_str().as_bytes())?;
    // SAFETY: path is a NUL-terminated string valid for the duration of mkfifo.
    if unsafe { libc::mkfifo(path.as_ptr(), 0o600) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    fn temporary() -> std::path::PathBuf {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("zopro-xtask-test-{}-{n}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        path
    }
    #[test]
    fn hash_pins_detect_mutation() {
        let directory = temporary();
        let file = directory.join("input");
        std::fs::write(&file, b"abc").unwrap();
        assert_eq!(
            sha256(&file).unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let pins = BTreeMap::from([(file.to_str().unwrap().into(), sha256(&file).unwrap())]);
        check_pins(&pins).unwrap();
        std::fs::write(&file, b"changed").unwrap();
        assert!(check_pins(&pins).is_err());
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn pinned_identity_detects_same_content_replacement_and_content_verification_catches_edits() {
        let directory = temporary();
        let file = directory.join("input");
        std::fs::write(&file, b"abc").unwrap();
        let pins = BTreeMap::from([(file.to_str().unwrap().into(), sha256(&file).unwrap())]);
        let guard = PinGuard::new(&pins).unwrap();
        guard.check_identity().unwrap();
        let next = directory.join("next");
        std::fs::write(&next, b"abc").unwrap();
        std::fs::rename(next, &file).unwrap();
        assert!(guard.check_identity().is_err());
        let guard = PinGuard::new(&pins).unwrap();
        std::fs::write(&file, b"xyz").unwrap();
        assert!(guard.verify().is_err());
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn successful_process_is_reused_and_modified_logs_are_rejected() {
        let directory = temporary();
        let output = directory.join("run");
        let marker = directory.join("marker");
        let command = vec![
            "sh".into(),
            "-c".into(),
            "test ! -e \"$1\" && touch \"$1\"".into(),
            "fixture".into(),
            marker.to_str().unwrap().into(),
        ];
        run(&command, &output, &BTreeMap::new(), 2, &[]).unwrap();
        run(&command, &output, &BTreeMap::new(), 2, &[]).unwrap();
        std::fs::write(output.join("stdout.log"), "changed").unwrap();
        assert!(run(&command, &output, &BTreeMap::new(), 2, &[]).is_err());
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn failure_retains_exit_logs_and_redacts_ambient_environment() {
        let directory = temporary();
        let output = directory.join("run");
        let env = BTreeMap::from([
            ("ZOPRO_TEST".into(), "yes".into()),
            ("AWS_SECRET_ACCESS_KEY".into(), "must not appear".into()),
        ]);
        let result = run(
            &[
                "sh".into(),
                "-c".into(),
                "echo diagnostic >&2; exit 7".into(),
            ],
            &output,
            &env,
            2,
            &[],
        );
        assert!(result.is_err());
        assert!(std::fs::read_to_string(output.join("stderr.log"))
            .unwrap()
            .contains("diagnostic"));
        let receipt = std::fs::read_to_string(output.join("command.json")).unwrap();
        assert!(receipt.contains("ZOPRO_TEST"));
        assert!(!receipt.contains("must not appear"));
        assert!(std::fs::read_to_string(output.join("exit.json"))
            .unwrap()
            .contains('7'));
        std::fs::remove_dir_all(directory).unwrap();
    }
}
