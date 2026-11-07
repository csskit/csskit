import { format, niceScale, xAxisTicks, yAxisStyle } from "./lib/chart-utils.js";

const MAX_POINTS = 30;

const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

function dateLabel(timestamp) {
	const date = new Date(timestamp);
	return `${String(date.getUTCDate()).padStart(2, "0")} ${MONTHS[date.getUTCMonth()]}`;
}

function round(value, decimals) {
	const scale = 10 ** decimals;
	return Math.round(value * scale) / scale;
}

function slug(name) {
	return name.replace(/[^\w-]+/g, "-");
}

function sampleByDay(history) {
	const byDay = new Map();
	for (const entry of [...history].reverse()) byDay.set(entry.timestamp.slice(0, 10), entry);
	const days = [...byDay.values()];
	if (days.length <= MAX_POINTS) return days;
	const step = (days.length - 1) / (MAX_POINTS - 1);
	return Array.from({ length: MAX_POINTS }, (_, i) => days[Math.round(i * step)]);
}

function collectSeries(entries, keys, value) {
	return keys
		.map((key) => ({ key, name: key, values: entries.map((entry) => value(entry, key)) }))
		.filter((series) => series.values.some((value) => value !== undefined));
}

function unionKeys(entries, field, prefix = "") {
	const keys = new Set();
	for (const entry of entries) {
		for (const key of Object.keys(entry[field] ?? {})) if (key.startsWith(prefix)) keys.add(key);
	}
	return [...keys].sort((a, b) => a.localeCompare(b, "en", { numeric: true }));
}

function splitOutliers(series) {
	const all = series.flatMap((line) => line.values.filter((value) => value !== undefined)).sort((a, b) => a - b);
	if (all.length < 3) return [series, []];
	const lowerQuartile = all[Math.floor(all.length * 0.25)];
	const upperQuartile = all[Math.floor(all.length * 0.75)];
	const spread = 3 * (upperQuartile - lowerQuartile);
	const isOutlier = (line) =>
		line.values.some((value) => value !== undefined && (value < lowerQuartile - spread || value > upperQuartile + spread));
	return [series.filter((line) => !isOutlier(line)), series.filter(isOutlier)];
}

function lineChart({ id, caption, note, labels, series, decimals, unit, fromZero = false, thresholds = [] }) {
	const values = series.flatMap((line) => line.values.filter((value) => value !== undefined));
	const { min, max } = niceScale(fromZero ? 0 : Math.min(...values), Math.max(...values), 4);
	const style = Object.entries(yAxisStyle(min, max, unit, thresholds)).map(([prop, value]) => `${prop}:${value}`);
	const named = series.map((line) => ({ ...line, id: `${id}-${slug(line.name)}` }));
	return {
		caption,
		note,
		unit,
		fromZero,
		thresholds: JSON.stringify(thresholds),
		style: style.join(";"),
		series: named,
		rows: labels.map((label, i) => ({
			label,
			cells: named.map((line) => {
				const current = line.values[i];
				const previous = line.values[i - 1];
				if (current === undefined) return { headers: line.id, name: line.name };
				return {
					headers: line.id,
					name: line.name,
					style: previous === undefined ? `--b:${current}` : `--a:${previous};--b:${current}`,
					text: format(current, decimals, unit),
				};
			}),
		})),
	};
}

function trendCharts({ id, caption, note, entries, series, ...options }) {
	const labels = entries.map((entry) => dateLabel(entry.timestamp));
	const [normal, outliers] = splitOutliers(series);
	const charts = [];
	if (normal.length) charts.push(lineChart({ id, caption, note, labels, series: normal, ...options }));
	if (outliers.length) {
		charts.push(
			lineChart({
				id: `${id}-outliers`,
				caption: `${caption}, outliers`,
				note: "Kept apart so they don't flatten the chart above.",
				labels,
				series: outliers,
				...options,
			}),
		);
	}
	return charts;
}

function wallClockMean(result) {
	return result?.results?.[0]?.summary?.time_wall_clock?.mean;
}

