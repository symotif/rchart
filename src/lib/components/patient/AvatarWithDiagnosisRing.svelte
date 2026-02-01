<script lang="ts">
	import { onMount } from 'svelte';
	import type { Patient, DiagnosisWithMedications, DiagnosisCategory, ClinicalScore } from '$lib/types/patient';
	import { CATEGORY_COLORS, CATEGORY_NAMES } from '$lib/types/patient';

	let {
		patient,
		diagnoses,
		scores = []
	}: {
		patient: Patient;
		diagnoses: DiagnosisWithMedications[];
		scores?: ClinicalScore[];
	} = $props();

	let svgRef: SVGSVGElement | null = $state(null);
	let isExpanded = $state(false);
	let isHovered = $state(false);

	// Ring dimensions
	const baseSize = 80;
	const hoverSize = 90;
	const ringWidth = 8;

	let currentSize = $derived(isHovered ? hoverSize : baseSize);

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

	function getInitials(firstName: string, lastName: string): string {
		return `${firstName.charAt(0)}${lastName.charAt(0)}`.toUpperCase();
	}

	function drawRing() {
		if (!svgRef || total === 0) return;

		const svg = svgRef;
		// Clear existing
		while (svg.firstChild) {
			svg.removeChild(svg.firstChild);
		}

		const size = currentSize;
		const radius = size / 2 - ringWidth / 2 - 2;
		const centerX = size / 2;
		const centerY = size / 2;
		const innerRadius = radius - ringWidth;

		let startAngle = -Math.PI / 2; // Start from top

		for (const data of categoryData()) {
			const sliceAngle = (data.count / total) * 2 * Math.PI;
			const endAngle = startAngle + sliceAngle;

			// Calculate path for donut arc
			const x1Outer = centerX + radius * Math.cos(startAngle);
			const y1Outer = centerY + radius * Math.sin(startAngle);
			const x2Outer = centerX + radius * Math.cos(endAngle);
			const y2Outer = centerY + radius * Math.sin(endAngle);

			const x1Inner = centerX + innerRadius * Math.cos(endAngle);
			const y1Inner = centerY + innerRadius * Math.sin(endAngle);
			const x2Inner = centerX + innerRadius * Math.cos(startAngle);
			const y2Inner = centerY + innerRadius * Math.sin(startAngle);

			const largeArcFlag = sliceAngle > Math.PI ? 1 : 0;

			const pathD = [
				`M ${x1Outer} ${y1Outer}`,
				`A ${radius} ${radius} 0 ${largeArcFlag} 1 ${x2Outer} ${y2Outer}`,
				`L ${x1Inner} ${y1Inner}`,
				`A ${innerRadius} ${innerRadius} 0 ${largeArcFlag} 0 ${x2Inner} ${y2Inner}`,
				'Z'
			].join(' ');

			const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
			path.setAttribute('d', pathD);
			path.setAttribute('fill', data.color);

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
		// Redraw when data or size changes
		categoryData();
		currentSize;
		drawRing();
	});

	onMount(() => {
		drawRing();
	});
</script>

<div class="relative inline-block">
	<!-- Avatar with Ring Container -->
	<button
		onclick={toggleExpanded}
		onmouseenter={() => (isHovered = true)}
		onmouseleave={() => (isHovered = false)}
		class="relative cursor-pointer transition-transform duration-200"
		style="width: {hoverSize}px; height: {hoverSize}px;"
		title="Click to see diagnosis categories and severity scores"
	>
		<!-- SVG Ring -->
		<svg
			bind:this={svgRef}
			width={currentSize}
			height={currentSize}
			viewBox="0 0 {currentSize} {currentSize}"
			class="absolute transition-all duration-200"
			style="top: {(hoverSize - currentSize) / 2}px; left: {(hoverSize - currentSize) / 2}px;"
		></svg>

		<!-- Avatar in the center -->
		<div
			class="absolute rounded-full overflow-hidden transition-all duration-200"
			style="
				width: {currentSize - ringWidth * 2 - 8}px;
				height: {currentSize - ringWidth * 2 - 8}px;
				top: {(hoverSize - (currentSize - ringWidth * 2 - 8)) / 2}px;
				left: {(hoverSize - (currentSize - ringWidth * 2 - 8)) / 2}px;
			"
		>
			{#if patient.photo_url}
				<img
					src={patient.photo_url}
					alt="{patient.first_name} {patient.last_name}"
					class="w-full h-full object-cover"
				/>
			{:else}
				<div
					class="w-full h-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-lg font-bold"
				>
					{getInitials(patient.first_name, patient.last_name)}
				</div>
			{/if}
		</div>
	</button>

	<!-- Expanded Panel -->
	{#if isExpanded}
		<!-- Click-away overlay -->
		<div class="fixed inset-0 z-40" onclick={toggleExpanded} role="presentation"></div>

		<div class="absolute top-full left-0 mt-2 w-64 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-600 z-50 p-4">
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
				{#if categoryData().length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No diagnoses</p>
				{:else}
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
				{/if}
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
