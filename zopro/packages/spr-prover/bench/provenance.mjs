import { execFile } from "node:child_process";
import { readFile, readdir, realpath } from "node:fs/promises";
import { url as inspectorUrl } from "node:inspector";
import {
  availableParallelism,
  cpus,
  freemem,
  loadavg,
  machine,
  release,
  totalmem,
  type,
  uptime,
  version,
} from "node:os";
import {
  basename,
  dirname,
  isAbsolute,
  relative,
  resolve,
  sep,
} from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { digest, stableJson } from "./records.mjs";

const exec = promisify(execFile);
export const packageRoot = fileURLToPath(new URL("../", import.meta.url));

export function runtimeFlags(args) {
  const allowed =
    /^--(?:(?:no-)?(?:jitless|opt|lazy|wasm-lazy-compilation|wasm-tier-up|wasm-dynamic-tiering|wasm-bounds-checks|wasm-stack-checks|liftoff|turbofan|experimental-wasm-simd)|(?:max-old-space-size|max-semi-space-size|stack-size|stack-trace-limit|v8-pool-size)=\d+)$/u;
  return {
    allowed: args.filter((arg) => allowed.test(arg)),
    redacted_count: args.filter((arg) => !allowed.test(arg)).length,
    all_flags_sha256: digest(stableJson(args)),
  };
}

export function assertBenchmarkEnvironment(host) {
  const diagnostics = {
    exec_arguments: host.exec_argv,
    node_options: host.selected_environment.NODE_OPTIONS,
    instrumentation: host.instrumentation,
  };
  if (
    host.exec_argv.redacted_count ||
    host.selected_environment.NODE_OPTIONS?.redacted_count ||
    host.instrumentation.coverage_enabled ||
    host.instrumentation.inspector_active ||
    host.instrumentation.node_test_context
  ) {
    const error = new Error(
      "Benchmark preflight rejected instrumentation or unsupported Node flags: " +
        JSON.stringify(diagnostics),
    );
    error.diagnostics = diagnostics;
    throw error;
  }
}

async function command(file, args) {
  try {
    return {
      command: [file, ...args],
      output: (
        await exec(file, args, { timeout: 10_000, maxBuffer: 1024 * 1024 })
      ).stdout.trim(),
    };
  } catch (error) {
    return { command: [file, ...args], unavailable: error.message };
  }
}

export async function powerSnapshot() {
  if (process.platform === "darwin") {
    const battery = await command("pmset", ["-g", "batt"]);
    return {
      source:
        /Now drawing from '([^']+)'/u.exec(battery.output ?? "")?.[1] ??
        "unknown",
      battery,
      configuration: await command("pmset", ["-g", "custom"]),
      thermal: await command("pmset", ["-g", "therm"]),
    };
  }
  if (process.platform === "linux") {
    const supplies = [];
    for (const name of await readdir("/sys/class/power_supply").catch(
      () => [],
    )) {
      const values = {};
      for (const property of ["type", "online", "status", "capacity"]) {
        values[property] = await readFile(
          `/sys/class/power_supply/${name}/${property}`,
          "utf8",
        )
          .then((value) => value.trim())
          .catch(() => null);
      }
      supplies.push({ name, ...values });
    }
    const source = supplies
      .filter((supply) => supply.online !== null)
      .map(({ name, online }) => ({ name, online }));
    return {
      source,
      supplies,
      governor: await readFile(
        "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor",
        "utf8",
      )
        .then((value) => value.trim())
        .catch(() => null),
    };
  }
  return { source: "unavailable", platform: process.platform };
}

export async function loadSnapshot({ processes = false } = {}) {
  const snapshot = {
    at: new Date().toISOString(),
    load_avg: loadavg(),
    free_memory_bytes: freemem(),
    uptime_seconds: uptime(),
    cpu_times: cpus().map(({ times }) => times),
    available_threads: availableParallelism(),
    cpu_count: cpus().length,
    power: await powerSnapshot(),
  };
  if (processes) {
    const listing = await command("ps", ["-eo", "pid,ppid,pcpu,pmem,comm"]);
    snapshot.process_load = listing.output
      ? listing.output
          .split("\n")
          .slice(1)
          .flatMap((line) => {
            const match =
              /^\s*(\d+)\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+(.+)$/u.exec(line);
            if (!match) return [];
            const executable = basename(match[5]);
            const name =
              /^(?:node(?:js)?|cargo|rustc|wasm-bindgen|pnpm|npm|tsx|codex|WindowServer|Orca(?: Helper.*)?|Google Chrome(?: Helper.*)?)$/u.test(
                executable,
              )
                ? executable
                : "other";
            return [
              {
                name,
                cpu_percent: Number(match[3]),
                memory_percent: Number(match[4]),
              },
            ];
          })
          .sort((a, b) => b.cpu_percent - a.cpu_percent)
          .slice(0, 15)
      : { unavailable: true };
  }
  return snapshot;
}

export function stableEnvironment(host, snapshot) {
  return {
    node: host.node,
    node_versions: host.node_versions,
    executable: host.executable,
    exec_argv: host.exec_argv,
    selected_environment: host.selected_environment,
    instrumentation: host.instrumentation,
    platform: host.platform,
    arch: host.arch,
    machine: host.machine,
    os_release: host.os_release,
    os_version: host.os_version,
    cpus: host.cpus.map(({ model }) => model),
    available_threads: snapshot.available_threads ?? host.available_threads,
    cpu_count: snapshot.cpu_count ?? host.cpus.length,
    total_memory_bytes: host.total_memory_bytes,
    boot_id: host.boot_id,
    power_source: snapshot.power.source,
    power_configuration:
      snapshot.power.configuration?.output ?? snapshot.power.governor ?? null,
  };
}

