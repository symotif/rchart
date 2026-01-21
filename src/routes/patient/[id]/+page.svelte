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

			// First seed the detail test data for this patient
			await invoke<string>('db_seed_patient_detail_test_data', { patientId });

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

	onMount(() => {
		const id = $page.params.id;
		if (id) {
			loadPatientData(id);
		}
	});

	// Get diagnoses array for TodosGoals
	const diagnosesArray = $derived(patientData?.diagnoses.map((d) => d.diagnosis) ?? []);
</script>

<div class="absolute left-20 top-20 right-0 bottom-0 p-4 overflow-auto">
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
		<!-- Grid Layout -->
		<div class="grid grid-cols-3 gap-4 min-h-full" style="grid-template-rows: minmax(200px, 1fr) minmax(250px, 1.2fr) 200px;">
			<!-- Row 1: Summary, Dx/Meds, Vitals/Labs -->
			<div class="col-span-1">
				<PatientSummary patient={patientData.patient} />
			</div>
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

			<!-- Row 2: Encounters, Histories, To-Dos/Goals -->
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
			<div class="col-span-1">
				<TodosGoals
					todos={patientData.todos}
					goals={patientData.goals}
					diagnoses={diagnosesArray}
				/>
			</div>

			<!-- Row 3: Timeline (full width) -->
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
