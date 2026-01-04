import ApexCharts from "apexcharts";

document.addEventListener("DOMContentLoaded", function () {
	if (window.benchmarkData && window.benchmarkData.length > 1) {
		initializeCharts();
	}
});

let charts = [];
let selectedFile = null; // null means show all files

function refreshCharts() {
	// Destroy existing charts
	charts.forEach((chart) => {
		if (chart) {
			chart.destroy();
		}
	});
	charts = [];

	// Recreate charts with current filter
	initializeCharts();

	// Update UI to show current filter state
	updateFilterUI();
}

function updateFilterUI() {
	// Remove existing filter indicator
	const existingIndicator = document.getElementById("filter-indicator");
	if (existingIndicator) {
		existingIndicator.remove();
	}

	// Add filter indicator if a file is selected
	if (selectedFile) {
		const indicator = document.createElement("div");
		indicator.id = "filter-indicator";
		indicator.style.cssText = `
			position: fixed;
			top: 10px;
			right: 10px;
			background: #007bff;
			color: white;
			padding: 10px 15px;
			border-radius: 5px;
			font-size: 14px;
			font-weight: bold;
			box-shadow: 0 2px 10px rgba(0,0,0,0.3);
			z-index: 1000;
			cursor: pointer;
		`;
		indicator.innerHTML = `
			📊 Showing: ${selectedFile.replace(/\./g, " ").replace(/-/g, " ")}
			<br><small style="opacity: 0.9;">Click to show all files</small>
		`;
		indicator.addEventListener("click", () => {
			selectedFile = null;
			refreshCharts();
		});
		document.body.appendChild(indicator);
	}
}

