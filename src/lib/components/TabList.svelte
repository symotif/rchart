<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		TabStore,
		ActiveTabStore,
		setActiveTab,
		removeTab,
		getTab
	} from '../../stores/TabStore';

	function handleTabClick(tabId: string) {
		const tab = getTab(tabId);
		if (tab) {
			setActiveTab(tabId);
			goto(tab.path);
		}
	}

	function handleRemoveTab(e: MouseEvent, tabId: string) {
		e.stopPropagation(); // Prevent tab click from firing
		const navigateTo = removeTab(tabId);
		if (navigateTo) {
			goto(navigateTo);
		}
	}
</script>

<!-- the list of tabs -->
<div class="flex bg-white mt-3 items-end">
	{#each $TabStore as tab (tab.id)}
		<div
			class="tab-item flex items-center px-4 py-1 border-t border-l border-r border-gray-300 rounded-t-lg cursor-pointer mr-1 transition-colors
				{$ActiveTabStore === tab.id ? 'bg-gray-200 border-b-white -mb-px' : 'bg-gray-100 hover:bg-gray-150'}"
			onclick={() => handleTabClick(tab.id)}
			onkeydown={(e) => e.key === 'Enter' && handleTabClick(tab.id)}
			role="tab"
			tabindex="0"
			aria-selected={$ActiveTabStore === tab.id}
		>
			<span class="text-gray-700 text-sm whitespace-nowrap">{tab.title}</span>

			<!-- delete tab button (don't show for Dashboard) -->
			{#if tab.id !== 'dashboard'}
				<button
					class="ml-2 text-gray-400 hover:text-red-600 text-xs font-bold leading-none p-1 rounded hover:bg-gray-300 transition-colors"
					onclick={(e) => handleRemoveTab(e, tab.id)}
					aria-label="Close tab"
				>
					✕
				</button>
			{/if}
		</div>
	{/each}
</div>

<style>
	.tab-item {
		min-width: 80px;
		max-width: 200px;
	}
</style>
