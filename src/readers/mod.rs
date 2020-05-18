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

pub use self::{
    code_section::{CodeSectionReader, FunctionBody, LocalsReader},
    data_section::{Data, DataKind, DataSectionReader},
    element_section::{
        Element,
        ElementItem,
        ElementItems,
        ElementItemsReader,
        ElementKind,
        ElementSectionReader,
    },
    export_section::{Export, ExportSectionReader},
    function_section::FunctionSectionReader,
    global_section::{Global, GlobalSectionReader},
    import_section::{Import, ImportSectionReader},
    init_expr::InitExpr,
    linking_section::LinkingSectionReader,
    memory_section::MemorySectionReader,
    module::{CustomSectionContent, ModuleReader, Section, SectionContent},
    name_section::{FunctionName, LocalName, ModuleName, Name, NameSectionReader, NamingReader},
    operators::OperatorsReader,
    producers_section::{
        ProducersField,
        ProducersFieldValue,
        ProducersFieldValuesReader,
        ProducersSectionReader,
    },
    reloc_section::{Reloc, RelocSectionReader},
    section_reader::{
        SectionIterator,
        SectionIteratorLimited,
        SectionReader,
        SectionWithLimitedItems,
    },
    table_section::TableSectionReader,
    type_section::TypeSectionReader,
};
use self::{
    data_count_section::read_data_count_section_content,
    sourcemappingurl_section::read_sourcemappingurl_section_content,
    start_section::read_start_section_content,
};
use super::{
    BinaryReader,
    BinaryReaderError,
    CustomSectionKind,
    ExternalKind,
    FuncType,
    GlobalType,
    ImportSectionEntryType,
    LinkingType,
    MemoryType,
    NameType,
    Naming,
    Operator,
    Range,
    RelocType,
    Result,
    SectionCode,
    SectionHeader,
    TableType,
    Type,
};

mod code_section;
mod data_count_section;
mod data_section;
mod element_section;
mod export_section;
mod function_section;
mod global_section;
mod import_section;
mod init_expr;
mod linking_section;
mod memory_section;
mod module;
mod name_section;
mod operators;
mod producers_section;
mod reloc_section;
mod section_reader;
mod sourcemappingurl_section;
mod start_section;
mod table_section;
mod type_section;
