//! Optional Linux worker placement for the verified SDK benchmark.
use std::{env, io, mem::size_of_val, sync::Arc};

fn allowed_cpus() -> io::Result<Vec<usize>> {
    let mut mask = vec![0usize; 16];
    loop {
        // SAFETY: the kernel writes at most the supplied byte length.
        if unsafe { libc::sched_getaffinity(0, size_of_val(&mask[..]), mask.as_mut_ptr().cast()) }
            == 0
        {
            let bits = usize::BITS as usize;
            return Ok((0..mask.len() * bits)
                .filter(|i| mask[i / bits] & (1 << (i % bits)) != 0)
                .collect());
        }
        let error = io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::EINVAL) {
            return Err(error);
        }
        // The kernel's CPU mask can be larger than libc::cpu_set_t.
        mask.resize(mask.len() * 2, 0);
    }
}

fn pin(cpu: usize) -> io::Result<usize> {
    let bits = usize::BITS as usize;
    let mut mask = vec![0usize; cpu / bits + 1];
    mask[cpu / bits] = 1 << (cpu % bits);
    // SAFETY: mask contains one valid CPU bit and lives through the syscall.
    if unsafe { libc::sched_setaffinity(0, size_of_val(&mask[..]), mask.as_ptr().cast()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    if allowed_cpus()? != [cpu] {
        return Err(io::Error::other("worker affinity changed during setup"));
    }
    Ok(cpu)
}

fn frontend_cpus(list: &str, allowed: &[usize]) -> Result<Vec<usize>, String> {
    let mut cpus = Vec::new();
    for field in list.split(',') {
        let cpu = field
            .trim()
            .parse()
            .map_err(|_| format!("invalid frontend CPU {field:?}"))?;
        if !allowed.contains(&cpu) || cpus.contains(&cpu) {
            return Err(format!("frontend CPU {cpu} is unavailable or repeated"));
        }
        cpus.push(cpu);
    }
    Ok(cpus)
}

pub(super) fn configure() -> Result<Option<Arc<rayon::ThreadPool>>, Box<dyn std::error::Error>> {
    let fixed = match env::var("ZOPRO_BENCH_WORKER_PLACEMENT").as_deref() {
        Ok("fixed") => true,
        Ok("unbound") | Err(env::VarError::NotPresent) => false,
        _ => return Err("worker placement must be fixed or unbound".into()),
    };
    let list = env::var_os("ZOPRO_BENCH_FRONT_CPUS");
    if list.is_none() && !fixed {
        return Ok(None);
    }
    let allowed = allowed_cpus()?;
    let main_width = rayon::current_num_threads();
    if fixed && main_width > allowed.len() {
        return Err("fixed placement requires one allowed CPU per main worker".into());
    }
    let frontend = if let Some(list) = list {
        let ids = frontend_cpus(
            list.to_str().ok_or("frontend CPU list must be UTF-8")?,
            &allowed,
        )?;
        if ids.len() >= main_width {
            return Err("the frontend pool must be smaller than the main pool".into());
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(ids.len())
            .build()?;
        let cpus: io::Result<Vec<_>> = pool
            .broadcast(|ctx| pin(ids[ctx.index()]))
            .into_iter()
            .collect();
        println!("FRONT_CPUS {}", serde_json::to_string(&cpus?)?);
        Some(Arc::new(pool))
    } else {
        None
    };
    if fixed {
        let cpus: io::Result<Vec<_>> = rayon::broadcast(|ctx| pin(allowed[ctx.index()]))
            .into_iter()
            .collect();
        println!("MAIN_CPUS {}", serde_json::to_string(&cpus?)?);
    }
    Ok(frontend)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontend_selection_respects_order_and_inherited_affinity() {
        let allowed = [3, 16, 2049];
        assert_eq!(frontend_cpus("2049, 3", &allowed).unwrap(), [2049, 3]);
        for invalid in ["", "3,3", "16,", "0", "-1", "3,7"] {
            assert!(frontend_cpus(invalid, &allowed).is_err());
        }
    }
}