function criterionSeries(entries, prefix, conversion, decimals, keys = unionKeys(entries, "criterion_results", prefix)) {
	return collectSeries(entries, keys, (entry, key) => {
		const estimate = entry.criterion_results?.[key]?.mean?.point_estimate;
		return estimate ? round(estimate / conversion, decimals) : undefined;
	}).map((line) => ({ ...line, name: line.key.slice(prefix.length) }));
}

function criterionCharts(entries, { id, prefix, caption, unit, conversion, decimals, thresholds }) {
	return trendCharts({
		id,
		caption,
		note: `Mean time per run, in ${unit.name}. Lower is better.`,
		entries,
		series: criterionSeries(entries, prefix, conversion, decimals),
		decimals,
		unit: unit.suffix,
		fromZero: true,
		thresholds,
	});
}

function queryGroups(entries, { id, prefix, caption, unit, conversion, decimals }) {
	const keys = unionKeys(entries, "criterion_results", prefix);
	const queries = [...new Set(keys.map((key) => key.slice(prefix.length).replace(/^[^_]+_/, "")))].sort();
	return queries.map((query) => ({
		name: query,
		charts: trendCharts({
			id: `${id}-${slug(query)}`,
			caption: `${caption}: ${query}`,
			note: `Mean time per run, in ${unit.name}. Lower is better.`,
			entries,
			series: criterionSeries(
				entries,
				prefix,
				conversion,
				decimals,
				keys.filter((key) => key.endsWith(`_${query}`)),
			).map((line) => ({ ...line, name: line.name.slice(0, -query.length - 1) })),
			decimals,
			unit: unit.suffix,
			fromZero: true,
		}),
	}));
}

function comparisonChart(latest, { id, caption, note, value, decimals, unit }) {
	const files = Object.entries(latest?.alternative_tools?.files ?? {}).filter(([, file]) => file.tools);
	const tools = [...new Set(files.flatMap(([, file]) => Object.keys(file.tools)))];
	const series = tools.map((tool) => ({ id: `${id}-${slug(tool)}`, name: tool }));
	const rows = files.map(([name, file]) => ({
		label: name,
		cells: series.map((bar) => {
			const tool = file.tools[bar.name];
			const measured = tool && !tool.error ? value(tool) : undefined;
			if (!measured) return { headers: bar.id, name: bar.name };
			const rounded = round(measured, decimals);
			return {
				headers: bar.id,
				name: bar.name,
				value: rounded,
				style: `--v:${rounded}`,
				text: format(rounded, decimals, unit),
			};
		}),
	}));
	const values = rows.flatMap((row) => row.cells.filter((cell) => cell.value).map((cell) => cell.value));
	const { max } = niceScale(0, Math.max(...values), 5);
	return { caption, note, unit, style: `--max:${max}`, ticks: xAxisTicks(max, unit), series, rows };
}

const MICROSECONDS = { name: "microseconds", suffix: " µs" };
const NANOSECONDS = { name: "nanoseconds", suffix: " ns" };