const PERFORMANCE_THRESHOLDS = {
	processing_time: {
		good: 13000, // 13ms
	},
	parsing: {
		good: 5000, // 5ms
		warning: 10000, // 10ms
	},
	lexing: {
		good: 1500, // 2ms
		warning: 5000, // 5ms
	},
	minification: {
		good: 3000, // 3ms
		warning: 8000, // 8ms
	},
	from_str: {
		good: 20, // 20ns
		warning: 40, // 40ns
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

function addThresholdAnnotations(thresholds) {
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
		stroke: { curve: "straight", width: 2 },
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
				text: "Time",
				style: { fontSize: fontSizes.axis },
			},
			type: "category",
			labels: {
				show: false,
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
		min: Math.max(0, minVal - padding),
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
			if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
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
	createCriterionParseSheetChart(data);
	createCriterionSelectorMatcherChart(data);
	createCriterionCollectorChart(data);
	createCriterionFromStrChart(data);

	createComparisonTimeChart(data);
	createComparisonSizeChart(data);

	setupThemeObserver();
}

function createTimeChart(data) {
	const chartElement = document.getElementById("processing-time-chart");
	if (!chartElement) return;
	const allFiles = Object.keys(data[0].hyperfine_results);
	const series = allFiles.map((file) => {
		const dataPoints = [...data].reverse()
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

	// Detect outliers
	let allValues = [];
	series.forEach((s) => {
		s.data.forEach((point) => allValues.push(point.y));
	});

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

	// Separate outlier series from normal series
	const normalSeries = [];
	const outlierSeries = [];

	series.forEach((s) => {
		const avgValue = s.data.reduce((sum, point) => sum + point.y, 0) / s.data.length;
		if (outliers.includes(avgValue) || s.data.some((point) => outliers.includes(point.y))) {
			outlierSeries.push(s);
		} else {
			normalSeries.push(s);
		}
	});

	// Create main chart with normal data
	const mainMin = normal.length > 0 ? Math.max(0, Math.min(...normal)) : 0;
	const mainMax = normal.length > 0 ? Math.max(...normal) : 100;

	const options = {
		series: normalSeries,
		...getBaseChartConfig(
			"Processing Time Trends",
			"Microseconds",
			{ min: mainMin, max: mainMax },
			PERFORMANCE_THRESHOLDS.processing_time,
		),
	};
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
				return y.toFixed(0) + "μs";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();

	// Create outlier chart if there are outliers
	if (hasOutliers && outlierSeries.length > 0) {
		const outlierChartId = "processing-time-outliers-chart";
		let outlierChartElement = document.getElementById(outlierChartId);

		if (!outlierChartElement) {
			outlierChartElement = document.createElement("div");
			outlierChartElement.id = outlierChartId;
			outlierChartElement.style.marginTop = "20px";
			chartElement.parentNode.insertBefore(outlierChartElement, chartElement.nextSibling);
		}

		const outlierMin = Math.min(...outliers);
		const outlierMax = Math.max(...outliers);

		const outlierOptions = {
			series: outlierSeries,
			...getBaseChartConfig(
				"Processing Time Trends (Outliers)",
				"Microseconds",
				{ min: outlierMin, max: outlierMax },
				PERFORMANCE_THRESHOLDS.processing_time,
			),
		};
		outlierOptions.chart.height = 300;
		outlierOptions.tooltip.y = {
			formatter: function (y) {
				if (typeof y !== "undefined") {
					return y.toFixed(0) + "μs";
				}
				return y;
			},
		};

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		charts.push(outlierChart);
		outlierChart.render();
	}
}

function createCompressionChart(data) {
	const chartElement = document.getElementById("compression-chart");
	if (!chartElement) return;
	const allFiles = Object.keys(data[0].hyperfine_results);
	const series = allFiles
		.map((file) => {
			const dataPoints = [...data].reverse()
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

	// Detect outliers
	let allValues = [];
	series.forEach((s) => {
		s.data.forEach((point) => {
			if (point !== null) allValues.push(point);
		});
	});

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

	// Separate outlier series from normal series
	const normalSeries = [];
	const outlierSeries = [];

	series.forEach((s) => {
		const validValues = s.data.filter((point) => point !== null);
		const avgValue =
			validValues.length > 0 ? validValues.reduce((sum, point) => sum + point, 0) / validValues.length : 0;
		if (outliers.includes(avgValue) || s.data.some((point) => point !== null && outliers.includes(point))) {
			outlierSeries.push(s);
		} else {
			normalSeries.push(s);
		}
	});

	// Create main chart with normal data
	const mainMin = normal.length > 0 ? Math.max(0, Math.min(...normal)) : 0;
	const mainMax = normal.length > 0 ? Math.min(100, Math.max(...normal)) : 100;

	const options = {
		series: normalSeries,
		...getBaseChartConfig("Compression Ratio Trends", "Percent", { min: mainMin, max: mainMax }),
	};
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
				return y.toFixed(1) + "% smaller";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();

	// Create outlier chart if there are outliers
	if (hasOutliers && outlierSeries.length > 0) {
		const outlierChartId = "compression-outliers-chart";
		let outlierChartElement = document.getElementById(outlierChartId);

		if (!outlierChartElement) {
			outlierChartElement = document.createElement("div");
			outlierChartElement.id = outlierChartId;
			outlierChartElement.style.marginTop = "20px";
			chartElement.parentNode.insertBefore(outlierChartElement, chartElement.nextSibling);
		}

		const outlierMin = Math.max(0, Math.min(...outliers));
		const outlierMax = Math.min(100, Math.max(...outliers));

		const outlierOptions = {
			series: outlierSeries,
			...getBaseChartConfig("Compression Ratio Trends (Outliers)", "Percent", { min: outlierMin, max: outlierMax }),
		};
		outlierOptions.chart.height = 300;
		outlierOptions.tooltip.y = {
			formatter: function (y) {
				if (typeof y !== "undefined") {
					return y.toFixed(1) + "% smaller";
				}
				return y;
			},
		};

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		charts.push(outlierChart);
		outlierChart.render();
	}
}

function createThroughputChart(data) {
	const chartElement = document.getElementById("throughput-chart");
	if (!chartElement) return;
	const allFiles = Object.keys(data[0].hyperfine_results);
	const series = allFiles
		.map((file) => {
			const dataPoints = [...data].reverse()
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

	// Detect outliers
	let allValues = [];
	series.forEach((s) => {
		s.data.forEach((point) => {
			if (point !== null) allValues.push(point);
		});
	});

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

	// Separate outlier series from normal series
	const normalSeries = [];
	const outlierSeries = [];

	series.forEach((s) => {
		const validValues = s.data.filter((point) => point !== null);
		const avgValue =
			validValues.length > 0 ? validValues.reduce((sum, point) => sum + point, 0) / validValues.length : 0;
		if (outliers.includes(avgValue) || s.data.some((point) => point !== null && outliers.includes(point))) {
			outlierSeries.push(s);
		} else {
			normalSeries.push(s);
		}
	});

	// Create main chart with normal data
	const mainMin = normal.length > 0 ? Math.max(0, Math.min(...normal)) : 0;

	const options = {
		series: normalSeries,
		...getBaseChartConfig(
			"Processing Throughput Trends",
			"Megabytes per Second",
			{ min: mainMin },
			PERFORMANCE_THRESHOLDS.throughput,
		),
	};
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
				return y.toFixed(1) + " MB/s";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();

	// Create outlier chart if there are outliers
	if (hasOutliers && outlierSeries.length > 0) {
		const outlierChartId = "throughput-outliers-chart";
		let outlierChartElement = document.getElementById(outlierChartId);

		if (!outlierChartElement) {
			outlierChartElement = document.createElement("div");
			outlierChartElement.id = outlierChartId;
			outlierChartElement.style.marginTop = "20px";
			chartElement.parentNode.insertBefore(outlierChartElement, chartElement.nextSibling);
		}

		const outlierMin = Math.max(0, Math.min(...outliers));
		const outlierMax = Math.max(...outliers);

		const outlierOptions = {
			series: outlierSeries,
			...getBaseChartConfig(
				"Processing Throughput Trends (Outliers)",
				"Megabytes per Second",
				{ min: outlierMin, max: outlierMax },
				PERFORMANCE_THRESHOLDS.throughput,
			),
		};
		outlierOptions.chart.height = 300;
		outlierOptions.tooltip.y = {
			formatter: function (y) {
				if (typeof y !== "undefined") {
					return y.toFixed(1) + " MB/s";
				}
				return y;
			},
		};

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		charts.push(outlierChart);
		outlierChart.render();
	}
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
		PERFORMANCE_THRESHOLDS.from_str,
	);
}

function createCriterionParseSheetChart(data) {
	createCriterionGroupChart(
		data,
		"parse_sheet",
		"criterion-parse-sheet-chart",
		"Linting Sheet Parsing Performance",
		"Microseconds",
		1000,
		1,
	);
}

function createCriterionSelectorMatcherChart(data) {
	createCriterionGroupChart(
		data,
		"selector_matching",
		"criterion-selector-matcher-chart",
		"Selector Matching Performance",
		"Microseconds",
		1000,
		1,
	);
}

function createCriterionCollectorChart(data) {
	createCriterionGroupChart(
		data,
		"collector",
		"criterion-collector-chart",
		"Linting Collection Performance",
		"Microseconds",
		1000,
		1,
	);
}

function detectOutliers(values, threshold = 3) {
	if (values.length < 3) return { outliers: [], normal: values };

	const sorted = [...values].sort((a, b) => a - b);
	const q1 = sorted[Math.floor(sorted.length * 0.25)];
	const q3 = sorted[Math.floor(sorted.length * 0.75)];
	const iqr = q3 - q1;
	const lowerBound = q1 - threshold * iqr;
	const upperBound = q3 + threshold * iqr;

	const outliers = values.filter((v) => v < lowerBound || v > upperBound);
	const normal = values.filter((v) => v >= lowerBound && v <= upperBound);

	return { outliers, normal };
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
			const dataPoints = [...data].reverse()
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

	// Detect outliers
	let allValues = [];
	series.forEach((s) => {
		s.data.forEach((point) => allValues.push(point.y));
	});

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

	// Separate outlier series from normal series
	const normalSeries = [];
	const outlierSeries = [];

	series.forEach((s) => {
		const avgValue = s.data.reduce((sum, point) => sum + point.y, 0) / s.data.length;
		if (outliers.includes(avgValue) || s.data.some((point) => outliers.includes(point.y))) {
			outlierSeries.push(s);
		} else {
			normalSeries.push(s);
		}
	});

	// Create main chart with normal data
	const mainMinVal = normal.length > 0 ? Math.min(0, ...normal) : 0;
	const mainMaxVal = normal.length > 0 ? Math.max(...normal) : 100;
	const mainRange = mainMaxVal - mainMinVal;
	const mainPadding = mainRange * 0.1;

	// Determine the unit suffix for tooltips
	const unitSuffix = yAxisTitle.includes("Nanoseconds")
		? "ns"
		: yAxisTitle.includes("Microseconds")
			? "μs"
			: yAxisTitle.includes("Milliseconds")
				? "ms"
				: "";

	// Create main chart
	const options = {
		series: normalSeries,
		...getCriterionChartConfig(
			title,
			yAxisTitle,
			mainMinVal,
			mainMaxVal,
			mainPadding,
			decimalPlaces,
			unitSuffix,
			thresholds,
		),
	};

	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();

	// Create outlier chart if there are outliers
	if (hasOutliers && outlierSeries.length > 0) {
		const outlierChartId = chartId.replace("-chart", "-outliers-chart");
		let outlierChartElement = document.getElementById(outlierChartId);

		// Create outlier chart container if it doesn't exist
		if (!outlierChartElement) {
			outlierChartElement = document.createElement("div");
			outlierChartElement.id = outlierChartId;
			outlierChartElement.style.marginTop = "20px";
			chartElement.parentNode.insertBefore(outlierChartElement, chartElement.nextSibling);
		}

		const outlierMinVal = Math.min(...outliers);
		const outlierMaxVal = Math.max(...outliers);
		const outlierRange = outlierMaxVal - outlierMinVal;
		const outlierPadding = outlierRange * 0.1;

		const outlierOptions = {
			series: outlierSeries,
			...getCriterionChartConfig(
				title + " (Outliers)",
				yAxisTitle,
				outlierMinVal,
				outlierMaxVal,
				outlierPadding,
				decimalPlaces,
				unitSuffix,
				thresholds,
			),
		};

		// Make outlier chart slightly smaller
		outlierOptions.chart.height = 300;

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		charts.push(outlierChart);
		outlierChart.render();
	}
}

function createComparisonTimeChart(data) {
	const chartElement = document.getElementById("alternatives-time-chart");
	if (!chartElement) return;

	// Get the latest benchmark data
	const latest = data[0];
	if (!latest.alternative_tools || !latest.alternative_tools.files) {
		console.log("No alternative tools data available");
		return;
	}

	const files = latest.alternative_tools.files;
	const categories = [];
	const series = {};

	// Collect all CSS files and tools
	for (const [filename, fileData] of Object.entries(files)) {
		if (!fileData.tools) continue;
		if (selectedFile && filename !== selectedFile) continue;

		categories.push(filename);

		for (const [toolName, toolData] of Object.entries(fileData.tools)) {
			if (!series[toolName]) {
				series[toolName] = [];
			}

			if (toolData.error || !toolData.results || !toolData.results[0]) {
				series[toolName].push(null);
			} else {
				// Convert seconds to milliseconds
				const timeMs = toolData.results[0].mean * 1000;
				series[toolName].push(timeMs);
			}
		}
	}

	// Convert to ApexCharts format
	const chartSeries = Object.entries(series).map(([toolName, values]) => ({
		name: toolName,
		data: values,
	}));

	const isDarkMode = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;

	const options = {
		series: chartSeries,
		chart: {
			type: "bar",
			height: 1200,
			toolbar: { show: true },
			zoom: { enabled: true },
			events: {
				dataPointSelection: function (event, chartContext, config) {
					if (categories[config.dataPointIndex]) {
						const clickedFile = categories[config.dataPointIndex];
						if (selectedFile === clickedFile) {
							// Double-click behavior: reset to show all files
							selectedFile = null;
						} else {
							// Single-click behavior: zoom into this file
							selectedFile = clickedFile;
						}
						refreshCharts();
					}
				},
			},
			...getChartOptions(),
		},
		plotOptions: {
			bar: {
				horizontal: true,
				barHeight: "75%",
				endingShape: "rounded",
			},
		},
		dataLabels: {
			enabled: false,
		},
		stroke: {
			show: true,
			width: 2,
			colors: ["transparent"],
		},
		xaxis: {
			categories: categories,
			title: {
				text: "Processing Time (ms)",
				style: { fontSize: "14px" },
			},
			labels: {
				formatter: function (y) {
					if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
						return y.toFixed(2) + "ms";
					}
					return y;
				},
				style: { fontSize: "12px" },
			},
		},
		yaxis: {
			title: {
				text: "CSS Files",
				style: { fontSize: "14px" },
			},
			labels: {
				style: {
					fontSize: "12px",
				},
			},
		},
		fill: {
			opacity: 1,
		},
		tooltip: {
			shared: true,
			intersect: false,
			y: {
				formatter: function (y) {
					if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
						return y.toFixed(2) + "ms";
					}
					return y;
				},
			},
		},
		legend: {
			position: "bottom",
			fontSize: "14px",
		},
		title: {
			text: "Processing Time Comparison" + (selectedFile ? ` - ${selectedFile}` : ""),
			align: "left",
			style: { fontSize: "18px" },
		},
		grid: {
			borderColor: getGridBorderColor(),
		},
		theme: getChartTheme(),
		colors: ["#e74c3c", "#4CE0B3", "#ffbf46", "#663399"],
	};

	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();
}

function createComparisonSizeChart(data) {
	const chartElement = document.getElementById("alternatives-size-chart");
	if (!chartElement) return;

	// Get the latest benchmark data
	const latest = data[0];
	if (!latest.alternative_tools || !latest.alternative_tools.files) {
		console.log("No alternative tools data available");
		return;
	}

	const files = latest.alternative_tools.files;
	const categories = [];
	const series = {};

	// Collect all CSS files and tools
	for (const [filename, fileData] of Object.entries(files)) {
		if (!fileData.tools) continue;
		if (selectedFile && filename !== selectedFile) continue;

		categories.push(filename);

		for (const [toolName, toolData] of Object.entries(fileData.tools)) {
			if (!series[toolName]) {
				series[toolName] = [];
			}

			if (toolData.error || !toolData.output_size || toolData.output_size === 0) {
				series[toolName].push(null);
			} else {
				// Convert bytes to KB
				const sizeKB = toolData.output_size / 1024;
				series[toolName].push(sizeKB);
			}
		}
	}

	// Convert to ApexCharts format
	const chartSeries = Object.entries(series).map(([toolName, values]) => ({
		name: toolName,
		data: values,
	}));

	const options = {
		series: chartSeries,
		chart: {
			type: "bar",
			height: 1200,
			toolbar: { show: true },
			zoom: { enabled: true },
			events: {
				dataPointSelection: function (event, chartContext, config) {
					if (categories[config.dataPointIndex]) {
						const clickedFile = categories[config.dataPointIndex];
						if (selectedFile === clickedFile) {
							// Double-click behavior: reset to show all files
							selectedFile = null;
						} else {
							// Single-click behavior: zoom into this file
							selectedFile = clickedFile;
						}
						refreshCharts();
					}
				},
			},
			...getChartOptions(),
		},
		plotOptions: {
			bar: {
				horizontal: true,
				barHeight: "75%",
				endingShape: "rounded",
			},
		},
		dataLabels: {
			enabled: false,
		},
		stroke: {
			show: true,
			width: 2,
			colors: ["transparent"],
		},
		xaxis: {
			categories: categories,
			title: {
				text: "Output Size (KB)",
				style: { fontSize: "14px" },
			},
			labels: {
				formatter: function (y) {
					if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
						return y.toFixed(1) + "KB";
					}
					return y;
				},
				style: { fontSize: "12px" },
			},
		},
		yaxis: {
			title: {
				text: "CSS Files",
				style: { fontSize: "14px" },
			},
			labels: {
				style: {
					fontSize: "12px",
				},
			},
		},
		fill: {
			opacity: 1,
		},
		tooltip: {
			shared: true,
			intersect: false,
			y: {
				formatter: function (y) {
					if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
						return y.toFixed(1) + "KB";
					}
					return y;
				},
			},
		},
		legend: {
			position: "bottom",
			fontSize: "14px",
		},
		title: {
			text: "Output Size Comparison" + (selectedFile ? ` - ${selectedFile}` : ""),
			align: "left",
			style: { fontSize: "18px" },
		},
		grid: {
			borderColor: getGridBorderColor(),
		},
		theme: getChartTheme(),
		colors: ["#e74c3c", "#4CE0B3", "#ffbf46", "#663399"],
	};

	const chart = new ApexCharts(chartElement, options);
	charts.push(chart);
	chart.render();
}
