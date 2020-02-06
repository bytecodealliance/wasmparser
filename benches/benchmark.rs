const VALIDATOR_CONFIG: Option<ValidatingParserConfig> = Some(ValidatingParserConfig {
    operator_config: OperatorValidatorConfig {
        enable_threads: true,
        enable_reference_types: true,
        enable_simd: true,
        enable_bulk_memory: true,
        enable_multi_value: true,
    },
});

#[macro_use]
extern crate criterion;
extern crate wasmparser;

use criterion::Criterion;
use wasmparser::{
    validate, OperatorValidatorConfig, Parser, ParserState, ValidatingParser,
    ValidatingParserConfig, WasmDecoder,
};

use std::fs::{read, read_dir};
use std::path::PathBuf;

fn read_all_wasm<'a, T>(mut d: T)
where
    T: WasmDecoder<'a>,
{
    loop {
        match *d.read() {
            ParserState::Error(ref e) => panic!("unexpected error {}", e),
            ParserState::EndWasm => return,
            _ => (),
        }
    }
}

static TEST_FILES: &[&'static str] = &[
    "atomic.0.wasm",
    "block.0.wasm",
    "br_table.0.wasm",
    "br_if.0.wasm",
    "call.0.wasm",
    "call_indirect.0.wasm",
    "conversions.0.wasm",
    "elem.0.wasm",
    "fac.0.wasm",
    "float_literals.0.wasm",
    "func.0.wasm",
    "get_local.0.wasm",
    "globals.0.wasm",
    "int_exprs.0.wasm",
    "if.0.wasm",
    "imports.0.wasm",
    "loop.0.wasm",
    "memory_grow.0.wasm",
    "return.0.wasm",
    "select.0.wasm",
    "simd.wasm",
    "type.0.wasm",
];

fn read_files_data() -> Vec<Vec<u8>> {
    let mut data: Vec<Vec<u8>> = vec![];
    let path = PathBuf::from("tests");
    for test_file in TEST_FILES {
        let path = path.join(test_file);
        data.push(read(&path).expect("wasm file data"));
    }
    data
}

fn it_works_benchmark(c: &mut Criterion) {
    let data = read_files_data();
    c.bench_function("it works benchmark", move |b| {
        for d in data.iter() {
            b.iter(|| read_all_wasm(Parser::new(d.as_slice())));
        }
    });
}

fn validator_not_fails_benchmark(c: &mut Criterion) {
    let data = read_files_data();
    c.bench_function("validator no fails benchmark", move |b| {
        for d in data.iter() {
            b.iter(|| read_all_wasm(ValidatingParser::new(d.as_slice(), VALIDATOR_CONFIG)));
        }
    });
}

fn validate_benchmark(c: &mut Criterion) {
    let data = read_files_data();
    c.bench_function("validate benchmark", move |b| {
        for d in data.iter() {
            b.iter(|| validate(d.as_slice(), VALIDATOR_CONFIG));
        }
    });
}

criterion_group!(
    benchmark,
    it_works_benchmark,
    validator_not_fails_benchmark,
    validate_benchmark
);
criterion_main!(benchmark);
