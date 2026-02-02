<script lang="ts">
	import { AppDataStore } from '../../stores/AppDataStore';
	import { onMount } from 'svelte';

	let mounted = $state(false);

	onMount(() => {
		mounted = true;
		AppDataStore.initialize();
	});

	// Derive state from store
	let isLoading = $derived($AppDataStore.isLoading);
	let isLoaded = $derived($AppDataStore.isLoaded);
	let loadingStep = $derived($AppDataStore.loadingStep);
	let loadingProgress = $derived($AppDataStore.loadingProgress);
	let error = $derived($AppDataStore.error);
</script>

{#if !mounted || isLoading || (!isLoaded && !error)}
	<div class="fixed inset-0 z-[9999] flex items-center justify-center bg-gray-900">
		<div class="flex flex-col items-center gap-6 p-8">
			<!-- Logo/App name -->
			<div class="flex items-center gap-3 mb-4">
				<i class="fa-solid fa-heart-pulse text-4xl text-blue-500"></i>
				<span class="text-3xl font-bold text-white">rchart</span>
			</div>

			<!-- Spinner -->
			<div class="relative">
				<!-- Outer ring -->
				<div class="w-20 h-20 rounded-full border-4 border-gray-700"></div>
				<!-- Spinning arc -->
				<div
					class="absolute top-0 left-0 w-20 h-20 rounded-full border-4 border-transparent border-t-blue-500 border-r-blue-500 animate-spin"
				></div>
				<!-- Inner icon -->
				<div class="absolute inset-0 flex items-center justify-center">
					<i class="fa-solid fa-database text-xl text-gray-500 animate-pulse"></i>
				</div>
			</div>

			<!-- Progress bar -->
			<div class="w-64 h-2 bg-gray-700 rounded-full overflow-hidden">
				<div
					class="h-full bg-gradient-to-r from-blue-500 to-blue-400 rounded-full transition-all duration-300 ease-out"
					style="width: {loadingProgress}%"
				></div>
			</div>

			<!-- Loading message -->
			<div class="text-center">
				<p class="text-gray-300 text-sm font-medium min-h-[1.5rem]">
					{loadingStep || 'Starting up...'}
				</p>
				<p class="text-gray-500 text-xs mt-1">
					{loadingProgress}%
				</p>
			</div>
		</div>
	</div>
{:else if error}
	<div class="fixed inset-0 z-[9999] flex items-center justify-center bg-gray-900">
		<div class="flex flex-col items-center gap-6 p-8 max-w-md">
			<!-- Error icon -->
			<div class="w-16 h-16 rounded-full bg-red-500/20 flex items-center justify-center">
				<i class="fa-solid fa-exclamation-triangle text-3xl text-red-500"></i>
			</div>

			<!-- Error message -->
			<div class="text-center">
				<h2 class="text-xl font-bold text-white mb-2">Failed to Load</h2>
				<p class="text-gray-400 text-sm mb-4">
					{error}
				</p>
				<button
					onclick={() => AppDataStore.initialize()}
					class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors"
				>
					<i class="fa-solid fa-rotate-right mr-2"></i>
					Retry
				</button>
			</div>
		</div>
	</div>
{/if}
