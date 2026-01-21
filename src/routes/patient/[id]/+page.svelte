<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { setActiveTab } from '../../../stores/TabStore';

	// Import components
	import PatientSummary from '$lib/components/patient/PatientSummary.svelte';
	import DiagnosisMedications from '$lib/components/patient/DiagnosisMedications.svelte';
	import VitalsLabsScores from '$lib/components/patient/VitalsLabsScores.svelte';
	import EncountersList from '$lib/components/patient/EncountersList.svelte';
	import Histories from '$lib/components/patient/Histories.svelte';
	import TodosGoals from '$lib/components/patient/TodosGoals.svelte';
	import Timeline from '$lib/components/patient/Timeline.svelte';

	import type { PatientFullData } from '$lib/types/patient';

	let patientData = $state<PatientFullData | null>(null);
	let loading = $state(true);
	let error = $state('');

	async function loadPatientData(id: string) {
		try {
			loading = true;
			error = '';
			const patientId = parseInt(id, 10);

			// Seed test data only if it doesn't already exist (no force reseed for performance)
			await invoke<string>('db_seed_patient_detail_test_data', { patientId, forceReseed: false });

			// Then fetch the full patient data
			patientData = await invoke<PatientFullData | null>('db_get_patient_full', { id: patientId });

			// Set this tab as active
			setActiveTab(`patient-${id}`);
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('Failed to load patient:', e);
		} finally {
			loading = false;
		}
	}

	// Track the current patient ID to detect changes
	let currentPatientId = $state<string | null>(null);

	// Watch for URL param changes (when switching between patient tabs)
	$effect(() => {
		const id = $page.params.id;
		if (id && id !== currentPatientId) {
			currentPatientId = id;
			loadPatientData(id);
		}
	});

	// Get diagnoses array for TodosGoals
	const diagnosesArray = $derived(patientData?.diagnoses.map((d) => d.diagnosis) ?? []);
</script>

<div class="absolute top-20 left-20 right-10 bottom-10 my-4 ml-5 mr-3 overflow-auto">
	{#if loading}
		<div class="flex items-center justify-center h-full">
			<div class="text-gray-600 dark:text-gray-400">
				<i class="fa-solid fa-spinner fa-spin mr-2"></i>
				Loading patient data...
			</div>
		</div>
	{:else if error}
		<div class="flex items-center justify-center h-full">
			<div class="text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 p-4 rounded-lg">
				<i class="fa-solid fa-exclamation-circle mr-2"></i>
				Error: {error}
			</div>
		</div>
	{:else if patientData}
		<!-- Grid Layout: 4 rows -->
		<!-- Row 1: Summary (full width, horizontal) -->
		<!-- Row 2: Dx/Meds, Vitals/Labs, Todos (top half) -->
		<!-- Row 3: Encounters, Histories, Todos (bottom half) -->
		<!-- Row 4: Timeline (full width) -->
		<div class="grid grid-cols-3 gap-4 min-h-full" style="grid-template-rows: auto minmax(200px, 1fr) minmax(200px, 1fr) 200px;">
			<!-- Row 1: Summary (full width, horizontal layout) -->
			<div class="col-span-3">
				<PatientSummary patient={patientData.patient} diagnoses={patientData.diagnoses} scores={patientData.clinical_scores} />
			</div>

			<!-- Row 2: Dx/Meds, Vitals/Labs, Todos (start) -->
			<div class="col-span-1">
				<DiagnosisMedications
					diagnoses={patientData.diagnoses}
					medications={patientData.medications}
				/>
			</div>
			<div class="col-span-1">
				<VitalsLabsScores
					vitals={patientData.vitals}
					labs={patientData.labs}
					scores={patientData.clinical_scores}
				/>
			</div>
			<div class="col-span-1 row-span-2">
				<TodosGoals
					todos={patientData.todos}
					goals={patientData.goals}
					diagnoses={diagnosesArray}
				/>
			</div>

			<!-- Row 3: Encounters, Histories (Todos spans from row 2) -->
			<div class="col-span-1">
				<EncountersList
					encounters={patientData.encounters}
					patientId={patientData.patient.id ?? 0}
					patientName="{patientData.patient.first_name} {patientData.patient.last_name}"
				/>
			</div>
			<div class="col-span-1">
				<Histories
					allergies={patientData.allergies}
					vaccinations={patientData.vaccinations}
					socialHistory={patientData.social_history}
					familyHistory={patientData.family_history}
				/>
			</div>

			<!-- Row 4: Timeline (full width) -->
			<div class="col-span-3">
				<Timeline events={patientData.timeline_events} />
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="text-gray-600 dark:text-gray-400">Patient not found</div>
		</div>
	{/if}
</div>
