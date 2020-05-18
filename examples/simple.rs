extern crate wasmparser;

use std::{env, fs::File, io, io::prelude::*, str};
use wasmparser::{Parser, ParserState, WasmDecoder};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} in.wasm", args[0]);
        return;
    }

    let buf: Vec<u8> = read_wasm(&args[1]).unwrap();
    let mut parser = Parser::new(&buf);
    loop {
        let state = parser.read();
        match *state {
            ParserState::BeginWasm { .. } => {
                println!("====== Module");
            }
            ParserState::ExportSectionEntry {
                field, ref kind, ..
            } => {
                println!("  Export {} {:?}", field, kind);
            }
            ParserState::ImportSectionEntry { module, field, .. } => {
                println!("  Import {}::{}", module, field)
            }
            ParserState::EndWasm => break,
            ParserState::Error(ref err) => panic!("Error: {:?}", err),
            _ => ( /* println!(" Other {:?}", state); */ ),
        }
    }
}

fn read_wasm(file: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut f = File::open(file)?;
    f.read_to_end(&mut data)?;
    Ok(data)
}
