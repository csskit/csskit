use bumpalo::Bump;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use css_lexer::Lexer;
use css_parse::Parser;
use csskit_ast::CsskitAtomSet;
use csskit_ast::sheet::Sheet;
use glob::glob;
#[cfg(target_family = "unix")]
use pprof::criterion::{Output, PProfProfiler};
use std::fs::read_to_string;

const FIXTURES_GLOB: &str = "../../coverage/linting/*.cks";

struct TestFile {
	name: String,
	source_text: String,
}

fn get_files() -> Vec<TestFile> {
	let mut files = vec![];
	for source_path in glob(FIXTURES_GLOB).unwrap().flatten() {
		files.push(TestFile {
			name: source_path.file_stem().unwrap().to_str().unwrap().to_owned(),
			source_text: read_to_string(&source_path).unwrap(),
		});
	}
	files
}

fn parse_sheet(c: &mut Criterion) {
	let mut group = c.benchmark_group("parse_sheet");
	for file in get_files() {
		group.throughput(Throughput::Bytes(file.source_text.len() as u64));
		group.bench_with_input(BenchmarkId::from_parameter(&file.name), &file.source_text, |b, source_text| {
			b.iter_with_large_drop(|| {
				let allocator = Bump::default();
				let lexer = Lexer::new(&CsskitAtomSet::ATOMS, &source_text);
				let _ = Parser::new(&allocator, source_text, lexer).parse_entirely::<Sheet>();

				allocator
			});
		});
	}
	group.finish();
}

#[cfg(target_family = "unix")]
criterion_group! {
	name = benches;
	config = Criterion::default()
		.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = parse_sheet
}

#[cfg(not(target_family = "unix"))]
criterion_group! {
	name = benches;
	config = Criterion::default();
	targets = parse_sheet
}

criterion_main!(benches);
