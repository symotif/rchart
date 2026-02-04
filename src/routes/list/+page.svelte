<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { addTab } from '../../stores/TabStore';
	import { invoke } from '@tauri-apps/api/tauri';
	import { AppDataStore } from '../../stores/AppDataStore';
	import type { Patient } from '$lib/types/patient';
	import type { PatientList, PatientListColumn, PatientListWithPatients } from '$lib/types/patientList';
	import { AVAILABLE_COLUMNS, LIST_COLORS, LIST_ICONS } from '$lib/types/patientList';

	let error = $state('');

	// Use preloaded data from AppDataStore
	let patientLists = $derived($AppDataStore.patientLists);
	let selectedListId = $state<number | null>(null);
	let currentListData = $state<PatientListWithPatients | null>(null);

	// New list modal
	let showNewListModal = $state(false);
	let newListForm = $state({
		name: '',
		description: '',
		color: '#3B82F6',
		icon: 'fa-list',
		columns: AVAILABLE_COLUMNS.filter(c => c.defaultVisible).map(c => c.key)
	});
	let newListError = $state<string | null>(null);
	let creatingList = $state(false);

	function calculateAge(dob: string): number {
		const birthDate = new Date(dob);
		const today = new Date();
		let age = today.getFullYear() - birthDate.getFullYear();
		const monthDiff = today.getMonth() - birthDate.getMonth();
		if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
			age--;
		}
		return age;
	}

	function formatDob(dob: string): string {
		const date = new Date(dob);
		return `${date.getMonth() + 1}/${date.getDate()}/${date.getFullYear() % 100}`;
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '-';
		const date = new Date(dateStr);
		return `${date.getMonth() + 1}/${date.getDate()}/${date.getFullYear() % 100}`;
	}

	function initializeFromStore() {
		// Select the default list or first list from preloaded data
		if (patientLists.length > 0 && !selectedListId) {
			const defaultList = patientLists.find(l => l.is_default) || patientLists[0];
			selectedListId = defaultList.id;
			loadListData(selectedListId!);
		}
	}

	function loadListData(listId: number) {
		// Get preloaded list data from store
		const listData = $AppDataStore.patientListsWithPatients.get(listId);
		if (listData) {
			currentListData = listData;
		}
	}

	function handleListChange(event: Event) {
		const select = event.target as HTMLSelectElement;
		const listId = parseInt(select.value);
		selectedListId = listId;
		loadListData(listId);
	}

	async function handleCreateList() {
		if (!newListForm.name.trim()) {
			newListError = 'Please enter a list name';
			return;
		}

		if (newListForm.columns.length === 0) {
			newListError = 'Please select at least one column';
			return;
		}

		creatingList = true;
		newListError = null;

		try {
			const newList: PatientList = {
				id: null,
				user_id: 1,
				name: newListForm.name.trim(),
				description: newListForm.description.trim() || null,
				color: newListForm.color,
				icon: newListForm.icon,
				is_default: false,
				sort_order: patientLists.length
			};

			const listId = await invoke<number>('db_create_patient_list', { list: newList });

			// Update columns for the new list
			const columns: PatientListColumn[] = newListForm.columns.map((key, index) => {
				const colInfo = AVAILABLE_COLUMNS.find(c => c.key === key)!;
				return {
					id: null,
					list_id: listId,
					column_key: key,
					column_label: colInfo.label,
					column_type: colInfo.type,
					is_visible: true,
					sort_order: index,
					width: 150
				};
			});

			await invoke('db_update_list_columns', { listId, columns });

			// Refresh the store and select the new list
			await AppDataStore.refreshPatientLists();
			selectedListId = listId;
			loadListData(listId);

			// Reset form and close modal
			showNewListModal = false;
			newListForm = {
				name: '',
				description: '',
				color: '#3B82F6',
				icon: 'fa-list',
				columns: AVAILABLE_COLUMNS.filter(c => c.defaultVisible).map(c => c.key)
			};
		} catch (e) {
			newListError = e instanceof Error ? e.message : String(e);
		} finally {
			creatingList = false;
		}
	}

	function toggleColumn(columnKey: string) {
		if (newListForm.columns.includes(columnKey)) {
			newListForm.columns = newListForm.columns.filter(c => c !== columnKey);
		} else {
			newListForm.columns = [...newListForm.columns, columnKey];
		}
	}

	onMount(() => {
		initializeFromStore();
	});

	// Re-initialize when store data changes
	$effect(() => {
		if ($AppDataStore.isLoaded && patientLists.length > 0 && !selectedListId) {
			initializeFromStore();
		}
	});

	function onPatientRowClick(patientId: string, patientName: string): void {
		const newTab = {
			id: `patient-${patientId}`,
			title: patientName,
			path: `/patient/${patientId}`
		};
		addTab(newTab);
		goto(`/patient/${patientId}`);
	}

	function getCellValue(patient: Patient, columnKey: string): string {
		switch (columnKey) {
			case 'name':
				return `${patient.first_name} ${patient.last_name}`;
			case 'dob':
				return formatDob(patient.dob);
			case 'age':
				return calculateAge(patient.dob).toString();
			case 'sex':
				return patient.sex;
			case 'gender':
				return patient.gender || '-';
			case 'phone':
				return patient.phone || '-';
			case 'email':
				return patient.email || '-';
			case 'address':
				return patient.address || '-';
			case 'insurance_provider':
				return patient.insurance_provider || '-';
			case 'preferred_pharmacy':
				return patient.preferred_pharmacy || '-';
			case 'last_visit':
				return '-'; // Would need to query encounters
			case 'next_appointment':
				return '-'; // Would need to query appointments
			case 'primary_diagnosis':
				return '-'; // Would need to query diagnoses
			default:
				return '-';
		}
	}

	// Get the selected list info for styling
	let selectedList = $derived(patientLists.find(l => l.id === selectedListId));
