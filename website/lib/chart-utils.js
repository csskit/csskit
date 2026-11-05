const formatters = new Map();

/** Formats `value` with fixed `decimals` and en-GB grouping, then appends `unit`. */
export function format(value, decimals, unit) {
	if (!formatters.has(decimals)) {
		formatters.set(
			decimals,
			new Intl.NumberFormat("en-GB", { minimumFractionDigits: decimals, maximumFractionDigits: decimals }),
		);
	}
	return formatters.get(decimals).format(value) + unit;
}

/** Returns an axis range covering `low` to `high`, split into `ticks` equal whole-number steps. */
export function niceScale(low, high, ticks) {
	const raw = Math.max(high - low, 1) / ticks;
	for (let exponent = Math.max(0, Math.floor(Math.log10(raw))); ; exponent++) {
		for (const mantissa of [1, 1.5, 2, 2.5, 3, 4, 5, 6, 8]) {
			const step = mantissa * 10 ** exponent;
			if (step < raw || !Number.isInteger(step)) continue;
			const min = Math.floor(low / step) * step;
			if (min + step * ticks >= high) return { min, max: min + step * ticks };
		}
	}
}

/** Inline style properties for a line chart's y axis spanning `min` to `max`, with up to two in-range threshold lines. */
export function yAxisStyle(min, max, unit, thresholds) {
	const ticks = [4, 3, 2, 1, 0].map((quarter) => format(min + ((max - min) * quarter) / 4, 0, unit));
	const props = {
		"--min": String(min),
		"--max": String(max),
		"--y-axis": `'${ticks.join("\\A ")}'`,
		"--y-axis-width": `${Math.max(...ticks.map((tick) => tick.length)) + 1}ch`,
		"--threshold-label": "none",
		"--threshold-2-label": "none",
	};
	thresholds
		.filter((threshold) => threshold.value >= min && threshold.value <= max)
		.slice(0, 2)
		.forEach((threshold, i) => {
			const prop = i === 0 ? "--threshold" : "--threshold-2";
			props[prop] = String(threshold.value);
			props[`${prop}-label`] = `'${threshold.label}'`;
			props[`${prop}-color`] = `var(${threshold.color})`;
		});
	return props;
}

/** Tick labels for a bar chart's x axis spanning 0 to `max`. */
export function xAxisTicks(max, unit) {
	return [0, 1, 2, 3, 4, 5].map((fifth) => format((max * fifth) / 5, 0, unit));
}
