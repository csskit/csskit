---
layout: markdown-base
title: "Performance Benchmarks"
description: "Benchmark data for csskit across different CSS frameworks and libraries"
css:
  - "benchmarks"
script:
  - "benchmarks"
---

# Performance Benchmarks

> [!NOTE]
> Benchmarks are automatically collected on every commit to the main branch
> using:
>
> - [Hyperfine](https://github.com/sharkdp/hyperfine): Command-line benchmarking
>   tool for measuring csskit's end-to-end processing time on real CSS files.
> - [Criterion.rs](https://github.com/bheisler/criterion.rs): Statistical
>   micro-benchmarking for precise measurements of core parsing operations.
>
> This is not meant as a comparison or value judgement of any css frameworks!
> They're used to gauge speed for this tool, nothing more.
>
> All benchmarks run on GitHub Actions. This can result in varied performance
> depending on contention.

<!-- markdownlint-disable -->

{% assign latest = benchmark-history[0] %}
{% assign total_files = latest.hyperfine_results.size %}
{% assign total_criterion = latest.criterion_results.size %}
{% assign avg_compression = 0 %}
{% assign compression_count = 0 %}
{% assign avg_throughput = 0 %}
{% assign throughput_count = 0 %}
{% for result in latest.hyperfine_results %}
{% assign data = result[1] %}
{% unless data.error %}
{% assign avg_compression = avg_compression | plus: data.compression_ratio %}
{% assign compression_count = compression_count | plus: 1 %}
{% assign mean_time = data.results[0].mean %}
{% assign throughput = data.input_size | divided_by: mean_time | divided_by: 1048576.0 %}
{% assign avg_throughput = avg_throughput | plus: throughput %}
{% assign throughput_count = throughput_count | plus: 1 %}
{% endunless %}
{% endfor %}

{% if compression_count > 0 %}
{% assign avg_compression = avg_compression | divided_by: compression_count %}
{% assign avg_compression_percent = 1 | minus: avg_compression | times: 100 %}
{% endif %}

{% if throughput_count > 0 %}
{% assign avg_throughput = avg_throughput | divided_by: throughput_count %}
{% endif %}

<summary>
	<div>
		<h2>CSS Files Tested</h2>
		<span>{{ total_files }}</span>
	</div>
	<div>
		<h2>Micro-benchmarks</h2>
		<span>{{ total_criterion }}</span>
	</div>
	<div>
		<h2>Avg. Size Reduction</h2>
		<span>{{ avg_compression_percent | round: 1 }}%</span>
	</div>
	<div>
		<h2>Avg. Throughput</h2>
		<span>{{ avg_throughput | round: 1 }} MB/s</span>
	</div>
</summary>

## Latest Results

<summary>
	<div>
		<h3>Last run</h3>
		<span>{{ latest.timestamp | date: "%B %d at %H:%M UTC" }}</span>
	</div>
	<div>
		<h3>Commit</h3>
		<span>
			<a href="https://github.com/csskit/csskit/commit/{{ latest.git_commit }}">
				<code>{{ latest.git_commit | slice: 0, 8 }}</code>
			</a>
		</span>
	</div>
</summary>
<!-- markdownlint-enable -->

### CSS Processing Performance

Hyperfine timing results for processing popular CSS frameworks with csskit min
command.

<!-- markdownlint-disable -->
<table>
	<thead>
		<tr>
			<th>File</th>
			<th>Processing Time</th>
			<th>Throughput</th>
			<th>Input Size</th>
			<th>Output Size</th>
			<th>Compression</th>
		</tr>
	</thead>
	<tbody>
		{% for result in latest.hyperfine_results %}
			{% assign file = result[0] %}
			{% assign data = result[1] %}

    		<tr>
    			<td>{{ file }}</td>
    			{% if data.error %}
    				<td class="error-cell" colspan="5">
    					<span class="error-text">❌ Benchmark failed</span>
    				</td>
    			{% elsif data.output_size == 0 %}
    				{% assign mean_time = data.results[0].mean %}
    				{% assign stddev = data.results[0].stddev %}
    				{% assign throughput = data.input_size | divided_by: mean_time | divided_by: 1048576.0 %}
    				{% assign input_kb = data.input_size | divided_by: 1024.0 %}

    				<td class="secondary">
    					<span class="primary">{{ mean_time | times: 1000 | round: 2 }}ms</span>
    					<span class="secondary">±{{ stddev | times: 1000 | round: 2 }}ms</span>
    				</td>
    				<td class="primary">{{ throughput | round: 1 }} MB/s</td>
    				<td>{{ input_kb | round: 1 }}KB</td>
    				<td>-</td>
    				<td class="parse-error">
    					<span class="error-text">❌ Parse failed</span>
    				</td>
    			{% else %}
    				{% assign mean_time = data.results[0].mean %}
    				{% assign stddev = data.results[0].stddev %}
    				{% assign throughput = data.input_size | divided_by: mean_time | divided_by: 1048576.0 %}
    				{% assign compression_percent = 1 | minus: data.compression_ratio | times: 100 %}
    				{% assign input_kb = data.input_size | divided_by: 1024.0 %}
    				{% assign output_kb = data.output_size | divided_by: 1024.0 %}

    				<td class="secondary">
    					<span class="primary">{{ mean_time | times: 1000 | round: 2 }}ms</span>
    					<span class="secondary">±{{ stddev | times: 1000 | round: 2 }}ms</span>
    				</td>
    				<td class="primary">{{ throughput | round: 1 }} MB/s</td>
    				<td>{{ input_kb | round: 1 }}KB</td>
    				<td>{{ output_kb | round: 1 }}KB</td>
    				<td class="compression-cell">
    					<span class="primary">{{ compression_percent | round: 1 }}%</span>
    					<span class="secondary">smaller</span>
    				</td>
    			{% endif %}
    		</tr>
    	{% endfor %}
    </tbody>

</table>
<!-- markdownlint-enable -->

## Historical Trends

{% assign history_count = benchmark-history | size %}

Performance data collected across {{ history_count }} benchmark runs.

### Processing Time Trends

**What this measures:** End-to-end processing time for minifying real CSS files
from popular frameworks using the `csskit min` command.

This chart tracks how long it takes csskit to completely process various CSS
files, including reading, parsing, minifying, and writing the output. Smaller
is better.

[We can perceive changes in as little as 13000 microseconds (13ms)](https://news.mit.edu/2014/in-the-blink-of-an-eye-0116),
anything under that would be considered "instant". csskit aims to minify most
content in under that time.

<div id="processing-time-chart" style="width: 100%; height: 400px;"></div>

### Throughput Trends

**What this measures:** Data processing speed in megabytes per second (MB/s) -
how much CSS csskit can process per unit of time.

This chart tracks how much css data csskit can read per second. The lines in
this chart should be closer together than processing time proving that
performance is consistent between different css, and there aren't performance
weak spots or bottlenecks depending on the css content.

Lines closer together means more consistent performance. Lines further parts
means unstable performance. Ideally all lines would be within 1mbps of each
other.

<div id="throughput-chart" style="width: 100%; height: 400px;"></div>

### Compression Ratio Trends

**What this measures:** How much smaller the output CSS files are compared to
their original size after minification.

This shows csskit's effectiveness at reducing file sizes through minification
techniques like removing whitespace, shortening values, and eliminating
redundant rules. Higher percentages mean better compression. 100% would mean the
minified file is half the size of the authored file, while 0% means it did not
compress it at all. Some of the benchmark files are already minified, which
helps to determine if csskit is compressing better than industry standard tools.

<div id="compression-chart" style="width: 100%; height: 400px;"></div>

### Criterion Benchmark Trends

These micro-benchmarks isolate specific parts of csskit's processing pipeline to
identify performance bottlenecks and validate optimizations at a granular level.

#### Parsing Performance

**What this measures:** Time spent converting CSS text into csskit's internal
Abstract Syntax Tree (AST) representation.

Parsing is a critical step that happens before any transformations. This
benchmark measures the core parsing engine's speed on real-world CSS files.
Optimizations here improve performance for all csskit operations since parsing
is always the first step.

<div id="criterion-parse-chart" style="width: 100%; height: 400px;"></div>

#### Lexing Performance

**What this measures:** Time spent breaking CSS text into tokens (keywords,
identifiers, values, etc.) - the first step of CSS processing.

Lexing (tokenization) is the foundation of CSS processing. The lexer must scan
every character and classify them into meaningful tokens. Lexer performance
directly impacts all downstream operations.

<div id="criterion-lex-chart" style="width: 100%; height: 400px;"></div>

#### Minification Performance

**What this measures:** Time spent applying minification transformations to already-parsed CSS.

This isolates the minification logic from parsing overhead, measuring how efficiently csskit can compress CSS once it's already in memory. Improvements here make the `csskit min` command faster without affecting other operations.

<div id="criterion-minify-chart" style="width: 100%; height: 400px;"></div>

#### String Parsing by Length

**What this measures:** How parsing performance scales with input size across different CSS string lengths.

This synthetic benchmark tests csskit's algorithmic efficiency by measuring parsing time across various input sizes. It helps identify whether the parser has good O(n) linear scaling or if there are performance cliffs at certain input sizes.

<div id="criterion-fromstr-chart" style="width: 100%; height: 400px;"></div>

<script>
// Make benchmark data available to JavaScript
window.benchmarkData = {{ benchmark-history | jsonify }};
</script>