export default {
	eleventyComputed: {
		benchmarkCharts(data) {
			const history = data["benchmark-history"];
			if (!history?.length) return {};
			const entries = sampleByDay(history);
			const files = unionKeys(entries, "hyperfine_results");
			const hyperfine = (value) => collectSeries(entries, files, (entry, file) => value(entry.hyperfine_results[file]));
			const throughput = (result) => {
				const mean = wallClockMean(result);
				return mean && result.input_size ? round(result.input_size / mean / 1048576, 1) : undefined;
			};
			const throughputHistory = history
				.flatMap((entry) => Object.values(entry.hyperfine_results ?? {}).map(throughput))
				.filter((value) => value !== undefined)
				.sort((a, b) => a - b);

			return {
				comparisonTime: comparisonChart(history[0], {
					id: "alternatives-time",
					caption: "Processing time comparison",
					note: "Mean time to minify each file, in milliseconds. Shorter is better.",
					value: (tool) => wallClockMean(tool) * 1000,
					decimals: 2,
					unit: " ms",
				}),
				comparisonSize: comparisonChart(history[0], {
					id: "alternatives-size",
					caption: "Output size comparison",
					note: "Size of each minified file, in kilobytes. Shorter is better.",
					value: (tool) => tool.output_size / 1024,
					decimals: 1,
					unit: " KB",
				}),
				processingTime: trendCharts({
					id: "processing-time",
					caption: "Processing time trends",
					note: "Mean time for csskit min, in microseconds. Lower is better.",
					entries,
					series: hyperfine((result) => {
						const mean = wallClockMean(result);
						return mean ? Math.round(mean * 1e6) : undefined;
					}),
					decimals: 0,
					unit: " µs",
					thresholds: [{ value: 13000, label: "Good", color: "--ok" }],
				}),
				throughput: trendCharts({
					id: "throughput",
					caption: "Throughput trends",
					note: "Megabytes of CSS processed per second. Higher is better.",
					entries,
					series: hyperfine(throughput),
					decimals: 1,
					unit: " MB/s",
					thresholds: [
						{ value: throughputHistory[Math.floor(throughputHistory.length * 0.9)], label: "Good", color: "--ok" },
						{ value: throughputHistory[Math.floor(throughputHistory.length * 0.1)], label: "Poor", color: "--danger" },
					],
				}),
				compression: trendCharts({
					id: "compression",
					caption: "Compression ratio trends",
					note: "Percentage smaller than the input. Higher is better.",
					entries,
					series: hyperfine((result) =>
						result?.compression_ratio && result.output_size > 0
							? round((1 - result.compression_ratio) * 100, 1)
							: undefined,
					),
					decimals: 1,
					unit: "%",
				}),
				parse: criterionCharts(entries, {
					id: "criterion-parse",
					prefix: "parse_popular/",
					caption: "Parsing performance",
					unit: MICROSECONDS,
					conversion: 1000,
					decimals: 1,
					thresholds: [
						{ value: 5000, label: "Good", color: "--ok" },
						{ value: 10000, label: "Warning", color: "--warn" },
					],
				}),
				lex: criterionCharts(entries, {
					id: "criterion-lex",
					prefix: "lex_popular/",
					caption: "Lexing performance",
					unit: MICROSECONDS,
					conversion: 1000,
					decimals: 1,
					thresholds: [
						{ value: 1500, label: "Good", color: "--ok" },
						{ value: 5000, label: "Warning", color: "--warn" },
					],
				}),
				minify: criterionCharts(entries, {
					id: "criterion-minify",
					prefix: "minify_popular/",
					caption: "Minification performance",
					unit: MICROSECONDS,
					conversion: 1000,
					decimals: 1,
					thresholds: [
						{ value: 3000, label: "Good", color: "--ok" },
						{ value: 8000, label: "Warning", color: "--warn" },
					],
				}),
				parseSheet: criterionCharts(entries, {
					id: "criterion-parse-sheet",
					prefix: "parse_sheet/",
					caption: "Linting sheet parsing performance",
					unit: MICROSECONDS,
					conversion: 1000,
					decimals: 1,
				}),
				fromStr: criterionCharts(entries, {
					id: "criterion-fromstr",
					prefix: "from_str_by_length/",
					caption: "String parsing by length",
					unit: NANOSECONDS,
					conversion: 1,
					decimals: 0,
					thresholds: [
						{ value: 20, label: "Good", color: "--ok" },
						{ value: 40, label: "Warning", color: "--warn" },
					],
				}),
				selectorMatching: queryGroups(entries, {
					id: "criterion-selector-matcher",
					prefix: "selector_matching/",
					caption: "Selector matching",
					unit: MICROSECONDS,
					conversion: 1000,
					decimals: 1,
				}),
				collector: queryGroups(entries, {
					id: "criterion-collector",
					prefix: "collector/",
					caption: "Linting collection",
					unit: MICROSECONDS,
					conversion: 1000,
					decimals: 0,
				}),
			};
		},
	},
};
