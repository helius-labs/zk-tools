import type {
  MergeInputs,
  Proof,
  ProofService,
  ProverInputs,
} from "@heliuslabs/spr/client";
export type { MergeInputs, Proof, ProverInputs } from "@heliuslabs/spr/client";

export interface SprProver extends ProofService {
  name(): string;
  digest(): string;
  inputs(): number;
  outputs(): number;
  verify(inputs: ProverInputs, proof: Proof): boolean;
  verifyMerge(inputs: MergeInputs, proof: Proof): boolean;
  export_raw(): Uint8Array;
  free(): void;
}

export interface Runtime {
  readonly SprProver: new (key: Uint8Array) => SprProver;
  readonly threads: number;
  /** Free loaded provers before closing. Start a new worker to use another runtime. */
  close(): Promise<void>;
}

/** Once per worker; threads: 0 (default) selects WASM without shared memory. */
export function init(options?: { readonly threads?: number }): Promise<Runtime>;

export interface LocalProofService extends ProofService {
  /** Fetch, validate and decode this key into WASM before a proof is timed. One key is resident at a time. */
  preload(keyName: string): Promise<void>;
  /** Drain accepted calls, free the loaded key and close the worker's runtime. */
  close(): Promise<void>;
}

export interface LocalProofServiceOptions {
  /** 0 (default) uses the single-threaded runtime; 1–64 starts a Rayon pool. */
  readonly threads?: number;
  /** Node only. Omit for the platform cache directory; browsers use Cache API with memory fallback. */
  readonly keyDirectory?: string;
}

/** One service owns this JavaScript worker's runtime. Uses the SDK's pinned production key set. */
export function createLocalProofService(
  options?: LocalProofServiceOptions,
): Promise<LocalProofService>;
