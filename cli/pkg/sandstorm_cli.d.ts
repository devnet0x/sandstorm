/* tslint:disable */
/* eslint-disable */
/**
*/
export function start(): void;
/**
* @param {string} command
* @param {string} program_json_str
* @param {string} air_public_input_json_str
* @param {Uint8Array | undefined} [_proof_file]
* @param {Uint8Array | undefined} [_trace_file]
* @param {Uint8Array | undefined} [_memory_file]
* @param {string | undefined} [_air_private_input_json_str]
* @returns {string}
*/
export function main2(command: string, program_json_str: string, air_public_input_json_str: string, _proof_file?: Uint8Array, _trace_file?: Uint8Array, _memory_file?: Uint8Array, _air_private_input_json_str?: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main2: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number) => void;
  readonly start: () => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
