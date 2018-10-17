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

use super::{BinaryReader, BinaryReaderError, Result, SectionCode, SectionHeader};

use super::{
    CodeSectionReader, ExportSectionReader, FunctionSectionReader, ImportSectionReader,
    TypeSectionReader,
};

#[derive(Debug)]
pub struct Section<'a> {
    pub code: SectionCode<'a>,
    offset: usize,
    data: &'a [u8],
}

impl<'a> Section<'a> {
    /// Creates reader for the type section. Available when the reader just read
    /// the type section.
    pub fn get_type_section_reader(&self) -> Result<TypeSectionReader> {
        match self.code {
            SectionCode::Type => TypeSectionReader::new(self.data, self.offset),
            _ => panic!("Invalid state for get_type_section_reader"),
        }
    }

    /// Creates reader for the function section. Available when the reader just read
    /// the function section.
    pub fn get_function_section_reader(&self) -> Result<FunctionSectionReader> {
        match self.code {
            SectionCode::Function => FunctionSectionReader::new(self.data, self.offset),
            _ => panic!("Invalid state for get_function_section_reader"),
        }
    }

    /// Creates reader for the code section. Available when the reader just read
    /// the code section.
    pub fn get_code_section_reader(&self) -> Result<CodeSectionReader> {
        match self.code {
            SectionCode::Code => CodeSectionReader::new(self.data, self.offset),
            _ => panic!("Invalid state for get_function_section_reader"),
        }
    }

    /// Creates reader for the export section. Available when the reader just read
    /// the export section.
    pub fn get_export_section_reader(&self) -> Result<ExportSectionReader> {
        match self.code {
            SectionCode::Export => ExportSectionReader::new(self.data, self.offset),
            _ => panic!("Invalid state for get_export_section_reader"),
        }
    }

    /// Creates reader for the import section. Available when the reader just read
    /// the import section.
    pub fn get_import_section_reader(&self) -> Result<ImportSectionReader> {
        match self.code {
            SectionCode::Import => ImportSectionReader::new(self.data, self.offset),
            _ => panic!("Invalid state for get_import_section_reader"),
        }
    }
}

/// Reads top-level WebAssembly file structure: header and sections.
pub struct ModuleReader<'a> {
    reader: BinaryReader<'a>,
    version: u32,
    read_ahead: Option<SectionHeader<'a>>,
}

impl<'a> ModuleReader<'a> {
    pub fn new(data: &[u8]) -> Result<ModuleReader> {
        let mut reader = BinaryReader::new(data);
        let version = reader.read_file_header()?;
        Ok(ModuleReader {
            reader,
            version,
            read_ahead: None,
        })
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn eof(&self) -> bool {
        self.read_ahead.is_none() && self.reader.eof()
    }

    fn verify_section_end(&self, end: usize) -> Result<()> {
        if self.reader.buffer.len() < end {
            return Err(BinaryReaderError {
                message: "Section body extends past end of file",
                offset: self.reader.buffer.len(),
            });
        }
        if self.reader.position > end {
            return Err(BinaryReaderError {
                message: "Section header is too big to fit into section body",
                offset: end,
            });
        }
        Ok(())
    }

    /// Reads next top-level record from the WebAssembly binary data.
    /// The methods returns reference to current state of the reader.
    ///
    /// # Examples
    /// ```
    /// # let data: &[u8] = &[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    /// #     0x01, 0x4, 0x01, 0x60, 0x00, 0x00, 0x03, 0x02, 0x01, 0x00,
    /// #     0x0a, 0x05, 0x01, 0x03, 0x00, 0x01, 0x0b];
    /// use wasmparser::ModuleReader;
    /// let mut reader = ModuleReader::new(data).expect("reader");
    /// let section = reader.read().expect("section #1");
    /// println!("First section {:?}", section);
    /// let section = reader.read().expect("section #2");
    /// println!("Second section {:?}", section);
    /// assert!(!reader.eof(), "there are more sections");
    /// ```
    pub fn read<'b>(&mut self) -> Result<Section<'b>>
    where
        'a: 'b,
    {
        let SectionHeader {
            code,
            payload_start,
            payload_len,
        } = match self.read_ahead.take() {
            Some(section_header) => section_header,
            None => self.reader.read_section_header()?,
        };
        let payload_end = payload_start + payload_len;
        self.verify_section_end(payload_end)?;
        let body_start = self.reader.position;
        self.reader.skip_to(payload_end);
        Ok(Section {
            code,
            offset: body_start,
            data: &self.reader.buffer[body_start..payload_end],
        })
    }

    fn ensure_read_ahead(&mut self) -> Result<()> {
        if self.read_ahead.is_none() && !self.eof() {
            self.read_ahead = Some(self.reader.read_section_header()?);
        }
        Ok(())
    }

    /// Skips custom sections.
    ///
    /// # Examples
    /// ```
    /// # let data: &[u8] = &[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    /// #     0x00, 0x8, 0x03, 0x63, 0x61, 0x74, 0x01, 0x02, 0x03, 0x04,
    /// #     0x01, 0x4, 0x01, 0x60, 0x00, 0x00, 0x03, 0x02, 0x01, 0x00,
    /// #     0x0a, 0x05, 0x01, 0x03, 0x00, 0x01, 0x0b];
    /// use wasmparser::ModuleReader;
    /// use wasmparser::SectionCode;
    /// let mut reader = ModuleReader::new(data).expect("reader");
    /// while { reader.skip_custom_sections(); !reader.eof() } {
    ///     let section = reader.read().expect("section");
    ///     if let SectionCode::Custom {..} = section.code { panic!("no custom"); }
    ///     println!("Section {:?}", section);
    /// }
    /// ```
    pub fn skip_custom_sections(&mut self) -> Result<()> {
        loop {
            self.ensure_read_ahead()?;
            match self.read_ahead {
                Some(SectionHeader {
                    code: SectionCode::Custom { .. },
                    payload_start,
                    payload_len,
                }) => {
                    self.verify_section_end(payload_start + payload_len)?;
                    // Skip section
                    self.read_ahead = None;
                    self.reader.skip_to(payload_start + payload_len);
                }
                _ => break,
            };
        }
        Ok(())
    }
}
