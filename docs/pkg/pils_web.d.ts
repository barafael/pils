/* tslint:disable */
/* eslint-disable */
/**
* @param {string} line
* @returns {string}
*/
export function process_str(line: string): string;
/**
* @returns {string}
*/
export function help_text(): string;
/**
* @returns {string}
*/
export function get_env_json(): string;
/**
* @returns {string}
*/
export function get_env_tuples(): string;
/**
* @returns {string}
*/
export function get_example_environment(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly process_str: (a: number, b: number, c: number) => void;
  readonly help_text: (a: number) => void;
  readonly get_env_json: (a: number) => void;
  readonly get_env_tuples: (a: number) => void;
  readonly get_example_environment: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
