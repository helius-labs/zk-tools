//! Builds a crate of zk circuits: exports each circuit's r1cs through the hook
//! its derive emits, generates or reuses setup keys, writes the committed
//! verifier modules, builds the wasm package with wasm-pack, and builds the
//! Solana program that verifies the proofs. Also imports ceremony keys and
//! exports their verifying keys.
//!
//! A circuit crate depends on `zolana-zk-dsl` (circuits deriving `ZkCircuit`)
//! or on `zolana-program` (ZK programs deriving `ZkProgramCircuit`); the build reads
//! which from cargo metadata and enables that crate's wasm features.

mod error;
mod node;
mod options;

use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    io::ErrorKind,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub use error::BuildError;
pub use options::{
    BuildOptions, CompileOptions, ExportVerifierOptions, ImportOptions, PackageSelection,
    SbfOptions, SetupOptions, WasmTarget,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{
    r1cs_hook::{HOOK_PREFIX, MARKER, OUT_DIR_ENV, PRODUCTION_KEY, VERIFIER_SUFFIX},
    Groth16Keys, R1cs, SetupKind, VerifyingKeyExport,
};

type Result<T, E = BuildError> = core::result::Result<T, E>;

const VERIFIER_MOD: &str = "mod.rs";
const STAGING: &str = ".staging";
const WASM_STAMP: &str = ".zolana-wasm.stamp";
const CIRCUIT_DIGEST: &str = "pub const ZOLANA_CIRCUIT_SHA256: &str =";
/// The packages whose code synthesizes the constraints, built optimized for the
/// r1cs export so it does not take minutes in debug.
const OPTIMIZED_PACKAGES: [&str; 9] = [
    "zolana-zk-dsl",
    "ark-ff",
    "ark-ec",
    "ark-bn254",
    "ark-poly",
    "ark-relations",
    "ark-r1cs-std",
    "ark-std",
    "light-poseidon",
];
const GADGETS: &str = "zolana-zk-gadgets";
const DSL: &str = "zolana-zk-dsl";
const PROGRAM: &str = "zolana-program";

const THREADS_LINK_ARGS: [&str; 7] = [
    "--shared-memory",
    "--max-memory=1073741824",
    "--import-memory",
    "--export=__wasm_init_tls",
    "--export=__tls_size",
    "--export=__tls_align",
    "--export=__tls_base",
];
const UNTYPED_THREAD_POOL: &str =
    "export function initThreadPool(num_threads: number): Promise<any>;";
const TYPED_THREAD_POOL: &str = "export function initThreadPool(threads: number): Promise<void>;";

fn wasm_output_complete(out: &Path, library: &str, target: WasmTarget, threads: bool) -> bool {
    let mut required = vec![
        out.join(format!("{library}_bg.wasm")),
        out.join(format!("{library}_bg.wasm.d.ts")),
        out.join(format!("{library}.js")),
        out.join(format!("{library}.d.ts")),
        out.join("package.json"),
    ];
    if target == WasmTarget::Nodejs {
        required.extend([out.join("node.mjs"), out.join("node.d.mts")]);
    }
    required.iter().all(|path| path.is_file())
        && (!threads || node::thread_helper_complete(out, library, target == WasmTarget::Nodejs))
}

/// Builds a Solana program that verifies zk circuits: compiles its circuits
/// (r1cs, keys, verifier modules), then builds its wasm package and its
/// SBF program in parallel.
pub fn build(opts: &BuildOptions) -> Result<()> {
    let circuits = CircuitCrate::resolve(&opts.compile.package)?;
    compile_circuits(&opts.compile, &circuits)?;
    if opts.compile.check {
        return Ok(());
    }
    std::thread::scope(|scope| {
        let wasm = (!opts.compile.skip_wasm)
            .then(|| scope.spawn(|| compile_wasm(&opts.compile, &circuits)));
        let sbf = if opts.sbf.skip_sbf {
            Ok(())
        } else {
            build_sbf(&opts.sbf, &circuits, opts.compile.test)
        };
        let wasm = wasm.map_or(Ok(()), |wasm| {
            wasm.join().unwrap_or(Err(BuildError::WasmBuildPanicked))
        });
        sbf.and(wasm)
    })
}

fn with_test_feature(features: &str, test: bool) -> String {
    let mut features = features
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();
    if test && !features.contains(&"insecure-test-setup") {
        features.push("insecure-test-setup");
    }
    features.join(",")
}

fn build_sbf(opts: &SbfOptions, circuits: &CircuitCrate, test: bool) -> Result<()> {
    let mut build_sbf = Command::new(cargo());
    build_sbf
        .args(["build-sbf", "--manifest-path"])
        .arg(&circuits.manifest);
    if let Some(dir) = &opts.sbf_out_dir {
        build_sbf.arg("--sbf-out-dir").arg(dir);
    }
    let features = with_test_feature(opts.sbf_features.as_deref().unwrap_or_default(), test);
    if !features.is_empty() {
        build_sbf.args(["--", "--features", &features]);
    }
    let status = build_sbf.status().map_err(|error| BuildError::Spawn {
        command: "cargo build-sbf".into(),
        hint: "; install the Solana platform tools",
        error,
    })?;
    if !status.success() {
        return Err(BuildError::CommandFailed {
            command: format!("cargo build-sbf for {}", circuits.manifest.display()),
            status,
            output: String::new(),
        });
    }
    Ok(())
}

/// Compiles the circuits of a crate: r1cs, keys and verifier modules,
/// then its wasm package. With `check`, compares the verifier modules instead
/// of writing them and builds no wasm.
pub fn compile(opts: &CompileOptions) -> Result<()> {
    let circuits = CircuitCrate::resolve(&opts.package)?;
    if opts.skip_r1cs && opts.skip_keys && opts.skip_wasm && !opts.check {
        return Err(BuildError::NothingToCompile);
    }
    compile_circuits(opts, &circuits)?;
    if !opts.skip_wasm && !opts.check {
        compile_wasm(opts, &circuits)?;
    }
    Ok(())
}

/// Where a circuit library a crate depends on is named in its manifest, which
/// is the name its features are enabled through.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Dependency {
    package: String,
    name: String,
}

struct CircuitCrate {
    manifest: PathBuf,
    dir: PathBuf,
    lib_name: String,
    target_dir: PathBuf,
    build_dir: PathBuf,
    verifier_dir: PathBuf,
    dependencies: Vec<Dependency>,
}

