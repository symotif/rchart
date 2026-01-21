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

	// Get all unique metrics for the graph buttons
	const allMetrics = $derived(() => {
		const vitalTypes = [...new Set(vitals.map((v) => v.vital_type))];
		const labNames = [...new Set(labs.map((l) => l.name))];
		const scoreTypes = [...new Set(scores.map((s) => s.score_type))];
		return [...vitalTypes, ...labNames, ...scoreTypes];
	});

	// Get all unique timestamps for table columns
	const allTimestamps = $derived(() => {
		const timestamps = new Set<string>();
		vitals.forEach((v) => timestamps.add(v.recorded_at));
		labs.forEach((l) => timestamps.add(l.recorded_at));
		scores.forEach((s) => timestamps.add(s.recorded_at));
		return [...timestamps].sort((a, b) => new Date(a).getTime() - new Date(b).getTime());
	});

	// Build table data: rows are metrics, columns are timestamps
	const tableData = $derived(() => {
		const metrics = allMetrics();
		const timestamps = allTimestamps();

		return metrics.map((metric) => {
			const row: Record<string, string | number | boolean | null> = { metric };

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

	async function drawGraph() {
		await tick();
		if (!graphContainer || !selectedMetric) return;

		const data = getDataForMetric(selectedMetric);
		if (data.length === 0) return;

		// Clear existing content
		graphContainer.innerHTML = '';

		const width = graphContainer.clientWidth;
		const height = graphContainer.clientHeight - 20;
		const margin = { top: 20, right: 30, bottom: 40, left: 50 };
		const innerWidth = width - margin.left - margin.right;
		const innerHeight = height - margin.top - margin.bottom;

		// Create SVG
		const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
		svg.setAttribute('width', String(width));
		svg.setAttribute('height', String(height));
		svg.classList.add('overflow-visible');

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
			text.setAttribute('y', String(margin.top + innerHeight + 20));
			text.setAttribute('text-anchor', 'middle');
			text.setAttribute('font-size', '10');
			text.setAttribute('fill', '#6b7280');
			text.textContent = formatDate(d.date.toISOString());
			svg.appendChild(text);
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
											<th class="px-2 py-1 text-center font-medium text-gray-700 dark:text-gray-300 whitespace-nowrap">
												{formatDate(ts)}
											</th>
										{/each}
									</tr>
								</thead>
								<tbody>
									{#each tableData() as row}
										<tr class="border-b border-gray-200 dark:border-gray-600">
											<td class="px-2 py-1 font-medium text-gray-800 dark:text-gray-200 whitespace-nowrap">
												{row.metric}
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
