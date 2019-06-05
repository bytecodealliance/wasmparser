/* Copyright 2017 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// The following limits are imposed by wasmparser on WebAssembly modules.
// The limits are agreed upon with other engines for consistency.
pub const MAX_WASM_TYPES: usize = 1000000;
pub const MAX_WASM_FUNCTIONS: usize = 1000000;
pub const _MAX_WASM_IMPORTS: usize = 100000;
pub const _MAX_WASM_EXPORTS: usize = 100000;
pub const MAX_WASM_GLOBALS: usize = 1000000;
pub const _MAX_WASM_DATA_SEGMENTS: usize = 100000;
pub const MAX_WASM_MEMORY_PAGES: usize = 65536;
pub const MAX_WASM_STRING_SIZE: usize = 100000;
pub const _MAX_WASM_MODULE_SIZE: usize = 1024 * 1024 * 1024; //= 1 GiB
pub const MAX_WASM_FUNCTION_SIZE: usize = 128 * 1024;
pub const MAX_WASM_FUNCTION_LOCALS: usize = 50000;
pub const MAX_WASM_FUNCTION_PARAMS: usize = 1000;
pub const MAX_WASM_FUNCTION_RETURNS: usize = 1000;
pub const _MAX_WASM_TABLE_SIZE: usize = 10000000;
pub const MAX_WASM_TABLE_ENTRIES: usize = 10000000;
pub const MAX_WASM_TABLES: usize = 1;
pub const MAX_WASM_MEMORIES: usize = 1;
