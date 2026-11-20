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

fn parse_cpus(list: &str, allowed: &[usize], pool: &str) -> Result<Vec<usize>, String> {
    let mut cpus = Vec::new();
    for field in list.split(',') {
        let cpu = field
            .trim()
            .parse()
            .map_err(|_| format!("invalid {pool} CPU {field:?}"))?;
        if !allowed.contains(&cpu) || cpus.contains(&cpu) {
            return Err(format!("{pool} CPU {cpu} is unavailable or repeated"));
        }
        cpus.push(cpu);
    }
    Ok(cpus)
}

fn main_cpus(list: Option<&str>, allowed: &[usize], workers: usize) -> Result<Vec<usize>, String> {
    if workers > allowed.len() {
        return Err("fixed placement requires one allowed CPU per main worker".into());
    }
    let Some(list) = list else {
        return Ok(allowed[..workers].to_vec());
    };
    let cpus = parse_cpus(list, allowed, "main")?;
    if cpus.len() != workers {
        return Err(format!(
            "main CPU list has {} entries for {workers} workers",
            cpus.len()
        ));
    }
    Ok(cpus)
}

pub(super) fn configure(
    dedicated_g2: bool,
) -> Result<Option<Arc<rayon::ThreadPool>>, Box<dyn std::error::Error>> {
    let fixed = match env::var("ZOPRO_BENCH_WORKER_PLACEMENT").as_deref() {
        Ok("fixed") => true,
        Ok("unbound") | Err(env::VarError::NotPresent) => false,
        _ => return Err("worker placement must be fixed or unbound".into()),
    };
    let list = env::var_os("ZOPRO_BENCH_FRONT_CPUS");
    if dedicated_g2 && (!fixed || list.is_none()) {
        return Err("exclusive G2 placement requires two fixed worker pools".into());
    }
    let main_list = match env::var("ZOPRO_BENCH_MAIN_CPUS") {
        Ok(list) => Some(list),
        Err(env::VarError::NotPresent) => None,
        Err(error) => return Err(error.into()),
    };
    if !fixed && main_list.is_some() {
        return Err("a main CPU list requires fixed placement".into());
    }
    if list.is_none() && !fixed {
        return Ok(None);
    }
    let allowed = allowed_cpus()?;
    let main_width = rayon::current_num_threads();
    let main = fixed
        .then(|| main_cpus(main_list.as_deref(), &allowed, main_width))
        .transpose()?;
    let frontend = if let Some(list) = list {
        let ids = parse_cpus(
            list.to_str().ok_or("frontend CPU list must be UTF-8")?,
            &allowed,
            "frontend",
        )?;
        if dedicated_g2
            && main
                .as_ref()
                .is_some_and(|cpus| ids.iter().any(|id| cpus.contains(id)))
        {
            return Err("exclusive G2 requires disjoint worker placement".into());
        }
        if ids.len() >= main_width && !dedicated_g2 {
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
    if let Some(main) = main {
        let cpus: io::Result<Vec<_>> = rayon::broadcast(|ctx| pin(main[ctx.index()]))
            .into_iter()
            .collect();
        println!("MAIN_CPUS {}", serde_json::to_string(&cpus?)?);
    }
    Ok(frontend)
}

pub(super) fn configure_solver()
-> Result<Option<Arc<rayon::ThreadPool>>, Box<dyn std::error::Error>> {
    let list = match env::var("ZOPRO_BENCH_SOLVER_CPUS") {
        Ok(list) => list,
        Err(env::VarError::NotPresent) => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let allowed = allowed_cpus()?;
    let front = parse_cpus(&env::var("ZOPRO_BENCH_FRONT_CPUS")?, &allowed, "frontend")?;
    let prep = parse_cpus(&list, &front, "preparation")?;
    if prep.len() >= front.len() {
        return Err("preparation CPU list must be a proper subset of frontend CPUs".into());
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(prep.len())
        .build()?;
    let cpus: io::Result<Vec<_>> = pool
        .broadcast(|ctx| pin(prep[ctx.index()]))
        .into_iter()
        .collect();
    println!("SOLVER_CPUS {}", serde_json::to_string(&cpus?)?);
    Ok(Some(Arc::new(pool)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontend_selection_respects_order_and_inherited_affinity() {
        let allowed = [3, 16, 2049];
        assert_eq!(
            parse_cpus("2049, 3", &allowed, "frontend").unwrap(),
            [2049, 3]
        );
        for invalid in ["", "3,3", "16,", "0", "-1", "3,7"] {
            assert!(parse_cpus(invalid, &allowed, "frontend").is_err());
        }
    }

    #[test]
    fn main_selection_matches_worker_count_and_affinity() {
        let allowed = [3, 16, 23, 2049];
        assert_eq!(main_cpus(None, &allowed, 3).unwrap(), [3, 16, 23]);
        assert_eq!(
            main_cpus(Some("2049,23,3"), &allowed, 3).unwrap(),
            [2049, 23, 3]
        );
        for invalid in ["3,16", "3,3,16", "3,16,17", "3,16,23,2049"] {
            assert!(main_cpus(Some(invalid), &allowed, 3).is_err());
        }
    }
}
