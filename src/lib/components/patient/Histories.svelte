<script lang="ts">
	import CollapsibleSection from '$lib/components/ui/CollapsibleSection.svelte';
	import type { Allergy, Vaccination, SocialHistory, FamilyHistory } from '$lib/types/patient';

	let {
		allergies,
		vaccinations,
		socialHistory,
		familyHistory
	}: {
		allergies: Allergy[];
		vaccinations: Vaccination[];
		socialHistory: SocialHistory[];
		familyHistory: FamilyHistory[];
	} = $props();

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
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full flex flex-col">
	<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
		<i class="fa-solid fa-clipboard-list text-green-500"></i>
		History
	</h3>

	<div class="flex-1 overflow-y-auto">
		<!-- Allergies -->
		<CollapsibleSection title="Allergies ({allergies.length})" defaultOpen={true}>
			{#snippet children()}
				{#if allergies.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No known allergies</p>
				{:else}
					<div class="space-y-2">
						{#each allergies as allergy}
							<div class="flex items-center justify-between p-2 bg-red-50 dark:bg-red-900/20 rounded">
								<div>
									<span class="font-medium text-gray-800 dark:text-gray-200">{allergy.allergen}</span>
									{#if allergy.reaction}
										<span class="text-sm text-gray-600 dark:text-gray-400"> - {allergy.reaction}</span>
									{/if}
								</div>
								{#if allergy.severity}
									<span class="text-xs px-2 py-0.5 rounded {getSeverityColor(allergy.severity)}">
										{allergy.severity}
									</span>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>

		<!-- Vaccinations -->
		<CollapsibleSection title="Vaccinations ({vaccinations.length})">
			{#snippet children()}
				{#if vaccinations.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No vaccinations recorded</p>
				{:else}
					<div class="space-y-2">
						{#each vaccinations as vax}
							<div class="flex items-center justify-between p-2 bg-green-50 dark:bg-green-900/20 rounded">
								<span class="font-medium text-gray-800 dark:text-gray-200">{vax.vaccine_name}</span>
								<span class="text-sm text-gray-500 dark:text-gray-400">{formatDate(vax.date_given)}</span>
							</div>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>

		<!-- Social History -->
		<CollapsibleSection title="Social History ({socialHistory.length})">
			{#snippet children()}
				{#if socialHistory.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No social history recorded</p>
				{:else}
					<div class="space-y-2">
						{#each socialHistory as item}
							<div class="p-2 bg-blue-50 dark:bg-blue-900/20 rounded">
								<div class="flex items-center gap-2">
									<span class="font-medium text-gray-800 dark:text-gray-200">{item.category}:</span>
									{#if item.status}
										<span class="text-xs px-2 py-0.5 rounded bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-300">
											{item.status}
										</span>
									{/if}
								</div>
								<p class="text-sm text-gray-600 dark:text-gray-400">{item.detail}</p>
							</div>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>

		<!-- Family History -->
		<CollapsibleSection title="Family History ({familyHistory.length})">
			{#snippet children()}
				{#if familyHistory.length === 0}
					<p class="text-sm text-gray-500 dark:text-gray-400 italic">No family history recorded</p>
				{:else}
					<div class="space-y-2">
						{#each familyHistory as item}
							<div class="p-2 bg-purple-50 dark:bg-purple-900/20 rounded">
								<div class="flex items-center justify-between">
									<span class="font-medium text-gray-800 dark:text-gray-200">{item.relation}</span>
									{#if item.age_at_onset}
										<span class="text-xs text-gray-500 dark:text-gray-400">Age {item.age_at_onset}</span>
									{/if}
								</div>
								<p class="text-sm text-gray-600 dark:text-gray-400">{item.condition}</p>
							</div>
						{/each}
					</div>
				{/if}
			{/snippet}
		</CollapsibleSection>
	</div>
</div>
