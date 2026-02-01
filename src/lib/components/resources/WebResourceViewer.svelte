<script lang="ts">
	import { ResourceStore } from '../../../stores/ResourceStore';
	import type { ResourceExtension } from '$lib/types/resource';

	interface Props {
		extension: ResourceExtension;
	}

	let { extension }: Props = $props();

	let isLoading = $state(true);
	let hasError = $state(false);
	let showLoginPrompt = $state(extension.requiresAuth && !extension.isAuthenticated);

	function handleLoad() {
		isLoading = false;
	}

	function handleError() {
		isLoading = false;
		hasError = true;
	}

	function dismissLoginPrompt() {
		showLoginPrompt = false;
		ResourceStore.setExtensionAuth(extension.id, true);
	}

	function openInBrowser() {
		if (extension.url) {
			// In Tauri, we can use shell.open, but for web we use window.open
			window.open(extension.url, '_blank');
		}
	}

	function getExtensionInfo(extId: string): { name: string; description: string; loginUrl: string; features: string[] } {
		switch (extId) {
			case 'openevidence':
				return {
					name: 'OpenEvidence',
					description: 'OpenEvidence provides AI-powered clinical answers sourced from peer-reviewed medical literature. Get evidence-based answers to your clinical questions.',
					loginUrl: 'https://www.openevidence.com/login',
					features: [
						'AI-powered clinical Q&A',
						'Peer-reviewed literature citations',
						'Drug information and interactions',
						'Clinical decision support'
					]
				};
			case 'uptodate':
				return {
					name: 'UpToDate',
					description: 'UpToDate is an evidence-based clinical decision support resource. Access comprehensive, current medical information written and edited by physicians.',
					loginUrl: 'https://www.uptodate.com/login',
					features: [
						'Evidence-based recommendations',
						'Drug information (Lexicomp)',
						'Medical calculators',
						'Patient education handouts',
						'Graphics and tables'
					]
				};
			default:
				return {
					name: extension.displayName,
					description: extension.description,
					loginUrl: extension.url || '',
					features: []
				};
		}
	}

	const extInfo = $derived(getExtensionInfo(extension.id));
</script>

<div class="flex flex-col h-full bg-white dark:bg-gray-900">
	{#if showLoginPrompt}
		<!-- Login/Auth prompt -->
		<div class="flex-1 flex items-center justify-center p-8">
			<div class="max-w-lg text-center">
				<div class="w-20 h-20 mx-auto mb-6 rounded-2xl {extension.iconColor} flex items-center justify-center">
					<i class="fa-solid {extension.icon} text-3xl text-white"></i>
				</div>

				<h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2">
					Connect to {extInfo.name}
				</h2>

				<p class="text-gray-600 dark:text-gray-400 mb-6">
					{extInfo.description}
				</p>

				{#if extInfo.features.length > 0}
					<div class="bg-gray-50 dark:bg-gray-800 rounded-xl p-4 mb-6 text-left">
						<h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">Features</h3>
						<ul class="space-y-2">
							{#each extInfo.features as feature}
								<li class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
									<i class="fa-solid fa-check text-green-500"></i>
									{feature}
								</li>
							{/each}
						</ul>
					</div>
				{/if}

				<div class="space-y-3">
					<button
						onclick={dismissLoginPrompt}
						class="w-full px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-xl transition-colors"
					>
						<i class="fa-solid fa-arrow-right-to-bracket mr-2"></i>
						Continue to {extInfo.name}
					</button>

					<button
						onclick={openInBrowser}
						class="w-full px-6 py-3 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 font-medium rounded-xl transition-colors"
					>
						<i class="fa-solid fa-arrow-up-right-from-square mr-2"></i>
						Open in External Browser
					</button>
				</div>

				<p class="mt-4 text-xs text-gray-500 dark:text-gray-500">
					You may need to log in with your institutional or personal credentials.
				</p>
			</div>
		</div>
	{:else}
		<!-- Toolbar -->
		<div class="flex items-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
			<div class="flex items-center gap-2 flex-1">
				<div class="w-6 h-6 rounded {extension.iconColor} flex items-center justify-center">
					<i class="fa-solid {extension.icon} text-xs text-white"></i>
				</div>
				<span class="text-sm font-medium text-gray-700 dark:text-gray-300">{extension.displayName}</span>
				{#if extension.url}
					<span class="text-xs text-gray-500 dark:text-gray-500 truncate max-w-md">{extension.url}</span>
				{/if}
			</div>

			<button
				onclick={openInBrowser}
				class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors"
				title="Open in external browser"
			>
				<i class="fa-solid fa-arrow-up-right-from-square"></i>
			</button>

			<button
				onclick={() => { isLoading = true; hasError = false; }}
				class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors"
				title="Refresh"
			>
				<i class="fa-solid fa-rotate-right"></i>
			</button>
		</div>

		<!-- Content area -->
		<div class="flex-1 relative">
			{#if isLoading}
				<div class="absolute inset-0 flex items-center justify-center bg-white dark:bg-gray-900">
					<div class="text-center">
						<div class="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
						<p class="text-gray-500 dark:text-gray-400">Loading {extension.displayName}...</p>
					</div>
				</div>
			{/if}

			{#if hasError}
				<div class="absolute inset-0 flex items-center justify-center bg-white dark:bg-gray-900">
					<div class="text-center max-w-md p-6">
						<div class="w-16 h-16 mx-auto mb-4 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center">
							<i class="fa-solid fa-triangle-exclamation text-2xl text-red-500"></i>
						</div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2">Unable to Load</h3>
						<p class="text-gray-600 dark:text-gray-400 mb-4">
							The resource could not be loaded. This may be due to network issues or the site blocking embedded content.
						</p>
						<div class="space-y-2">
							<button
								onclick={() => { isLoading = true; hasError = false; }}
								class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
							>
								Try Again
							</button>
							<button
								onclick={openInBrowser}
								class="w-full px-4 py-2 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg transition-colors"
							>
								Open in External Browser
							</button>
						</div>
					</div>
				</div>
			{/if}

			{#if extension.url && !hasError}
				<iframe
					src={extension.url}
					title={extension.displayName}
					class="w-full h-full border-0"
					onload={handleLoad}
					onerror={handleError}
					sandbox="allow-same-origin allow-scripts allow-popups allow-forms allow-downloads"
				></iframe>
			{/if}
		</div>
	{/if}
</div>
