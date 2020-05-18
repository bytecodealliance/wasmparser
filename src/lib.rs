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

//! A simple event-driven library for parsing WebAssembly binary files
//! (or streams).
//!
//! The parser library reports events as they happend and only stores
//! parsing information for a brief period of time, making it very fast
//! and memory-efficient. The event-driven model, however, has some drawbacks.
//! If you need random access to the entire WebAssembly data-structure,
//! this is not the right library for you. You could however, build such
//! a data-structure using this library.

use crate::binary_reader::SectionHeader;
pub(crate) use crate::module_resources::{wasm_func_type_inputs, wasm_func_type_outputs};
pub use crate::{
    binary_reader::{BinaryReader, Range},
    module_resources::{
        WasmFuncType,
        WasmGlobalType,
        WasmMemoryType,
        WasmModuleResources,
        WasmTableType,
        WasmType,
    },
    operators_validator::OperatorValidatorConfig,
    parser::{
        ElemSectionEntryTable,
        LocalName,
        NameEntry,
        Parser,
        ParserInput,
        ParserState,
        RelocEntry,
        WasmDecoder,
    },
    primitives::{
        BinaryReaderError,
        BrTable,
        CustomSectionKind,
        ExternalKind,
        FuncType,
        GlobalType,
        Ieee32,
        Ieee64,
        ImportSectionEntryType,
        LinkingType,
        MemoryImmediate,
        MemoryType,
        NameType,
        Naming,
        Operator,
        RelocType,
        ResizableLimits,
        Result,
        SectionCode,
        TableType,
        Type,
        TypeOrFuncType,
        V128,
    },
    readers::{
        CodeSectionReader,
        CustomSectionContent,
        Data,
        DataKind,
        DataSectionReader,
        Element,
        ElementItem,
        ElementItems,
        ElementItemsReader,
        ElementKind,
        ElementSectionReader,
        Export,
        ExportSectionReader,
        FunctionBody,
        FunctionSectionReader,
        Global,
        GlobalSectionReader,
        Import,
        ImportSectionReader,
        InitExpr,
        LinkingSectionReader,
        LocalsReader,
        MemorySectionReader,
        ModuleReader,
        Name,
        NameSectionReader,
        NamingReader,
        OperatorsReader,
        ProducersField,
        ProducersFieldValue,
        ProducersFieldValuesReader,
        ProducersSectionReader,
        Reloc,
        RelocSectionReader,
        Section,
        SectionContent,
        SectionIterator,
        SectionIteratorLimited,
        SectionReader,
        SectionWithLimitedItems,
        TableSectionReader,
        TypeSectionReader,
    },
    validator::{
        validate,
        validate_function_body,
        ValidatingOperatorParser,
        ValidatingParser,
        ValidatingParserConfig,
    },
};

mod binary_reader;
mod limits;
mod module_resources;
mod operators_validator;
mod parser;
mod primitives;
mod readers;
mod tests;
mod validator;
