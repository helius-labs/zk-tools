import { spawnSync } from "node:child_process";
import { copyFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const root = fileURLToPath(new URL("../", import.meta.url));
const crate = resolve(root, "../../crates/zopro-spr");
const toolchain = "nightly-2025-11-15";

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: crate,
    stdio: "inherit",
    ...options,
  });
  if (result.error) throw result.error;
  if (result.status !== 0)
    throw new Error(`${command} exited with ${result.status}`);
}

const bindgen = spawnSync("wasm-bindgen", ["--version"], { encoding: "utf8" });
if (bindgen.stdout?.trim() !== "wasm-bindgen 0.2.128") {
  throw new Error("Install wasm-bindgen-cli 0.2.128 before building");
}

for (const variant of ["single", "threads"]) {
  const target = resolve(root, `../../target/spr-wasm-${variant}`);
  const out = resolve(root, `runtime/${variant}`);
  const env = {
    ...process.env,
    CARGO_TARGET_DIR: target,
    CARGO_PROFILE_RELEASE_LTO: "false",
  };
  delete env.CARGO_ENCODED_RUSTFLAGS;
  // Override the crate's shared-memory flags only for the single-thread build.
  if (variant === "single") {
    env.RUSTFLAGS =
      "-C target-feature=+bulk-memory,+mutable-globals,+simd128 -C link-arg=--max-memory=4294967296";
  } else {
    delete env.RUSTFLAGS;
  }
  run(
    "rustup",
    [
      "run",
      toolchain,
      "cargo",
      "rustc",
      "--release",
      "--locked",
      "--target",
      "wasm32-unknown-unknown",
      "--crate-type",
      "cdylib",
    ],
    { env },
  );
  await rm(out, { recursive: true, force: true });
  await mkdir(out, { recursive: true });
  run("wasm-bindgen", [
    "--target",
    "web",
    "--out-name",
    "zopro_spr",
    "--out-dir",
    out,
    resolve(target, "wasm32-unknown-unknown/release/zopro_spr.wasm"),
  ]);

  if (variant === "threads") {
    const path = resolve(out, "zopro_spr.js");
    const generated = await readFile(path, "utf8");
    const helper =
      /^import \{ startWorkers \} from '[^']+\/workerHelpers\.js';$/gm;
    if ([...generated.matchAll(helper)].length !== 1)
      throw new Error("Expected one wasm-bindgen-rayon helper import");
    // Host-specific JS glue shares the exact same threaded WASM binary.
    await writeFile(
      path,
      generated.replace(
        helper,
        'import { startWorkers } from "../../src/browser-rayon.js";',
      ),
    );
    await writeFile(
      resolve(out, "zopro_spr_node.js"),
      generated.replace(
        helper,
        'import { startWorkers } from "../../src/node-rayon.js";',
      ),
    );
    await rm(resolve(out, "snippets"), { recursive: true, force: true });
  }
}

for (const license of ["LICENSE-APACHE", "LICENSE-MIT"]) {
  await copyFile(
    resolve(crate, "../zopro-core", license),
    resolve(root, "runtime", license),
  );
}
