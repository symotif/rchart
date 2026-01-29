<script lang="ts">
	import { ResourceStore } from '../../stores/ResourceStore';
	import DrugLibrary from '$lib/components/resources/DrugLibrary.svelte';
	import WebResourceViewer from '$lib/components/resources/WebResourceViewer.svelte';

	const tabs = ResourceStore.tabs;
	const activeTabId = ResourceStore.activeTabId;
	const extensions = ResourceStore.extensions;

	let showExtensionPicker = $state(false);

	function getActiveExtension() {
		const tabId = $activeTabId;
		const tab = $tabs.find(t => t.id === tabId);
		if (!tab) return null;
		return $extensions.find(e => e.id === tab.extensionId) || null;
	}

	function handleTabClick(tabId: string) {
		ResourceStore.setActiveTab(tabId);
	}

	function handleCloseTab(e: MouseEvent, tabId: string) {
		e.stopPropagation();
		ResourceStore.closeTab(tabId);
	}

	function handleOpenExtension(extensionId: string) {
		ResourceStore.openTab(extensionId);
		showExtensionPicker = false;
	}

	const activeExtension = $derived(getActiveExtension());
</script>

<div class="fixed inset-0 left-20 top-[108px] bottom-6 flex flex-col bg-gray-100 dark:bg-gray-900">
	<!-- Resource Tabs Bar -->
	<div class="flex items-center gap-1 px-2 py-2 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<!-- Tabs -->
		<div class="flex items-center gap-1 flex-1 overflow-x-auto">
			{#each $tabs as tab}
				{@const ext = $extensions.find(e => e.id === tab.extensionId)}
				<div
					class="group flex items-center gap-2 px-3 py-1.5 rounded-lg transition-all min-w-0 cursor-pointer
						{$activeTabId === tab.id
							? 'bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300'
							: 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'}"
					onclick={() => handleTabClick(tab.id)}
					onkeydown={(e) => e.key === 'Enter' && handleTabClick(tab.id)}
					role="tab"
					tabindex="0"
					aria-selected={$activeTabId === tab.id}
				>
					{#if ext}
						<div class="w-5 h-5 rounded {ext.iconColor} flex items-center justify-center flex-shrink-0">
							<i class="fa-solid {ext.icon} text-[10px] text-white"></i>
						</div>
					{/if}
					<span class="text-sm font-medium truncate max-w-32">{tab.title}</span>
					<button
						onclick={(e) => handleCloseTab(e, tab.id)}
						class="p-0.5 rounded hover:bg-gray-200 dark:hover:bg-gray-600 opacity-0 group-hover:opacity-100 transition-opacity"
						aria-label="Close tab"
					>
						<i class="fa-solid fa-xmark text-xs"></i>
					</button>
				</div>
			{/each}
		</div>

		<!-- Add Extension Button -->
		<div class="relative">
			<button
				onclick={() => showExtensionPicker = !showExtensionPicker}
				class="flex items-center gap-1 px-3 py-1.5 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
			>
				<i class="fa-solid fa-plus"></i>
				<span>Add Resource</span>
			</button>

			<!-- Extension Picker Dropdown -->
			{#if showExtensionPicker}
				<div class="absolute right-0 top-full mt-2 w-72 bg-white dark:bg-gray-800 rounded-xl shadow-xl border border-gray-200 dark:border-gray-700 z-50 overflow-hidden">
					<div class="p-3 border-b border-gray-200 dark:border-gray-700">
						<h3 class="text-sm font-semibold text-gray-900 dark:text-gray-100">Available Resources</h3>
						<p class="text-xs text-gray-500 dark:text-gray-400">Open a resource in a new tab</p>
					</div>
					<div class="p-2 max-h-64 overflow-y-auto">
						{#each $extensions.filter(e => e.isEnabled) as ext}
							<button
								onclick={() => handleOpenExtension(ext.id)}
								class="w-full flex items-center gap-3 p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors text-left"
							>
								<div class="w-10 h-10 rounded-lg {ext.iconColor} flex items-center justify-center flex-shrink-0">
									<i class="fa-solid {ext.icon} text-white"></i>
								</div>
								<div class="flex-1 min-w-0">
									<h4 class="text-sm font-medium text-gray-900 dark:text-gray-100">{ext.displayName}</h4>
									<p class="text-xs text-gray-500 dark:text-gray-400 truncate">{ext.description}</p>
								</div>
								{#if ext.requiresAuth}
									<i class="fa-solid fa-lock text-xs text-gray-400" title="Requires login"></i>
								{/if}
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	</div>

	<!-- Tab Content -->
	<div class="flex-1 overflow-hidden">
		{#if $tabs.length === 0}
			<!-- Empty state -->
			<div class="flex flex-col items-center justify-center h-full text-center p-8">
				<div class="w-24 h-24 rounded-2xl bg-gradient-to-br from-blue-500 to-purple-500 flex items-center justify-center mb-6">
					<i class="fa-solid fa-book-medical text-4xl text-white"></i>
				</div>
				<h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2">Medical Resources</h2>
				<p class="text-gray-600 dark:text-gray-400 mb-6 max-w-md">
					Access drug information, clinical references, and evidence-based resources to support your clinical decisions.
				</p>

				<div class="grid gap-3 w-full max-w-md">
					{#each $extensions.filter(e => e.isEnabled) as ext}
						<button
							onclick={() => handleOpenExtension(ext.id)}
							class="flex items-center gap-4 p-4 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-blue-300 dark:hover:border-blue-600 hover:shadow-md transition-all text-left"
						>
							<div class="w-12 h-12 rounded-xl {ext.iconColor} flex items-center justify-center flex-shrink-0">
								<i class="fa-solid {ext.icon} text-xl text-white"></i>
							</div>
							<div class="flex-1">
								<h3 class="font-semibold text-gray-900 dark:text-gray-100">{ext.displayName}</h3>
								<p class="text-sm text-gray-500 dark:text-gray-400">{ext.description}</p>
							</div>
							<i class="fa-solid fa-chevron-right text-gray-400"></i>
						</button>
					{/each}
				</div>
			</div>
		{:else if activeExtension}
			<!-- Active tab content -->
			{#if activeExtension.type === 'drug-library'}
				<DrugLibrary />
			{:else if activeExtension.type === 'openevidence' || activeExtension.type === 'uptodate' || activeExtension.type === 'external'}
				<WebResourceViewer extension={activeExtension} />
			{/if}
		{/if}
	</div>
</div>

<!-- Click outside to close extension picker -->
{#if showExtensionPicker}
	<button
		class="fixed inset-0 z-40"
		onclick={() => showExtensionPicker = false}
		aria-label="Close menu"
	></button>
{/if}
