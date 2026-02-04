<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';
	import { setActiveTab, addTab } from '../../../../../stores/TabStore';

	import type { Encounter, Patient } from '$lib/types/patient';

	let encounter = $state<Encounter | null>(null);
	let patient = $state<Patient | null>(null);
	let loading = $state(true);
	let error = $state('');

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			weekday: 'long',
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	async function loadEncounter(patientId: string, encounterId: string) {
		try {
			loading = true;
			error = '';

			// Fetch patient info
			patient = await invoke<Patient | null>('db_get_patient', { id: parseInt(patientId, 10) });

			// Fetch encounter
			encounter = await invoke<Encounter | null>('db_get_encounter', {
				encounterId: parseInt(encounterId, 10)
			});

			if (patient && encounter) {
				setActiveTab(`encounter-${encounterId}`);
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('Failed to load encounter:', e);
		} finally {
			loading = false;
		}
	}

	function goBackToPatient() {
		const patientId = $page.params.id;
		goto(`/patient/${patientId}`);
	}

	function handleEditNote() {
		const patientId = $page.params.id;
		const encounterId = $page.params.encounterId;
		const tab = {
			id: `note-${encounterId}-${patientId}`,
			title: `Edit Note - ${patient?.first_name} ${patient?.last_name}`,
			path: `/patient/${patientId}/note/${encounterId}`
		};
		addTab(tab);
		goto(tab.path);
	}

	function handlePrint() {
		window.print();
	}

	function handleBillingView() {
		// Placeholder for billing view functionality
		alert('Billing view will be implemented');
	}

	onMount(() => {
		const patientId = $page.params.id;
		const encounterId = $page.params.encounterId;
		if (patientId && encounterId) {
			loadEncounter(patientId, encounterId);
		}
	});
</script>

<div class="absolute top-24 left-20 right-10 bottom-10 my-4 ml-5 mr-3 overflow-auto">
	{#if loading}
		<div class="flex items-center justify-center h-full">
			<div class="text-gray-600 dark:text-gray-400">
				<i class="fa-solid fa-spinner fa-spin mr-2"></i>
				Loading encounter...
			</div>
		</div>
	{:else if error}
		<div class="flex items-center justify-center h-full">
			<div class="text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 p-4 rounded-lg">
				<i class="fa-solid fa-exclamation-circle mr-2"></i>
				Error: {error}
			</div>
		</div>
	{:else if encounter && patient}
		<div class="max-w-4xl mx-auto">
			<!-- Header -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-4">
				<div class="flex items-center justify-between mb-4">
					<button
						class="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors"
						onclick={goBackToPatient}
					>
						<i class="fa-solid fa-arrow-left mr-2"></i>
						Back to Patient
					</button>
					<span
						class="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded-full text-sm"
					>
						{encounter.encounter_type}
					</span>
				</div>

				<h1 class="text-2xl font-bold text-gray-800 dark:text-gray-100 mb-2">
					{patient.first_name} {patient.last_name}
				</h1>
				<div class="flex items-center gap-4 text-gray-600 dark:text-gray-400">
					<span>
						<i class="fa-solid fa-calendar mr-1"></i>
						{formatDate(encounter.encounter_date)}
					</span>
					{#if encounter.provider}
						<span>
							<i class="fa-solid fa-user-doctor mr-1"></i>
							{encounter.provider}
						</span>
					{/if}
					{#if encounter.location}
						<span>
							<i class="fa-solid fa-location-dot mr-1"></i>
							{encounter.location}
						</span>
					{/if}
				</div>

				{#if encounter.chief_complaint}
					<div class="mt-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg border border-yellow-200 dark:border-yellow-800">
						<span class="text-sm font-medium text-yellow-800 dark:text-yellow-200">Chief Complaint:</span>
						<span class="text-yellow-700 dark:text-yellow-300 ml-2">{encounter.chief_complaint}</span>
					</div>
				{/if}
			</div>

			<!-- Full Note Content -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-4 flex items-center gap-2">
					<i class="fa-solid fa-file-medical text-green-500"></i>
					Clinical Note
				</h2>

				{#if encounter.note_content}
					<div class="prose prose-sm dark:prose-invert max-w-none">
						<pre class="whitespace-pre-wrap font-sans text-gray-700 dark:text-gray-300 bg-gray-50 dark:bg-gray-700 p-4 rounded-lg text-sm leading-relaxed">{@html encounter.note_content}</pre>
					</div>
				{:else}
					<p class="text-gray-500 dark:text-gray-400 italic">No clinical note content available.</p>
				{/if}
			</div>

			<!-- Actions -->
			<div class="mt-4 flex gap-3">
				<button
					onclick={handleEditNote}
					class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
				>
					<i class="fa-solid fa-edit mr-2"></i>
					Edit Note
				</button>
				<button
					onclick={handlePrint}
					class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
				>
					<i class="fa-solid fa-print mr-2"></i>
					Print
				</button>
				<button
					onclick={handleBillingView}
					class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
				>
					<i class="fa-solid fa-file-invoice-dollar mr-2"></i>
					Billing View
				</button>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="text-gray-600 dark:text-gray-400">Encounter not found</div>
		</div>
	{/if}
</div>
