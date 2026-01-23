<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';
	import { setActiveTab, addTab, removeTab } from '../../../../../stores/TabStore';

	import NoteEditor from '$lib/components/note/NoteEditor.svelte';
	import type { Encounter, Patient } from '$lib/types/patient';
	import type { UserFullData } from '$lib/types/user';

	let encounter = $state<Encounter | null>(null);
	let patient = $state<Patient | null>(null);
	let loading = $state(true);
	let error = $state('');
	let saving = $state(false);
	let zenModeDefault = $state(false);

	// noteId can be 'new' for creating a new note, or an encounter ID for editing
	let isNewNote = $derived($page.params.noteId === 'new');

	async function loadData(patientId: string, noteId: string) {
		try {
			loading = true;
			error = '';

			// Fetch user settings to check zen mode default
			const userData = await invoke<UserFullData | null>('db_get_current_user');
			if (userData?.settings?.zen_mode_default) {
				zenModeDefault = true;
			}

			// Fetch patient info
			patient = await invoke<Patient | null>('db_get_patient', { id: parseInt(patientId, 10) });

			// If editing an existing encounter, fetch it
			if (noteId !== 'new') {
				encounter = await invoke<Encounter | null>('db_get_encounter', {
					encounterId: parseInt(noteId, 10)
				});
			} else {
				// Create a new empty encounter object for new notes
				encounter = {
					id: null,
					patient_id: parseInt(patientId, 10),
					encounter_date: new Date().toISOString().split('T')[0],
					encounter_type: 'Progress Note',
					chief_complaint: null,
					summary: null,
					note_content: null,
					provider: 'Dr. Madeline Chu',
					location: 'Main Clinic'
				};
			}

			if (patient) {
				setActiveTab(`note-${noteId}-${patientId}`);
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('Failed to load data:', e);
		} finally {
			loading = false;
		}
	}

	async function handleSign(noteData: {
		noteContent: string;
		chiefComplaint: string;
		noteType: string;
		cosigner: { name: string; initials: string } | null;
	}) {
		if (!encounter || !patient) return;

		try {
			saving = true;
			const patientId = parseInt($page.params.id, 10);
			const noteId = $page.params.noteId;

			const encounterData: Encounter = {
				id: isNewNote ? null : encounter.id,
				patient_id: patientId,
				encounter_date: encounter.encounter_date,
				encounter_type: noteData.noteType,
				chief_complaint: noteData.chiefComplaint || null,
				summary: null,
				note_content: noteData.noteContent,
				provider: encounter.provider,
				location: encounter.location
			};

			if (isNewNote) {
				// Create new encounter
				const newId = await invoke<number>('db_create_encounter', { encounter: encounterData });
				console.log('Created new encounter with ID:', newId);
			} else {
				// Update existing encounter
				await invoke('db_update_encounter', { encounter: encounterData });
				console.log('Updated encounter');
			}

			// Navigate back to patient page
			const currentTabId = `note-${noteId}-${patientId}`;
			removeTab(currentTabId);
			goto(`/patient/${patientId}`);
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('Failed to sign note:', e);
			alert('Failed to save note: ' + error);
		} finally {
			saving = false;
		}
	}

	async function handleSave(noteData: {
		noteContent: string;
		chiefComplaint: string;
		noteType: string;
		cosigner: { name: string; initials: string } | null;
	}) {
		if (!encounter || !patient) return;

		try {
			saving = true;
			const patientId = parseInt($page.params.id, 10);

			const encounterData: Encounter = {
				id: isNewNote ? null : encounter.id,
				patient_id: patientId,
				encounter_date: encounter.encounter_date,
				encounter_type: noteData.noteType,
				chief_complaint: noteData.chiefComplaint || null,
				summary: null,
				note_content: noteData.noteContent,
				provider: encounter.provider,
				location: encounter.location
			};

			if (isNewNote) {
				// Create new encounter as draft
				const newId = await invoke<number>('db_create_encounter', { encounter: encounterData });
				console.log('Saved draft with ID:', newId);

				// Update the URL to reflect the new encounter ID
				const newPath = `/patient/${patientId}/note/${newId}`;
				const newTabId = `note-${newId}-${patientId}`;
				const oldTabId = `note-new-${patientId}`;

				// Update the encounter object with the new ID
				encounter = { ...encounterData, id: newId };

				// Remove old tab and add new one with correct ID
				removeTab(oldTabId);
				addTab({
					id: newTabId,
					title: `Edit Note - ${patient.first_name} ${patient.last_name}`,
					path: newPath
				});
				goto(newPath, { replaceState: true });
			} else {
				// Update existing encounter
				await invoke('db_update_encounter', { encounter: encounterData });
				console.log('Draft saved');
			}

			// Show success feedback
			alert('Draft saved successfully!');
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('Failed to save draft:', e);
			alert('Failed to save draft: ' + error);
		} finally {
			saving = false;
		}
	}

	onMount(() => {
		const patientId = $page.params.id;
		const noteId = $page.params.noteId;
		if (patientId && noteId) {
			loadData(patientId, noteId);
		}
	});
</script>

<div class="absolute top-20 left-20 right-10 bottom-10 my-4 ml-5 mr-3">
	{#if loading}
		<div class="flex items-center justify-center h-full">
			<div class="text-gray-600 dark:text-gray-400">
				<i class="fa-solid fa-spinner fa-spin mr-2"></i>
				Loading...
			</div>
		</div>
	{:else if error}
		<div class="flex items-center justify-center h-full">
			<div class="text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 p-4 rounded-lg">
				<i class="fa-solid fa-exclamation-circle mr-2"></i>
				Error: {error}
			</div>
		</div>
	{:else if patient}
		<NoteEditor
			{patient}
			{encounter}
			patientId={parseInt($page.params.id, 10)}
			{saving}
			{zenModeDefault}
			onSign={handleSign}
			onSave={handleSave}
		/>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="text-gray-600 dark:text-gray-400">Patient not found</div>
		</div>
	{/if}
</div>
