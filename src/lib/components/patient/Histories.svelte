<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import CollapsibleSection from '$lib/components/ui/CollapsibleSection.svelte';
	import type { Allergy, Vaccination, SocialHistory, FamilyHistory } from '$lib/types/patient';
	import { ToastStore } from '../../../stores/ToastStore';

	let {
		allergies,
		vaccinations,
		socialHistory,
		familyHistory,
		patientId,
		onDataChange
	}: {
		allergies: Allergy[];
		vaccinations: Vaccination[];
		socialHistory: SocialHistory[];
		familyHistory: FamilyHistory[];
		patientId: number;
		onDataChange?: () => void;
	} = $props();

	// Edit mode state
	type EditType = 'allergy' | 'vaccination' | 'social' | 'family' | null;
	let editMode = $state<EditType>(null);
	let editItem = $state<Allergy | Vaccination | SocialHistory | FamilyHistory | null>(null);
	let isNew = $state(false);
	let isSaving = $state(false);

	// Form state for each type
	let allergyForm = $state({ allergen: '', reaction: '', severity: 'mild' });
	let vaccinationForm = $state({ vaccine_name: '', date_given: '' });
	let socialForm = $state({ category: '', detail: '', status: '' });
	let familyForm = $state({ relation: '', condition: '', age_at_onset: '' });

	const severityOptions = ['mild', 'moderate', 'severe'];
	const socialCategories = ['Tobacco', 'Alcohol', 'Drugs', 'Exercise', 'Diet', 'Occupation', 'Living Situation', 'Other'];
	const familyRelations = ['Mother', 'Father', 'Sister', 'Brother', 'Maternal Grandmother', 'Maternal Grandfather', 'Paternal Grandmother', 'Paternal Grandfather', 'Aunt', 'Uncle', 'Child'];

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function getSeverityColor(severity: string | null): string {
		switch (severity?.toLowerCase()) {
			case 'severe':
				return 'bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-300';
			case 'moderate':
				return 'bg-orange-100 dark:bg-orange-900 text-orange-700 dark:text-orange-300';
			case 'mild':
				return 'bg-yellow-100 dark:bg-yellow-900 text-yellow-700 dark:text-yellow-300';
			default:
				return 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300';
		}
	}

	function openAdd(type: EditType) {
		editMode = type;
		isNew = true;
		editItem = null;
		// Reset forms
		if (type === 'allergy') allergyForm = { allergen: '', reaction: '', severity: 'mild' };
		if (type === 'vaccination') vaccinationForm = { vaccine_name: '', date_given: new Date().toISOString().split('T')[0] };
		if (type === 'social') socialForm = { category: 'Tobacco', detail: '', status: '' };
		if (type === 'family') familyForm = { relation: 'Mother', condition: '', age_at_onset: '' };
	}

	function openEdit(type: EditType, item: Allergy | Vaccination | SocialHistory | FamilyHistory) {
		editMode = type;
		isNew = false;
		editItem = item;
		// Populate forms
		if (type === 'allergy') {
			const a = item as Allergy;
			allergyForm = { allergen: a.allergen, reaction: a.reaction || '', severity: a.severity || 'mild' };
		}
		if (type === 'vaccination') {
			const v = item as Vaccination;
			vaccinationForm = { vaccine_name: v.vaccine_name, date_given: v.date_given };
		}
		if (type === 'social') {
			const s = item as SocialHistory;
			socialForm = { category: s.category, detail: s.detail, status: s.status || '' };
		}
		if (type === 'family') {
			const f = item as FamilyHistory;
			familyForm = { relation: f.relation, condition: f.condition, age_at_onset: f.age_at_onset?.toString() || '' };
		}
	}

	function closeModal() {
		editMode = null;
		editItem = null;
		isNew = false;
	}

	async function saveAllergy() {
		if (!allergyForm.allergen.trim()) return;
		isSaving = true;
		try {
			const allergy: Allergy = {
				id: isNew ? null : (editItem as Allergy)?.id ?? null,
				patient_id: patientId,
				allergen: allergyForm.allergen,
				reaction: allergyForm.reaction || null,
				severity: allergyForm.severity || null
			};
			if (isNew) {
				await invoke('db_create_allergy', { allergy });
				ToastStore.success('Allergy added');
			} else {
				await invoke('db_update_allergy', { allergy });
				ToastStore.success('Allergy updated');
			}
			closeModal();
			onDataChange?.();
		} catch (error) {
			ToastStore.error(`Failed to save allergy: ${error}`);
		} finally {
			isSaving = false;
		}
	}

	async function saveVaccination() {
		if (!vaccinationForm.vaccine_name.trim()) return;
		isSaving = true;
		try {
			const vaccination: Vaccination = {
				id: isNew ? null : (editItem as Vaccination)?.id ?? null,
				patient_id: patientId,
				vaccine_name: vaccinationForm.vaccine_name,
				date_given: vaccinationForm.date_given
			};
			if (isNew) {
				await invoke('db_create_vaccination', { vaccination });
				ToastStore.success('Vaccination added');
			} else {
				await invoke('db_update_vaccination', { vaccination });
				ToastStore.success('Vaccination updated');
			}
			closeModal();
			onDataChange?.();
		} catch (error) {
			ToastStore.error(`Failed to save vaccination: ${error}`);
		} finally {
			isSaving = false;
		}
	}

	async function saveSocialHistory() {
		if (!socialForm.detail.trim()) return;
		isSaving = true;
		try {
			const history: SocialHistory = {
				id: isNew ? null : (editItem as SocialHistory)?.id ?? null,
				patient_id: patientId,
				category: socialForm.category,
				detail: socialForm.detail,
				status: socialForm.status || null
			};
			if (isNew) {
				await invoke('db_create_social_history', { history });
				ToastStore.success('Social history added');
			} else {
				await invoke('db_update_social_history', { history });
				ToastStore.success('Social history updated');
			}
			closeModal();
			onDataChange?.();
		} catch (error) {
			ToastStore.error(`Failed to save social history: ${error}`);
		} finally {
			isSaving = false;
		}
	}

	async function saveFamilyHistory() {
		if (!familyForm.condition.trim()) return;
		isSaving = true;
		try {
			const history: FamilyHistory = {
				id: isNew ? null : (editItem as FamilyHistory)?.id ?? null,
				patient_id: patientId,
				relation: familyForm.relation,
				condition: familyForm.condition,
				age_at_onset: familyForm.age_at_onset ? parseInt(familyForm.age_at_onset) : null
			};
			if (isNew) {
				await invoke('db_create_family_history', { history });
				ToastStore.success('Family history added');
			} else {
				await invoke('db_update_family_history', { history });
				ToastStore.success('Family history updated');
			}
			closeModal();
			onDataChange?.();
		} catch (error) {
			ToastStore.error(`Failed to save family history: ${error}`);
		} finally {
			isSaving = false;
		}
	}

	async function deleteItem() {
		if (!editItem || !editItem.id) return;
		isSaving = true;
		try {
			if (editMode === 'allergy') {
				await invoke('db_delete_allergy', { id: editItem.id });
				ToastStore.success('Allergy deleted');
			} else if (editMode === 'vaccination') {
				await invoke('db_delete_vaccination', { id: editItem.id });
				ToastStore.success('Vaccination deleted');
			} else if (editMode === 'social') {
				await invoke('db_delete_social_history', { id: editItem.id });
				ToastStore.success('Social history deleted');
			} else if (editMode === 'family') {
				await invoke('db_delete_family_history', { id: editItem.id });
				ToastStore.success('Family history deleted');
			}
			closeModal();
			onDataChange?.();
		} catch (error) {
			ToastStore.error(`Failed to delete: ${error}`);
		} finally {
			isSaving = false;
		}
	}

	function handleSave() {
		if (editMode === 'allergy') saveAllergy();
		else if (editMode === 'vaccination') saveVaccination();
		else if (editMode === 'social') saveSocialHistory();
		else if (editMode === 'family') saveFamilyHistory();
	}
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full flex flex-col">
	<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
		<i class="fa-solid fa-clipboard-list text-green-500"></i>
		Histories
	</h3>

	<div class="flex-1 overflow-y-auto">
		<!-- Allergies -->
		<CollapsibleSection title="Allergies ({allergies.length})" defaultOpen={true}>
			{#snippet headerActions()}
				<button
					onclick={() => openAdd('allergy')}
					class="p-1 text-gray-400 hover:text-blue-500 transition-colors"
					title="Add allergy"
				>
					<i class="fa-solid fa-plus text-xs"></i>
				</button>
			{/snippet}
			{#snippet children()}
				{#if allergies.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No known allergies</p>
				{:else}
					<div class="space-y-2">
						{#each allergies as allergy}
							<button
								onclick={() => openEdit('allergy', allergy)}
								class="w-full text-left flex items-center justify-between p-2 bg-red-50 dark:bg-red-900/20 rounded hover:bg-red-100 dark:hover:bg-red-900/40 transition-colors group"
							>
								<div>
									<span class="font-medium text-gray-800 dark:text-gray-200">{allergy.allergen}</span>
									{#if allergy.reaction}
										<span class="text-sm text-gray-600 dark:text-gray-400"> - {allergy.reaction}</span>
									{/if}
								</div>
								<div class="flex items-center gap-2">
									{#if allergy.severity}
										<span class="text-xs px-2 py-0.5 rounded {getSeverityColor(allergy.severity)}">
											{allergy.severity}
										</span>
									{/if}
									<i class="fa-solid fa-pencil text-xs text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"></i>
								</div>
							</button>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>

		<!-- Vaccinations -->
		<CollapsibleSection title="Vaccinations ({vaccinations.length})">
			{#snippet headerActions()}
				<button
					onclick={() => openAdd('vaccination')}
					class="p-1 text-gray-400 hover:text-blue-500 transition-colors"
					title="Add vaccination"
				>
					<i class="fa-solid fa-plus text-xs"></i>
				</button>
			{/snippet}
			{#snippet children()}
				{#if vaccinations.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No vaccinations recorded</p>
				{:else}
					<div class="space-y-2">
						{#each vaccinations as vax}
							<button
								onclick={() => openEdit('vaccination', vax)}
								class="w-full text-left flex items-center justify-between p-2 bg-green-50 dark:bg-green-900/20 rounded hover:bg-green-100 dark:hover:bg-green-900/40 transition-colors group"
							>
								<span class="font-medium text-gray-800 dark:text-gray-200">{vax.vaccine_name}</span>
								<div class="flex items-center gap-2">
									<span class="text-sm text-gray-500 dark:text-gray-400">{formatDate(vax.date_given)}</span>
									<i class="fa-solid fa-pencil text-xs text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"></i>
								</div>
							</button>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>

		<!-- Social History -->
		<CollapsibleSection title="Social History ({socialHistory.length})">
			{#snippet headerActions()}
				<button
					onclick={() => openAdd('social')}
					class="p-1 text-gray-400 hover:text-blue-500 transition-colors"
					title="Add social history"
				>
					<i class="fa-solid fa-plus text-xs"></i>
				</button>
			{/snippet}
			{#snippet children()}
				{#if socialHistory.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No social history recorded</p>
				{:else}
					<div class="space-y-2">
						{#each socialHistory as item}
							<button
								onclick={() => openEdit('social', item)}
								class="w-full text-left p-2 bg-blue-50 dark:bg-blue-900/20 rounded hover:bg-blue-100 dark:hover:bg-blue-900/40 transition-colors group"
							>
								<div class="flex items-center justify-between">
									<div class="flex items-center gap-2">
										<span class="font-medium text-gray-800 dark:text-gray-200">{item.category}:</span>
										{#if item.status}
											<span class="text-xs px-2 py-0.5 rounded bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-300">
												{item.status}
											</span>
										{/if}
									</div>
									<i class="fa-solid fa-pencil text-xs text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"></i>
								</div>
								<p class="text-sm text-gray-600 dark:text-gray-400">{item.detail}</p>
							</button>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>

		<!-- Family History -->
		<CollapsibleSection title="Family History ({familyHistory.length})">
			{#snippet headerActions()}
				<button
					onclick={() => openAdd('family')}
					class="p-1 text-gray-400 hover:text-blue-500 transition-colors"
					title="Add family history"
				>
					<i class="fa-solid fa-plus text-xs"></i>
				</button>
			{/snippet}
			{#snippet children()}
				{#if familyHistory.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No family history recorded</p>
				{:else}
					<div class="space-y-2">
						{#each familyHistory as item}
							<button
								onclick={() => openEdit('family', item)}
								class="w-full text-left p-2 bg-purple-50 dark:bg-purple-900/20 rounded hover:bg-purple-100 dark:hover:bg-purple-900/40 transition-colors group"
							>
								<div class="flex items-center justify-between">
									<span class="font-medium text-gray-800 dark:text-gray-200">{item.relation}</span>
									<div class="flex items-center gap-2">
										{#if item.age_at_onset}
											<span class="text-xs text-gray-500 dark:text-gray-400">Age {item.age_at_onset}</span>
										{/if}
										<i class="fa-solid fa-pencil text-xs text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"></i>
									</div>
								</div>
								<p class="text-sm text-gray-600 dark:text-gray-400">{item.condition}</p>
							</button>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>
	</div>
</div>

<!-- Edit Modal -->
{#if editMode}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onclick={closeModal}>
		<div
			class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md mx-4 overflow-hidden"
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Modal Header -->
			<div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
					{isNew ? 'Add' : 'Edit'}
					{editMode === 'allergy' ? 'Allergy' :
					 editMode === 'vaccination' ? 'Vaccination' :
					 editMode === 'social' ? 'Social History' : 'Family History'}
				</h2>
				<button
					onclick={closeModal}
					class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-full transition-colors"
				>
					<i class="fa-solid fa-times text-gray-500"></i>
				</button>
			</div>

			<!-- Modal Body -->
			<div class="p-4 space-y-4">
				{#if editMode === 'allergy'}
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Allergen <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={allergyForm.allergen}
							placeholder="e.g., Penicillin"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						/>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Reaction</label>
						<input
							type="text"
							bind:value={allergyForm.reaction}
							placeholder="e.g., Hives, Anaphylaxis"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						/>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Severity</label>
						<select
							bind:value={allergyForm.severity}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						>
							{#each severityOptions as severity}
								<option value={severity}>{severity.charAt(0).toUpperCase() + severity.slice(1)}</option>
							{/each}
						</select>
					</div>
				{:else if editMode === 'vaccination'}
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Vaccine Name <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={vaccinationForm.vaccine_name}
							placeholder="e.g., Influenza, COVID-19"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						/>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Date Given <span class="text-red-500">*</span>
						</label>
						<input
							type="date"
							bind:value={vaccinationForm.date_given}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						/>
					</div>
				{:else if editMode === 'social'}
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Category</label>
						<select
							bind:value={socialForm.category}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						>
							{#each socialCategories as cat}
								<option value={cat}>{cat}</option>
							{/each}
						</select>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Details <span class="text-red-500">*</span>
						</label>
						<textarea
							bind:value={socialForm.detail}
							rows={3}
							placeholder="Describe the details..."
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 resize-none"
						></textarea>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Status</label>
						<input
							type="text"
							bind:value={socialForm.status}
							placeholder="e.g., Current, Former, Never"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						/>
					</div>
				{:else if editMode === 'family'}
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Relation</label>
						<select
							bind:value={familyForm.relation}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						>
							{#each familyRelations as rel}
								<option value={rel}>{rel}</option>
							{/each}
						</select>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Condition <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={familyForm.condition}
							placeholder="e.g., Diabetes, Heart Disease"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						/>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Age at Onset</label>
						<input
							type="number"
							bind:value={familyForm.age_at_onset}
							min="0"
							max="120"
							placeholder="Optional"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500"
						/>
					</div>
				{/if}
			</div>

			<!-- Modal Footer -->
			<div class="flex justify-between items-center p-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700/50">
				<div>
					{#if !isNew}
						<button
							onclick={deleteItem}
							disabled={isSaving}
							class="px-3 py-2 text-sm font-medium text-red-600 hover:text-red-700 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors disabled:opacity-50"
						>
							<i class="fa-solid fa-trash mr-1"></i>
							Delete
						</button>
					{/if}
				</div>
				<div class="flex gap-3">
					<button
						onclick={closeModal}
						class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-lg transition-colors"
					>
						Cancel
					</button>
					<button
						onclick={handleSave}
						disabled={isSaving}
						class="px-4 py-2 text-sm font-medium text-white bg-blue-500 hover:bg-blue-600 rounded-lg transition-colors disabled:opacity-50 flex items-center gap-2"
					>
						{#if isSaving}
							<i class="fa-solid fa-spinner fa-spin"></i>
						{:else}
							<i class="fa-solid fa-check"></i>
						{/if}
						{isNew ? 'Add' : 'Save'}
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
