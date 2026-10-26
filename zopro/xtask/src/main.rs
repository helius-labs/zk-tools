mod config;
mod instructions;
mod prepare;
mod process;
mod results;
mod runner;
mod screen;
mod workflow;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(error) = execute() {
        eprintln!("xtask: {error}");
        std::process::exit(1);
    }
}

fn execute() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.first().is_some_and(|s| s == "impact") {
        if !(4..=5).contains(&args.len()) {
            return Err("usage: cargo xtask impact PROOF_MS COMPONENT_BEFORE_US COMPONENT_AFTER_US [TARGET_MS]".into());
        }
        let number = |i: usize| -> Result<f64> {
            Ok(args[i].to_str().ok_or("numbers must be UTF-8")?.parse()?)
        };
        let value = screen::impact(
            number(1)?,
            number(2)?,
            number(3)?,
            if args.len() == 5 {
                Some(number(4)?)
            } else {
                None
            },
        )?;
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }
    if args.first().is_some_and(|s| s == "screen") {
        if args.len() < 4 {
            return Err(
                "usage: cargo xtask screen FULL_CONFIG_JSON OUTPUT CASE [CASE ...] [--resume]"
                    .into(),
            );
        }
        let resume = args.last().is_some_and(|s| s == "--resume");
        let mut names = Vec::new();
        for arg in &args[3..args.len() - usize::from(resume)] {
            names.push(arg.to_str().ok_or("case names must be UTF-8")?.to_owned());
        }
        let config = serde_json::from_slice(&std::fs::read(&args[1])?)?;
        let config = screen::prepare(config, &names)?;
        let value = runner::evaluate_screen(config, &std::path::absolute(&args[2])?, resume)?;
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }
    if args.first().is_some_and(|s| s == "instructions") {
        if args.len() != 3 {
            return Err("usage: cargo xtask instructions CONFIG_JSON OUTPUT_DIRECTORY".into());
        }
        let config = serde_json::from_slice(&std::fs::read(&args[1])?)?;
        let result = instructions::evaluate(config, &std::path::absolute(&args[2])?)?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }
    if args.first().is_some_and(|s| s == "prepare") {
        if !(4..=5).contains(&args.len()) {
            return Err(
                "usage: cargo xtask prepare MANIFEST SOURCE_ROOT DESTINATION [OVERLAY]".into(),
            );
        }
        let destination = std::path::absolute(&args[3])?;
        let snapshot = prepare::prepare(
            std::path::Path::new(&args[1]),
            std::path::Path::new(&args[2]),
            &destination,
            args.get(4).map(std::path::Path::new),
        )?;
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
        return Ok(());
    }
    if args.first().is_some_and(|s| s == "workflow") {
        if !(3..=4).contains(&args.len()) || args.get(3).is_some_and(|s| s != "--resume") {
            return Err("usage: cargo xtask workflow PLAN_JSON OUTPUT [--resume]".into());
        }
        let plan = serde_json::from_slice(&std::fs::read(&args[1])?)?;
        let result = workflow::evaluate(plan, &std::path::absolute(&args[2])?, args.len() == 4)?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }
    if !(3..=4).contains(&args.len())
        || args[0] != "perf"
        || args.get(3).is_some_and(|s| s != "--resume")
    {
        return Err("usage: cargo xtask perf CONFIG_JSON OUTPUT_DIRECTORY [--resume]".into());
    }
    let config = serde_json::from_slice(&std::fs::read(&args[1])?)?;
    let output = std::path::absolute(&args[2])?;
    let result = runner::evaluate(config, &output, args.len() == 4)?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
