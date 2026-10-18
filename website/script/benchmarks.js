import ApexCharts from "apexcharts";

document.addEventListener("DOMContentLoaded", function () {
	if (window.benchmarkData && window.benchmarkData.length > 1) {
		initializeCharts();
	}
});

let charts = [];
let selectedFile = null; // null means show all files

const MAX_POINTS = 30;

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
			Showing: ${selectedFile.replace(/\./g, " ").replace(/-/g, " ")}
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
			height: 460,
			zoom: { enabled: true, type: "x" },
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
			type: "datetime",
			title: {
				text: "Date",
				style: { fontSize: fontSizes.axis },
			},
			labels: {
				datetimeUTC: false,
				style: { fontSize: "12px" },
			},
			tooltip: { enabled: false },
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
			height: 80,
			fontSize: fontSizes.legend,
		},
		tooltip: {
			shared: true,
			intersect: false,
			style: {
				fontSize: fontSizes.tooltip,
			},
			x: {
				format: "dd MMM yyyy HH:mm",
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

// --- Adaptive sampling ---

/**
 * Coalesce entries by calendar day, keeping the last run of each day.
 * Entries must be sorted oldest-first.
 */
function coalesceByDay(entries) {
	const byDay = new Map();
	for (const entry of entries) {
		const day = entry.timestamp.slice(0, 10); // YYYY-MM-DD
		byDay.set(day, entry); // later entries overwrite earlier ones
	}
	return Array.from(byDay.values());
}

/**
 * Thin an array to at most maxPoints by picking evenly spaced entries.
 * Always includes first and last.
 */
function thinEvenly(entries, maxPoints) {
	if (entries.length <= maxPoints) return entries;
	const result = [];
	const step = (entries.length - 1) / (maxPoints - 1);
	for (let i = 0; i < maxPoints; i++) {
		result.push(entries[Math.round(i * step)]);
	}
	return result;
}

/**
 * Given a full list of entries (oldest-first), return a sampled subset
 * of at most MAX_POINTS for display. Coalesces same-day runs first, then
 * thins if still too many.
 */
function sampleEntries(entries) {
	const coalesced = coalesceByDay(entries);
	return thinEvenly(coalesced, MAX_POINTS);
}

/**
 * Filter entries to a timestamp window [minMs, maxMs], then resample.
 */
function sampleEntriesWindow(entries, minMs, maxMs) {
	const windowed = entries.filter((e) => {
		const ms = new Date(e.timestamp).getTime();
		return ms >= minMs && ms <= maxMs;
	});
	// Within a zoom window show all coalesced points (no further thinning)
	// unless there are still more than MAX_POINTS
	const coalesced = coalesceByDay(windowed);
	return thinEvenly(coalesced, MAX_POINTS);
}

/**
 * Attach zoom/reset handlers to a chart that update its series from fullSeriesBuilder.
 * fullSeriesBuilder(entries) -> series array
 * fullEntries: all benchmark entries, oldest-first
 */
function attachZoomHandlers(chart, fullEntries, fullSeriesBuilder) {
	// We patch the chart options after render to inject event handlers,
	// since ApexCharts events must be in the initial config.
	// Instead we use the chart's internal event system via updateOptions.
	// NOTE: zoomed and beforeResetZoom must be set in initial chart config.
	// This function is a no-op placeholder; see buildZoomEvents() below.
}

/**
 * Build the chart.events block for adaptive zoom.
 *
 * The chart is always rendered with the full coalesced-by-day dataset.
 * ApexCharts handles zoom natively within that dataset. On reset we
 * ensure the coalesced view is restored (in case updateSeries was called
 * from elsewhere).
 *
 * sampledEntries: the initial sample shown on render - restored on reset
 * seriesBuilder(entries) -> ApexCharts series array
 * getChartRef: () -> ApexCharts instance (populated after render)
 */
function sampledTimeRange(sampledEntries) {
	const min = new Date(sampledEntries[0].timestamp).getTime();
	const max = new Date(sampledEntries[sampledEntries.length - 1].timestamp).getTime();
	return { min, max };
}

function buildZoomEvents(sampledEntries, seriesBuilder, getChartRef) {
	const { min, max } = sampledTimeRange(sampledEntries);
	return {
		beforeZoom: function (_ctx, { xaxis }) {
			return {
				xaxis: {
					min: Math.max(xaxis.min, min),
					max: Math.min(xaxis.max, max),
				},
			};
		},
		beforeResetZoom: function (_ctx) {
			return { xaxis: { min, max } };
		},
	};
}

function initializeCharts() {
	const data = window.benchmarkData;

	if (data.length < 2) return;

	// data is newest-first; reverse to oldest-first for sampling
	const entries = [...data].reverse();

	// Create all charts
	createTimeChart(entries);
	createCompressionChart(entries);
	createThroughputChart(entries);
	createCriterionParseChart(entries);
	createCriterionLexChart(entries);
	createCriterionMinifyChart(entries);
	createCriterionParseSheetChart(entries);
	createCriterionSelectorMatcherChart(entries);
	createCriterionCollectorChart(entries);
	createCriterionFromStrChart(entries);

	createComparisonTimeChart(data);
	createComparisonSizeChart(data);

	setupThemeObserver();
}

function createTimeChart(entries) {
	const chartElement = document.getElementById("processing-time-chart");
	if (!chartElement) return;

	const allFiles = Object.keys(entries[0].hyperfine_results);

	function buildSeries(sampledEntries) {
		return allFiles.map((file) => {
			const dataPoints = sampledEntries
				.map((entry) => {
					const fileData = entry.hyperfine_results[file];
					if (fileData && fileData.results && fileData.results[0]) {
						return {
							x: new Date(entry.timestamp).getTime(),
							y: Math.round(fileData.results[0].summary.time_wall_clock.mean * 1000000), // microseconds
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
	}

	const sampled = sampleEntries(entries);
	const series = buildSeries(sampled);

	// Detect outliers
	let allValues = [];
	series.forEach((s) => {
		s.data.forEach((point) => allValues.push(point.y));
	});

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

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

	function buildNormalSeries(e) {
		return buildSeries(e).filter((s) => normalSeries.some((ns) => ns.name === s.name));
	}
	function buildOutlierSeries(e) {
		return buildSeries(e).filter((s) => outlierSeries.some((os) => os.name === s.name));
	}

	const mainMin = normal.length > 0 ? Math.max(0, Math.min(...normal)) : 0;
	const mainMax = normal.length > 0 ? Math.max(...normal) : 100;

	let chartRef = null;
	const options = {
		series: normalSeries,
		...getBaseChartConfig(
			"Processing Time Trends",
			"Microseconds",
			{ min: mainMin, max: mainMax },
			PERFORMANCE_THRESHOLDS.processing_time,
		),
	};
	options.chart.events = buildZoomEvents(sampled, buildNormalSeries, () => chartRef);
	Object.assign(options.xaxis, sampledTimeRange(sampled));
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
				return y.toFixed(0) + "μs";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	chartRef = chart;
	charts.push(chart);
	chart.render();

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

		let outlierChartRef = null;
		const outlierOptions = {
			series: outlierSeries,
			...getBaseChartConfig(
				"Processing Time Trends (Outliers)",
				"Microseconds",
				{ min: outlierMin, max: outlierMax },
				PERFORMANCE_THRESHOLDS.processing_time,
			),
		};
		outlierOptions.chart.height = 460;
		outlierOptions.chart.events = buildZoomEvents(sampled, buildOutlierSeries, () => outlierChartRef);
		Object.assign(outlierOptions.xaxis, sampledTimeRange(sampled));
		outlierOptions.tooltip.y = {
			formatter: function (y) {
				if (typeof y !== "undefined") {
					return y.toFixed(0) + "μs";
				}
				return y;
			},
		};

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		outlierChartRef = outlierChart;
		charts.push(outlierChart);
		outlierChart.render();
	}
}

function createCompressionChart(entries) {
	const chartElement = document.getElementById("compression-chart");
	if (!chartElement) return;

	const allFiles = Object.keys(entries[0].hyperfine_results);

	function buildSeries(sampledEntries) {
		return allFiles
			.map((file) => {
				const dataPoints = sampledEntries
					.map((entry) => {
						const fileData = entry.hyperfine_results[file];
						if (fileData && fileData.compression_ratio && fileData.output_size > 0) {
							const compressionPercent = (1 - fileData.compression_ratio) * 100;
							return {
								x: new Date(entry.timestamp).getTime(),
								y: Math.round(compressionPercent * 10) / 10,
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
			.filter((s) => s.data.length > 0);
	}

	const sampled = sampleEntries(entries);
	const series = buildSeries(sampled);

	let allValues = [];
	series.forEach((s) => s.data.forEach((point) => allValues.push(point.y)));

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

	const normalSeries = [];
	const outlierSeries = [];
	series.forEach((s) => {
		const validValues = s.data.filter((p) => p !== null);
		const avgValue = validValues.length > 0 ? validValues.reduce((sum, p) => sum + p.y, 0) / validValues.length : 0;
		if (outliers.includes(avgValue) || s.data.some((p) => p !== null && outliers.includes(p.y))) {
			outlierSeries.push(s);
		} else {
			normalSeries.push(s);
		}
	});

	function buildNormalSeries(e) {
		return buildSeries(e).filter((s) => normalSeries.some((ns) => ns.name === s.name));
	}
	function buildOutlierSeries(e) {
		return buildSeries(e).filter((s) => outlierSeries.some((os) => os.name === s.name));
	}

	const mainMin = normal.length > 0 ? Math.max(0, Math.min(...normal.map((p) => (typeof p === "object" ? p.y : p)))) : 0;
	const mainMax = normal.length > 0 ? Math.min(100, Math.max(...normal.map((p) => (typeof p === "object" ? p.y : p)))) : 100;

	let chartRef = null;
	const options = {
		series: normalSeries,
		...getBaseChartConfig("Compression Ratio Trends", "Percent", { min: mainMin, max: mainMax }),
	};
	options.chart.events = buildZoomEvents(sampled, buildNormalSeries, () => chartRef);
	Object.assign(options.xaxis, sampledTimeRange(sampled));
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
				return y.toFixed(1) + "% smaller";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	chartRef = chart;
	charts.push(chart);
	chart.render();

	if (hasOutliers && outlierSeries.length > 0) {
		const outlierChartId = "compression-outliers-chart";
		let outlierChartElement = document.getElementById(outlierChartId);
		if (!outlierChartElement) {
			outlierChartElement = document.createElement("div");
			outlierChartElement.id = outlierChartId;
			outlierChartElement.style.marginTop = "20px";
			chartElement.parentNode.insertBefore(outlierChartElement, chartElement.nextSibling);
		}

		const outlierMin = Math.max(0, Math.min(...outliers.map((p) => (typeof p === "object" ? p.y : p))));
		const outlierMax = Math.min(100, Math.max(...outliers.map((p) => (typeof p === "object" ? p.y : p))));

		let outlierChartRef = null;
		const outlierOptions = {
			series: outlierSeries,
			...getBaseChartConfig("Compression Ratio Trends (Outliers)", "Percent", { min: outlierMin, max: outlierMax }),
		};
		outlierOptions.chart.height = 460;
		outlierOptions.chart.events = buildZoomEvents(sampled, buildOutlierSeries, () => outlierChartRef);
		Object.assign(outlierOptions.xaxis, sampledTimeRange(sampled));
		outlierOptions.tooltip.y = {
			formatter: function (y) {
				if (typeof y !== "undefined") {
					return y.toFixed(1) + "% smaller";
				}
				return y;
			},
		};

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		outlierChartRef = outlierChart;
		charts.push(outlierChart);
		outlierChart.render();
	}
}

function createThroughputChart(entries) {
	const chartElement = document.getElementById("throughput-chart");
	if (!chartElement) return;

	const allFiles = Object.keys(entries[0].hyperfine_results);

	function buildSeries(sampledEntries) {
		return allFiles
			.map((file) => {
				const dataPoints = sampledEntries
					.map((entry) => {
						const fileData = entry.hyperfine_results[file];
						if (fileData && fileData.results && fileData.results[0] && fileData.input_size) {
							const throughputMBps = fileData.input_size / fileData.results[0].summary.time_wall_clock.mean / (1024 * 1024);
							return {
								x: new Date(entry.timestamp).getTime(),
								y: Math.round(throughputMBps * 10) / 10,
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
			.filter((s) => s.data.length > 0);
	}

	// Compute low/high water marks from full history (p10/p90 across all files)
	const historyValues = buildSeries(entries)
		.flatMap((s) => s.data.map((p) => p.y))
		.filter((v) => typeof v === "number" && !isNaN(v))
		.sort((a, b) => a - b);
	const p10 = historyValues[Math.floor(historyValues.length * 0.1)] ?? 0;
	const p90 = historyValues[Math.floor(historyValues.length * 0.9)] ?? 0;
	const throughputThresholds = { bad: Math.round(p10 * 10) / 10, good: Math.round(p90 * 10) / 10 };

	const sampled = sampleEntries(entries);
	const series = buildSeries(sampled);

	let allValues = [];
	series.forEach((s) => s.data.forEach((point) => allValues.push(point.y)));

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

	const normalSeries = [];
	const outlierSeries = [];
	series.forEach((s) => {
		const validValues = s.data.filter((p) => p !== null);
		const avgValue = validValues.length > 0 ? validValues.reduce((sum, p) => sum + p.y, 0) / validValues.length : 0;
		if (outliers.includes(avgValue) || s.data.some((p) => p !== null && outliers.includes(p.y))) {
			outlierSeries.push(s);
		} else {
			normalSeries.push(s);
		}
	});

	function buildNormalSeries(e) {
		return buildSeries(e).filter((s) => normalSeries.some((ns) => ns.name === s.name));
	}
	function buildOutlierSeries(e) {
		return buildSeries(e).filter((s) => outlierSeries.some((os) => os.name === s.name));
	}

	const mainMin = normal.length > 0 ? Math.max(0, Math.min(...normal.map((p) => (typeof p === "object" ? p.y : p)))) : 0;

	let chartRef = null;
	const options = {
		series: normalSeries,
		...getBaseChartConfig(
			"Processing Throughput Trends",
			"Megabytes per Second",
			{ min: mainMin },
			throughputThresholds,
		),
	};
	options.chart.events = buildZoomEvents(sampled, buildNormalSeries, () => chartRef);
	Object.assign(options.xaxis, sampledTimeRange(sampled));
	options.tooltip.y = {
		formatter: function (y) {
			if (typeof y !== "undefined" && typeof y === "number" && !isNaN(y)) {
				return y.toFixed(1) + " MB/s";
			}
			return y;
		},
	};
	const chart = new ApexCharts(chartElement, options);
	chartRef = chart;
	charts.push(chart);
	chart.render();

	if (hasOutliers && outlierSeries.length > 0) {
		const outlierChartId = "throughput-outliers-chart";
		let outlierChartElement = document.getElementById(outlierChartId);
		if (!outlierChartElement) {
			outlierChartElement = document.createElement("div");
			outlierChartElement.id = outlierChartId;
			outlierChartElement.style.marginTop = "20px";
			chartElement.parentNode.insertBefore(outlierChartElement, chartElement.nextSibling);
		}

		const outlierMin = Math.max(0, Math.min(...outliers.map((p) => (typeof p === "object" ? p.y : p))));
		const outlierMax = Math.max(...outliers.map((p) => (typeof p === "object" ? p.y : p)));

		let outlierChartRef = null;
		const outlierOptions = {
			series: outlierSeries,
			...getBaseChartConfig(
				"Processing Throughput Trends (Outliers)",
				"Megabytes per Second",
				{ min: outlierMin, max: outlierMax },
				throughputThresholds,
			),
		};
		outlierOptions.chart.height = 460;
		outlierOptions.chart.events = buildZoomEvents(sampled, buildOutlierSeries, () => outlierChartRef);
		Object.assign(outlierOptions.xaxis, sampledTimeRange(sampled));
		outlierOptions.tooltip.y = {
			formatter: function (y) {
				if (typeof y !== "undefined") {
					return y.toFixed(1) + " MB/s";
				}
				return y;
			},
		};

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		outlierChartRef = outlierChart;
		charts.push(outlierChart);
		outlierChart.render();
	}
}

function createCriterionParseChart(entries) {
	createCriterionGroupChart(
		entries,
		"parse_popular",
		"criterion-parse-chart",
		"Parsing Performance",
		"Microseconds",
		1000,
		1,
		PERFORMANCE_THRESHOLDS.parsing,
	);
}

function createCriterionLexChart(entries) {
	createCriterionGroupChart(
		entries,
		"lex_popular",
		"criterion-lex-chart",
		"Lexing Performance",
		"Microseconds",
		1000,
		1,
		PERFORMANCE_THRESHOLDS.lexing,
	);
}

function createCriterionMinifyChart(entries) {
	createCriterionGroupChart(
		entries,
		"minify_popular",
		"criterion-minify-chart",
		"Minification Performance",
		"Microseconds",
		1000,
		1,
		PERFORMANCE_THRESHOLDS.minification,
	);
}

function createCriterionFromStrChart(entries) {
	createCriterionGroupChart(
		entries,
		"from_str_by_length",
		"criterion-fromstr-chart",
		"String Parsing by Length",
		"Nanoseconds",
		1,
		0,
		PERFORMANCE_THRESHOLDS.from_str,
	);
}

function createCriterionParseSheetChart(entries) {
	createCriterionGroupChart(
		entries,
		"parse_sheet",
		"criterion-parse-sheet-chart",
		"Linting Sheet Parsing Performance",
		"Microseconds",
		1000,
		1,
	);
}

function createCriterionSelectorMatcherChart(entries) {
	createCriterionQueryFilteredChart(entries, {
		groupPrefix: "selector_matching",
		chartId: "criterion-selector-matcher-chart",
		selectId: "selector-matcher-query-select",
		title: "Selector Matching Performance",
		yAxisTitle: "Microseconds",
		conversionFactor: 1000,
		decimalPlaces: 1,
	});
}

/**
 * Create a criterion group chart that is filtered by a secondary "query file" dimension.
 * Keys must follow the pattern "<groupPrefix>/<css_file>_<query_file>".
 * A <select> is injected above the chart to switch between query files.
 */
function createCriterionQueryFilteredChart(entries, {
	groupPrefix,
	chartId,
	selectId,
	title,
	yAxisTitle,
	conversionFactor,
	decimalPlaces,
	thresholds = null,
}) {
	const chartElement = document.getElementById(chartId);
	if (!chartElement) return;

	const allBenchmarks = Object.keys(entries[0].criterion_results).filter((b) =>
		b.startsWith(groupPrefix + "/"),
	);
	if (allBenchmarks.length === 0) return;

	const names = allBenchmarks.map((b) => b.replace(groupPrefix + "/", ""));
	const queryFiles = [...new Set(names.map((n) => n.replace(/^[^_]+_/, "")))].sort();

	let select = document.getElementById(selectId);
	if (!select) {
		const wrapper = document.createElement("div");
		wrapper.style.cssText = "margin-bottom: 0.5rem;";
		select = document.createElement("select");
		select.id = selectId;
		queryFiles.forEach((q) => {
			const opt = document.createElement("option");
			opt.value = q;
			opt.textContent = q.replace(/-/g, " ").replace(/_/g, " ");
			select.appendChild(opt);
		});
		wrapper.appendChild(select);
		chartElement.parentNode.insertBefore(wrapper, chartElement);
	}

	const outlierId = chartId.replace("-chart", "-outliers-chart");

	function renderForQuery(queryFile) {
		[chartId, outlierId].forEach((id) => {
			const idx = charts.findIndex((c) => c && c.el && c.el.id === id);
			if (idx !== -1) {
				charts[idx].destroy();
				charts.splice(idx, 1);
			}
			if (id === outlierId) {
				const el = document.getElementById(id);
				if (el) el.remove();
			}
		});

		const filtered = allBenchmarks.filter((b) => b.endsWith("_" + queryFile));
		const filteredEntries = entries.map((e) => ({
			...e,
			criterion_results: Object.fromEntries(
				Object.entries(e.criterion_results).filter(([k]) => filtered.includes(k)),
			),
		}));

		createCriterionGroupChart(
			filteredEntries,
			groupPrefix,
			chartId,
			`${title} - ${queryFile.replace(/-/g, " ")}`,
			yAxisTitle,
			conversionFactor,
			decimalPlaces,
			thresholds,
		);
	}

	select.addEventListener("change", () => renderForQuery(select.value));
	renderForQuery(queryFiles[0]);
}

function createCriterionCollectorChart(entries) {
	createCriterionQueryFilteredChart(entries, {
		groupPrefix: "collector",
		chartId: "criterion-collector-chart",
		selectId: "collector-query-select",
		title: "Linting Collection Performance",
		yAxisTitle: "Milliseconds",
		conversionFactor: 1000000,
		decimalPlaces: 2,
	});
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
	entries,
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

	const criterionBenchmarks = Object.keys(entries[0].criterion_results).filter((benchmark) =>
		benchmark.startsWith(groupPrefix + "/"),
	);
	if (criterionBenchmarks.length === 0) return;

	const unitSuffix = yAxisTitle.includes("Nanoseconds")
		? "ns"
		: yAxisTitle.includes("Microseconds")
			? "μs"
			: yAxisTitle.includes("Milliseconds")
				? "ms"
				: "";

	function buildSeries(sampledEntries) {
		return criterionBenchmarks
			.map((benchmark) => {
				const dataPoints = sampledEntries
					.map((entry) => {
						const benchmarkData = entry.criterion_results[benchmark];
						if (benchmarkData && benchmarkData.mean && benchmarkData.mean.point_estimate) {
							const convertedTime = benchmarkData.mean.point_estimate / conversionFactor;
							const roundedTime =
								Math.round(convertedTime * Math.pow(10, decimalPlaces)) / Math.pow(10, decimalPlaces);
							return {
								x: new Date(entry.timestamp).getTime(),
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
			.filter((s) => s.data.length > 0);
	}

	const sampled = sampleEntries(entries);
	const series = buildSeries(sampled);

	let allValues = [];
	series.forEach((s) => s.data.forEach((point) => allValues.push(point.y)));

	const { outliers, normal } = detectOutliers(allValues);
	const hasOutliers = outliers.length > 0;

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

	function buildNormalSeries(e) {
		return buildSeries(e).filter((s) => normalSeries.some((ns) => ns.name === s.name));
	}
	function buildOutlierSeries(e) {
		return buildSeries(e).filter((s) => outlierSeries.some((os) => os.name === s.name));
	}

	const mainMinVal = normal.length > 0 ? Math.min(0, ...normal) : 0;
	const mainMaxVal = normal.length > 0 ? Math.max(...normal) : 100;
	const mainRange = mainMaxVal - mainMinVal;
	const mainPadding = mainRange * 0.1;

	let chartRef = null;
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
	options.chart.events = buildZoomEvents(sampled, buildNormalSeries, () => chartRef);
	Object.assign(options.xaxis, sampledTimeRange(sampled));

	const chart = new ApexCharts(chartElement, options);
	chartRef = chart;
	charts.push(chart);
	chart.render();

	if (hasOutliers && outlierSeries.length > 0) {
		const outlierChartId = chartId.replace("-chart", "-outliers-chart");
		let outlierChartElement = document.getElementById(outlierChartId);
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

		let outlierChartRef = null;
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
		outlierOptions.chart.height = 460;
		outlierOptions.chart.events = buildZoomEvents(sampled, buildOutlierSeries, () => outlierChartRef);
		Object.assign(outlierOptions.xaxis, sampledTimeRange(sampled));

		const outlierChart = new ApexCharts(outlierChartElement, outlierOptions);
		outlierChartRef = outlierChart;
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
				const timeMs = toolData.results[0].summary.time_wall_clock.mean * 1000;
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