export async function hostMetadata() {
  let boot;
  if (process.platform === "darwin")
    boot = await command("sysctl", ["-n", "kern.boottime"]);
  else if (process.platform === "linux")
    boot = {
      output: await readFile("/proc/sys/kernel/random/boot_id", "utf8")
        .then((value) => value.trim())
        .catch(() => null),
    };
  const names = [
    "NODE_OPTIONS",
    "UV_THREADPOOL_SIZE",
    "RAYON_NUM_THREADS",
    "ZOPRO_GPU",
    "ZOPRO_G2_GPU_SHARE",
    "ZOPRO_METAL_SIZE",
    "ZOPRO_METAL_HOST_REDUCE",
    "ZOPRO_METAL_TG_REDUCE",
  ];
  return {
    node: process.version,
    node_versions: process.versions,
    executable: {
      path: process.execPath,
      sha256: digest(await readFile(process.execPath)),
    },
    exec_argv: runtimeFlags(process.execArgv),
    selected_environment: Object.fromEntries(
      names.map((name) => {
        const value = process.env[name];
        if (value === undefined) return [name, null];
        if (name === "NODE_OPTIONS")
          return [
            name,
            runtimeFlags(value.trim().split(/\s+/u).filter(Boolean)),
          ];
        return [
          name,
          /^\d+(?:\.\d+)?$/u.test(value)
            ? value
            : { redacted: true, sha256: digest(value) },
        ];
      }),
    ),
    instrumentation: {
      coverage_enabled: Boolean(process.env.NODE_V8_COVERAGE),
      inspector_active: Boolean(inspectorUrl()),
      node_test_context: Boolean(process.env.NODE_TEST_CONTEXT),
    },
    platform: process.platform,
    arch: process.arch,
    machine: machine(),
    os_type: type(),
    os_release: release(),
    os_version: version(),
    cpus: cpus().map(({ model, speed }) => ({ model, reported_mhz: speed })),
    available_threads: availableParallelism(),
    total_memory_bytes: totalmem(),
    boot_id: boot?.output ?? null,
    hardware:
      process.platform === "darwin"
        ? await command("sysctl", [
            "hw.model",
            "hw.physicalcpu",
            "hw.logicalcpu",
            "hw.perflevel0.name",
            "hw.perflevel0.physicalcpu",
            "hw.perflevel1.name",
            "hw.perflevel1.physicalcpu",
          ])
        : null,
    toolchains: {
      rust: await command("rustup", [
        "run",
        "nightly-2025-11-15",
        "rustc",
        "--version",
      ]),
      wasm_bindgen: await command("wasm-bindgen", ["--version"]),
    },
  };
}

export async function fileHash(path) {
  const bytes = await readFile(path);
  return { size: bytes.length, sha256: digest(bytes) };
}

export async function sourceMetadata(runDirectory, directory = packageRoot) {
  const root = (
    await exec("git", ["rev-parse", "--show-toplevel"], { cwd: directory })
  ).stdout.trim();
  const git = async (...args) =>
    (await exec("git", args, { cwd: root })).stdout.trim();
  const physical = async (path) => {
    try {
      return await realpath(path);
    } catch (error) {
      if (error.code !== "ENOENT") throw error;
      return resolve(await physical(dirname(path)), basename(path));
    }
  };
  const relativeOutput = relative(root, await physical(resolve(runDirectory)));
  const within =
    relativeOutput &&
    relativeOutput !== ".." &&
    !relativeOutput.startsWith(`..${sep}`) &&
    !isAbsolute(relativeOutput);
  if (!relativeOutput)
    throw new Error("The benchmark output directory cannot be the source root");
  const literal = relativeOutput.split(sep).join("/");
  const exclusions = within ? [`:(exclude,literal)${literal}`] : [];
  if (within && (await git("ls-files", "--", `:(literal)${literal}`)))
    throw new Error("Benchmark output directory contains tracked source files");
  const status = await git(
    "status",
    "--porcelain=v1",
    "--untracked-files=all",
    "--",
    ".",
    ...exclusions,
  );
  if (status)
    throw new Error(
      `Benchmark source must be clean before timing or resuming:\n${status}`,
    );
  return {
    root,
    sha: await git("rev-parse", "HEAD"),
    tree: await git("rev-parse", "HEAD^{tree}"),
    clean: true,
    excluded_output_directory: within ? relativeOutput : null,
  };
}

export async function artifactMetadata() {
  const runtime = {};
  for (const path of [
    "runtime/single/zopro_spr.js",
    "runtime/single/zopro_spr_bg.wasm",
    "runtime/threads/zopro_spr.js",
    "runtime/threads/zopro_spr_node.js",
    "runtime/threads/zopro_spr_bg.wasm",
  ]) {
    runtime[path] = await fileHash(resolve(packageRoot, path));
  }
  const sdkEntry = fileURLToPath(import.meta.resolve("@heliuslabs/spr/client"));
  const sdkPackage = resolve(dirname(sdkEntry), "../../package.json");
  const sdk = JSON.parse(await readFile(sdkPackage, "utf8"));
  return {
    runtime,
    package_lock: await fileHash(resolve(packageRoot, "package-lock.json")),
    rust_lock: await fileHash(resolve(packageRoot, "../../Cargo.lock")),
    key_manifest: await fileHash(
      resolve(packageRoot, "../../tools/spr/zopro-keys.lock"),
    ),
    sdk: {
      version: sdk.version,
      entry: sdkEntry,
      package: await fileHash(sdkPackage),
    },
  };
}
