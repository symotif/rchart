<script lang="ts">
	import { goto } from '$app/navigation';
	import { addTab } from '../../../stores/TabStore';
	import type { Encounter } from '$lib/types/patient';

	let { encounters, patientId, patientName }: { encounters: Encounter[]; patientId: number; patientName: string } = $props();

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function openEncounter(encounter: Encounter) {
		const encounterId = encounter.id;
		const tab = {
			id: `encounter-${encounterId}`,
			title: `${patientName} - ${formatDate(encounter.encounter_date)}`,
			path: `/patient/${patientId}/encounter/${encounterId}`
		};
		addTab(tab);
		goto(tab.path);
	}
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full flex flex-col">
	<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
		<i class="fa-solid fa-file-medical text-blue-500"></i>
		Encounters
	</h3>

	<div class="flex-1 overflow-y-auto space-y-2">
		{#if encounters.length === 0}
			<p class="text-sm text-gray-500 dark:text-gray-400 italic">No encounters recorded</p>
		{:else}
			{#each encounters as encounter}
				<button
					class="w-full text-left p-3 bg-gray-50 dark:bg-gray-700 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors border border-gray-200 dark:border-gray-600"
					onclick={() => openEncounter(encounter)}
				>
					<div class="flex items-center justify-between mb-1">
						<span class="text-sm font-medium text-gray-800 dark:text-gray-200">
							{formatDate(encounter.encounter_date)}
						</span>
						<span class="text-xs px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded">
							{encounter.encounter_type}
						</span>
					</div>
					{#if encounter.chief_complaint}
						<p class="text-xs text-gray-600 dark:text-gray-400 truncate">
							{encounter.chief_complaint}
						</p>
					{/if}
					{#if encounter.provider}
						<p class="text-xs text-gray-400 dark:text-gray-500 mt-1">
							{encounter.provider}
						</p>
					{/if}
				</button>
			{/each}
		{/if}
	</div>
</div>
