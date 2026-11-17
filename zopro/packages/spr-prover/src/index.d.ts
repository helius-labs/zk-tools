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