</script>

<div class="absolute left-20 top-24 right-0 bottom-5 px-5 py-6 overflow-hidden flex flex-col">
	<!-- Header with List Selector -->
	<div class="flex items-center justify-between mb-4 flex-shrink-0">
		<div class="flex items-center gap-4">
			<!-- List Selector -->
			<div class="flex items-center gap-2">
				{#if selectedList}
					<i class="fa-solid {selectedList.icon || 'fa-list'} text-lg" style="color: {selectedList.color}"></i>
				{/if}
				<select
					value={selectedListId?.toString() || ''}
					onchange={handleListChange}
					class="px-4 py-2 text-sm font-medium border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent min-w-[200px]"
				>
					{#each patientLists as list}
						<option value={list.id?.toString()}>
							{list.name}
							{#if list.is_default}(Default){/if}
						</option>
					{/each}
				</select>
			</div>

			<!-- New List Button -->
			<button
				onclick={() => showNewListModal = true}
				class="flex items-center gap-2 px-3 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
			>
				<i class="fa-solid fa-plus"></i>
				New List
			</button>
		</div>

		<!-- Patient count -->
		{#if currentListData}
			<div class="text-sm text-gray-500 dark:text-gray-400">
				{currentListData.patients.length} patient{currentListData.patients.length !== 1 ? 's' : ''}
			</div>
		{/if}
	</div>

	<!-- Table Container -->
	{#if error}
		<div class="bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 rounded-lg p-4">
			<p class="text-red-600 dark:text-red-400">Error: {error}</p>
			<button onclick={() => AppDataStore.refreshPatientLists()} class="mt-2 text-sm text-red-600 dark:text-red-400 underline">
				Try again
			</button>
		</div>
	{:else if !currentListData || currentListData.patients.length === 0}
		<div class="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
			<div class="text-center">
				<i class="fa-solid fa-users text-6xl mb-4 opacity-30"></i>
				<p class="text-lg">No patients in this list</p>
				<p class="text-sm">Add patients to this list to see them here</p>
			</div>
		</div>
	{:else}
		<div class="flex-1 overflow-auto rounded-lg shadow bg-white dark:bg-gray-800">
			<table class="w-full">
				<thead class="bg-gray-50 dark:bg-gray-700 border-b-2 border-gray-200 dark:border-gray-600 sticky top-0">
					<tr>
						<th class="p-3 text-sm font-semibold tracking-wide text-left text-gray-700 dark:text-gray-200">#</th>
						{#each currentListData.columns.filter(c => c.is_visible) as column}
							<th class="p-3 text-sm font-semibold tracking-wide text-left text-gray-700 dark:text-gray-200">
								{column.column_label}
							</th>
						{/each}
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-100 dark:divide-gray-600">
					{#each currentListData.patients as patient, index}
						<tr
							onclick={() => onPatientRowClick(String(patient.id), `${patient.first_name} ${patient.last_name}`)}
							class="bg-white dark:bg-gray-800 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
						>
							<td class="px-3 py-4 text-sm text-left text-gray-700 dark:text-gray-300 whitespace-nowrap">
								{(index + 1).toString().padStart(2, '0')}
							</td>
							{#each currentListData.columns.filter(c => c.is_visible) as column}
								<td class="px-3 py-4 text-sm text-left text-gray-700 dark:text-gray-300 whitespace-nowrap">
									{getCellValue(patient, column.column_key)}
								</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<!-- New List Modal -->
{#if showNewListModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
		<div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto">
			<!-- Modal Header -->
			<div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
					<i class="fa-solid fa-plus mr-2 text-blue-500"></i>
					Create New Patient List
				</h2>
				<button
					onclick={() => showNewListModal = false}
					class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-full transition-colors"
				>
					<i class="fa-solid fa-times text-gray-500"></i>
				</button>
			</div>

			<!-- Modal Body -->
			<div class="p-4 space-y-4">
				{#if newListError}
					<div class="bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 rounded-lg p-3">
						<p class="text-sm text-red-600 dark:text-red-400">{newListError}</p>
					</div>
				{/if}

				<!-- List Name -->
				<div>
					<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
						List Name <span class="text-red-500">*</span>
					</label>
					<input
						type="text"
						bind:value={newListForm.name}
						placeholder="e.g., Cardiology Clinic"
						class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					/>
				</div>

				<!-- Description -->
				<div>
					<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
						Description
					</label>
					<input
						type="text"
						bind:value={newListForm.description}
						placeholder="Optional description"
						class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					/>
				</div>

				<!-- Color & Icon -->
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Color
						</label>
						<div class="flex flex-wrap gap-2">
							{#each LIST_COLORS as color}
								<button
									onclick={() => newListForm.color = color.value}
									class="w-8 h-8 rounded-full border-2 transition-all {newListForm.color === color.value ? 'border-gray-800 dark:border-white scale-110' : 'border-transparent'}"
									style="background-color: {color.value}"
									title={color.label}
								></button>
							{/each}
						</div>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Icon
						</label>
						<div class="flex flex-wrap gap-2">
							{#each LIST_ICONS as icon}
								<button
									onclick={() => newListForm.icon = icon.value}
									class="w-8 h-8 rounded-lg border-2 flex items-center justify-center transition-all {newListForm.icon === icon.value ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30' : 'border-gray-200 dark:border-gray-600 hover:border-gray-300'}"
									title={icon.label}
								>
									<i class="fa-solid {icon.value} text-sm" style="color: {newListForm.color}"></i>
								</button>
							{/each}
						</div>
					</div>
				</div>

				<!-- Column Selection -->
				<div>
					<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
						Columns <span class="text-red-500">*</span>
					</label>
					<div class="grid grid-cols-2 gap-2 max-h-48 overflow-y-auto p-2 border border-gray-200 dark:border-gray-600 rounded-lg">
						{#each AVAILABLE_COLUMNS as column}
							<label class="flex items-center gap-2 p-2 hover:bg-gray-50 dark:hover:bg-gray-700 rounded cursor-pointer">
								<input
									type="checkbox"
									checked={newListForm.columns.includes(column.key)}
									onchange={() => toggleColumn(column.key)}
									class="rounded text-blue-500 focus:ring-blue-500"
								/>
								<span class="text-sm text-gray-700 dark:text-gray-300">{column.label}</span>
							</label>
						{/each}
					</div>
				</div>
			</div>

			<!-- Modal Footer -->
			<div class="flex justify-end gap-3 p-4 border-t border-gray-200 dark:border-gray-700">
				<button
					onclick={() => showNewListModal = false}
					disabled={creatingList}
					class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={handleCreateList}
					disabled={creatingList}
					class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors disabled:opacity-50 flex items-center gap-2"
				>
					{#if creatingList}
						<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
						Creating...
					{:else}
						<i class="fa-solid fa-plus"></i>
						Create List
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}
