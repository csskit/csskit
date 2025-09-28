import ApexCharts from "apexcharts";

document.addEventListener("DOMContentLoaded", function () {
	if (window.benchmarkData && window.benchmarkData.length > 1) {
		initializeCharts();
	}
	setupInteractiveElements();
});

let charts = [];
const PERFORMANCE_THRESHOLDS = {
	processing_time: {
		good: 13000, // Under 13ms is good
	},
	parsing: {
		good: 5000, // Under 5ms is good
		warning: 10000, // 5-10ms is warning
	},
	lexing: {
		good: 2000, // Under 2ms is good
		warning: 5000, // 2-5ms is warning
	},
	minification: {
		good: 3000, // Under 3ms is good
		warning: 8000, // 3-8ms is warning
	},
};

function getChartTheme() {
	const isDarkMode = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
	if (isDarkMode) {
		return {
			mode: "dark",
			palette: "palette4",
		};
	} else {
		return {
			mode: "light",
		};
	}
}

function getChartOptions() {
	const isDarkMode = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
	return {
		animations: {
			enabled: false,
		},
		background: isDarkMode ? "#0f0f0f" : "#ffffff",
	};
}

function getGridBorderColor() {
	const isDarkMode = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
	return isDarkMode ? "rgba(255, 255, 255, 0.2)" : "rgba(0, 0, 0, 0.2)";
}

function addThresholdAnnotations(thresholds, yAxisMax) {
	if (!thresholds) return [];

	const annotations = [];
	const isDarkMode = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;

	// Add good threshold line
	if (thresholds.good !== undefined) {
		annotations.push({
			y: thresholds.good,
			borderColor: isDarkMode ? "#22c55e" : "#16a34a", // green-500/600
			borderWidth: 2,
			strokeDashArray: 5,
			label: {
				text: "Good",
				position: "right",
				style: {
					color: isDarkMode ? "#22c55e" : "#16a34a",
					background: "transparent",
					fontSize: "12px",
				},
			},
		});
	}

	// Add warning threshold line
	if (thresholds.warning !== undefined) {
		annotations.push({
			y: thresholds.warning,
			borderColor: isDarkMode ? "#f59e0b" : "#d97706", // amber-500/600
			borderWidth: 2,
			strokeDashArray: 5,
			label: {
				text: "Warning",
				position: "right",
				style: {
					color: isDarkMode ? "#f59e0b" : "#d97706",
					background: "transparent",
					fontSize: "12px",
				},
			},
		});
	}

	// Add bad threshold line (for throughput where low is bad)
	if (thresholds.bad !== undefined) {
		annotations.push({
			y: thresholds.bad,
			borderColor: isDarkMode ? "#ef4444" : "#dc2626", // red-500/600
			borderWidth: 2,
			strokeDashArray: 5,
			label: {
				text: "Poor",
				position: "right",
				style: {
					color: isDarkMode ? "#ef4444" : "#dc2626",
					background: "transparent",
					fontSize: "12px",
				},
			},
		});
	}

	return annotations;
}

function getBaseChartConfig(title, yAxisTitle, yAxisOptions = {}, thresholds = null) {
	const fontSizes = {
		title: "18px",
		subtitle: "16px",
		axis: "14px",
		legend: "14px",
		tooltip: "14px",
	};

	const config = {
		chart: {
			type: "line",
			height: 400,
			zoom: { enabled: true },
			toolbar: { show: true },
			...getChartOptions(),
		},
		dataLabels: { enabled: false },
		stroke: { curve: "smooth", width: 2 },
		title: {
			text: title,
			align: "left",
			style: {
				fontSize: fontSizes.title,
			},
		},
		grid: {
			borderColor: getGridBorderColor(),
		},
		xaxis: {
			title: {
				text: "Commit",
				style: { fontSize: fontSizes.axis },
			},
			type: "category",
			labels: {
				style: { fontSize: fontSizes.axis },
			},
		},
		yaxis: {
			title: {
				text: yAxisTitle,
				style: { fontSize: fontSizes.axis },
			},
			labels: {
				style: { fontSize: fontSizes.axis },
			},
			...yAxisOptions,
		},
		legend: {
			position: "bottom",
			offsetY: 0,
			fontSize: fontSizes.legend,
		},
		tooltip: {
			shared: true,
			intersect: false,
			style: {
				fontSize: fontSizes.tooltip,
			},
		},
		theme: getChartTheme(),
	};

	// Add threshold annotations if provided
	if (thresholds) {
		const yMax = yAxisOptions.max || 100000; // Default high value if no max specified
		const thresholdAnnotations = addThresholdAnnotations(thresholds, yMax);
		if (thresholdAnnotations.length > 0) {
			config.annotations = {
				yaxis: thresholdAnnotations,
			};
		}
	}

	return config;
}

