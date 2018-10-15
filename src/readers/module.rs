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

use std::iter::Iterator;

use super::{BinaryReader, BinaryReaderError, Range, Result, SectionCode, SectionHeader};

#[derive(Debug)]
pub enum ModuleReaderState<'a> {
    Initial,
    Error(BinaryReaderError),
    Header { version: u32 },
    Section { code: SectionCode<'a>, range: Range },
    End,
}

/// Reads top-level WebAssembly file structure: header and sections.
pub struct ModuleReader<'a> {
    reader: BinaryReader<'a>,
    state: ModuleReaderState<'a>,
}

impl<'a> ModuleReader<'a> {
    pub fn new(data: &[u8]) -> ModuleReader {
        return ModuleReader {
            reader: BinaryReader::new(data),
            state: ModuleReaderState::Initial,
        };
    }

    fn set_error_state(&mut self, message: &'static str, offset: usize) {
        self.state = ModuleReaderState::Error(BinaryReaderError { message, offset });
    }

    fn read_header(&mut self) {
        match self.reader.read_file_header() {
            Ok(version) => self.state = ModuleReaderState::Header { version },
            Err(err) => self.state = ModuleReaderState::Error(err),
        };
    }

    fn read_section(&mut self) {
        if self.reader.eof() {
            self.state = ModuleReaderState::End;
            return;
        }
        let SectionHeader {
            code,
            payload_start,
            payload_len,
        } = match self.reader.read_section_header() {
            Ok(header) => header,
            Err(err) => {
                self.state = ModuleReaderState::Error(err);
                return;
            }
        };
        let payload_end = payload_start + payload_len;
        if self.reader.buffer.len() < payload_end {
            self.set_error_state(
                "Section body extends past end of file",
                self.reader.buffer.len(),
            );
            return;
        }
        if self.reader.position > payload_end {
            self.set_error_state(
                "Section header is too big to fit into section body",
                payload_end,
            );
            return;
        }
        let range = Range {
            start: self.reader.position,
            end: payload_end,
        };
        self.reader.skip_to(payload_end);
        self.state = ModuleReaderState::Section { code, range };
    }

    /// Reads next top-level record from the WebAssembly binary data.
    /// The methods returns reference to current state of the reader.
    /// See `ModuleReaderState` enum.
    ///
    /// # Examples
    /// ```
    /// # let data: &[u8] = &[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    /// #     0x01, 0x4, 0x01, 0x60, 0x00, 0x00, 0x03, 0x02, 0x01, 0x00,
    /// #     0x0a, 0x05, 0x01, 0x03, 0x00, 0x01, 0x0b];
    /// use wasmparser::{ModuleReader, ModuleReaderState};
    /// let mut reader = ModuleReader::new(data);
    /// {
    ///     let state = reader.read();
    ///     println!("First state {:?}", state);
    /// }
    /// {
    ///     let state = reader.read();
    ///     println!("Second state {:?}", state);
    /// }
    /// ```
    pub fn read(&mut self) -> &ModuleReaderState<'a> {
        match self.state {
            ModuleReaderState::Initial => self.read_header(),
            ModuleReaderState::End | ModuleReaderState::Error(_) => panic!("Unexpected state"),
            ModuleReaderState::Header { .. } | ModuleReaderState::Section { .. } => {
                self.read_section()
            }
        }
        &self.state
    }

    /// Creates iterator that reads sections data. The iterator is possible to
    /// create after file header is read.
    ///
    /// # Examples
    /// ```
    /// # let data: &[u8] = &[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    /// #     0x01, 0x4, 0x01, 0x60, 0x00, 0x00, 0x03, 0x02, 0x01, 0x00,
    /// #     0x0a, 0x05, 0x01, 0x03, 0x00, 0x01, 0x0b];
    /// use wasmparser::ModuleReader;
    /// let mut reader = ModuleReader::new(data);
    /// reader.read(); // skip header
    /// for i in reader.get_sections_iter() {
    ///     println!("Section {:?} {:?}", i.unwrap().code, i.unwrap().range);
    /// }    
    /// ```
    pub fn get_sections_iter(&'a mut self) -> SectionsIterator<'a> {
        match self.state {
            ModuleReaderState::Header { .. } => SectionsIterator {
                module_reader: self,
                eof: false,
            },
            _ => panic!("Unexpected state for get_sections_iterator"),
        }
    }
}

pub struct SectionsIterator<'a> {
    module_reader: &'a mut ModuleReader<'a>,
    eof: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct SectionsIteratorItem<'a> {
    pub code: SectionCode<'a>,
    pub range: Range,
}

impl<'a> Iterator for SectionsIterator<'a> {
    type Item = Result<SectionsIteratorItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }
        match *self.module_reader.read() {
            ModuleReaderState::End => {
                self.eof = true;
                return None;
            }
            ModuleReaderState::Error(err) => {
                self.eof = true;
                return Some(Err(err));
            }
            ModuleReaderState::Section { code, range } => {
                return Some(Ok(SectionsIteratorItem { code, range }));
            }
            ModuleReaderState::Initial | ModuleReaderState::Header { .. } => {
                unreachable!("Unexpected state")
            }
        }
    }
}
