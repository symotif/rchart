<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		SyncStatusStore,
		isOnline,
		overallStatus,
		isSyncing,
		lastEdited,
		institution,
		localNodesStatus,
		centralDbStatus,
		connectionMode,
		startConnectionMonitoring,
		getFormattedLastEdited
	} from '../../stores/SyncStatusStore';
	import { STATUS_COLORS, CONNECTION_MODE_NAMES } from '../types/sync';
	import type { SyncStatus } from '../types/sync';

	// Cleanup function for connection monitoring
	let stopMonitoring: (() => void) | null = null;

	// Formatted last edited time - reactive
	let formattedLastEdited = $derived(
		$lastEdited
			? $lastEdited.toLocaleTimeString('en-US', {
					hour: 'numeric',
					minute: '2-digit',
					second: '2-digit',
					hour12: true,
					timeZoneName: 'short'
				})
			: 'Never'
	);

	// Get status colors based on overall status
	let statusColors = $derived(STATUS_COLORS[$overallStatus] || STATUS_COLORS.disconnected);

	// Status display text
	let statusText = $derived($isOnline ? 'Online' : 'Disconnected');

	// Build sync status text
	let syncStatusText = $derived.by(() => {
		if ($connectionMode === 'offline') {
			return 'Offline Mode';
		}

		const parts: string[] = [];

		// Local nodes status
		if ($localNodesStatus.enabled) {
			const nodeText = `${$localNodesStatus.connectedNodes}/${$localNodesStatus.totalNodes} nodes`;
			parts.push(nodeText);
		}

		// Central DB status
		if ($centralDbStatus.enabled) {
			const dbStatus = $centralDbStatus.status === 'connected' ? 'DB Connected' : 'DB Disconnected';
			parts.push(dbStatus);
		}

		return parts.length > 0 ? parts.join(' • ') : 'Not configured';
	});

	onMount(() => {
		// Start monitoring internet connection
		stopMonitoring = startConnectionMonitoring(30000);
	});

	onDestroy(() => {
		if (stopMonitoring) {
			stopMonitoring();
		}
	});
</script>

<div
	class={`fixed bottom-0 left-0 h-6 w-screen z-[100] ${statusColors.bg} bg-[length:200%_100%] animate-shimmer`}
>
	<div class="flex justify-between text-xs p-1 mx-1">
		<!-- Left Section -->
		<div class="flex items-center ml-5 gap-2">
			<!-- Syncing Indicator -->
			{#if $isSyncing}
				<div class="flex items-center gap-1">
					<svg
						class="w-3 h-3 animate-spin text-white"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
					>
						<circle
							class="opacity-25"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
					<span class="text-white font-medium">Syncing...</span>
				</div>
				<span class="text-gray-200 mx-1">|</span>
			{/if}

			<!-- Connection Status -->
			<div class="flex items-center gap-1">
				<!-- Status dot -->
				<span
					class={`w-2 h-2 rounded-full ${
						$overallStatus === 'connected'
							? 'bg-green-200'
							: $overallStatus === 'syncing'
								? 'bg-yellow-200 animate-pulse'
								: $overallStatus === 'error'
									? 'bg-red-200'
									: 'bg-gray-300'
					}`}
				></span>
				<p class={`font-bold ${statusColors.text}`}>
					{statusText}
				</p>
			</div>

			<span class="text-gray-200">|</span>

			<!-- Institution (if connected) -->
			{#if $institution && $isOnline}
				<div class="flex items-center gap-1">
					<span class="text-gray-100">Connected to</span>
					<span class={`font-bold ${statusColors.text}`}>{$institution.name}</span>
					{#if $institution.department}
						<span class="text-gray-200">• {$institution.department}</span>
					{/if}
				</div>
				<span class="text-gray-200">|</span>
			{/if}

			<!-- Sync Status Details -->
			<div class="flex items-center gap-1">
				{#if $connectionMode !== 'offline'}
					<!-- Local Nodes -->
					{#if $localNodesStatus.enabled}
						<div class="flex items-center gap-1">
							<i class="fa-solid fa-network-wired text-gray-100 text-[10px]"></i>
							<span
								class={`font-medium ${
									$localNodesStatus.status === 'connected'
										? 'text-green-200'
										: $localNodesStatus.status === 'syncing'
											? 'text-yellow-200'
											: 'text-gray-200'
								}`}
							>
								{$localNodesStatus.connectedNodes}/{$localNodesStatus.totalNodes}
							</span>
							<span class="text-gray-100">nodes</span>
						</div>
					{/if}

					<!-- Separator if both enabled -->
					{#if $localNodesStatus.enabled && $centralDbStatus.enabled}
						<span class="text-gray-200 mx-1">•</span>
					{/if}

					<!-- Central DB -->
					{#if $centralDbStatus.enabled}
						<div class="flex items-center gap-1">
							<i class="fa-solid fa-database text-gray-100 text-[10px]"></i>
							<span
								class={`font-medium ${
									$centralDbStatus.status === 'connected'
										? 'text-green-200'
										: $centralDbStatus.status === 'syncing'
											? 'text-yellow-200'
											: 'text-gray-200'
								}`}
							>
								{$centralDbStatus.status === 'connected' ? 'Synced' : 'Not synced'}
							</span>
						</div>
					{/if}
				{:else}
					<span class="text-gray-200 italic">Offline Mode</span>
				{/if}
			</div>
		</div>

		<!-- Right Section -->
		<div class="flex items-center gap-2 mr-2">
			<!-- Connection Mode Badge -->
			<span
				class="px-2 py-0.5 rounded text-[10px] font-medium bg-black/20 text-white"
			>
				{CONNECTION_MODE_NAMES[$connectionMode]}
			</span>

			<span class="text-gray-200">|</span>

			<!-- Last Edited -->
			<div class="flex items-center gap-1">
				<i class="fa-solid fa-clock text-gray-100 text-[10px]"></i>
				<span class="text-gray-100">Last Edit</span>
				<span class="font-bold text-white">{formattedLastEdited}</span>
			</div>
		</div>
	</div>
</div>
