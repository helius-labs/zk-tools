//! Optional perf control and process accounting for SDK benchmarks.
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
    time::{Duration, Instant},
};

pub struct Control {
    command: File,
    acknowledgement: File,
    enabled: bool,
}

impl Control {
    pub fn from_env() -> io::Result<Option<Self>> {
        match (
            std::env::var_os("ZOPRO_BENCH_PERF_CTL"),
            std::env::var_os("ZOPRO_BENCH_PERF_ACK"),
        ) {
            (None, None) => Ok(None),
            (Some(command), Some(ack)) => {
                Self::open(Path::new(&command), Path::new(&ack)).map(Some)
            }
            _ => Err(io::Error::other(
                "both perf control FIFO paths are required",
            )),
        }
    }

    #[cfg(target_os = "linux")]
    fn open(command: &Path, acknowledgement: &Path) -> io::Result<Self> {
        use std::os::unix::{
            fs::{FileTypeExt, OpenOptionsExt},
            io::AsRawFd,
        };
        let open = |path: &Path| -> io::Result<File> {
            if !std::fs::metadata(path)?.file_type().is_fifo() {
                return Err(io::Error::other("perf control path must be a FIFO"));
            }
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(libc::O_NONBLOCK | libc::O_CLOEXEC)
                .open(path)?;
            // Check the opened descriptor as well as the path before using it.
            if !file.metadata()?.file_type().is_fifo() || file.as_raw_fd() < 0 {
                return Err(io::Error::other("invalid perf FIFO"));
            }
            Ok(file)
        };
        Ok(Self {
            command: open(command)?,
            acknowledgement: open(acknowledgement)?,
            enabled: false,
        })
    }

    #[cfg(not(target_os = "linux"))]
    fn open(_: &Path, _: &Path) -> io::Result<Self> {
        Err(io::Error::other("perf control currently requires Linux"))
    }

    #[cfg(target_os = "linux")]
    fn send(&mut self, command: &[u8]) -> io::Result<()> {
        use std::os::fd::AsRawFd;
        self.command.write_all(command)?;
        self.command.flush()?;
        let deadline = Instant::now() + Duration::from_secs(3);
        let mut reply = Vec::with_capacity(32);
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "perf acknowledgement timed out",
                ));
            }
            let mut descriptor = libc::pollfd {
                fd: self.acknowledgement.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            // SAFETY: poll receives one initialized descriptor valid through the call.
            let result = unsafe {
                libc::poll(
                    &mut descriptor,
                    1,
                    remaining.as_millis().max(1).min(i32::MAX as u128) as i32,
                )
            };
            if result < 0 {
                let error = io::Error::last_os_error();
                if error.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }
            if result == 0 {
                continue;
            }
            let mut bytes = [0u8; 32];
            match self.acknowledgement.read(&mut bytes) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "perf acknowledgement closed",
                    ))
                }
                Ok(n) => reply.extend_from_slice(&bytes[..n]),
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => continue,
                Err(error) => return Err(error),
            }
            if reply.len() > 64 {
                return Err(io::Error::other("oversized perf acknowledgement"));
            }
            if reply.contains(&b'\n') {
                if reply
                    .split(|b| b.is_ascii_whitespace() || *b == 0)
                    .filter(|s| !s.is_empty())
                    .eq([b"ack".as_slice()])
                {
                    return Ok(());
                }
                return Err(io::Error::other("invalid perf acknowledgement"));
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn send(&mut self, _: &[u8]) -> io::Result<()> {
        Err(io::Error::other("perf control requires Linux"))
    }

    pub fn enable(&mut self) -> io::Result<()> {
        self.send(b"enable\n")?;
        self.enabled = true;
        Ok(())
    }
    pub fn disable(&mut self) -> io::Result<()> {
        if self.enabled {
            self.enabled = false;
            self.send(b"disable\n")?;
        }
        Ok(())
    }
}
impl Drop for Control {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}

pub struct Usage(libc::rusage);
impl Usage {
    pub fn now() -> io::Result<Self> {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        // SAFETY: getrusage writes the complete structure on successful return.
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self(unsafe { usage.assume_init() }))
    }
    pub fn difference(&self, before: &Self) -> serde_json::Value {
        let ms = |time: libc::timeval| time.tv_sec as f64 * 1000. + time.tv_usec as f64 / 1000.;
        let peak = self.0.ru_maxrss as u64;
        let peak = if cfg!(target_vendor = "apple") {
            peak / 1024
        } else {
            peak
        };
        serde_json::json!({"user_cpu_ms":ms(self.0.ru_utime)-ms(before.0.ru_utime),
            "system_cpu_ms":ms(self.0.ru_stime)-ms(before.0.ru_stime),
            "minor_faults":self.0.ru_minflt-before.0.ru_minflt,
            "voluntary_switches":self.0.ru_nvcsw-before.0.ru_nvcsw,
            "involuntary_switches":self.0.ru_nivcsw-before.0.ru_nivcsw,
            "whole_process_maxrss_kib":peak})
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader},
        os::unix::{ffi::OsStrExt, fs::OpenOptionsExt},
    };
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    fn fixture() -> std::path::PathBuf {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("zopro-perf-control-{}-{n}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        for name in ["ctl", "ack"] {
            let p = std::ffi::CString::new(path.join(name).as_os_str().as_bytes()).unwrap();
            assert_eq!(unsafe { libc::mkfifo(p.as_ptr(), 0o600) }, 0);
        }
        path
    }
    #[test]
    fn fifo_acknowledgements_gate_enable_and_drop_disables() {
        let directory = fixture();
        let server_dir = directory.clone();
        let server = std::thread::spawn(move || {
            let mut input = BufReader::new(
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(server_dir.join("ctl"))
                    .unwrap(),
            );
            let mut output = OpenOptions::new()
                .read(true)
                .write(true)
                .open(server_dir.join("ack"))
                .unwrap();
            for expected in ["enable\n", "disable\n"] {
                let mut command = String::new();
                input.read_line(&mut command).unwrap();
                assert_eq!(command, expected);
                output.write_all(b"ack\n").unwrap();
            }
        });
        {
            let mut control =
                Control::open(&directory.join("ctl"), &directory.join("ack")).unwrap();
            control.enable().unwrap();
        }
        server.join().unwrap();
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn ordinary_files_and_invalid_acknowledgements_fail() {
        let directory = fixture();
        std::fs::write(directory.join("ordinary"), b"").unwrap();
        assert!(Control::open(&directory.join("ordinary"), &directory.join("ack")).is_err());
        let mut control = Control::open(&directory.join("ctl"), &directory.join("ack")).unwrap();
        let mut ack = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(directory.join("ack"))
            .unwrap();
        ack.write_all(b"wrong\n").unwrap();
        assert!(control.enable().is_err());
        drop(control);
        std::fs::remove_dir_all(directory).unwrap();
    }
}
