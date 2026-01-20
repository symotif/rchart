<script lang="ts">
	import {
		TabStore,
		ActiveTabStore,
		addTab,
		setActiveTab,
		removeTab
	} from '../../stores/TabStore';

	let newTabTitle: string = $state('');

	function handleAddTab() {
		const newTab = {
			id: `tab-${Date.now()}`,
			title: newTabTitle
		};
		addTab(newTab);
		newTabTitle = ''; // reset title after adding
	}

	function handleTabClick(tabId: string) {
		setActiveTab(tabId);
	}

	function handleRemoveTab(tabId: string) {
		removeTab(tabId);
	}
</script>

<!-- the list of tabs -->
<div class="flex bg-white mt-3">
	{#each $TabStore as tab (tab.id)}
		<div>
			<!-- a single tab -->
			<div
				class="text-gray-500 px-4 pt-1 inline-flex border-r border-t border-l border-gray-400 active:rounded-t-lg active:bg-gray-300 active:border-none"
			>
				{tab.title}

				<!-- delete tab button -->
				<button
					class="text-black text-sm -mt-2 -mr-3 ml-2 hover:text-red-700"
					onclick={() => handleRemoveTab(tab.id)}
				>
					X
				</button>
			</div>
		</div>
	{/each}
</div>

<style>
</style>
