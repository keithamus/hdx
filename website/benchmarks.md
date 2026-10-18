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
> This is not meant as a comparison, endorsement, or value judgement of any css
> frameworks, libraries or tools! They're used to gauge speed for this tool,
> nothing more.
>
> All benchmarks run on GitHub Actions. This can result in varied performance
> depending on contention. [There are some details about the hardware GitHub
> Actions run on](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
> but these numbers should be viewed as _relative performance_, and your hardware
> will likely be different.

## Contents

- [CSS Processing Performance](#css-processing-performance)
- [CSS Tool Comparison](#css-tool-comparison)
  - [Processing Time Comparison](#processing-time-comparison)
  - [Output Size Comparison](#output-size-comparison)
- [Historical Trends](#historical-trends)
  - [Processing Time Trends](#processing-time-trends)
  - [Throughput Trends](#throughput-trends)
  - [Compression Ratio Trends](#compression-ratio-trends)
  - [Criterion Benchmark Trends](#criterion-benchmark-trends)
    - [Parsing Performance](#parsing-performance)
    - [Lexing Performance](#lexing-performance)
    - [Minification Performance](#minification-performance)
    - [Linting Sheet Parsing Performance](#linting-sheet-parsing-performance)
    - [Selector Matching Performance](#selector-matching-performance)
    - [Linting Collection Performance](#linting-collection-performance)
    - [String Parsing by Length](#string-parsing-by-length)

<!-- markdownlint-disable -->

{%- assign latest = benchmark-history[0] -%}
{%- assign total_files = latest.hyperfine_results.size -%}
{%- assign total_criterion = latest.criterion_results.size -%}
{%- assign avg_compression = 0 -%}
{%- assign compression_count = 0 -%}
{%- assign avg_throughput = 0 -%}
{%- assign throughput_count = 0 -%}
{%- for result in latest.hyperfine_results -%}
{%- assign data = result[1] -%}
{%- unless data.error -%}
{%- assign avg_compression = avg_compression | plus: data.compression_ratio -%}
{%- assign compression_count = compression_count | plus: 1 -%}
{%- assign mean_time = data.results[0].summary.time_wall_clock.mean -%}
{%- assign throughput = data.input_size | divided_by: mean_time | divided_by: 1048576.0 -%}
{%- assign avg_throughput = avg_throughput | plus: throughput -%}
{%- assign throughput_count = throughput_count | plus: 1 -%}
{%- endunless -%}
{%- endfor -%}
{%- if compression_count > 0 -%}
{%- assign avg_compression = avg_compression | divided_by: compression_count -%}
{%- assign avg_compression_percent = 1 | minus: avg_compression | times: 100 -%}
{%- endif %}
{%- if throughput_count > 0 -%}
{%- assign avg_throughput = avg_throughput | divided_by: throughput_count -%}
{%- endif -%}

<div summary>
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
</div>
<!-- markdownlint-enable -->

### CSS Processing Performance

Hyperfine timing results for processing popular CSS frameworks with csskit min
command.

<div scroller>
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
		{%- for result in latest.hyperfine_results -%}
			{%- assign file = result[0] -%}
			{%- assign data = result[1] -%}

    		<tr>
    			<td>{{ file }}</td>
    			{% if data.error %}
    				<td class="error-cell" colspan="5">
    					<span class="error-text">❌ Benchmark failed</span>
    				</td>
    			{% elsif data.output_size == 0 %}
    				{% assign mean_time = data.results[0].summary.time_wall_clock.mean %}
    				{% assign stddev = data.results[0].summary.time_wall_clock.stddev %}
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
    				{% assign mean_time = data.results[0].summary.time_wall_clock.mean %}
    				{% assign stddev = data.results[0].summary.time_wall_clock.stddev %}
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
</div>
<!-- markdownlint-enable -->

## CSS Tool Comparison

This comparison shows how csskit performs relative to other widely-used CSS
minification tools:

- **lightningcss**: Fast native CSS processor built with Rust
- **cssnano**: Popular PostCSS-based minifier
- **esbuild**: Fast JavaScript/CSS bundler with minification

### Processing Time Comparison

**What this measures:** End-to-end processing time for minifying real CSS files
from popular frameworks using the tools minify command.

This chart tracks how long it takes for popular css tools to minify various CSS
files, compared with each other. Smaller bars are better.

<div id="alternatives-time-chart" style="width: 100%; height: 500px;"></div>

### Output Size Comparison

**What this measures:** Size of the resulting minified CSS files from popular
frameworks using the tools minify command.

This chart tracks how small each tool can compress a CSS file by. Smaller file
sizes, represented by smaller bars, are better.

<div id="alternatives-size-chart" style="width: 100%; height: 500px;"></div>

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
means unstable performance. Ideally all lines would be within a few mbps of each
other. There may be some outliers though: very small files tend to be a low
slower (due to repeated memory drops after each parse).

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
is always the first step. Lower numbers are better.

<div id="criterion-parse-chart" style="width: 100%; height: 400px;"></div>

#### Lexing Performance

**What this measures:** Time spent breaking CSS text into tokens (keywords,
identifiers, values, etc.) - the first step of CSS processing.

Lexing (tokenization) is the foundation of CSS processing. The lexer must scan
every character and classify them into meaningful tokens. Lexer performance
directly impacts all downstream operations.

The target is to have most files lex in under 1.5ms (1500 microseconds).

<div id="criterion-lex-chart" style="width: 100%; height: 400px;"></div>

#### Minification Performance

**What this measures:** Time spent applying minification transformations to
already-parsed CSS.

This isolates the minification logic from parsing overhead, measuring how
efficiently csskit can compress CSS once it's already in memory. Improvements
here make the `csskit min` command faster without affecting other operations.

<div id="criterion-minify-chart" style="width: 100%; height: 400px;"></div>

#### Linting Sheet Parsing Performance

**What this measures:** Time spent parsing csskit linting rule files (.cks) into
the internal query representation.

Linting rules use a CSS-like syntax to define selectors and diagnostics. This
benchmark measures how quickly csskit can parse these rule definitions before
applying them to CSS files. Lower numbers are better.

<div id="criterion-parse-sheet-chart" style="width: 100%; height: 400px;"></div>

#### Selector Matching Performance

**What this measures:** Time spent matching CSS selectors from linting rules
against parsed CSS AST nodes.

Selector matching is the core of the linting engine - it identifies which CSS
nodes match the query selectors defined in linting rules. This benchmark
measures the efficiency of the matching algorithm, which needs to traverse the
AST and evaluate complex selectors. Lower numbers are better.

<div id="criterion-selector-matcher-chart" style="width: 100%; height: 400px;"></div>

#### Linting Collection Performance

**What this measures:** End-to-end time for collecting statistics and generating
diagnostics from linting rules applied to CSS files.

This benchmark measures the complete linting pipeline: parsing rules, matching
selectors, collecting statistics, evaluating conditions, and generating
diagnostics. It represents the real-world performance of csskit's linting
capabilities. Lower numbers are better.

<div id="criterion-collector-chart" style="width: 100%; height: 400px;"></div>

#### String Parsing by Length

**What this measures:** The time it takes for csskit to look up a keyword. CSS
has a lot of keywords for a programming language, with close to 2,500
different keywords like `width`, `inherit`, `red`, and so on. An important
but small aspect of parsing is determining one keyword from another.

Comparing strings between a list of 2500 possibilities would take too long so
instead csskit performs some tricks to speed this up. Extracting a keyword
should take mere nanoseconds, and should be roughly consistent for keywords of
any length. The target is under 20ns per lookup, regardless of the length.

<div id="criterion-fromstr-chart" style="width: 100%; height: 400px;"></div>

<script>
// Make benchmark data available to JavaScript
window.benchmarkData = {{ benchmark-history | jsonify }};
</script>