impl CircuitCrate {
    fn resolve(selection: &PackageSelection) -> Result<Self> {
        let manifest = match &selection.manifest_path {
            Some(path) => path.clone(),
            None => nearest_manifest()?,
        };
        let output = Command::new(cargo())
            .args([
                "metadata",
                "--format-version",
                "1",
                "--no-deps",
                "--manifest-path",
            ])
            .arg(&manifest)
            .stderr(Stdio::inherit())
            .output()
            .map_err(|error| BuildError::Spawn {
                command: format!("cargo metadata for {}", manifest.display()),
                hint: "",
                error,
            })?;
        if !output.status.success() {
            return Err(BuildError::CommandFailed {
                command: format!("cargo metadata for {}", manifest.display()),
                status: output.status,
                output: String::new(),
            });
        }
        let metadata_error = |reason| BuildError::Metadata {
            manifest: manifest.clone(),
            reason,
        };
        let metadata: Value = serde_json::from_slice(&output.stdout)
            .map_err(|_| metadata_error("its output is not JSON"))?;
        let canonical = manifest
            .canonicalize()
            .map_err(BuildError::file("resolving", &manifest))?;
        let packages = metadata
            .get("packages")
            .and_then(Value::as_array)
            .ok_or_else(|| metadata_error("it lists no packages"))?;
        let package = match &selection.package {
            Some(name) => packages
                .iter()
                .find(|package| package.get("name").and_then(Value::as_str) == Some(name))
                .ok_or_else(|| BuildError::NoSuchPackage {
                    manifest: canonical.clone(),
                    package: name.clone(),
                })?,
            None => packages
                .iter()
                .find(|package| {
                    package
                        .get("manifest_path")
                        .and_then(Value::as_str)
                        .is_some_and(|path| Path::new(path) == canonical)
                })
                .ok_or_else(|| BuildError::NotAPackage(canonical.clone()))?,
        };
        let manifest = PathBuf::from(
            package
                .get("manifest_path")
                .and_then(Value::as_str)
                .ok_or_else(|| metadata_error("a package has no manifest"))?,
        );
        let name = package
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| metadata_error("a package has no name"))?;
        let lib_name = package
            .get("targets")
            .and_then(Value::as_array)
            .and_then(|targets| {
                targets.iter().find(|target| {
                    target
                        .get("crate_types")
                        .and_then(Value::as_array)
                        .is_some_and(|types| types.iter().any(|kind| kind == "cdylib"))
                })
            })
            .and_then(|target| target.get("name"))
            .and_then(Value::as_str)
            .unwrap_or(name)
            .replace('-', "_");
        let target_dir = PathBuf::from(
            metadata
                .get("target_directory")
                .and_then(Value::as_str)
                .ok_or_else(|| metadata_error("it names no target directory"))?,
        );
        let dir = manifest
            .parent()
            .ok_or_else(|| metadata_error("a manifest has no directory"))?
            .to_path_buf();
        Ok(Self {
            build_dir: target_dir.join("zk").join(name),
            verifier_dir: dir.join("src").join("verifier"),
            dependencies: dependencies(package),
            manifest,
            dir,
            lib_name,
            target_dir,
        })
    }

    fn dependency(&self, package: &str) -> Option<&Dependency> {
        self.dependencies
            .iter()
            .find(|dependency| dependency.package == package)
    }

    /// The crate whose wasm features build the wasm runtime: `zolana-program`
    /// for a ZK program, `zolana-zk-dsl` for a standalone circuit.
    fn circuit_library(&self) -> Result<&Dependency> {
        self.dependency(PROGRAM)
            .or_else(|| self.dependency(DSL))
            .ok_or_else(|| BuildError::NoCircuitLibrary(self.manifest.clone()))
    }

    fn optimized_packages(&self) -> Vec<&str> {
        let program = self.dependency(PROGRAM).is_some();
        OPTIMIZED_PACKAGES
            .into_iter()
            .chain(program.then_some(PROGRAM))
            .chain((program || self.dependency(GADGETS).is_some()).then_some(GADGETS))
            .collect()
    }
}

/// The normal dependencies of a package from `cargo metadata --no-deps`, with
/// the name each is declared under.
fn dependencies(package: &Value) -> Vec<Dependency> {
    package
        .get("dependencies")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|dependency| {
            dependency
                .get("kind")
                .is_none_or(|kind| kind.is_null() || kind == "normal")
        })
        .filter_map(|dependency| {
            let package = dependency.get("name").and_then(Value::as_str)?;
            let name = dependency
                .get("rename")
                .and_then(Value::as_str)
                .unwrap_or(package);
            Some(Dependency {
                package: package.to_string(),
                name: name.to_string(),
            })
        })
        .collect()
}

