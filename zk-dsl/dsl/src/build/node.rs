use std::path::Path;

use super::{BuildError, Result};

pub(super) fn write_entry(out: &Path, library: &str, threads: bool) -> Result<()> {
    if threads {
        let helper = thread_helper(out, library)?;
        std::fs::write(&helper, include_str!("node_worker.mjs"))
            .map_err(BuildError::file("writing", &helper))?;
    }

    let package_path = out.join("package.json");
    let package =
        std::fs::read(&package_path).map_err(BuildError::file("reading", &package_path))?;
    let mut package: serde_json::Value = serde_json::from_slice(&package)
        .map_err(|_| BuildError::NodeThreadBootstrap(package_path.clone()))?;
    let object = package
        .as_object_mut()
        .ok_or_else(|| BuildError::NodeThreadBootstrap(package_path.clone()))?;
    object.insert(
        "type".into(),
        if threads { "module" } else { "commonjs" }.into(),
    );
    object.insert("main".into(), "node.mjs".into());
    object.insert("module".into(), "node.mjs".into());
    object.insert("types".into(), "node.d.mts".into());
    let files = object
        .get_mut("files")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| BuildError::NodeThreadBootstrap(package_path.clone()))?;
    for file in ["node.mjs", "node.d.mts"] {
        if !files.iter().any(|entry| entry.as_str() == Some(file)) {
            files.push(file.into());
        }
    }
    std::fs::write(
        &package_path,
        serde_json::to_vec_pretty(&package)
            .map_err(|_| BuildError::NodeThreadBootstrap(package_path.clone()))?,
    )
    .map_err(BuildError::file("writing", &package_path))?;

    let entry = if threads {
        format!(
            r#"import {{ readFileSync }} from 'node:fs';
import {{ availableParallelism }} from 'node:os';
import * as wasm from './{library}.js';

let initialization;
let wasmInitialized = false;
export let proverThreads = 0;

export default async function init(options = {{}}) {{
  const available = availableParallelism();
  const threads = options.threads ?? (proverThreads || available);
  if (!Number.isSafeInteger(threads) || threads < 1 || threads > available) {{
    throw new Error(`threads must be between 1 and ${{available}}`);
  }}
  if (initialization) {{
    if (threads !== proverThreads) throw new Error('The prover thread count is already initialized');
    await initialization;
    return wasm;
  }}
  proverThreads = threads;
  initialization = (async () => {{
    if (!wasmInitialized) {{
      wasm.initSync({{ module: readFileSync(new URL('./{library}_bg.wasm', import.meta.url)) }});
      wasmInitialized = true;
    }}
    await wasm.initProverThreads(threads);
  }})().catch((error) => {{
    initialization = undefined;
    proverThreads = 0;
    throw error;
  }});
  await initialization;
  return wasm;
}}
export * from './{library}.js';
"#
        )
    } else {
        format!(
            r#"import * as wasm from './{library}.js';
export const proverThreads = 1;
export default async function init(options = {{}}) {{
  if (options.threads !== undefined && options.threads !== 1) {{
    throw new Error('This WASM package was built with --single-threaded');
  }}
  return wasm;
}}
export * from './{library}.js';
"#
        )
    };
    let entry_path = out.join("node.mjs");
    std::fs::write(&entry_path, entry).map_err(BuildError::file("writing", &entry_path))?;
    let declarations = out.join("node.d.mts");
    std::fs::write(&declarations, format!(
        "export interface NodeWasmOptions {{ threads?: number; }}\nexport declare const proverThreads: number;\nexport default function init(options?: NodeWasmOptions): Promise<typeof import('./{library}.js')>;\nexport * from './{library}.js';\n"
    )).map_err(BuildError::file("writing", &declarations))?;
    Ok(())
}

fn thread_helper(out: &Path, library: &str) -> Result<std::path::PathBuf> {
    let script_path = out.join(format!("{library}.js"));
    let script =
        std::fs::read_to_string(&script_path).map_err(BuildError::file("reading", &script_path))?;
    let relative = script
        .split(['\'', '"'])
        .find(|part| {
            part.starts_with("./snippets/wasm-bindgen-rayon-")
                && part.ends_with("/src/workerHelpers.no-bundler.js")
        })
        .ok_or_else(|| BuildError::NodeThreadBootstrap(script_path.clone()))?;
    let helper = out.join(relative.trim_start_matches("./"));
    if !helper.is_file() {
        return Err(BuildError::NodeThreadBootstrap(helper));
    }
    Ok(helper)
}

pub(super) fn thread_helper_complete(out: &Path, library: &str, node: bool) -> bool {
    thread_helper(out, library).is_ok_and(|helper| {
        !node
            || std::fs::read(&helper)
                .is_ok_and(|current| current == include_bytes!("node_worker.mjs"))
    })
}
