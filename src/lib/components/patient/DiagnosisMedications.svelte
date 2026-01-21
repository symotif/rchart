<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type { DiagnosisWithMedications, Medication, DiagnosisCategory } from '$lib/types/patient';
	import { CATEGORY_COLORS } from '$lib/types/patient';

	let {
		diagnoses,
		medications
	}: {
		diagnoses: DiagnosisWithMedications[];
		medications: Medication[];
	} = $props();

	let containerRef: HTMLDivElement | null = $state(null);
	let svgRef: SVGSVGElement | null = $state(null);
	let hoveredDiagnosisId: number | null = $state(null);
	let hoveredMedicationId: number | null = $state(null);

	// Get category color for a diagnosis
	function getCategoryColor(category: DiagnosisCategory | null | undefined): string {
		if (!category) return '#6b7280'; // Default gray
		return CATEGORY_COLORS[category] || '#6b7280';
	}

	// Build a map of medication IDs to diagnoses that use them
	const medicationToDiagnoses = $derived(() => {
		const map = new Map<number, number[]>();
		for (const d of diagnoses) {
			for (const medId of d.medication_ids) {
				if (!map.has(medId)) {
					map.set(medId, []);
				}
				map.get(medId)!.push(d.diagnosis.id!);
			}
		}
		return map;
	});

	function getMedicationById(id: number): Medication | undefined {
		return medications.find((m) => m.id === id);
	}

	function isConnectionHighlighted(diagId: number, medId: number): boolean {
		if (hoveredDiagnosisId === diagId) return true;
		if (hoveredMedicationId === medId) return true;
		return false;
	}

	function isDiagnosisHighlighted(diagId: number): boolean {
		if (hoveredDiagnosisId === diagId) return true;
		if (hoveredMedicationId !== null) {
			const diagIds = medicationToDiagnoses().get(hoveredMedicationId);
			return diagIds?.includes(diagId) ?? false;
		}
		return false;
	}

	function isMedicationHighlighted(medId: number): boolean {
		if (hoveredMedicationId === medId) return true;
		if (hoveredDiagnosisId !== null) {
			const diag = diagnoses.find((d) => d.diagnosis.id === hoveredDiagnosisId);
			return diag?.medication_ids.includes(medId) ?? false;
		}
		return false;
	}

	async function drawConnections() {
		await tick();
		if (!svgRef || !containerRef) return;

		const svg = svgRef;
		// Clear existing paths
		while (svg.firstChild) {
			svg.removeChild(svg.firstChild);
		}

		const containerRect = containerRef.getBoundingClientRect();

		for (const d of diagnoses) {
			const diagEl = containerRef.querySelector(`[data-diagnosis-id="${d.diagnosis.id}"]`);
			if (!diagEl) continue;

			const diagRect = diagEl.getBoundingClientRect();

			for (const medId of d.medication_ids) {
				const medEl = containerRef.querySelector(`[data-medication-id="${medId}"]`);
				if (!medEl) continue;

				const medRect = medEl.getBoundingClientRect();

				// Calculate positions relative to container
				const x1 = diagRect.right - containerRect.left;
				const y1 = diagRect.top + diagRect.height / 2 - containerRect.top;
				const x2 = medRect.left - containerRect.left;
				const y2 = medRect.top + medRect.height / 2 - containerRect.top;

				// Create curved path
				const midX = (x1 + x2) / 2;
				const pathD = `M ${x1} ${y1} C ${midX} ${y1}, ${midX} ${y2}, ${x2} ${y2}`;

				const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
				path.setAttribute('d', pathD);
				path.setAttribute('fill', 'none');
				path.setAttribute('stroke-width', '2');
				path.setAttribute('data-diag', String(d.diagnosis.id));
				path.setAttribute('data-med', String(medId));
				path.classList.add('connection-line');

				// Set color based on highlight state
				const highlighted = isConnectionHighlighted(d.diagnosis.id!, medId);
				path.setAttribute('stroke', highlighted ? '#6366f1' : '#d1d5db');
				path.setAttribute('opacity', highlighted ? '1' : '0.5');

				svg.appendChild(path);
			}
		}
	}

	$effect(() => {
		// Redraw when hover state changes
		hoveredDiagnosisId;
		hoveredMedicationId;
		drawConnections();
	});

	onMount(() => {
		drawConnections();
		// Redraw on window resize
		const handleResize = () => drawConnections();
		window.addEventListener('resize', handleResize);
		return () => window.removeEventListener('resize', handleResize);
	});
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full flex flex-col">
	<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
		<i class="fa-solid fa-pills text-orange-500"></i>
		Diagnoses & Medications
	</h3>

	<div bind:this={containerRef} class="flex-1 relative overflow-hidden">
		<!-- SVG for connection lines -->
		<svg
			bind:this={svgRef}
			class="absolute inset-0 w-full h-full pointer-events-none"
			style="z-index: 0;"
		></svg>

		<div class="flex gap-8 h-full relative" style="z-index: 1;">
			<!-- Diagnoses column -->
			<div class="flex-1 space-y-2 overflow-y-auto">
				<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2">
					Diagnoses
				</h4>
				{#if diagnoses.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No active diagnoses</p>
				{:else}
					{#each diagnoses as d}
						{@const categoryColor = getCategoryColor(d.diagnosis.category)}
						<div
							data-diagnosis-id={d.diagnosis.id}
							class="p-3 rounded-lg border-l-4 border transition-all cursor-pointer
								{isDiagnosisHighlighted(d.diagnosis.id!)
								? 'bg-indigo-50 dark:bg-indigo-900/30 border-r-indigo-400 border-t-indigo-400 border-b-indigo-400 dark:border-r-indigo-500 dark:border-t-indigo-500 dark:border-b-indigo-500'
								: 'bg-gray-50 dark:bg-gray-700 border-r-gray-200 border-t-gray-200 border-b-gray-200 dark:border-r-gray-600 dark:border-t-gray-600 dark:border-b-gray-600'}"
							style="border-left-color: {categoryColor};"
							onmouseenter={() => (hoveredDiagnosisId = d.diagnosis.id)}
							onmouseleave={() => (hoveredDiagnosisId = null)}
							role="button"
							tabindex="0"
						>
							<div class="font-medium text-gray-800 dark:text-gray-200 text-sm">
								{d.diagnosis.name}
							</div>
							{#if d.diagnosis.icd_code}
								<div class="text-xs text-gray-500 dark:text-gray-400">
									{d.diagnosis.icd_code}
								</div>
							{/if}
						</div>
					{/each}
				{/if}
			</div>

			<!-- Medications column -->
			<div class="flex-1 space-y-2 overflow-y-auto">
				<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2">
					Medications
				</h4>
				{#if medications.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No active medications</p>
				{:else}
					{#each medications as med}
						<div
							data-medication-id={med.id}
							class="p-3 rounded-lg border-2 transition-all cursor-pointer
								{isMedicationHighlighted(med.id!)
								? 'bg-indigo-50 dark:bg-indigo-900/30 border-indigo-400 dark:border-indigo-500'
								: 'bg-gray-50 dark:bg-gray-700 border-gray-200 dark:border-gray-600'}"
							onmouseenter={() => (hoveredMedicationId = med.id)}
							onmouseleave={() => (hoveredMedicationId = null)}
							role="button"
							tabindex="0"
						>
							<div class="font-medium text-gray-800 dark:text-gray-200 text-sm">
								{med.name}
							</div>
							<div class="text-xs text-gray-500 dark:text-gray-400">
								{med.dose ?? ''} {med.frequency ?? ''}
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>
	</div>
</div>
