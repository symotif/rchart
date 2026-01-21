<script lang="ts">
	import { onMount, tick } from 'svelte';
	import TabContainer from '$lib/components/ui/TabContainer.svelte';
	import type { Vital, Lab, ClinicalScore } from '$lib/types/patient';

	let {
		vitals,
		labs,
		scores
	}: {
		vitals: Vital[];
		labs: Lab[];
		scores: ClinicalScore[];
	} = $props();

	let activeTab = $state('table');
	let selectedMetric = $state<string | null>(null);
	let graphContainer: HTMLDivElement | null = $state(null);

	const tabs = [
		{ id: 'table', label: 'Table' },
		{ id: 'graphs', label: 'Graphs' }
	];

	// Get all unique metrics for the graph buttons (combine BP+HR as one option)
	const allMetrics = $derived(() => {
		const vitalTypes = [...new Set(vitals.map((v) => v.vital_type))];
		const labNames = [...new Set(labs.map((l) => l.name))];
		const scoreTypes = [...new Set(scores.map((s) => s.score_type))];

		// Replace separate BP and HR with combined option
		const hasBP = vitalTypes.includes('BP');
		const hasHR = vitalTypes.includes('HR');
		const filteredVitals = vitalTypes.filter(t => t !== 'BP' && t !== 'HR');

		const result = [];
		if (hasBP && hasHR) {
			result.push('BP + HR');
		} else if (hasBP) {
			result.push('BP');
		} else if (hasHR) {
			result.push('HR');
		}

		return [...result, ...filteredVitals, ...labNames, ...scoreTypes];
	});

	// Get all unique timestamps for table columns
	const allTimestamps = $derived(() => {
		const timestamps = new Set<string>();
		vitals.forEach((v) => timestamps.add(v.recorded_at));
		labs.forEach((l) => timestamps.add(l.recorded_at));
		scores.forEach((s) => timestamps.add(s.recorded_at));
		return [...timestamps].sort((a, b) => new Date(a).getTime() - new Date(b).getTime());
	});

	// Get unique metrics for table rows (keep BP and HR separate in table)
	const tableMetrics = $derived(() => {
		const vitalTypes = [...new Set(vitals.map((v) => v.vital_type))];
		const labNames = [...new Set(labs.map((l) => l.name))];
		const scoreTypes = [...new Set(scores.map((s) => s.score_type))];
		return [...vitalTypes, ...labNames, ...scoreTypes];
	});

	// Build table data: rows are metrics, columns are timestamps
	const tableData = $derived(() => {
		const metrics = tableMetrics();
		const timestamps = allTimestamps();

		return metrics.map((metric) => {
			const row: Record<string, string | number | boolean | null> = { metric };

			// Get unit for this metric
			const vitalUnit = vitals.find(v => v.vital_type === metric)?.unit;
			const labUnit = labs.find(l => l.name === metric)?.unit;
			row.unit = vitalUnit || labUnit || '';

			for (const ts of timestamps) {
				// Check vitals
				const vital = vitals.find((v) => v.vital_type === metric && v.recorded_at === ts);
				if (vital) {
					row[ts] =
						vital.value_secondary !== null
							? `${vital.value}/${vital.value_secondary}`
							: vital.value;
					continue;
				}

				// Check labs
				const lab = labs.find((l) => l.name === metric && l.recorded_at === ts);
				if (lab) {
					row[ts] = lab.value;
					row[`${ts}_abnormal`] = lab.is_abnormal ?? false;
					continue;
				}

				// Check scores
				const score = scores.find((s) => s.score_type === metric && s.recorded_at === ts);
				if (score) {
					row[ts] = score.max_score ? `${score.score}/${score.max_score}` : score.score;
					continue;
				}

				row[ts] = null;
			}

			return row;
		});
	});

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	function formatDateTime(dateStr: string): string {
		const date = new Date(dateStr);
		const dateFormatted = date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
		const timeFormatted = date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
		return `${dateFormatted}\n${timeFormatted}`;
	}

	function getDataForMetric(metric: string): { date: Date; value: number; valueSecondary?: number }[] {
		// Check vitals
		const vitalData = vitals
			.filter((v) => v.vital_type === metric)
			.map((v) => ({
				date: new Date(v.recorded_at),
				value: v.value,
				valueSecondary: v.value_secondary ?? undefined
			}));
		if (vitalData.length > 0) return vitalData;

		// Check labs
		const labData = labs
			.filter((l) => l.name === metric)
			.map((l) => ({
				date: new Date(l.recorded_at),
				value: l.value
			}));
		if (labData.length > 0) return labData;

		// Check scores
		const scoreData = scores
			.filter((s) => s.score_type === metric)
			.map((s) => ({
				date: new Date(s.recorded_at),
				value: s.score
			}));
		return scoreData;
	}

	function getUnitForMetric(metric: string): string {
		if (metric === 'BP' || metric === 'BP + HR') return 'mmHg';
		if (metric === 'HR') return 'bpm';
		const vital = vitals.find(v => v.vital_type === metric);
		if (vital) return vital.unit || '';
		const lab = labs.find(l => l.name === metric);
		if (lab) return lab.unit || '';
		return '';
	}

	async function drawGraph() {
		await tick();
		if (!graphContainer || !selectedMetric) return;

		// Clear existing content
		graphContainer.innerHTML = '';

		const width = graphContainer.clientWidth;
		const height = graphContainer.clientHeight - 20;
		const margin = { top: 20, right: 60, bottom: 50, left: 60 };
		const innerWidth = width - margin.left - margin.right;
		const innerHeight = height - margin.top - margin.bottom;

		// Create SVG
		const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
		svg.setAttribute('width', String(width));
		svg.setAttribute('height', String(height));
		svg.classList.add('overflow-visible');

		// Handle combined BP + HR graph
		if (selectedMetric === 'BP + HR') {
			const bpData = getDataForMetric('BP');
			const hrData = getDataForMetric('HR');

			if (bpData.length === 0 && hrData.length === 0) return;

			// Get all dates
			const allDates = [...new Set([
				...bpData.map(d => d.date.getTime()),
				...hrData.map(d => d.date.getTime())
			])].sort((a, b) => a - b);

			const minDate = Math.min(...allDates);
			const maxDate = Math.max(...allDates);
			const dateRange = maxDate - minDate || 1;

			// BP scale (left y-axis)
			const bpValues = bpData.flatMap(d => [d.value, d.valueSecondary ?? d.value]);
			const bpMin = Math.min(...bpValues) - 10;
			const bpMax = Math.max(...bpValues) + 10;

			// HR scale (right y-axis)
			const hrValues = hrData.map(d => d.value);
			const hrMin = Math.min(...hrValues) - 10;
			const hrMax = Math.max(...hrValues) + 10;

			const xScale = (date: Date) =>
				margin.left + ((date.getTime() - minDate) / dateRange) * innerWidth;
			const yScaleBP = (value: number) =>
				margin.top + innerHeight - ((value - bpMin) / (bpMax - bpMin)) * innerHeight;
			const yScaleHR = (value: number) =>
				margin.top + innerHeight - ((value - hrMin) / (hrMax - hrMin)) * innerHeight;

			// Draw axes
			const xAxis = document.createElementNS('http://www.w3.org/2000/svg', 'line');
			xAxis.setAttribute('x1', String(margin.left));
			xAxis.setAttribute('y1', String(margin.top + innerHeight));
			xAxis.setAttribute('x2', String(margin.left + innerWidth));
			xAxis.setAttribute('y2', String(margin.top + innerHeight));
			xAxis.setAttribute('stroke', '#9ca3af');
			svg.appendChild(xAxis);

			// Left Y-axis (BP)
			const yAxisLeft = document.createElementNS('http://www.w3.org/2000/svg', 'line');
			yAxisLeft.setAttribute('x1', String(margin.left));
			yAxisLeft.setAttribute('y1', String(margin.top));
			yAxisLeft.setAttribute('x2', String(margin.left));
			yAxisLeft.setAttribute('y2', String(margin.top + innerHeight));
			yAxisLeft.setAttribute('stroke', '#3b82f6');
			yAxisLeft.setAttribute('stroke-width', '2');
			svg.appendChild(yAxisLeft);

			// Right Y-axis (HR)
			const yAxisRight = document.createElementNS('http://www.w3.org/2000/svg', 'line');
			yAxisRight.setAttribute('x1', String(margin.left + innerWidth));
			yAxisRight.setAttribute('y1', String(margin.top));
			yAxisRight.setAttribute('x2', String(margin.left + innerWidth));
			yAxisRight.setAttribute('y2', String(margin.top + innerHeight));
			yAxisRight.setAttribute('stroke', '#f59e0b');
			yAxisRight.setAttribute('stroke-width', '2');
			svg.appendChild(yAxisRight);

			// Y-axis labels - Left (BP)
			const bpLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			bpLabel.setAttribute('x', String(margin.left - 45));
			bpLabel.setAttribute('y', String(margin.top + innerHeight / 2));
			bpLabel.setAttribute('text-anchor', 'middle');
			bpLabel.setAttribute('font-size', '11');
			bpLabel.setAttribute('fill', '#3b82f6');
			bpLabel.setAttribute('transform', `rotate(-90, ${margin.left - 45}, ${margin.top + innerHeight / 2})`);
			bpLabel.textContent = 'BP (mmHg)';
			svg.appendChild(bpLabel);

			// Y-axis labels - Right (HR)
			const hrLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			hrLabel.setAttribute('x', String(margin.left + innerWidth + 45));
			hrLabel.setAttribute('y', String(margin.top + innerHeight / 2));
			hrLabel.setAttribute('text-anchor', 'middle');
			hrLabel.setAttribute('font-size', '11');
			hrLabel.setAttribute('fill', '#f59e0b');
			hrLabel.setAttribute('transform', `rotate(90, ${margin.left + innerWidth + 45}, ${margin.top + innerHeight / 2})`);
			hrLabel.textContent = 'HR (bpm)';
			svg.appendChild(hrLabel);

			// Y-axis tick labels - Left (BP)
			const bpTicks = [bpMin, (bpMin + bpMax) / 2, bpMax];
			for (const tick of bpTicks) {
				const tickLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
				tickLabel.setAttribute('x', String(margin.left - 8));
				tickLabel.setAttribute('y', String(yScaleBP(tick) + 4));
				tickLabel.setAttribute('text-anchor', 'end');
				tickLabel.setAttribute('font-size', '10');
				tickLabel.setAttribute('fill', '#3b82f6');
				tickLabel.textContent = Math.round(tick).toString();
				svg.appendChild(tickLabel);
			}

			// Y-axis tick labels - Right (HR)
			const hrTicks = [hrMin, (hrMin + hrMax) / 2, hrMax];
			for (const tick of hrTicks) {
				const tickLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
				tickLabel.setAttribute('x', String(margin.left + innerWidth + 8));
				tickLabel.setAttribute('y', String(yScaleHR(tick) + 4));
				tickLabel.setAttribute('text-anchor', 'start');
				tickLabel.setAttribute('font-size', '10');
				tickLabel.setAttribute('fill', '#f59e0b');
				tickLabel.textContent = Math.round(tick).toString();
				svg.appendChild(tickLabel);
			}

			// Draw BP systolic line
			if (bpData.length > 1) {
				const pathD = bpData
					.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(d.date)} ${yScaleBP(d.value)}`)
					.join(' ');
				const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
				path.setAttribute('d', pathD);
				path.setAttribute('fill', 'none');
				path.setAttribute('stroke', '#3b82f6');
				path.setAttribute('stroke-width', '2');
				svg.appendChild(path);
			}

			// Draw BP diastolic line
			if (bpData.length > 1) {
				const pathD = bpData
					.filter(d => d.valueSecondary !== undefined)
					.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(d.date)} ${yScaleBP(d.valueSecondary!)}`)
					.join(' ');
				const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
				path.setAttribute('d', pathD);
				path.setAttribute('fill', 'none');
				path.setAttribute('stroke', '#10b981');
				path.setAttribute('stroke-width', '2');
				svg.appendChild(path);
			}

			// Draw HR line
			if (hrData.length > 1) {
				const pathD = hrData
					.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(d.date)} ${yScaleHR(d.value)}`)
					.join(' ');
				const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
				path.setAttribute('d', pathD);
				path.setAttribute('fill', 'none');
				path.setAttribute('stroke', '#f59e0b');
				path.setAttribute('stroke-width', '2');
				path.setAttribute('stroke-dasharray', '5,3');
				svg.appendChild(path);
			}

			// Draw BP points
			for (const d of bpData) {
				const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
				circle.setAttribute('cx', String(xScale(d.date)));
				circle.setAttribute('cy', String(yScaleBP(d.value)));
				circle.setAttribute('r', '4');
				circle.setAttribute('fill', '#3b82f6');
				svg.appendChild(circle);

				if (d.valueSecondary !== undefined) {
					const circle2 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
					circle2.setAttribute('cx', String(xScale(d.date)));
					circle2.setAttribute('cy', String(yScaleBP(d.valueSecondary)));
					circle2.setAttribute('r', '4');
					circle2.setAttribute('fill', '#10b981');
					svg.appendChild(circle2);
				}
			}

			// Draw HR points
			for (const d of hrData) {
				const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
				circle.setAttribute('cx', String(xScale(d.date)));
				circle.setAttribute('cy', String(yScaleHR(d.value)));
				circle.setAttribute('r', '4');
				circle.setAttribute('fill', '#f59e0b');
				svg.appendChild(circle);
			}

			// Date labels
			const uniqueDates = [...new Set(allDates)];
			for (const dateTime of uniqueDates) {
				const date = new Date(dateTime);
				const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
				text.setAttribute('x', String(xScale(date)));
				text.setAttribute('y', String(margin.top + innerHeight + 15));
				text.setAttribute('text-anchor', 'middle');
				text.setAttribute('font-size', '10');
				text.setAttribute('fill', '#6b7280');
				text.textContent = formatDate(date.toISOString());
				svg.appendChild(text);

				// Time label
				const timeText = document.createElementNS('http://www.w3.org/2000/svg', 'text');
				timeText.setAttribute('x', String(xScale(date)));
				timeText.setAttribute('y', String(margin.top + innerHeight + 28));
				timeText.setAttribute('text-anchor', 'middle');
				timeText.setAttribute('font-size', '9');
				timeText.setAttribute('fill', '#9ca3af');
				timeText.textContent = date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
				svg.appendChild(timeText);
			}

			// Legend
			const legendY = margin.top - 5;
			// BP Systolic
			const leg1 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			leg1.setAttribute('cx', String(margin.left + 10));
			leg1.setAttribute('cy', String(legendY));
			leg1.setAttribute('r', '4');
			leg1.setAttribute('fill', '#3b82f6');
			svg.appendChild(leg1);
			const leg1Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			leg1Text.setAttribute('x', String(margin.left + 20));
			leg1Text.setAttribute('y', String(legendY + 4));
			leg1Text.setAttribute('font-size', '10');
			leg1Text.setAttribute('fill', '#6b7280');
			leg1Text.textContent = 'Systolic';
			svg.appendChild(leg1Text);

			// BP Diastolic
			const leg2 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			leg2.setAttribute('cx', String(margin.left + 70));
			leg2.setAttribute('cy', String(legendY));
			leg2.setAttribute('r', '4');
			leg2.setAttribute('fill', '#10b981');
			svg.appendChild(leg2);
			const leg2Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			leg2Text.setAttribute('x', String(margin.left + 80));
			leg2Text.setAttribute('y', String(legendY + 4));
			leg2Text.setAttribute('font-size', '10');
			leg2Text.setAttribute('fill', '#6b7280');
			leg2Text.textContent = 'Diastolic';
			svg.appendChild(leg2Text);

			// HR
			const leg3 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			leg3.setAttribute('cx', String(margin.left + 140));
			leg3.setAttribute('cy', String(legendY));
			leg3.setAttribute('r', '4');
			leg3.setAttribute('fill', '#f59e0b');
			svg.appendChild(leg3);
			const leg3Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			leg3Text.setAttribute('x', String(margin.left + 150));
			leg3Text.setAttribute('y', String(legendY + 4));
			leg3Text.setAttribute('font-size', '10');
			leg3Text.setAttribute('fill', '#6b7280');
			leg3Text.textContent = 'HR';
			svg.appendChild(leg3Text);

			graphContainer.appendChild(svg);
			return;
		}

		// Single metric graph (original logic with units added)
		const data = getDataForMetric(selectedMetric);
		if (data.length === 0) return;

		const unit = getUnitForMetric(selectedMetric);

		// Create scales
		const minDate = Math.min(...data.map((d) => d.date.getTime()));
		const maxDate = Math.max(...data.map((d) => d.date.getTime()));
		const dateRange = maxDate - minDate || 1;

		const maxValue = Math.max(...data.map((d) => Math.max(d.value, d.valueSecondary ?? 0)));
		const minValue = Math.min(...data.map((d) => Math.min(d.value, d.valueSecondary ?? d.value)));
		const valueRange = maxValue - minValue || 1;
		const valuePadding = valueRange * 0.1;

		const xScale = (date: Date) =>
			margin.left + ((date.getTime() - minDate) / dateRange) * innerWidth;
		const yScale = (value: number) =>
			margin.top + innerHeight - ((value - minValue + valuePadding) / (valueRange + valuePadding * 2)) * innerHeight;

		// Draw axes
		const xAxis = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		xAxis.setAttribute('x1', String(margin.left));
		xAxis.setAttribute('y1', String(margin.top + innerHeight));
		xAxis.setAttribute('x2', String(margin.left + innerWidth));
		xAxis.setAttribute('y2', String(margin.top + innerHeight));
		xAxis.setAttribute('stroke', '#9ca3af');
		svg.appendChild(xAxis);

		const yAxis = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		yAxis.setAttribute('x1', String(margin.left));
		yAxis.setAttribute('y1', String(margin.top));
		yAxis.setAttribute('x2', String(margin.left));
		yAxis.setAttribute('y2', String(margin.top + innerHeight));
		yAxis.setAttribute('stroke', '#9ca3af');
		svg.appendChild(yAxis);

		// Y-axis label with unit
		if (unit) {
			const yLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			yLabel.setAttribute('x', String(margin.left - 40));
			yLabel.setAttribute('y', String(margin.top + innerHeight / 2));
			yLabel.setAttribute('text-anchor', 'middle');
			yLabel.setAttribute('font-size', '11');
			yLabel.setAttribute('fill', '#6b7280');
			yLabel.setAttribute('transform', `rotate(-90, ${margin.left - 40}, ${margin.top + innerHeight / 2})`);
			yLabel.textContent = `${selectedMetric} (${unit})`;
			svg.appendChild(yLabel);
		}

		// Y-axis tick labels
		const yTicks = [minValue - valuePadding, (minValue + maxValue) / 2, maxValue + valuePadding];
		for (const tick of yTicks) {
			const tickLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			tickLabel.setAttribute('x', String(margin.left - 8));
			tickLabel.setAttribute('y', String(yScale(tick) + 4));
			tickLabel.setAttribute('text-anchor', 'end');
			tickLabel.setAttribute('font-size', '10');
			tickLabel.setAttribute('fill', '#6b7280');
			tickLabel.textContent = Math.round(tick).toString();
			svg.appendChild(tickLabel);
		}

		// Draw primary line
		if (data.length > 1) {
			const pathD = data
				.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(d.date)} ${yScale(d.value)}`)
				.join(' ');

			const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
			path.setAttribute('d', pathD);
			path.setAttribute('fill', 'none');
			path.setAttribute('stroke', '#3b82f6');
			path.setAttribute('stroke-width', '2');
			svg.appendChild(path);
		}

		// Draw secondary line (for BP diastolic)
		const hasSecondary = data.some((d) => d.valueSecondary !== undefined);
		if (hasSecondary && data.length > 1) {
			const pathD = data
				.filter((d) => d.valueSecondary !== undefined)
				.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(d.date)} ${yScale(d.valueSecondary!)}`)
				.join(' ');

			const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
			path.setAttribute('d', pathD);
			path.setAttribute('fill', 'none');
			path.setAttribute('stroke', '#10b981');
			path.setAttribute('stroke-width', '2');
			svg.appendChild(path);
		}

		// Draw points
		for (const d of data) {
			const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			circle.setAttribute('cx', String(xScale(d.date)));
			circle.setAttribute('cy', String(yScale(d.value)));
			circle.setAttribute('r', '4');
			circle.setAttribute('fill', '#3b82f6');
			svg.appendChild(circle);

			if (d.valueSecondary !== undefined) {
				const circle2 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
				circle2.setAttribute('cx', String(xScale(d.date)));
				circle2.setAttribute('cy', String(yScale(d.valueSecondary)));
				circle2.setAttribute('r', '4');
				circle2.setAttribute('fill', '#10b981');
				svg.appendChild(circle2);
			}

			// Date label
			const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			text.setAttribute('x', String(xScale(d.date)));
			text.setAttribute('y', String(margin.top + innerHeight + 15));
			text.setAttribute('text-anchor', 'middle');
			text.setAttribute('font-size', '10');
			text.setAttribute('fill', '#6b7280');
			text.textContent = formatDate(d.date.toISOString());
			svg.appendChild(text);

			// Time label
			const timeText = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			timeText.setAttribute('x', String(xScale(d.date)));
			timeText.setAttribute('y', String(margin.top + innerHeight + 28));
			timeText.setAttribute('text-anchor', 'middle');
			timeText.setAttribute('font-size', '9');
			timeText.setAttribute('fill', '#9ca3af');
			timeText.textContent = d.date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
			svg.appendChild(timeText);
		}

		graphContainer.appendChild(svg);
	}

	$effect(() => {
		if (activeTab === 'graphs' && selectedMetric) {
			drawGraph();
		}
	});

	$effect(() => {
		// Set default selected metric when switching to graphs tab
		if (activeTab === 'graphs' && !selectedMetric && allMetrics().length > 0) {
			selectedMetric = allMetrics()[0];
		}
	});

	onMount(() => {
		if (allMetrics().length > 0) {
			selectedMetric = allMetrics()[0];
		}
	});
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full flex flex-col">
	<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
		<i class="fa-solid fa-chart-line text-teal-500"></i>
		Vitals & Labs
	</h3>

	<div class="flex-1 min-h-0">
		<TabContainer {tabs} bind:activeTab>
			{#snippet children(currentTab)}
				{#if currentTab === 'table'}
					<!-- Table View -->
					<div class="overflow-auto h-full">
						{#if tableData().length === 0}
							<p class="text-sm text-gray-500 dark:text-gray-400 italic">No data recorded</p>
						{:else}
							<table class="min-w-full text-sm">
								<thead class="sticky top-0 bg-gray-100 dark:bg-gray-700">
									<tr>
										<th class="px-2 py-1 text-left font-medium text-gray-700 dark:text-gray-300 whitespace-nowrap">
											Metric
										</th>
										{#each allTimestamps() as ts}
											<th class="px-2 py-1 text-center font-medium text-gray-700 dark:text-gray-300">
												<div class="whitespace-nowrap">{formatDate(ts)}</div>
												<div class="text-xs font-normal text-gray-500 dark:text-gray-400 whitespace-nowrap">
													{new Date(ts).toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}
												</div>
											</th>
										{/each}
									</tr>
								</thead>
								<tbody>
									{#each tableData() as row}
										<tr class="border-b border-gray-200 dark:border-gray-600">
											<td class="px-2 py-1 font-medium text-gray-800 dark:text-gray-200 whitespace-nowrap">
												{row.metric}
												{#if row.unit}
													<span class="text-xs text-gray-500 dark:text-gray-400">({row.unit})</span>
												{/if}
											</td>
											{#each allTimestamps() as ts}
												<td
													class="px-2 py-1 text-center whitespace-nowrap
														{row[`${ts}_abnormal`] ? 'text-red-600 dark:text-red-400 font-medium' : 'text-gray-600 dark:text-gray-400'}"
												>
													{row[ts] ?? '-'}
												</td>
											{/each}
										</tr>
									{/each}
								</tbody>
							</table>
						{/if}
					</div>
				{:else if currentTab === 'graphs'}
					<!-- Graph View -->
					<div class="h-full flex flex-col">
						<!-- Metric buttons -->
						<div class="flex flex-wrap gap-2 mb-3">
							{#each allMetrics() as metric}
								<button
									class="px-3 py-1 text-xs rounded-full transition-colors
										{selectedMetric === metric
										? 'bg-blue-500 text-white'
										: 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'}"
									onclick={() => (selectedMetric = metric)}
								>
									{metric}
								</button>
							{/each}
						</div>

						<!-- Graph container -->
						<div bind:this={graphContainer} class="flex-1 min-h-0 bg-gray-50 dark:bg-gray-700 rounded-lg p-2">
							{#if !selectedMetric}
								<p class="text-sm text-gray-500 dark:text-gray-400 italic text-center mt-4">
									Select a metric to view graph
								</p>
							{/if}
						</div>
					</div>
				{/if}
			{/snippet}
		</TabContainer>
	</div>
</div>