function getCriterionChartConfig(
	title,
	yAxisTitle,
	minVal,
	maxVal,
	padding,
	decimalPlaces,
	unitSuffix,
	thresholds = null,
) {
	const yAxisOptions = {
		min: minVal - padding,
		max: maxVal + padding,
		decimalsInFloat: decimalPlaces,
	};

	const config = getBaseChartConfig(title, yAxisTitle, yAxisOptions, thresholds);
	config.stroke.width = 3;
	config.markers = {
		size: 4,
		hover: { size: 6 },
	};
	config.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined") {
				return y.toFixed(decimalPlaces) + unitSuffix;
			}
			return y;
		},
	};

	return config;
}

function setupThemeObserver() {
	const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

	mediaQuery.addEventListener("change", (e) => {
		charts.forEach((chart) => {
			if (chart) {
				chart.updateOptions({
					theme: getChartTheme(),
					grid: {
						borderColor: getGridBorderColor(),
					},
					chart: getChartOptions(),
				});
			}
		});
	});
}

function initializeCharts() {
	const data = window.benchmarkData;

	if (data.length < 2) return;

	// Create all charts
	createTimeChart(data);
	createCompressionChart(data);
	createThroughputChart(data);
	createCriterionParseChart(data);
	createCriterionLexChart(data);
	createCriterionMinifyChart(data);
	createCriterionFromStrChart(data);

	setupThemeObserver();
}

function createTimeChart(data) {
	const chartElement = document.getElementById("processing-time-chart");
	if (!chartElement) return;
	const allFiles = Object.keys(data[0].hyperfine_results);
	const series = allFiles.map((file) => {
		const dataPoints = data
			.map((entry, index) => {
				const fileData = entry.hyperfine_results[file];
				if (fileData && fileData.results && fileData.results[0]) {
					return {
						x: `${entry.git_commit.slice(0, 8)}-${index}`,
						y: Math.round(fileData.results[0].mean * 1000000), // microseconds
					};
				}
				return null;
			})
			.filter((point) => point !== null);
		return {
			name: file.replace(/\./g, " ").replace(/-/g, " "),
			data: dataPoints,
		};
	});
	const options = {
		series: series,
		...getBaseChartConfig("Processing Time Trends", "Microseconds", { min: 0 }, PERFORMANCE_THRESHOLDS.processing_time),
	};
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined") {
				return y.toFixed(0) + "μs";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();
}

function createCompressionChart(data) {
	const chartElement = document.getElementById("compression-chart");
	if (!chartElement) return;
	const allFiles = Object.keys(data[0].hyperfine_results);
	const series = allFiles
		.map((file) => {
			const dataPoints = data
				.map((entry, index) => {
					const fileData = entry.hyperfine_results[file];
					if (fileData && fileData.compression_ratio && fileData.output_size > 0) {
						const compressionPercent = (1 - fileData.compression_ratio) * 100;
						return {
							x: `${entry.git_commit.slice(0, 8)}-${index}`,
							y: Math.round(compressionPercent * 10) / 10, // 1dp
						};
					}
					return null;
				})
				.filter((point) => point !== null);

			return {
				name: file.replace(/\./g, " ").replace(/-/g, " "),
				data: dataPoints,
			};
		})
		.filter((series) => series.data.length > 0);
	const options = {
		series: series,
		...getBaseChartConfig("Compression Ratio Trends", "Percent", { min: 0, max: 100 }),
	};
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined") {
				return y.toFixed(1) + "% smaller";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();
}

function createThroughputChart(data) {
	const chartElement = document.getElementById("throughput-chart");
	if (!chartElement) return;
	const allFiles = Object.keys(data[0].hyperfine_results);
	const series = allFiles
		.map((file) => {
			const dataPoints = data
				.map((entry, index) => {
					const fileData = entry.hyperfine_results[file];
					if (fileData && fileData.results && fileData.results[0] && fileData.input_size) {
						// Calculate throughput: input_size (bytes) / time (seconds) = bytes/sec -> MB/s
						const throughputMBps = fileData.input_size / fileData.results[0].mean / (1024 * 1024);
						return {
							x: `${entry.git_commit.slice(0, 8)}-${index}`,
							y: Math.round(throughputMBps * 10) / 10, // 1dp
						};
					}
					return null;
				})
				.filter((point) => point !== null);

			return {
				name: file.replace(/\./g, " ").replace(/-/g, " "),
				data: dataPoints,
			};
		})
		.filter((series) => series.data.length > 0);
	const options = {
		series: series,
		...getBaseChartConfig(
			"Processing Throughput Trends",
			"Megabytes per Second",
			{ min: 0 },
			PERFORMANCE_THRESHOLDS.throughput,
		),
	};
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined") {
				return y.toFixed(1) + " MB/s";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();
}

