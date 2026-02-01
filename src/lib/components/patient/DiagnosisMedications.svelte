<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { DiagnosisWithMedications, Medication, DiagnosisCategory, Prescription } from '$lib/types/patient';
	import { CATEGORY_COLORS } from '$lib/types/patient';
	import { ToastStore } from '../../../stores/ToastStore';

	let {
		diagnoses,
		medications,
		patientId
	}: {
		diagnoses: DiagnosisWithMedications[];
		medications: Medication[];
		patientId: number;
	} = $props();

	let containerRef: HTMLDivElement | null = $state(null);
	let svgRef: SVGSVGElement | null = $state(null);
	let hoveredDiagnosisId: number | null = $state(null);
	let hoveredMedicationId: number | null = $state(null);

	// Prescription modal state
	let showRxModal = $state(false);
	let selectedMedsForRx = $state<Set<number>>(new Set());
	let rxButtonBounce = $state(false);

	// Prescription form state
	let rxForm = $state<Record<number, {
		quantity: number;
		days: number;
		refills: number;
		sig: string;
	}>>({});

	// Sig options
	const SIG_OPTIONS = [
		{ value: 'QD', label: 'QD (Once daily)' },
		{ value: 'BID', label: 'BID (Twice daily)' },
		{ value: 'TID', label: 'TID (Three times daily)' },
		{ value: 'QID', label: 'QID (Four times daily)' },
		{ value: 'QHS', label: 'QHS (At bedtime)' },
		{ value: 'PRN', label: 'PRN (As needed)' },
		{ value: 'AC', label: 'AC (Before meals)' },
		{ value: 'PC', label: 'PC (After meals)' },
		{ value: 'Q4H', label: 'Q4H (Every 4 hours)' },
		{ value: 'Q6H', label: 'Q6H (Every 6 hours)' },
		{ value: 'Q8H', label: 'Q8H (Every 8 hours)' },
		{ value: 'Q12H', label: 'Q12H (Every 12 hours)' },
		{ value: 'STAT', label: 'STAT (Immediately)' },
	];

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

	// Prescription functions
	function toggleMedForRx(medId: number) {
		const newSet = new Set(selectedMedsForRx);
		if (newSet.has(medId)) {
			newSet.delete(medId);
		} else {
			newSet.add(medId);
			// Initialize form data for this med if not exists
			if (!rxForm[medId]) {
				const med = getMedicationById(medId);
				rxForm[medId] = {
					quantity: 30,
					days: 30,
					refills: 0,
					sig: med?.frequency || 'QD'
				};
			}
		}
		selectedMedsForRx = newSet;

		// Trigger bounce animation
		if (newSet.size > 0) {
			rxButtonBounce = true;
			setTimeout(() => {
				rxButtonBounce = false;
			}, 600);
		}
	}

	function openRxModal(preselectedMedId?: number) {
		if (preselectedMedId !== undefined) {
			// If clicking from a med's refill button, ensure it's selected
			if (!selectedMedsForRx.has(preselectedMedId)) {
				toggleMedForRx(preselectedMedId);
			}
		}

		// Initialize form data for all selected meds
		selectedMedsForRx.forEach(medId => {
			if (!rxForm[medId]) {
				const med = getMedicationById(medId);
				rxForm[medId] = {
					quantity: 30,
					days: 30,
					refills: 0,
					sig: med?.frequency || 'QD'
				};
			}
		});

		showRxModal = true;
	}

	function closeRxModal() {
		showRxModal = false;
	}

	function toggleMedInModal(medId: number) {
		const newSet = new Set(selectedMedsForRx);
		if (newSet.has(medId)) {
			newSet.delete(medId);
		} else {
			newSet.add(medId);
			// Initialize form data
			if (!rxForm[medId]) {
				const med = getMedicationById(medId);
				rxForm[medId] = {
					quantity: 30,
					days: 30,
					refills: 0,
					sig: med?.frequency || 'QD'
				};
			}
		}
		selectedMedsForRx = newSet;
	}

	let isSubmitting = $state(false);

	async function submitPrescriptions() {
		if (isSubmitting || selectedMedsForRx.size === 0) return;

		isSubmitting = true;

		try {
			// Build prescription objects
			const prescriptions: Prescription[] = Array.from(selectedMedsForRx).map(medId => {
				const med = getMedicationById(medId);
				const form = rxForm[medId];
				return {
					id: null,
					patient_id: patientId,
					medication_id: medId,
					quantity: form.quantity,
					days_supply: form.days,
					refills: form.refills,
					sig: form.sig,
					pharmacy: null, // Could add pharmacy selection in future
					prescriber_id: null, // Current user would be set on backend
					status: 'sent',
					prescribed_date: null,
					filled_date: null,
					notes: null
				};
			});

			// Send to backend
			await invoke<number[]>('db_create_prescriptions', { prescriptions });

			// Show success toast
			const medNames = Array.from(selectedMedsForRx)
				.map(id => getMedicationById(id)?.name)
				.filter(Boolean)
				.join(', ');

			ToastStore.success(`Prescription sent: ${medNames}`);

			// Reset state
			selectedMedsForRx = new Set();
			rxForm = {};
			showRxModal = false;
		} catch (error) {
			console.error('Failed to send prescriptions:', error);
			ToastStore.error(`Failed to send prescription: ${error}`);
		} finally {
			isSubmitting = false;
		}
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

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full flex flex-col relative">
	<div class="flex items-center justify-between mb-3">
		<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 flex items-center gap-2">
			<i class="fa-solid fa-pills text-orange-500"></i>
			Diagnoses & Medications
		</h3>
	</div>

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
						{@const isSelected = selectedMedsForRx.has(med.id!)}
						<div
							data-medication-id={med.id}
							class="group relative p-3 rounded-lg border-2 transition-all cursor-pointer
								{isMedicationHighlighted(med.id!)
								? 'bg-indigo-50 dark:bg-indigo-900/30 border-indigo-400 dark:border-indigo-500'
								: isSelected
								? 'bg-green-50 dark:bg-green-900/30 border-green-400 dark:border-green-500'
								: 'bg-gray-50 dark:bg-gray-700 border-gray-200 dark:border-gray-600'}"
							onmouseenter={() => (hoveredMedicationId = med.id)}
							onmouseleave={() => (hoveredMedicationId = null)}
							role="button"
							tabindex="0"
						>
							<div class="font-medium text-gray-800 dark:text-gray-200 text-sm pr-8">
								{med.name}
							</div>
							<div class="text-xs text-gray-500 dark:text-gray-400">
								{med.dose ?? ''} {med.frequency ?? ''}
							</div>

							<!-- Refill button on hover -->
							<button
								onclick={(e) => { e.stopPropagation(); toggleMedForRx(med.id!); }}
								class="absolute right-2 top-1/2 -translate-y-1/2 w-7 h-7 rounded-full flex items-center justify-center transition-all
									{isSelected
									? 'bg-green-500 text-white opacity-100'
									: 'bg-blue-500 text-white opacity-0 group-hover:opacity-100'}"
								title={isSelected ? 'Remove from prescription' : 'Add to prescription'}
							>
								<i class="fa-solid {isSelected ? 'fa-check' : 'fa-prescription'} text-xs"></i>
							</button>
						</div>
					{/each}
				{/if}
			</div>
		</div>
	</div>

	<!-- Rx Button (bottom right) -->
	<button
		onclick={() => openRxModal()}
		class="absolute bottom-4 right-4 flex items-center gap-2 px-4 py-2 rounded-lg font-medium text-sm transition-all shadow-lg
			{selectedMedsForRx.size > 0
			? 'bg-green-500 hover:bg-green-600 text-white'
			: 'bg-blue-500 hover:bg-blue-600 text-white'}
			{rxButtonBounce ? 'animate-bounce-custom' : ''}"
	>
		<i class="fa-solid fa-prescription-bottle-medical"></i>
		<span>Rx</span>
		{#if selectedMedsForRx.size > 0}
			<span class="bg-white text-green-600 rounded-full w-5 h-5 flex items-center justify-center text-xs font-bold">
				{selectedMedsForRx.size}
			</span>
		{/if}
	</button>
</div>

<!-- Prescription Modal -->
{#if showRxModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
		<div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-2xl mx-4 max-h-[90vh] overflow-hidden flex flex-col">
			<!-- Modal Header -->
			<div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 flex items-center gap-2">
					<i class="fa-solid fa-prescription text-blue-500"></i>
					Prescribe Medications
				</h2>
				<button
					onclick={closeRxModal}
					class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-full transition-colors"
				>
					<i class="fa-solid fa-times text-gray-500"></i>
				</button>
			</div>

			<!-- Modal Body -->
			<div class="flex-1 overflow-y-auto p-4">
				<!-- Medication Selection -->
				<div class="mb-4">
					<h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						Select Medications to Refill
					</h3>
					<div class="grid grid-cols-2 gap-2 max-h-32 overflow-y-auto p-2 border border-gray-200 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700/50">
						{#each medications as med}
							{@const isSelected = selectedMedsForRx.has(med.id!)}
							<label class="flex items-center gap-2 p-2 hover:bg-gray-100 dark:hover:bg-gray-600 rounded cursor-pointer">
								<input
									type="checkbox"
									checked={isSelected}
									onchange={() => toggleMedInModal(med.id!)}
									class="rounded text-blue-500 focus:ring-blue-500"
								/>
								<span class="text-sm text-gray-700 dark:text-gray-300">{med.name}</span>
								<span class="text-xs text-gray-500 dark:text-gray-400">({med.dose})</span>
							</label>
						{/each}
					</div>
				</div>

				<!-- Prescription Details for each selected med -->
				{#if selectedMedsForRx.size > 0}
					<div class="space-y-4">
						{#each Array.from(selectedMedsForRx) as medId}
							{@const med = getMedicationById(medId)}
							{#if med && rxForm[medId]}
								<div class="p-4 border border-gray-200 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700/50">
									<div class="flex items-center justify-between mb-3">
										<h4 class="font-medium text-gray-800 dark:text-gray-200">
											{med.name} <span class="text-sm font-normal text-gray-500">({med.dose})</span>
										</h4>
										<button
											onclick={() => toggleMedInModal(medId)}
											class="text-red-500 hover:text-red-600 text-sm"
										>
											<i class="fa-solid fa-times"></i> Remove
										</button>
									</div>

									<div class="grid grid-cols-4 gap-3">
										<!-- Quantity -->
										<div>
											<label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
												Quantity
											</label>
											<input
												type="number"
												bind:value={rxForm[medId].quantity}
												min="1"
												class="w-full px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
											/>
										</div>

										<!-- Days Supply -->
										<div>
											<label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
												Days Supply
											</label>
											<input
												type="number"
												bind:value={rxForm[medId].days}
												min="1"
												class="w-full px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
											/>
										</div>

										<!-- Refills -->
										<div>
											<label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
												Refills
											</label>
											<input
												type="number"
												bind:value={rxForm[medId].refills}
												min="0"
												max="12"
												class="w-full px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
											/>
										</div>

										<!-- Sig -->
										<div>
											<label class="block text-xs font-medium text-gray-600 dark:text-gray-400 mb-1">
												Sig (How to take)
											</label>
											<select
												bind:value={rxForm[medId].sig}
												class="w-full px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
											>
												{#each SIG_OPTIONS as opt}
													<option value={opt.value}>{opt.value}</option>
												{/each}
											</select>
										</div>
									</div>

									<!-- Sig description -->
									<div class="mt-2 text-xs text-gray-500 dark:text-gray-400">
										{SIG_OPTIONS.find(o => o.value === rxForm[medId].sig)?.label || ''}
									</div>
								</div>
							{/if}
						{/each}
					</div>
				{:else}
					<div class="text-center py-8 text-gray-500 dark:text-gray-400">
						<i class="fa-solid fa-prescription-bottle text-4xl mb-2 opacity-30"></i>
						<p>Select medications above to prescribe</p>
					</div>
				{/if}
			</div>

			<!-- Modal Footer -->
			<div class="flex justify-between items-center p-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700/50 flex-shrink-0">
				<div class="text-sm text-gray-500 dark:text-gray-400">
					{#if selectedMedsForRx.size > 0}
						{selectedMedsForRx.size} medication{selectedMedsForRx.size !== 1 ? 's' : ''} selected
					{/if}
				</div>
				<div class="flex gap-3">
					<button
						onclick={closeRxModal}
						class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-lg transition-colors"
					>
						Cancel
					</button>
					<button
						onclick={submitPrescriptions}
						disabled={selectedMedsForRx.size === 0 || isSubmitting}
						class="px-4 py-2 text-sm font-medium text-white bg-green-500 hover:bg-green-600 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
					>
						{#if isSubmitting}
							<i class="fa-solid fa-spinner fa-spin"></i>
							Sending...
						{:else}
							<i class="fa-solid fa-paper-plane"></i>
							Send to Pharmacy
						{/if}
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	@keyframes bounce-custom {
		0%, 100% {
			transform: translateY(0);
		}
		25% {
			transform: translateY(-8px);
		}
		50% {
			transform: translateY(0);
		}
		75% {
			transform: translateY(-4px);
		}
	}

	.animate-bounce-custom {
		animation: bounce-custom 0.6s ease-in-out;
	}
</style>
