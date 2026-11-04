//! zopro-xark's commands:
//!
//!   zopro-xark rows <circuit.xbc> <pk.bin> <out r1cs.stream>
//!   zopro-xark witness <circuit.xbc> <pk.bin> <inputs.json> <out dir>
//!   zopro-xark bench <circuit dir> <key.zopro> <inputs.json> [rounds]
//!
//! `rows` writes the rows the key was made for, for `zopro-bench pack-xark`.
//! `witness` writes the zopro witness Xark solves from the inputs (`witness`
//! in the out directory, as `zopro-bench key-file` reads it). `bench` proves
//! the inputs with Xark's prover and with the zopro key, each proof
//! verified, and prints both's times; `RAYON_NUM_THREADS` sets the threads.
use std::{path::Path, process::ExitCode, time::Instant};

use xark_groth16::{ProverConfig, SerializedPk};
use zopro_xark::{Circuit, read_inputs};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["rows", xbc, pk, out] => rows(xbc, pk, out),
        ["witness", xbc, pk, inputs, out] => witness(xbc, pk, inputs, out),
        ["bench", dir, key, inputs] => bench(dir, key, inputs, 5),
        ["bench", dir, key, inputs, rounds] => match rounds.parse() {
            Ok(rounds) => bench(dir, key, inputs, rounds),
            Err(_) => Err("rounds is a number".into()),
        },
        _ => Err(
            "usage: zopro-xark rows <circuit.xbc> <pk.bin> <out r1cs.stream> | \
                  witness <circuit.xbc> <pk.bin> <inputs.json> <out dir> | \
                  bench <circuit dir> <key.zopro> <inputs.json> [rounds]"
                .into(),
        ),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("zopro-xark: {error}");
            ExitCode::FAILURE
        }
    }
}

fn read(path: impl AsRef<Path>) -> Result<Vec<u8>, String> {
    let path = path.as_ref();
    std::fs::read(path).map_err(|e| format!("reading {}: {e}", path.display()))
}

fn write(path: impl AsRef<Path>, bytes: &[u8]) -> Result<(), String> {
    let path = path.as_ref();
    std::fs::write(path, bytes).map_err(|e| format!("writing {}: {e}", path.display()))
}

fn read_inputs_file(path: &str) -> Result<Vec<(String, String)>, String> {
    read_inputs(&String::from_utf8_lossy(&read(path)?))
}

/// The circuit and the rows of its key, with the `r1cs.stream` `xark setup`
/// keeps beside a minimized key, as Xark's prover loads them.
fn load(xbc: &str, pk: &str) -> Result<(Circuit, Vec<u8>), String> {
    let key = read(pk)?;
    let cached = std::fs::read(Path::new(pk).with_file_name("r1cs.stream")).ok();
    Ok((Circuit::load(&read(xbc)?, &key, cached)?, key))
}

fn rows(xbc: &str, pk: &str, out: &str) -> Result<(), String> {
    let (circuit, _) = load(xbc, pk)?;
    let rows = circuit.rows();
    write(out, &rows)?;
    let layout = circuit.source().layout();
    println!(
        "ZOPRO xark-rows rows={} variables={} public={} bytes={}",
        circuit.source().num_constraints(),
        layout.num_vars(),
        layout.num_instance() - 1,
        rows.len()
    );
    Ok(())
}

fn witness(xbc: &str, pk: &str, inputs: &str, out: &str) -> Result<(), String> {
    let (circuit, _) = load(xbc, pk)?;
    let witness = circuit.witness(read_inputs_file(inputs)?)?;
    std::fs::create_dir_all(out).map_err(|e| format!("creating {out}: {e}"))?;
    write(
        Path::new(out).join("witness"),
        &zopro_prover::witness::encode(&witness),
    )
}

fn median(times: &mut [f64]) -> f64 {
    times.sort_by(f64::total_cmp);
    times.get(times.len() / 2).copied().unwrap_or(f64::NAN)
}

fn ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

fn bench(dir: &str, key: &str, inputs: &str, rounds: usize) -> Result<(), String> {
    let dir = Path::new(dir);
    let xbc = dir.join("circuit.xbc");
    let pk = dir.join("pk.bin");
    let (xbc, pk) = (
        xbc.to_str().ok_or("a path that is not UTF-8")?,
        pk.to_str().ok_or("a path that is not UTF-8")?,
    );
    let inputs = read_inputs_file(inputs)?;
    let threads = rayon::current_num_threads();

    // Xark: the circuit and the key's rows, the witness, the proof (the key
    // streamed from its bytes and decompressed, as Xark proves; verified).
    let started = Instant::now();
    let (circuit, pk_bytes) = load(xbc, pk)?;
    let xark_load = ms(started);
    let (mut solve, mut xark) = (Vec::new(), Vec::new());
    let mut z = Vec::new();
    for _ in 0..rounds {
        let started = Instant::now();
        z = circuit.assignment(inputs.clone())?;
        solve.push(ms(started));
        let started = Instant::now();
        let mut pk = SerializedPk::from_bytes(&pk_bytes).map_err(|e| e.to_string())?;
        xark_groth16::prove(
            circuit.source(),
            &z,
            &mut pk,
            &mut rand::rngs::OsRng,
            &ProverConfig::default(),
        )
        .map_err(|e| format!("Xark's proof: {e}"))?;
        xark.push(ms(started));
    }

    // zopro: the key, then proofs of the same witness (verified).
    let started = Instant::now();
    let mut prover = zopro_prover::Prover::load(&read(key)?)?;
    let zopro_load = ms(started);
    let witness = zopro_xark::zopro_witness(&z)?;
    let mut zopro = Vec::new();
    for _ in 0..rounds {
        let started = Instant::now();
        prover
            .prove(&witness)
            .map_err(|e| format!("zopro's proof: {e}"))?;
        zopro.push(ms(started));
    }
    println!(
        "ZOPRO xark-bench {} threads={threads} rounds={rounds} xark_load_ms={xark_load:.0} \
         xark_solve_ms={:.0} xark_prove_ms={:.0} zopro_load_ms={zopro_load:.0} zopro_prove_ms={:.0}",
        dir.file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default(),
        median(&mut solve),
        median(&mut xark),
        median(&mut zopro),
    );
    Ok(())
}
