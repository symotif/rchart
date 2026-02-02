<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		TabStore,
		ActiveTabStore,
		HistoryIndexStore,
		HistoryStore,
		setActiveTab,
		removeTab,
		getTab,
		getTabByPath,
		addToHistory,
		goBack,
		goForward
	} from '../../stores/TabStore';

	// Reactive check for back/forward availability
	let canBack = $derived($HistoryIndexStore > 0);
	let canForward = $derived($HistoryIndexStore < $HistoryStore.length - 1);

	function handleTabClick(tabId: string) {
		const tab = getTab(tabId);
		if (tab) {
			setActiveTab(tabId);
			addToHistory(tab.path);
			goto(tab.path);
		}
	}

	function handleRemoveTab(e: MouseEvent, tabId: string) {
		e.stopPropagation();
		const navigateTo = removeTab(tabId);
		if (navigateTo && navigateTo !== 'no-tabs') {
			goto(navigateTo);
		}
		// If 'no-tabs', layout will show the version screen
	}

	function handleBack() {
		const path = goBack();
		if (path) {
			const tab = getTabByPath(path);
			if (tab) {
				setActiveTab(tab.id);
			}
			goto(path);
		}
	}

	function handleForward() {
		const path = goForward();
		if (path) {
			const tab = getTabByPath(path);
			if (tab) {
				setActiveTab(tab.id);
			}
			goto(path);
		}
	}
</script>

<!-- the list of tabs with navigation -->
<div class="flex bg-white dark:bg-gray-800 mt-3 items-end -ml-5">
	<!-- Back/Forward Navigation Buttons (combined single element) -->
	<div class="nav-buttons flex items-end mr-0.5 bg-gray-100 dark:bg-gray-900 border-t border-r border-gray-300 dark:border-gray-600 rounded-tr-lg overflow-hidden h-[28px]">
		<button
			class="px-2.5 h-full flex items-center transition-colors border-r border-gray-300 dark:border-gray-600
				{canBack
					? 'hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-300'
					: 'text-gray-300 dark:text-gray-600 cursor-not-allowed'}"
			onclick={handleBack}
			disabled={!canBack}
			title="Go back"
		>
			<i class="fa-solid fa-chevron-left text-sm"></i>
		</button>
		<button
			class="px-2.5 h-full flex items-center transition-colors
				{canForward
					? 'hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-300'
					: 'text-gray-300 dark:text-gray-600 cursor-not-allowed'}"
			onclick={handleForward}
			disabled={!canForward}
			title="Go forward"
		>
			<i class="fa-solid fa-chevron-right text-sm"></i>
		</button>
	</div>

	<!-- Tab List -->
	{#each $TabStore as tab (tab.id)}
		<div
			class="tab-item flex items-center pl-3 pr-1.5 pt-2 pb-1 border-t border-l border-r border-gray-300 dark:border-gray-600 rounded-t-lg cursor-pointer mr-0.5 transition-colors h-[28px]
				{$ActiveTabStore === tab.id ? 'bg-gray-200 dark:bg-gray-700 border-b-white dark:border-b-gray-800 -mb-px' : 'bg-gray-100 dark:bg-gray-900 hover:bg-gray-150 dark:hover:bg-gray-600'}"
			onclick={() => handleTabClick(tab.id)}
			onkeydown={(e) => e.key === 'Enter' && handleTabClick(tab.id)}
			role="tab"
			tabindex="0"
			aria-selected={$ActiveTabStore === tab.id}
		>
			<span class="text-gray-700 dark:text-gray-200 text-sm whitespace-nowrap">{tab.title}</span>

			<button
				class="ml-1.5 text-gray-400 hover:text-red-600 dark:text-gray-500 dark:hover:text-red-400 text-xs font-bold leading-none p-0.5 rounded hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
				onclick={(e) => handleRemoveTab(e, tab.id)}
				aria-label="Close tab"
			>
				✕
			</button>
		</div>
	{/each}
</div>

<style>
	.tab-item {
		min-width: 80px;
		max-width: 200px;
	}
</style>
