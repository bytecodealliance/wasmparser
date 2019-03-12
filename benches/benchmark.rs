pub fn read_file_data(path: &PathBuf) -> Vec<u8> {
    println!("Parsing {:?}", path);
    let mut data = Vec::new();
    let mut f = File::open(path).ok().unwrap();
    f.read_to_end(&mut data).unwrap();
    data
}

const VALIDATOR_CONFIG: Option<ValidatingParserConfig> = Some(ValidatingParserConfig {
    operator_config: OperatorValidatorConfig {
        enable_threads: true,
        enable_reference_types: true,
        enable_simd: true,
        enable_bulk_memory: true,
    },
    mutable_global_imports: true,
});

#[macro_use]
extern crate criterion;
extern crate wasmparser;

use criterion::Criterion;
use wasmparser::{Parser, OperatorValidatorConfig, ValidatingParser, ValidatingParserConfig};


use std::fs::{read_dir, File};
use std::path::PathBuf;
use std::io::Read;

fn it_works_benchmark(c: &mut Criterion) {
    c.bench_function("it works benchmark", move |b| {
        for entry in read_dir("tests").unwrap() {
            let dir = entry.unwrap();
            if !dir.file_type().unwrap().is_file() {
                continue;
            }
            let data = read_file_data(&dir.path());
            b.iter(|| Parser::new(data.as_slice()))
        }
    });
}

fn validator_not_fails_benchmark(c: &mut Criterion) {
    c.bench_function("validator no fails benchmark", move |b| {
        for entry in read_dir("tests").unwrap() {
            let dir = entry.unwrap();
            if !dir.file_type().unwrap().is_file() {
                continue;
            }
            let data = read_file_data(&dir.path());
            b.iter(|| ValidatingParser::new(data.as_slice(), VALIDATOR_CONFIG));
        }
    });
}


criterion_group!(benchmark, it_works_benchmark, validator_not_fails_benchmark);
criterion_main!(benchmark);