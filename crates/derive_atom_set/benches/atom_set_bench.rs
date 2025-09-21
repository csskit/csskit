use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use css_ast::{AtomSet, CssAtomSet};

fn bench(c: &mut Criterion) {
	let mut group = c.benchmark_group("from_str_by_length");
	group.sample_size(10);
	group.measurement_time(std::time::Duration::from_millis(1000));
	group.warm_up_time(std::time::Duration::from_millis(500));

	// Test cases grouped by length and lookup strategy
	let test_cases = vec![
		// Short strings (tuple matching)
		("1_char", vec!["a", "b", "c", "d", "e"]),
		("2_char", vec!["px", "em", "ex", "pt", "pc", "pp", "qq"]),
		("3_char", vec!["dpx", "dpi", "dpc", "ppx", "swi", "qqq"]),
		("4_char", vec!["auto", "none", "left", "flex", "grid", "blob", "dont"]),
		("5_char", vec!["block", "table", "fixed", "style", "missd", "match"]),
		// Medium strings (u64 lookup)
		("6_char", vec!["medium", "normal", "center", "static", "border", "nmatch"]),
		("7_char", vec!["display", "content", "padding", "outline", "inherit", "nomatch", "missing"]),
		("8_char", vec!["position", "absolute", "relative", "missings", "nomatchs"]),
		// Longer strings (u128 lookup)
		("10_char", vec!["background", "flex-start", "text-align"]),
		("12_char", vec!["border-width", "border-style", "border-color"]),
		("14_char", vec!["justify-content", "align-content", "grid-template"]),
		("16_char", vec!["text-decoration", "border-radius"]),
		// Very long strings (>16 chars) - multi-u128 lookup
		(
			"very_long",
			vec![
				"-webkit-autofill-strong-password-viewable",
				"this-is-a-very-long-string-that-does-not-match-anything",
			],
		),
		("mixed_case", vec!["background", "BACKGROUND", "BackGround", "fLeX-dIrEcTiOn"]),
	];

	for (category, strings) in test_cases {
		group.bench_with_input(BenchmarkId::new("lookup", category), &strings, |b, strings| {
			b.iter(|| {
				for &s in strings {
					black_box(CssAtomSet::from_str(black_box(s)));
				}
			});
		});
	}

	group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