function createCriterionParseChart(data) {
	createCriterionGroupChart(
		data,
		"parse_popular",
		"criterion-parse-chart",
		"Parsing Performance",
		"Microseconds",
		1000,
		1,
		PERFORMANCE_THRESHOLDS.parsing,
	);
}

function createCriterionLexChart(data) {
	createCriterionGroupChart(
		data,
		"lex_popular",
		"criterion-lex-chart",
		"Lexing Performance",
		"Microseconds",
		1000,
		1,
		PERFORMANCE_THRESHOLDS.lexing,
	);
}

function createCriterionMinifyChart(data) {
	createCriterionGroupChart(
		data,
		"minify_popular",
		"criterion-minify-chart",
		"Minification Performance",
		"Microseconds",
		1000,
		1,
		PERFORMANCE_THRESHOLDS.minification,
	);
}

function createCriterionFromStrChart(data) {
	createCriterionGroupChart(
		data,
		"from_str_by_length",
		"criterion-fromstr-chart",
		"String Parsing by Length",
		"Nanoseconds",
		1,
		0,
	);
}

function createCriterionGroupChart(
	data,
	groupPrefix,
	chartId,
	title,
	yAxisTitle,
	conversionFactor = 1000,
	decimalPlaces = 1,
	thresholds = null,
) {
	const chartElement = document.getElementById(chartId);
	if (!chartElement) return;
	const criterionBenchmarks = Object.keys(data[0].criterion_results).filter((benchmark) =>
		benchmark.startsWith(groupPrefix + "/"),
	);
	if (criterionBenchmarks.length === 0) return;
	const series = criterionBenchmarks
		.map((benchmark) => {
			const dataPoints = data
				.map((entry, index) => {
					const benchmarkData = entry.criterion_results[benchmark];
					if (benchmarkData && benchmarkData.mean && benchmarkData.mean.point_estimate) {
						// Convert nanoseconds using the provided conversion factor and precision
						const convertedTime = benchmarkData.mean.point_estimate / conversionFactor;
						const roundedTime = Math.round(convertedTime * Math.pow(10, decimalPlaces)) / Math.pow(10, decimalPlaces);

						return {
							x: `${entry.git_commit.slice(0, 8)}-${index}`, // Add index to ensure uniqueness
							y: roundedTime,
						};
					}
					return null;
				})
				.filter((point) => point !== null);

			return {
				name: benchmark
					.replace(groupPrefix + "/", "")
					.replace(/\./g, " ")
					.replace(/-/g, " "),
				data: dataPoints,
			};
		})
		.filter((series) => series.data.length > 0);
	let allValues = [];
	series.forEach((s) => {
		s.data.forEach((point) => allValues.push(point.y));
	});
	const minVal = Math.min(0, ...allValues);
	const maxVal = Math.max(...allValues);
	const range = maxVal - minVal;
	const padding = range * 0.1; // Add 10% padding
	console.log(`${groupPrefix} chart data:`, {
		seriesCount: series.length,
		valueCount: allValues.length,
		minVal,
		maxVal,
		range,
		sampleSeries: series[0]?.name,
		sampleData: series[0]?.data,
	});

	// Determine the unit suffix for tooltips
	const unitSuffix = yAxisTitle.includes("ns")
		? "ns"
		: yAxisTitle.includes("μs")
			? "μs"
			: yAxisTitle.includes("ms")
				? "ms"
				: "";

	const options = {
		series: series,
		...getCriterionChartConfig(title, yAxisTitle, minVal, maxVal, padding, decimalPlaces, unitSuffix, thresholds),
	};

	const chart = new ApexCharts(chartElement, options);
	charts.push(chart); // Store for theme updates
	chart.render();
}

function setupInteractiveElements() {
	// Show all criterion results
	window.showAllCriterion = function () {
		const hiddenRows = document.querySelectorAll(".criterion-hidden");
		hiddenRows.forEach((row) => {
			row.classList.remove("criterion-hidden");
		});

		const moreLink = document.querySelector(".more-results");
		if (moreLink) {
			moreLink.style.display = "none";
		}
	};

	// Add hover effects to table rows
	const tableRows = document.querySelectorAll(".results-table tbody tr, .criterion-table tbody tr");
	tableRows.forEach((row) => {
		row.addEventListener("mouseenter", function () {
			this.style.transform = "scale(1.01)";
			this.style.transition = "transform 0.1s ease-in-out";
		});

		row.addEventListener("mouseleave", function () {
			this.style.transform = "scale(1)";
		});
	});
}
