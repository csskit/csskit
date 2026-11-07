import { niceScale, xAxisTicks, yAxisStyle } from "../lib/chart-utils.js";

const ZOOM_RADIUS = 3;
const fullRows = new WeakMap();
let shiftHeld = false;

function visibleValues(table, prop) {
	const columns = new Set(
		[...table.tHead.rows[0].cells].filter((cell) => cell.querySelector("input:checked")).map((cell) => cell.cellIndex),
	);
	const values = [];
	for (const row of table.tBodies[0].rows) {
		for (const cell of row.cells) {
			if (!columns.has(cell.cellIndex)) continue;
			const value = parseFloat(cell.style.getPropertyValue(prop));
			if (!Number.isNaN(value)) values.push(value);
		}
	}
	return values;
}

function rescaleBars(table) {
	const values = visibleValues(table, "--v");
	if (!values.length) return;
	const { max } = niceScale(0, Math.max(...values), 5);
	table.style.setProperty("--max", String(max));
	const labels = xAxisTicks(max, table.dataset.unit);
	[...table.tFoot.rows[0].cells].slice(1).forEach((cell, i) => {
		cell.textContent = labels[i];
	});
}

function rescaleLines(table) {
	const values = visibleValues(table, "--b");
	if (!values.length) return;
	const fromZero = table.hasAttribute("data-from-zero") && !fullRows.has(table);
	const { min, max } = niceScale(fromZero ? 0 : Math.min(...values), Math.max(...values), 4);
	const style = yAxisStyle(min, max, table.dataset.unit, JSON.parse(table.dataset.thresholds));
	for (const [prop, value] of Object.entries(style)) table.style.setProperty(prop, value);
}

function rescale(table) {
	if (table.classList.contains("bars")) rescaleBars(table);
	else rescaleLines(table);
}

function toggleZoom(table, row) {
	const tbody = row.parentElement;
	if (fullRows.has(table)) {
		tbody.replaceChildren(...fullRows.get(table));
		fullRows.delete(table);
		delete table.dataset.zoomed;
	} else {
		const rows = [...tbody.rows];
		const size = ZOOM_RADIUS * 2 + 1;
		const start = Math.max(0, Math.min(row.sectionRowIndex - ZOOM_RADIUS, rows.length - size));
		fullRows.set(table, rows);
		tbody.replaceChildren(...rows.slice(start, start + size));
		table.dataset.zoomed = "";
	}
	rescaleLines(table);
}

document.addEventListener("pointerdown", (event) => {
	shiftHeld = event.shiftKey;
});

document.addEventListener("keydown", (event) => {
	shiftHeld = event.shiftKey;
});

document.addEventListener("change", (event) => {
	const table = event.target.closest(".chart");
	if (!table || !event.target.closest("thead")) return;
	if (shiftHeld) {
		for (const input of table.tHead.querySelectorAll("input")) {
			if (input !== event.target) input.checked = !input.checked;
		}
	}
	rescale(table);
});

document.addEventListener("click", (event) => {
	const row = event.target.closest(".lines tbody tr");
	if (row) toggleZoom(row.closest(".chart"), row);
});

for (const table of document.querySelectorAll(".chart")) table.dataset.interactive = "";
