<script lang="ts">
	import { onMount } from 'svelte';
	import type { DiagnosisWithMedications, DiagnosisCategory, ClinicalScore } from '$lib/types/patient';
	import { CATEGORY_COLORS, CATEGORY_NAMES } from '$lib/types/patient';

	let { diagnoses, scores = [] }: { diagnoses: DiagnosisWithMedications[]; scores?: ClinicalScore[] } = $props();

	let svgRef: SVGSVGElement | null = $state(null);
	let isExpanded = $state(false);

	// Count diagnoses by category
	const categoryCounts = $derived(() => {
		const counts = new Map<DiagnosisCategory, number>();
		for (const d of diagnoses) {
			const cat = d.diagnosis.category;
			if (cat) {
				counts.set(cat, (counts.get(cat) || 0) + 1);
			}
		}
		return counts;
	});

	// Convert to array for rendering
	const categoryData = $derived(() => {
		const data: { category: DiagnosisCategory; count: number; color: string; name: string }[] = [];
		categoryCounts().forEach((count, category) => {
			data.push({
				category,
				count,
				color: CATEGORY_COLORS[category],
				name: CATEGORY_NAMES[category]
			});
		});
		// Sort by count descending
		return data.sort((a, b) => b.count - a.count);
	});

	const total = $derived(categoryData().reduce((sum, d) => sum + d.count, 0));

	// Get the latest score for each type
	const latestScores = $derived(() => {
		const latest = new Map<string, ClinicalScore>();
		for (const score of scores) {
			const existing = latest.get(score.score_type);
			if (!existing || new Date(score.recorded_at) > new Date(existing.recorded_at)) {
				latest.set(score.score_type, score);
			}
		}
		return Array.from(latest.values());
	});

	function drawPieChart() {
		if (!svgRef || total === 0) return;

		const svg = svgRef;
		// Clear existing
		while (svg.firstChild) {
			svg.removeChild(svg.firstChild);
		}

		const width = 70;
		const height = 70;
		const radius = Math.min(width, height) / 2 - 2;
		const centerX = width / 2;
		const centerY = height / 2;

		let startAngle = -Math.PI / 2; // Start from top

		for (const data of categoryData()) {
			const sliceAngle = (data.count / total) * 2 * Math.PI;
			const endAngle = startAngle + sliceAngle;

			// Calculate path for pie slice
			const x1 = centerX + radius * Math.cos(startAngle);
			const y1 = centerY + radius * Math.sin(startAngle);
			const x2 = centerX + radius * Math.cos(endAngle);
			const y2 = centerY + radius * Math.sin(endAngle);

			const largeArcFlag = sliceAngle > Math.PI ? 1 : 0;

			const pathD = [
				`M ${centerX} ${centerY}`,
				`L ${x1} ${y1}`,
				`A ${radius} ${radius} 0 ${largeArcFlag} 1 ${x2} ${y2}`,
				'Z'
			].join(' ');

			const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
			path.setAttribute('d', pathD);
			path.setAttribute('fill', data.color);
			path.setAttribute('stroke', 'white');
			path.setAttribute('stroke-width', '1');

			// Add tooltip via title element
			const title = document.createElementNS('http://www.w3.org/2000/svg', 'title');
			title.textContent = `${data.name}: ${data.count}`;
			path.appendChild(title);

			svg.appendChild(path);

			startAngle = endAngle;
		}
	}

	function getScoreColor(score: ClinicalScore): string {
		if (!score.max_score) return 'text-gray-600 dark:text-gray-400';
		const ratio = score.score / score.max_score;
		if (ratio >= 0.7) return 'text-red-600 dark:text-red-400';
		if (ratio >= 0.4) return 'text-yellow-600 dark:text-yellow-400';
		return 'text-green-600 dark:text-green-400';
	}

	function toggleExpanded() {
		isExpanded = !isExpanded;
	}

	$effect(() => {
		// Redraw when data changes
		categoryData();
		drawPieChart();
	});

	onMount(() => {
		drawPieChart();
	});
</script>

<div class="relative">
	<!-- Pie Chart (clickable) -->
	<button
		onclick={toggleExpanded}
		class="flex items-center justify-center cursor-pointer hover:opacity-80 transition-opacity p-1 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700"
		title="Click to expand details"
	>
		<svg
			bind:this={svgRef}
			width="70"
			height="70"
			viewBox="0 0 70 70"
			class="flex-shrink-0"
		></svg>
	</button>

	<!-- Expanded Panel -->
	{#if isExpanded}
		<div class="absolute top-full right-0 mt-2 w-64 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-600 z-50 p-4">
			<!-- Close button -->
			<button
				onclick={toggleExpanded}
				class="absolute top-2 right-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
				aria-label="Close"
			>
				<i class="fa-solid fa-times"></i>
			</button>

			<!-- Legend Section -->
			<div class="mb-4">
				<h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2">
					Problem Categories
				</h4>
				<div class="space-y-1">
					{#each categoryData() as data}
						<div class="flex items-center justify-between">
							<div class="flex items-center gap-2">
								<div
									class="w-3 h-3 rounded-sm flex-shrink-0"
									style="background-color: {data.color}"
								></div>
								<span class="text-sm text-gray-700 dark:text-gray-300">
									{data.name}
								</span>
							</div>
							<span class="text-sm font-medium text-gray-600 dark:text-gray-400">
								{data.count}
							</span>
						</div>
					{/each}
				</div>
			</div>

			<!-- Clinical Scores Section -->
			{#if latestScores().length > 0}
				<div class="border-t border-gray-200 dark:border-gray-600 pt-3">
					<h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2">
						Severity Scores
					</h4>
					<div class="space-y-2">
						{#each latestScores() as score}
							<div class="flex items-center justify-between">
								<span class="text-sm text-gray-700 dark:text-gray-300">
									{score.score_type}
								</span>
								<div class="flex items-center gap-2">
									<span class="text-sm font-bold {getScoreColor(score)}">
										{score.score}{score.max_score ? `/${score.max_score}` : ''}
									</span>
									{#if score.interpretation}
										<span class="text-xs text-gray-500 dark:text-gray-400">
											({score.interpretation})
										</span>
									{/if}
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>
