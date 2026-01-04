use bumpalo::Bump;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::Parser;
use csskit_ast::sheet::Sheet;
use csskit_ast::{Collector, CsskitAtomSet};
use glob::glob;
#[cfg(target_family = "unix")]
use pprof::criterion::{Output, PProfProfiler};
use std::fs::read_to_string;

const CSS_FIXTURES_GLOB: &str = "../../coverage/popular/*.css";
const QUERY_FIXTURES_GLOB: &str = "../../coverage/linting/*.cks";

struct CssFile {
	name: String,
	source_text: String,
}

struct QueryFile {
	name: String,
	source_text: String,
}

fn get_css_files() -> Vec<CssFile> {
	let mut files = vec![];
	for source_path in glob(CSS_FIXTURES_GLOB).unwrap().flatten() {
		files.push(CssFile {
			name: source_path.file_stem().unwrap().to_str().unwrap().to_owned(),
			source_text: read_to_string(&source_path).unwrap(),
		});
	}
	files
}

fn get_query_files() -> Vec<QueryFile> {
	let mut files = vec![];
	for source_path in glob(QUERY_FIXTURES_GLOB).unwrap().flatten() {
		files.push(QueryFile {
			name: source_path.file_stem().unwrap().to_str().unwrap().to_owned(),
			source_text: read_to_string(&source_path).unwrap(),
		});
	}
	files
}

fn collector(c: &mut Criterion) {
	let mut group = c.benchmark_group("collector");

	// Get CSS and query files
	let css_files = get_css_files();
	let query_files = get_query_files();

	// Benchmark all combinations of CSS and query files
	for css_file in &css_files {
		for query_file in &query_files {
			let combined_name = format!("{}_{}", css_file.name, query_file.name);
			group.throughput(Throughput::Bytes(css_file.source_text.len() as u64));

			group.bench_function(BenchmarkId::from_parameter(&combined_name), |b| {
				b.iter_with_large_drop(|| {
					let allocator = Bump::default();
					{
						// Parse the CSS stylesheet
						let css_lexer = Lexer::new(&CssAtomSet::ATOMS, &css_file.source_text);
						let css_result =
							Parser::new(&allocator, &css_file.source_text, css_lexer).parse_entirely::<StyleSheet>();

						// Parse the query sheet
						let query_lexer = Lexer::new(&CsskitAtomSet::ATOMS, &query_file.source_text);
						let query_result =
							Parser::new(&allocator, &query_file.source_text, query_lexer).parse_entirely::<Sheet>();

						if let (Some(stylesheet), Some(sheet)) = (css_result.output, query_result.output) {
							// Create collector and run collection
							let mut collector = Collector::new(&sheet, &query_file.source_text, &allocator);
							collector.collect(&stylesheet, &css_file.source_text);

							// Get stats and diagnostics
							let _stats = black_box(collector.stats());
							let _diagnostics: Vec<_> =
								black_box(collector.diagnostics(&css_file.source_text).collect());
						}
					}
					allocator
				});
			});
		}
	}

	group.finish();
}

#[cfg(target_family = "unix")]
criterion_group! {
	name = benches;
	config = Criterion::default()
		.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = collector
}

#[cfg(not(target_family = "unix"))]
criterion_group! {
	name = benches;
	config = Criterion::default();
	targets = collector
}

criterion_main!(benches);
