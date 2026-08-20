use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::{Arena, CursorCompactWriteSink, CursorOverlaySink, Parser, ToCursors};
use csskit_transform::{CssMinifierFeature, Transformer};
use glob::glob;
#[cfg(target_family = "unix")]
use pprof::criterion::{Output, PProfProfiler};
use std::fs::read_to_string;

const FIXTURES_GLOB: &str = "../../coverage/popular/*.css";

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

fn popular(c: &mut Criterion) {
	let mut group = c.benchmark_group("minify_popular");
	for file in get_files() {
		group.throughput(Throughput::Bytes(file.source_text.len() as u64));
		group.bench_with_input(BenchmarkId::from_parameter(&file.name), &file.source_text, |b, source_text| {
			b.iter_with_large_drop(|| {
				let alloc = Arena::default();
				{
					let mut transformer =
						Transformer::new_in(&alloc, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, source_text);
					let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
					let mut result =
						Parser::new(&alloc, source_text.as_str(), lexer).parse_entirely::<StyleSheet>().with_trivia();
					let mut string = css_parse::String::new_in(&alloc);
					if let Some(stylesheet) = result.output.as_mut() {
						transformer.transform(stylesheet);
						let overlays = transformer.overlays();
						let mut sink = CursorOverlaySink::new(
							source_text,
							&overlays,
							CursorCompactWriteSink::new(source_text, &mut string),
						);
						result.to_cursors(&mut sink);
					}
				}
				alloc
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
	targets = popular
}

#[cfg(not(target_family = "unix"))]
criterion_group! {
	name = benches;
	config = Criterion::default()
	targets = popular
}

criterion_main!(benches);