fn compile_circuits(opts: &CompileOptions, circuits: &CircuitCrate) -> Result<()> {
    let r1cs_dir = output_dir(
        opts.r1cs_out
            .clone()
            .unwrap_or_else(|| circuits.build_dir.clone()),
    )?;
    let exported = if opts.skip_r1cs {
        Vec::new()
    } else {
        compile_r1cs(opts, circuits, &r1cs_dir)?
    };
    for r1cs in &exported {
        let module = circuits.verifier_dir.join(renamed(r1cs, "vk.rs")?);
        check_production_digest(&module, r1cs, load_r1cs(r1cs)?.digest())?;
    }
    let exported_glue = exported
        .iter()
        .map(|path| Ok(r1cs_dir.join(renamed(path, VERIFIER_SUFFIX)?)))
        .collect::<Result<Vec<_>>>()?;
    if !opts.skip_keys {
        let r1cs = if exported.is_empty() {
            inputs_or_all(Vec::new(), &r1cs_dir, "r1cs", "zolana zk compile")?
        } else {
            exported
        };
        write_keys(&r1cs, circuits, opts.test)?;
    }
    if opts.skip_r1cs && opts.skip_keys && !opts.check {
        return Ok(());
    }
    let glue = inputs_or_all(
        exported_glue,
        &r1cs_dir,
        VERIFIER_SUFFIX,
        "zolana zk compile",
    )?;
    let keys = glue
        .iter()
        .map(|path| {
            let name = circuit_name(path)?;
            Ok(KeyModule {
                source: circuits.build_dir.join(format!("{name}.vk.rs")),
                name,
                replace_production: false,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let modules = verifier_modules(&glue, &keys, circuits)?;
    if opts.check {
        check_verifier_modules(&modules)
    } else {
        write_verifier_modules(&modules)
    }
}

fn compile_r1cs(
    opts: &CompileOptions,
    circuits: &CircuitCrate,
    out: &Path,
) -> Result<Vec<PathBuf>> {
    let mut cargo = Command::new(cargo());
    cargo.args(["test", "--lib", "--config", "profile.dev.debug=0"]);
    for package in circuits.optimized_packages() {
        cargo
            .arg("--config")
            .arg(format!("profile.dev.package.{package}.opt-level=3"));
    }
    let output = cargo
        .arg("--manifest-path")
        .arg(&circuits.manifest)
        .args([
            "--features",
            &with_test_feature(&format!("{},r1cs-export", opts.r1cs_features), opts.test),
            "--",
            HOOK_PREFIX,
            "--nocapture",
            "--test-threads=1",
        ])
        .env(OUT_DIR_ENV, out)
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| BuildError::Spawn {
            command: format!("cargo test for {}", circuits.manifest.display()),
            hint: "",
            error,
        })?;
    let printed = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() {
        return Err(BuildError::CommandFailed {
            command: format!("exporting the circuits of {}", circuits.manifest.display()),
            status: output.status,
            output: format!("\n{printed}"),
        });
    }
    let written: Vec<PathBuf> = printed
        .lines()
        .filter_map(|line| line.split_once(MARKER).map(|(_, path)| path))
        .map(PathBuf::from)
        .collect();
    if written.is_empty() {
        return Err(BuildError::NoCircuits(circuits.manifest.clone()));
    }
    for path in &written {
        let r1cs = load_r1cs(path)?;
        println!(
            "{}: {} constraints, {} public inputs, {} private variables",
            path.display(),
            r1cs.constraints(),
            r1cs.public_inputs(),
            r1cs.private_variables()
        );
    }
    Ok(written)
}

fn write_keys(r1cs: &[PathBuf], circuits: &CircuitCrate, test: bool) -> Result<()> {
    let out = output_dir(circuits.build_dir.clone())?;
    for path in r1cs {
        let circuit = load_r1cs(path)?;
        let pk = out.join(renamed(path, "pk")?);
        let committed = circuits.verifier_dir.join(renamed(path, "vk.rs")?);
        check_production_digest(&committed, path, circuit.digest())?;
        let protected = is_production_key(&committed);
        let keys = ensure_key(&pk, path, &circuit, test, protected)?;
        if protected {
            check_imported_checksum(&committed, &keys)?;
        }
        export_module(&keys, &pk, &out, keys.setup_kind())?;
    }
    Ok(())
}

/// Loads validated local key material before deciding whether a new setup is
/// needed. Imported ceremony keys require an explicit import to replace them.
fn ensure_key(
    pk: &Path,
    path: &Path,
    circuit: &R1cs,
    test: bool,
    protected: bool,
) -> Result<Groth16Keys> {
    let expected = if test {
        SetupKind::InsecureTest
    } else {
        SetupKind::SingleParty
    };
    let existing = match std::fs::read(pk) {
        Ok(bytes) => {
            let manifest = pk.with_extension("pk.sha256");
            let keys = if manifest.is_file() {
                let checksums = key_checksums(&manifest)?;
                let canonical = pk
                    .canonicalize()
                    .map_err(BuildError::file("resolving", pk))?;
                let checksum =
                    checksums
                        .get(&canonical)
                        .ok_or_else(|| BuildError::MissingChecksum {
                            key: pk.to_path_buf(),
                            manifest,
                        })?;
                Groth16Keys::from_bytes_checked(&bytes, checksum)
            } else {
                Groth16Keys::from_bytes(&bytes)
            }
            .map_err(BuildError::prover("loading", pk))?;
            Some(keys)
        }
        Err(error) if error.kind() == ErrorKind::NotFound => None,
        Err(error) => return Err(BuildError::file("reading", pk)(error)),
    };
    if protected || existing.as_ref().is_some_and(Groth16Keys::is_imported_zkey) {
        let keys = existing.ok_or_else(|| BuildError::ImportedKeyUnavailable(pk.to_path_buf()))?;
        if !keys.is_imported_zkey() {
            return Err(BuildError::ImportedKeyUnavailable(pk.to_path_buf()));
        }
        if test {
            return Err(BuildError::ImportedKeyTestMode(pk.to_path_buf()));
        }
        if keys.circuit_digest() != &circuit.digest() {
            return Err(BuildError::ProductionKeyCircuitMismatch {
                module: pk.to_path_buf(),
                r1cs: path.to_path_buf(),
            });
        }
        save_key(&keys, pk)?;
        println!("{}: imported ceremony key reused", pk.display());
        return Ok(keys);
    }
    if let Some(keys) = existing {
        if keys.circuit_digest() == &circuit.digest() && keys.setup_kind() == expected {
            save_key(&keys, pk)?;
            println!("{}: {} key reused", pk.display(), expected.label());
            return Ok(keys);
        }
    }
    let keys = if test {
        Groth16Keys::new_with_test_setup(circuit)
    } else {
        Groth16Keys::new_with_random_setup(circuit)
    }
    .map_err(BuildError::prover("setting up", path))?;
    save_key(&keys, pk)?;
    println!("{}: {} key generated", pk.display(), expected.label());
    Ok(keys)
}

fn is_production_key(module: &Path) -> bool {
    std::fs::read_to_string(module).is_ok_and(|text| {
        text.contains(PRODUCTION_KEY)
            && !text.contains("ZOLANA_SETUP_KIND: &str = \"single-party\"")
    })
}

fn check_imported_checksum(module: &Path, keys: &Groth16Keys) -> Result<()> {
    let source = std::fs::read_to_string(module).map_err(BuildError::file("reading", module))?;
    let checksum = source
        .split_once("VERIFYINGKEY_PROVING_KEY_SHA256: [u8; 32] = [")
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(bytes, _)| {
            bytes
                .split(',')
                .filter(|v| !v.trim().is_empty())
                .map(|v| v.trim().trim_end_matches("u8").parse::<u8>())
                .collect::<core::result::Result<Vec<_>, _>>()
        });
    let bytes = keys
        .to_bytes()
        .map_err(BuildError::prover("serializing", module))?;
    let current: [u8; 32] = Sha256::digest(bytes).into();
    if !matches!(checksum, Some(Ok(checksum)) if checksum == current) {
        return Err(BuildError::ImportedKeyChecksumMismatch(
            module.to_path_buf(),
        ));
    }
    Ok(())
}

fn production_key_digest(module: &Path) -> Result<[u8; 32]> {
    let text = std::fs::read_to_string(module).map_err(BuildError::file("reading", module))?;
    let encoded = text
        .split_once(CIRCUIT_DIGEST)
        .and_then(|(_, rest)| rest.split_once('"'))
        .and_then(|(_, rest)| rest.split_once('"'))
        .map(|(digest, _)| digest)
        .ok_or_else(|| BuildError::ProductionKeyMissingCircuitDigest(module.to_path_buf()))?;
    let mut digest = [0u8; 32];
    hex::decode_to_slice(encoded, &mut digest)
        .map_err(|_| BuildError::ProductionKeyMissingCircuitDigest(module.to_path_buf()))?;
    Ok(digest)
}

fn check_production_digest(module: &Path, r1cs: &Path, current: [u8; 32]) -> Result<()> {
    if is_production_key(module) && production_key_digest(module)? != current {
        return Err(BuildError::ProductionKeyCircuitMismatch {
            module: module.to_path_buf(),
            r1cs: r1cs.to_path_buf(),
        });
    }
    Ok(())
}

fn export_module(keys: &Groth16Keys, pk: &Path, out: &Path, setup: SetupKind) -> Result<()> {
    let module = renamed(pk, "vk.rs")?;
    let staging = output_dir(out.join(STAGING))?;
    keys.export_verifying_key(&VerifyingKeyExport {
        output_dir: &staging,
        output_filename: &module,
        setup,
    })
    .map_err(BuildError::prover("exporting the verifying key of", pk))?;
    let target = out.join(&module);
    write_if_changed(&target, &rustfmt_staged(&staging.join(&module))?)?;
    println!("{}", target.display());
    Ok(())
}

fn rustfmt_staged(staged: &Path) -> Result<Vec<u8>> {
    let formatted = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .arg(staged)
        .output()
        .map_err(|error| BuildError::Spawn {
            command: format!("rustfmt on {}", staged.display()),
            hint: "; install it with `rustup component add rustfmt`",
            error,
        })?;
    if !formatted.status.success() {
        return Err(BuildError::CommandFailed {
            command: format!("rustfmt on {}", staged.display()),
            status: formatted.status,
            output: format!("\n{}", String::from_utf8_lossy(&formatted.stderr)),
        });
    }
    let generated = std::fs::read(staged).map_err(BuildError::file("reading", staged))?;
    std::fs::remove_file(staged).map_err(BuildError::file("removing", staged))?;
    Ok(generated)
}

fn write_if_changed(path: &Path, contents: &[u8]) -> Result<()> {
    if std::fs::read(path).is_ok_and(|current| current == contents) {
        return Ok(());
    }
    std::fs::write(path, contents).map_err(BuildError::file("writing", path))
}

struct KeyModule {
    name: String,
    source: PathBuf,
    replace_production: bool,
}

struct VerifierModules {
    dir: PathBuf,
    names: Vec<String>,
    files: Vec<(PathBuf, Vec<u8>)>,
}

fn verifier_modules(
    glue: &[PathBuf],
    keys: &[KeyModule],
    circuits: &CircuitCrate,
) -> Result<VerifierModules> {
    let names = glue
        .iter()
        .map(|path| Ok((circuit_name(path)?, path)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let dir = circuits.verifier_dir.clone();
    let staging = output_dir(circuits.build_dir.join(STAGING))?;
    let mut files = Vec::new();
    for (name, path) in &names {
        let staged = staging.join(format!("{name}.rs"));
        std::fs::copy(path, &staged).map_err(BuildError::file("copying", *path))?;
        files.push((dir.join(format!("{name}.rs")), rustfmt_staged(&staged)?));
    }
    for key in keys {
        if !names.contains_key(&key.name) {
            return Err(BuildError::KeyWithoutVerifier(key.name.clone()));
        }
        let module = dir.join(format!("{}.vk.rs", key.name));
        if !key.replace_production && is_production_key(&module) {
            println!("{}: production key kept", module.display());
            continue;
        }
        let contents =
            std::fs::read(&key.source).map_err(BuildError::file("reading", &key.source))?;
        files.push((module, contents));
    }
    let listing: String = names
        .keys()
        .map(|name| format!("pub mod {name};\n"))
        .collect();
    files.push((dir.join(VERIFIER_MOD), listing.into_bytes()));
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(VerifierModules {
        dir,
        names: names.into_keys().collect(),
        files,
    })
}

fn write_verifier_modules(modules: &VerifierModules) -> Result<()> {
    output_dir(modules.dir.clone())?;
    for (path, contents) in &modules.files {
        write_if_changed(path, contents)?;
        println!("{}", path.display());
    }
    for path in ungenerated_modules(modules)? {
        eprintln!(
            "warning: {} is not a generated verifier module",
            path.display()
        );
    }
    Ok(())
}

fn check_verifier_modules(modules: &VerifierModules) -> Result<()> {
    let mut stale = Vec::new();
    for (path, contents) in &modules.files {
        match std::fs::read(path) {
            Ok(current) if current == *contents => {}
            Ok(_) => stale.push(format!(
                "{}: differs from the generated module",
                path.display()
            )),
            Err(error) if error.kind() == ErrorKind::NotFound => {
                stale.push(format!("{}: missing", path.display()))
            }
            Err(error) => return Err(BuildError::file("reading", path)(error)),
        }
    }
    for path in ungenerated_modules(modules)? {
        stale.push(format!("{}: not generated", path.display()));
    }
    if stale.is_empty() {
        println!("{}: up to date", modules.dir.display());
        return Ok(());
    }
    for line in &stale {
        eprintln!("{line}");
    }
    Err(BuildError::StaleVerifierModules {
        count: stale.len(),
        dir: modules.dir.clone(),
    })
}

fn ungenerated_modules(modules: &VerifierModules) -> Result<Vec<PathBuf>> {
    if !modules.dir.is_dir() {
        return Ok(Vec::new());
    }
    let expected: BTreeSet<String> = modules
        .names
        .iter()
        .flat_map(|name| [format!("{name}.rs"), format!("{name}.vk.rs")])
        .chain([VERIFIER_MOD.to_string()])
        .collect();
    let mut found = Vec::new();
    for entry in
        std::fs::read_dir(&modules.dir).map_err(BuildError::file("reading", &modules.dir))?
    {
        let path = entry
            .map_err(BuildError::file("reading", &modules.dir))?
            .path();
        let generated = path
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|name| expected.contains(name));
        if path.extension() == Some(OsStr::new("rs")) && !generated {
            found.push(path);
        }
    }
    found.sort();
    Ok(found)
}

fn circuit_name(glue: &Path) -> Result<String> {
    glue.file_name()
        .and_then(OsStr::to_str)
        .and_then(|name| name.strip_suffix(VERIFIER_SUFFIX))
        .and_then(|name| name.strip_suffix('.'))
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .ok_or_else(|| BuildError::NotAVerifierModule(glue.to_path_buf()))
}

fn compile_wasm(opts: &CompileOptions, circuits: &CircuitCrate) -> Result<()> {
    let out = output_dir(
        opts.wasm_out
            .clone()
            .unwrap_or_else(|| circuits.build_dir.join("wasm")),
    )?;
    let out = std::path::absolute(&out).map_err(BuildError::file("resolving", &out))?;
    let threads = opts.threads();
    let library = &circuits.circuit_library()?.name;

    let mut features = vec![opts.wasm_features.clone()];
    if !opts.no_prover {
        features.push(format!("{library}/wasm-prover"));
    }
    if threads {
        features.push(format!("{library}/wasm-threads"));
    }
    if opts.verifier {
        features.push(format!("{library}/wasm-verify"));
    }
    let features = with_test_feature(&features.join(","), opts.test);
    let digest = hex::encode(Sha256::digest(features.as_bytes()));
    let variant = if threads { "threads" } else { "single" };
    let target_dir = circuits.target_dir.join(format!(
        "zk-wasm-{variant}-{}",
        digest.get(..12).unwrap_or(&digest)
    ));

    let mut rustflags = vec![format!(
        "-C target-feature={}",
        if threads {
            "+atomics,+bulk-memory,+simd128"
        } else {
            "+simd128"
        }
    )];
    if threads {
        rustflags.extend(
            THREADS_LINK_ARGS
                .iter()
                .map(|arg| format!("-C link-arg={arg}")),
        );
    }

    let target = match opts.wasm_target {
        WasmTarget::Web => "web",
        WasmTarget::Nodejs if threads => "web",
        WasmTarget::Nodejs => "nodejs",
    };
    let mut cargo_args = vec!["--features", features.as_str()];
    if threads {
        cargo_args.extend(["-Z", "build-std=panic_abort,std"]);
    }
    let configure = |command: &mut Command| {
        command
            .current_dir(&circuits.dir)
            .envs([
                ("CARGO_PROFILE_RELEASE_OPT_LEVEL", "3"),
                ("CARGO_PROFILE_RELEASE_LTO", "fat"),
                ("CARGO_PROFILE_RELEASE_CODEGEN_UNITS", "1"),
                ("CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS", "false"),
                ("CARGO_PROFILE_RELEASE_PANIC", "abort"),
                ("CARGO_INCREMENTAL", "0"),
            ])
            .env(
                "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS",
                rustflags.join(" "),
            )
            .env("CARGO_TARGET_DIR", &target_dir)
            .env_remove("RUSTFLAGS")
            .env_remove("CARGO_ENCODED_RUSTFLAGS");
        if threads {
            command
                .env("RUSTUP_TOOLCHAIN", &opts.nightly)
                .env_remove("CARGO");
        }
    };

    let mut build = Command::new("cargo");
    build
        .args([
            "build",
            "--lib",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .args(&cargo_args);
    configure(&mut build);
    let build_command = format!("the wasm build of {}", circuits.manifest.display());
    let status = build.status().map_err(|error| BuildError::Spawn {
        command: build_command.clone(),
        hint: "",
        error,
    })?;
    if !status.success() {
        return Err(BuildError::CommandFailed {
            command: build_command,
            status,
            output: String::new(),
        });
    }
    let artifact = target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(format!("{}.wasm", circuits.lib_name));
    let compiled = std::fs::read(&artifact).map_err(BuildError::file("reading", &artifact))?;
    let stamp = format!(
        "{}\n{target}\nrequested={:?}\nthreads={}\nwasm-opt={}\nnode-bootstrap={}\n",
        hex::encode(Sha256::digest(&compiled)),
        opts.wasm_target,
        threads,
        opts.wasm_opt,
        hex::encode(Sha256::digest(
            [
                include_bytes!("node.rs").as_slice(),
                include_bytes!("node_worker.mjs").as_slice()
            ]
            .concat()
        ))
    );
    let stamp_path = out.join(WASM_STAMP);
    let module = out.join(format!("{}_bg.wasm", circuits.lib_name));
    if wasm_output_complete(&out, &circuits.lib_name, opts.wasm_target, threads)
        && std::fs::read_to_string(&stamp_path).is_ok_and(|current| current == stamp)
    {
        println!("{} (unchanged)", module.display());
        return Ok(());
    }

    let mut wasm_pack = Command::new("wasm-pack");
    wasm_pack
        .arg("build")
        .arg(&circuits.dir)
        .args(["--release", "--target", target])
        .args((!opts.wasm_opt).then_some("--no-opt"))
        .arg("--out-dir")
        .arg(&out)
        .arg("--")
        .args(&cargo_args);
    configure(&mut wasm_pack);
    let status = wasm_pack.status().map_err(|error| BuildError::Spawn {
        command: "wasm-pack".into(),
        hint: "; install it with `cargo install wasm-pack`",
        error,
    })?;
    if !status.success() {
        return Err(BuildError::CommandFailed {
            command: format!("wasm-pack for {}", circuits.manifest.display()),
            status,
            output: String::new(),
        });
    }
    if threads {
        type_thread_pool(&out.join(format!("{}.d.ts", circuits.lib_name)))?;
    }
    if opts.wasm_target == WasmTarget::Nodejs {
        node::write_entry(&out, &circuits.lib_name, threads)?;
    }
    std::fs::write(&stamp_path, stamp).map_err(BuildError::file("writing", &stamp_path))?;
    println!("{}", module.display());
    Ok(())
}

fn type_thread_pool(declarations: &Path) -> Result<()> {
    let text =
        std::fs::read_to_string(declarations).map_err(BuildError::file("reading", declarations))?;
    if !text.contains(UNTYPED_THREAD_POOL) {
        return Err(BuildError::ThreadPoolDeclaration(
            declarations.to_path_buf(),
        ));
    }
    std::fs::write(
        declarations,
        text.replace(UNTYPED_THREAD_POOL, TYPED_THREAD_POOL),
    )
    .map_err(BuildError::file("writing", declarations))
}

/// Generates or reuses keys for the crate's r1cs files. Test setups use a
/// fixed public seed; the default uses fresh single-party randomness.
pub fn setup(opts: &SetupOptions) -> Result<()> {
    let circuits = CircuitCrate::resolve(&opts.package)?;
    let inputs = inputs_or_all(
        opts.r1cs.clone(),
        &circuits.build_dir,
        "r1cs",
        "zolana zk compile",
    )?;
    let out = output_dir(
        opts.out
            .clone()
            .unwrap_or_else(|| circuits.build_dir.clone()),
    )?;
    for path in inputs {
        let circuit = load_r1cs(&path)?;
        let pk = out.join(renamed(&path, "pk")?);
        let committed = circuits.verifier_dir.join(renamed(&path, "vk.rs")?);
        check_production_digest(&committed, &path, circuit.digest())?;
        let protected = is_production_key(&committed);
        let keys = ensure_key(&pk, &path, &circuit, opts.test, protected)?;
        if protected {
            check_imported_checksum(&committed, &keys)?;
        }
    }
    Ok(())
}

/// Converts a ceremony zkey into a proving key, checked against the r1cs of
/// its circuit.
pub fn import(opts: &ImportOptions) -> Result<()> {
    let build = CircuitCrate::resolve(&opts.package)?.build_dir;
    let r1cs_path = match &opts.r1cs {
        Some(path) => path.clone(),
        None => build.join(renamed(&opts.zkey, "r1cs")?),
    };
    let zkey = std::fs::read(&opts.zkey).map_err(BuildError::file("reading", &opts.zkey))?;
    let keys = Groth16Keys::from_zkey_for_r1cs(&zkey, &load_r1cs(&r1cs_path)?)
        .map_err(BuildError::prover("importing", &opts.zkey))?;
    let pk = output_dir(opts.out.clone().unwrap_or(build))?.join(renamed(&r1cs_path, "pk")?);
    save_key(&keys, &pk)?;
    println!("{}", pk.display());
    Ok(())
}

// Record the checksum when trusted key material is serialized.
fn save_key(keys: &Groth16Keys, path: &Path) -> Result<()> {
    let bytes = keys
        .to_bytes()
        .map_err(BuildError::prover("serializing", path))?;
    write_if_changed(path, &bytes)?;
    let name = file_name(path)?;
    let manifest = path.with_extension("pk.sha256");
    write_if_changed(
        &manifest,
        format!("{}  {name}\n", hex::encode(Sha256::digest(&bytes))).as_bytes(),
    )
}

/// Reads a sha256sum manifest: one `<sha256>  <file>` line per key, file names
/// relative to the manifest.
pub fn key_checksums(manifest: &Path) -> Result<BTreeMap<PathBuf, [u8; 32]>> {
    let text = std::fs::read_to_string(manifest).map_err(BuildError::file("reading", manifest))?;
    let directory = manifest.parent().ok_or(BuildError::ChecksumManifest {
        manifest: manifest.to_path_buf(),
        line: 0,
        reason: "the manifest has no directory",
    })?;
    let invalid = |line: usize, reason| BuildError::ChecksumManifest {
        manifest: manifest.to_path_buf(),
        line,
        reason,
    };
    let mut checksums = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (digest, name) = line
            .split_once(char::is_whitespace)
            .ok_or_else(|| invalid(line_number, "expected `<sha256>  <file>`"))?;
        let mut checksum = [0; 32];
        hex::decode_to_slice(digest, &mut checksum)
            .map_err(|_| invalid(line_number, "invalid SHA-256"))?;
        let name = name
            .trim_start()
            .strip_prefix('*')
            .unwrap_or(name.trim_start());
        if name.is_empty() {
            return Err(invalid(line_number, "missing key filename"));
        }
        let path = directory.join(name);
        let path = path
            .canonicalize()
            .map_err(BuildError::file("resolving", &path))?;
        if checksums.insert(path.clone(), checksum).is_some() {
            return Err(BuildError::DuplicateChecksum(path));
        }
    }
    if checksums.is_empty() {
        return Err(BuildError::NoChecksums(manifest.to_path_buf()));
    }
    Ok(checksums)
}

/// Exports the verifying key module of each proving key, checked against a
/// trusted checksum manifest, into the committed verifier modules.
pub fn export_verifier(opts: &ExportVerifierOptions) -> Result<()> {
    let checksums = key_checksums(&opts.checksums)?;
    let circuits = CircuitCrate::resolve(&opts.package)?;
    let inputs = inputs_or_all(
        opts.pk.clone(),
        &circuits.build_dir,
        "pk",
        "zolana zk setup or zolana zk import",
    )?;
    let out = output_dir(
        opts.out
            .clone()
            .unwrap_or_else(|| circuits.build_dir.clone()),
    )?;
    let mut keys = Vec::new();
    for pk in inputs {
        let canonical = pk
            .canonicalize()
            .map_err(BuildError::file("resolving", &pk))?;
        let checksum = checksums
            .get(&canonical)
            .ok_or_else(|| BuildError::MissingChecksum {
                key: pk.clone(),
                manifest: opts.checksums.clone(),
            })?;
        let loaded = Groth16Keys::load_checked(&pk, checksum)
            .map_err(BuildError::prover("checking", &pk))?;
        loaded
            .check_setup_kind(opts.setup)
            .map_err(BuildError::prover("checking setup provenance of", &pk))?;
        if matches!(opts.setup, SetupKind::Production | SetupKind::SingleParty) {
            let r1cs = circuits.build_dir.join(renamed(&pk, "r1cs")?);
            if loaded.circuit_digest() != &load_r1cs(&r1cs)?.digest() {
                return Err(BuildError::ProductionKeyCircuitMismatch { module: pk, r1cs });
            }
        }
        export_module(&loaded, &pk, &out, opts.setup)?;
        keys.push(KeyModule {
            name: file_stem(&pk)?.to_string(),
            source: out.join(renamed(&pk, "vk.rs")?),
            replace_production: true,
        });
    }
    let glue = inputs_or_all(
        Vec::new(),
        &circuits.build_dir,
        VERIFIER_SUFFIX,
        "zolana zk compile",
    )?;
    write_verifier_modules(&verifier_modules(&glue, &keys, &circuits)?)
}

fn nearest_manifest() -> Result<PathBuf> {
    let cwd =
        std::env::current_dir().map_err(BuildError::file("reading", "the current directory"))?;
    cwd.ancestors()
        .map(|dir| dir.join("Cargo.toml"))
        .find(|manifest| manifest.is_file())
        .ok_or(BuildError::NoManifest(cwd))
}

fn cargo() -> OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into())
}

fn output_dir(dir: PathBuf) -> Result<PathBuf> {
    std::fs::create_dir_all(&dir).map_err(BuildError::file("creating", &dir))?;
    Ok(dir)
}

fn load_r1cs(path: &Path) -> Result<R1cs> {
    R1cs::load(path).map_err(BuildError::prover("reading", path))
}

fn file_stem(path: &Path) -> Result<&str> {
    path.file_stem()
        .and_then(OsStr::to_str)
        .ok_or_else(|| BuildError::FileName(path.to_path_buf()))
}

fn file_name(path: &Path) -> Result<&str> {
    path.file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| BuildError::FileName(path.to_path_buf()))
}

fn renamed(path: &Path, extension: &str) -> Result<String> {
    Ok(format!("{}.{extension}", file_stem(path)?))
}

fn inputs_or_all(
    inputs: Vec<PathBuf>,
    dir: &Path,
    extension: &'static str,
    producer: &'static str,
) -> Result<Vec<PathBuf>> {
    if !inputs.is_empty() {
        return Ok(inputs);
    }
    let suffix = format!(".{extension}");
    let mut found = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).map_err(BuildError::file("reading", dir))? {
            let path = entry.map_err(BuildError::file("reading", dir))?.path();
            if path
                .file_name()
                .and_then(OsStr::to_str)
                .is_some_and(|name| name.ends_with(&suffix))
            {
                found.push(path);
            }
        }
    }
    if found.is_empty() {
        return Err(BuildError::NoInputs {
            extension,
            dir: dir.to_path_buf(),
            producer,
        });
    }
    found.sort();
    Ok(found)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    use crate::{
        circuit::{Assert, Circuit, CircuitType, U64},
        conversion::{Allocator, Placeholder, ProofInput},
        CircuitError, ZkCircuit,
    };

    struct KeyInputs<const DIFFERENT: bool>;
    struct KeyCircuit<const DIFFERENT: bool> {
        x: U64,
        y: U64,
    }
    impl<const DIFFERENT: bool> CircuitType for KeyCircuit<DIFFERENT> {}
    impl<const DIFFERENT: bool> ProofInput for KeyInputs<DIFFERENT> {
        type Circuit = KeyCircuit<DIFFERENT>;
        fn instantiate(
            &self,
            allocator: &Allocator,
        ) -> core::result::Result<Self::Circuit, CircuitError> {
            Ok(KeyCircuit {
                x: 0u64.instantiate(allocator)?,
                y: 0u64.instantiate(allocator)?,
            })
        }
    }
    impl<const DIFFERENT: bool> Placeholder for KeyInputs<DIFFERENT> {
        fn placeholder() -> core::result::Result<Self, CircuitError> {
            Ok(Self)
        }
    }
    impl<const DIFFERENT: bool> Circuit for KeyCircuit<DIFFERENT> {
        type Output = ();
        fn circuit(&self) -> core::result::Result<(), CircuitError> {
            if DIFFERENT {
                self.x.assert_not_equal(&self.y, "different")
            } else {
                self.x.assert_equal(&self.y, "equal")
            }
        }
    }
    type EqualInputs = KeyInputs<false>;
    type ChangedInputs = KeyInputs<true>;

    struct KeyFixture {
        dir: PathBuf,
        circuits: CircuitCrate,
        path: PathBuf,
    }

    impl KeyFixture {
        fn new(label: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "zolana-build-keys-{label}-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            let mut circuits = crate_with(json!([]));
            circuits.build_dir = dir.clone();
            circuits.verifier_dir = dir.join("verifier");
            let path = dir.join("sample.r1cs");
            std::fs::write(&path, EqualInputs::export_r1cs().unwrap()).unwrap();
            Self {
                dir,
                circuits,
                path,
            }
        }
        fn pk(&self) -> PathBuf {
            self.dir.join("sample.pk")
        }
        fn keys(&self, test: bool) -> Groth16Keys {
            ensure_key(
                &self.pk(),
                &self.path,
                &load_r1cs(&self.path).unwrap(),
                test,
                false,
            )
            .unwrap()
        }
        fn imported(&self) -> Groth16Keys {
            // Test fixture: emulate serialized imported-key provenance.
            let mut bytes = self.keys(false).to_bytes().unwrap();
            bytes[0] = 1;
            let keys = Groth16Keys::from_bytes(&bytes).unwrap();
            save_key(&keys, &self.pk()).unwrap();
            keys
        }
    }
    impl Drop for KeyFixture {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.dir).unwrap();
        }
    }

    #[test]
    fn random_setup_is_fresh_and_test_setup_is_reproducible() {
        let random_a = KeyFixture::new("random-a").keys(false);
        let random_b = KeyFixture::new("random-b").keys(false);
        let test_a = KeyFixture::new("test-a").keys(true);
        let test_b = KeyFixture::new("test-b").keys(true);
        assert_eq!(
            (
                random_a.setup_kind(),
                test_a.setup_kind(),
                random_a.to_bytes().unwrap() == random_b.to_bytes().unwrap(),
                test_a.to_bytes().unwrap() == test_b.to_bytes().unwrap()
            ),
            (SetupKind::SingleParty, SetupKind::InsecureTest, false, true)
        );
    }

    #[test]
    fn unchanged_keys_and_checksum_keep_their_bytes_and_mtime() {
        for test in [false, true] {
            let fixture = KeyFixture::new("reuse");
            let before = fixture.keys(test).to_bytes().unwrap();
            let checksum = fixture.pk().with_extension("pk.sha256");
            let old = std::time::UNIX_EPOCH + std::time::Duration::from_secs(100);
            for path in [&fixture.pk(), &checksum] {
                std::fs::File::options()
                    .write(true)
                    .open(path)
                    .unwrap()
                    .set_modified(old)
                    .unwrap();
            }
            let after = fixture.keys(test).to_bytes().unwrap();
            assert_eq!(
                (
                    before == after,
                    std::fs::metadata(fixture.pk()).unwrap().modified().unwrap(),
                    std::fs::metadata(checksum).unwrap().modified().unwrap()
                ),
                (true, old, old)
            );
        }
    }

    #[test]
    fn changed_r1cs_and_setup_mode_regenerate_local_keys() {
        let fixture = KeyFixture::new("changes");
        let random = fixture.keys(false).to_bytes().unwrap();
        let test = fixture.keys(true).to_bytes().unwrap();
        let random_again = fixture.keys(false).to_bytes().unwrap();
        std::fs::write(&fixture.path, ChangedInputs::export_r1cs().unwrap()).unwrap();
        let changed = fixture.keys(false);
        assert_eq!(
            (
                random != test,
                test != random_again,
                random_again != changed.to_bytes().unwrap(),
                changed.circuit_digest() == &load_r1cs(&fixture.path).unwrap().digest()
            ),
            (true, true, true, true)
        );
    }

    #[test]
    fn missing_verifier_and_checksum_are_restored_without_new_setup() {
        let fixture = KeyFixture::new("restore");
        write_keys(&[fixture.path.clone()], &fixture.circuits, false).unwrap();
        let before = std::fs::read(fixture.pk()).unwrap();
        let vk = fixture.dir.join("sample.vk.rs");
        let expected_vk = std::fs::read(&vk).unwrap();
        std::fs::remove_file(&vk).unwrap();
        std::fs::remove_file(fixture.pk().with_extension("pk.sha256")).unwrap();
        write_keys(&[fixture.path.clone()], &fixture.circuits, false).unwrap();
        assert_eq!(
            (
                std::fs::read(fixture.pk()).unwrap(),
                std::fs::read(vk).unwrap()
            ),
            (before, expected_vk)
        );
        assert!(!is_production_key(&fixture.dir.join("sample.vk.rs")));
    }

    #[test]
    fn imported_keys_are_reused_but_never_replaced_by_test_or_changed_circuits() {
        let fixture = KeyFixture::new("imported");
        let keys = fixture.imported();
        let before = std::fs::read(fixture.pk()).unwrap();
        export_module(
            &keys,
            &fixture.pk(),
            &fixture.circuits.verifier_dir,
            SetupKind::Production,
        )
        .unwrap();
        let vk = fixture.circuits.verifier_dir.join("sample.vk.rs");
        let original_module = std::fs::read(&vk).unwrap();
        write_keys(&[fixture.path.clone()], &fixture.circuits, false).unwrap();
        let test = write_keys(&[fixture.path.clone()], &fixture.circuits, true);
        std::fs::write(&fixture.path, ChangedInputs::export_r1cs().unwrap()).unwrap();
        let changed = write_keys(&[fixture.path.clone()], &fixture.circuits, false);
        assert_eq!(
            (
                matches!(test, Err(BuildError::ImportedKeyTestMode(_))),
                matches!(
                    changed,
                    Err(BuildError::ProductionKeyCircuitMismatch { .. })
                ),
                std::fs::read(fixture.pk()).unwrap(),
                std::fs::read(vk).unwrap()
            ),
            (true, true, before, original_module)
        );
    }

    #[test]
    fn single_party_committed_modules_allow_missing_keys_and_changed_circuits() {
        let fixture = KeyFixture::new("random-committed");
        write_keys(&[fixture.path.clone()], &fixture.circuits, false).unwrap();
        std::fs::create_dir_all(&fixture.circuits.verifier_dir).unwrap();
        let committed = fixture.circuits.verifier_dir.join("sample.vk.rs");
        std::fs::copy(fixture.dir.join("sample.vk.rs"), &committed).unwrap();
        let before = std::fs::read(fixture.pk()).unwrap();
        std::fs::remove_file(fixture.pk()).unwrap();
        write_keys(&[fixture.path.clone()], &fixture.circuits, false).unwrap();
        let restored = std::fs::read(fixture.pk()).unwrap();
        std::fs::write(&fixture.path, ChangedInputs::export_r1cs().unwrap()).unwrap();
        write_keys(&[fixture.path.clone()], &fixture.circuits, false).unwrap();
        let changed = Groth16Keys::from_bytes(&std::fs::read(fixture.pk()).unwrap()).unwrap();
        assert_eq!(
            (
                before != restored,
                restored != changed.to_bytes().unwrap(),
                changed.circuit_digest() == &load_r1cs(&fixture.path).unwrap().digest()
            ),
            (true, true, true)
        );
    }

    #[test]
    fn a_different_imported_key_cannot_replace_the_committed_key_implicitly() {
        let fixture = KeyFixture::new("wrong-import");
        let keys = fixture.imported();
        export_module(
            &keys,
            &fixture.pk(),
            &fixture.circuits.verifier_dir,
            SetupKind::Production,
        )
        .unwrap();
        std::fs::remove_file(fixture.pk()).unwrap();
        fixture.imported();
        assert!(matches!(
            write_keys(&[fixture.path.clone()], &fixture.circuits, false),
            Err(BuildError::ImportedKeyChecksumMismatch(_))
        ));
    }

    #[test]
    fn imported_keys_stay_protected_without_a_committed_module() {
        let fixture = KeyFixture::new("unexported-import");
        fixture.imported();
        let circuit = load_r1cs(&fixture.path).unwrap();
        assert!(matches!(
            ensure_key(&fixture.pk(), &fixture.path, &circuit, true, false),
            Err(BuildError::ImportedKeyTestMode(_))
        ));
        std::fs::write(&fixture.path, ChangedInputs::export_r1cs().unwrap()).unwrap();
        assert!(matches!(
            ensure_key(
                &fixture.pk(),
                &fixture.path,
                &load_r1cs(&fixture.path).unwrap(),
                false,
                false
            ),
            Err(BuildError::ProductionKeyCircuitMismatch { .. })
        ));
    }

    #[test]
    fn missing_imported_keys_and_wrong_checksums_fail_without_replacement() {
        let fixture = KeyFixture::new("missing-import");
        let circuit = load_r1cs(&fixture.path).unwrap();
        assert!(matches!(
            ensure_key(&fixture.pk(), &fixture.path, &circuit, false, true),
            Err(BuildError::ImportedKeyUnavailable(_))
        ));
        fixture.keys(false);
        let before = std::fs::read(fixture.pk()).unwrap();
        std::fs::write(
            fixture.pk().with_extension("pk.sha256"),
            format!("{}  sample.pk\n", "00".repeat(32)),
        )
        .unwrap();
        assert!(matches!(
            ensure_key(&fixture.pk(), &fixture.path, &circuit, false, false),
            Err(BuildError::Prover { .. })
        ));
        assert_eq!(std::fs::read(fixture.pk()).unwrap(), before);
    }

    #[test]
    fn exported_single_party_keys_have_no_test_guard_and_cannot_be_mislabeled() {
        let fixture = KeyFixture::new("labels");
        let random = fixture.keys(false);
        export_module(&random, &fixture.pk(), &fixture.dir, SetupKind::SingleParty).unwrap();
        let module = std::fs::read_to_string(fixture.dir.join("sample.vk.rs")).unwrap();
        assert_eq!(
            (
                module.contains("ZOLANA_SETUP_KIND: &str = \"single-party\""),
                module.contains(PRODUCTION_KEY),
                module.contains("compile_error!")
            ),
            (true, true, false)
        );
        for kind in [SetupKind::Production, SetupKind::InsecureTest] {
            assert!(random
                .export_verifying_key(&VerifyingKeyExport {
                    output_dir: &fixture.dir,
                    output_filename: "wrong.rs",
                    setup: kind,
                })
                .is_err());
        }
    }

    fn crate_with(declared: Value) -> CircuitCrate {
        CircuitCrate {
            manifest: PathBuf::from("Cargo.toml"),
            dir: PathBuf::new(),
            lib_name: String::new(),
            target_dir: PathBuf::new(),
            build_dir: PathBuf::new(),
            verifier_dir: PathBuf::new(),
            dependencies: dependencies(&json!({ "dependencies": declared })),
        }
    }

    #[test]
    fn a_zk_program_builds_the_wasm_runtime_through_zolana_program() {
        let program = crate_with(json!([
            { "name": "zolana-program", "rename": null, "kind": null },
            { "name": "serde", "rename": null, "kind": null },
        ]));
        assert_eq!(
            (
                program
                    .circuit_library()
                    .map(|library| library.name.as_str())
                    .ok(),
                program.optimized_packages().contains(&PROGRAM),
                program.optimized_packages().contains(&GADGETS),
            ),
            (Some("zolana-program"), true, true)
        );
    }

    #[test]
    fn a_standalone_circuit_builds_it_through_the_renamed_dsl() {
        let standalone = crate_with(json!([
            { "name": "zolana-zk-dsl", "rename": "zk", "kind": null },
            { "name": "zolana-program", "rename": null, "kind": "dev" },
        ]));
        assert_eq!(
            (
                standalone
                    .circuit_library()
                    .map(|library| library.name.as_str())
                    .ok(),
                standalone.optimized_packages().contains(&PROGRAM),
                standalone.optimized_packages().contains(&GADGETS),
            ),
            (Some("zk"), false, false)
        );
    }

    #[test]
    fn a_crate_without_a_circuit_library_has_no_wasm_runtime() {
        let unrelated = crate_with(json!([{ "name": "serde", "rename": null, "kind": null }]));
        assert!(matches!(
            unrelated.circuit_library(),
            Err(BuildError::NoCircuitLibrary(_))
        ));
    }

    #[test]
    fn only_explicit_test_builds_enable_the_test_key_compile_guard() {
        assert_eq!(
            [
                with_test_feature("", false),
                with_test_feature("", true),
                with_test_feature("circuits,r1cs-export", true),
                with_test_feature("bpf-entrypoint", true),
                with_test_feature("wasm,insecure-test-setup", true),
            ],
            [
                "",
                "insecure-test-setup",
                "circuits,r1cs-export,insecure-test-setup",
                "bpf-entrypoint,insecure-test-setup",
                "wasm,insecure-test-setup"
            ]
        );
    }

    #[test]
    fn the_default_options_match_the_cli_defaults() {
        let options = CompileOptions::default();
        assert_eq!(
            (
                options.r1cs_features.as_str(),
                options.wasm_features.as_str(),
                options.wasm_target,
                options.nightly.as_str(),
                options.threads(),
            ),
            (
                "circuits",
                "wasm",
                WasmTarget::Web,
                "nightly-2026-05-18",
                true
            )
        );
    }

    #[test]
    fn node_provers_use_threads_unless_explicitly_disabled() {
        let mut options = CompileOptions {
            wasm_target: WasmTarget::Nodejs,
            ..CompileOptions::default()
        };
        assert!(options.threads());
        options.single_threaded = true;
        assert!(!options.threads());
        options.single_threaded = false;
        options.no_prover = true;
        assert!(!options.threads());
    }

    #[test]
    fn wasm_cache_requires_every_generated_entry() {
        let dir = std::env::temp_dir().join(format!(
            "zolana-wasm-cache-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        for name in [
            "sample_bg.wasm",
            "sample_bg.wasm.d.ts",
            "sample.js",
            "sample.d.ts",
            "package.json",
        ] {
            std::fs::write(dir.join(name), []).unwrap();
        }
        assert!(wasm_output_complete(&dir, "sample", WasmTarget::Web, false));
        assert!(!wasm_output_complete(
            &dir,
            "sample",
            WasmTarget::Nodejs,
            false
        ));
        std::fs::write(dir.join("node.mjs"), []).unwrap();
        std::fs::write(dir.join("node.d.mts"), []).unwrap();
        assert!(wasm_output_complete(
            &dir,
            "sample",
            WasmTarget::Nodejs,
            false
        ));
        assert!(!wasm_output_complete(
            &dir,
            "sample",
            WasmTarget::Nodejs,
            true
        ));
        let helper = dir.join("snippets/wasm-bindgen-rayon-test/src/workerHelpers.no-bundler.js");
        std::fs::create_dir_all(helper.parent().unwrap()).unwrap();
        std::fs::write(&helper, include_bytes!("node_worker.mjs")).unwrap();
        std::fs::write(
            dir.join("sample.js"),
            "import './snippets/wasm-bindgen-rayon-test/src/workerHelpers.no-bundler.js';",
        )
        .unwrap();
        assert!(wasm_output_complete(
            &dir,
            "sample",
            WasmTarget::Nodejs,
            true
        ));
        std::fs::remove_file(dir.join("sample.js")).unwrap();
        assert!(!wasm_output_complete(
            &dir,
            "sample",
            WasmTarget::Nodejs,
            false
        ));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn production_digest_reads_rustfmt_wrapped_key_modules() {
        let path = std::env::temp_dir().join(format!(
            "zolana-key-digest-{}-{:?}.rs",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::write(
            &path,
            format!("{CIRCUIT_DIGEST}\n    \"{}\";\n", "ab".repeat(32)),
        )
        .unwrap();
        assert_eq!(production_key_digest(&path).unwrap(), [0xab; 32]);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn production_key_with_a_stale_circuit_digest_is_rejected() {
        let path = std::env::temp_dir().join(format!(
            "zolana-stale-key-{}-{:?}.rs",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::write(
            &path,
            format!(
                "{PRODUCTION_KEY};\n{CIRCUIT_DIGEST}\n    \"{}\";\n",
                "ab".repeat(32)
            ),
        )
        .unwrap();
        assert!(check_production_digest(&path, Path::new("changed.r1cs"), [0xab; 32]).is_ok());
        assert!(matches!(
            check_production_digest(&path, Path::new("changed.r1cs"), [0xcd; 32]),
            Err(BuildError::ProductionKeyCircuitMismatch { .. })
        ));
        std::fs::remove_file(path).unwrap();
    }

    #[cfg(feature = "external-tools")]
    #[test]
    fn node_initialization_can_retry_after_a_worker_error() {
        let dir = std::env::temp_dir().join(format!(
            "zolana-node-retry-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let helper = dir.join("snippets/wasm-bindgen-rayon-test/src/workerHelpers.no-bundler.js");
        std::fs::create_dir_all(helper.parent().unwrap()).unwrap();
        std::fs::write(&helper, []).unwrap();
        std::fs::write(dir.join("package.json"), r#"{"files":[]}"#).unwrap();
        std::fs::write(dir.join("sample_bg.wasm"), [0u8]).unwrap();
        std::fs::write(
            dir.join("sample.js"),
            "import './snippets/wasm-bindgen-rayon-test/src/workerHelpers.no-bundler.js';\nlet attempts = 0;\nexport function initSync() {}\nexport async function initProverThreads() { if (++attempts === 1) throw new Error('temporary'); }\nexport function getAttempts() { return attempts; }\n",
        )
        .unwrap();
        node::write_entry(&dir, "sample", true).unwrap();
        let script = "const wasm = await import(process.argv[1]);\ntry { await wasm.default({threads: 1}); throw new Error('first attempt succeeded'); } catch (error) { if (error.message !== 'temporary') throw error; }\nawait wasm.default({threads: 1});\nif (wasm.getAttempts() !== 2 || wasm.proverThreads !== 1) throw new Error('retry did not initialize');\n";
        let output = Command::new("node")
            .args(["--input-type=module", "-e", script])
            .arg(dir.join("node.mjs"))
            .output()
            .unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
