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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly process_str: (a: number, b: number, c: number) => void;
  readonly help_text: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
