<script lang="ts">
	import type { Snippet } from 'svelte';

	type Tab = {
		id: string;
		label: string;
	};

	let {
		tabs,
		activeTab = $bindable(),
		children
	}: {
		tabs: Tab[];
		activeTab: string;
		children: Snippet<[string]>;
	} = $props();

	function selectTab(tabId: string) {
		activeTab = tabId;
	}
</script>

<div class="h-full flex flex-col">
	<!-- Tab buttons -->
	<div class="flex border-b border-gray-200 dark:border-gray-600 mb-2">
		{#each tabs as tab}
			<button
				class="px-4 py-2 text-sm font-medium transition-colors border-b-2 -mb-px
					{activeTab === tab.id
					? 'text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400'
					: 'text-gray-500 dark:text-gray-400 border-transparent hover:text-gray-700 dark:hover:text-gray-300'}"
				onclick={() => selectTab(tab.id)}
			>
				{tab.label}
			</button>
		{/each}
	</div>

	<!-- Tab content -->
	<div class="flex-1 overflow-auto">
		{@render children(activeTab)}
	</div>
</div>
