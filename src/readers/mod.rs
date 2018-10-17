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

use super::{
    BinaryReader, BinaryReaderError, ExternalKind, FuncType, GlobalType, ImportSectionEntryType,
    MemoryType, Result, SectionCode,
};

use super::SectionHeader;

pub use self::code_section::CodeSectionReader;
pub use self::data_section::Data;
pub use self::data_section::DataSectionReader;
pub use self::export_section::Export;
pub use self::export_section::ExportSectionReader;
pub use self::function_section::FunctionSectionReader;
pub use self::global_section::Global;
pub use self::global_section::GlobalSectionReader;
pub use self::import_section::Import;
pub use self::import_section::ImportSectionReader;
pub use self::init_expr::InitExpr;
pub use self::memory_section::MemorySectionReader;
pub use self::module::ModuleReader;
pub use self::module::Section;
pub use self::type_section::TypeSectionReader;

mod code_section;
mod data_section;
mod export_section;
mod function_section;
mod global_section;
mod import_section;
mod init_expr;
mod memory_section;
mod module;
mod type_section;
