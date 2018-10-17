/* Copyright 2018 Mozilla Foundation
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

use super::{BinaryReader, BinaryReaderError, FuncType, Result, SectionCode};

use super::SectionHeader;

pub use self::code_section::CodeSectionReader;
pub use self::function_section::FunctionSectionReader;
pub use self::module::ModuleReader;
pub use self::module::Section;
pub use self::type_section::TypeSectionReader;

mod code_section;
mod function_section;
mod module;
mod type_section;
