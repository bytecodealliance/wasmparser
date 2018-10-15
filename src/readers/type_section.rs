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

use super::{BinaryReader, BinaryReaderError, FuncType};

pub enum TypeSectionReaderState {
    Initial,
    Error(BinaryReaderError),
    Count(u32),
    Entry {
        ty: FuncType,
        index: u32,
        count: u32,
    },
    End,
}

pub struct TypeSectionReader<'a> {
    reader: BinaryReader<'a>,
    state: TypeSectionReaderState,
}

impl<'a> TypeSectionReader<'a> {
    pub fn new(data: &'a [u8]) -> TypeSectionReader<'a> {
        TypeSectionReader {
            reader: BinaryReader::new(data),
            state: TypeSectionReaderState::Initial,
        }
    }

    pub fn read_count(&mut self) {
        match self.reader.read_var_u32() {
            Ok(count) => self.state = TypeSectionReaderState::Count(count),
            Err(err) => self.state = TypeSectionReaderState::Error(err),
        }
    }

    pub fn read_entry(&mut self, index: u32, count: u32) {
        match self.reader.read_func_type() {
            Ok(ty) => self.state = TypeSectionReaderState::Entry { ty, index, count },
            Err(err) => self.state = TypeSectionReaderState::Error(err),
        }
    }

    /// Reads content of the type section. See also `TypeSectionReaderState` enum.
    ///
    /// # Examples
    /// ```
    /// # let data: &[u8] = &[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    /// #     0x01, 0x4, 0x01, 0x60, 0x00, 0x00, 0x03, 0x02, 0x01, 0x00,
    /// #     0x0a, 0x05, 0x01, 0x03, 0x00, 0x01, 0x0b];
    /// use wasmparser::{ModuleReader, ModuleReaderState, TypeSectionReaderState};
    /// let mut reader = ModuleReader::new(data);
    /// match *reader.read() {
    ///     ModuleReaderState::Header { .. } => (),
    ///     _ => panic!("header expected")   
    /// };
    /// let mut type_reader = match *reader.read() {
    ///     ModuleReaderState::Section { .. } => reader.get_type_section_reader(),
    ///     _ => panic!("section expected")
    ///
    /// };
    /// match *type_reader.read() {
    ///     TypeSectionReaderState::Count(count) => assert!(count == 1),
    ///     _ => panic!("type count expected")
    /// };
    /// match *type_reader.read() {
    ///     TypeSectionReaderState::Entry { ref ty, .. } => println!("Type {:?}", ty),
    ///     _ => panic!("section expected")
    /// };
    /// ```
    pub fn read(&mut self) -> &TypeSectionReaderState {
        match self.state {
            TypeSectionReaderState::Initial => self.read_count(),
            TypeSectionReaderState::Error(_) | TypeSectionReaderState::End => {
                panic!("Unexpected state")
            }
            TypeSectionReaderState::Count(count) if count > 0 => self.read_entry(0, count),
            TypeSectionReaderState::Entry { index, count, .. } if index + 1 < count => {
                self.read_entry(index + 1, count)
            }
            TypeSectionReaderState::Count(_) | TypeSectionReaderState::Entry { .. } => {
                if self.reader.eof() {
                    self.state = TypeSectionReaderState::End;
                } else {
                    self.state = TypeSectionReaderState::Error(BinaryReaderError {
                        message: "Unexpected type section end",
                        offset: self.reader.position,
                    });
                }
            }
        };
        &self.state
    }
}
