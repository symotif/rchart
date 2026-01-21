<script lang="ts">
	import { ConnectionStatusStore, setConnectionStatus } from '../../stores/ConnectionStatusStore';
</script>

<div
	class={`fixed bottom-0 left-0 h-6 w-screen z-[100] ${
		$ConnectionStatusStore
			? 'bg-gradient-to-r from-green-600 via-green-400 to-green-600 bg-[length:200%_100%] animate-shimmer'
			: 'bg-gradient-to-r from-red-400 via-red-600 to-red-400 bg-[length:200%_100%] animate-shimmer'
	}`}
>
	<div class="flex justify-between text-xs p-1 mx-1">
		<!-- Left Section -->
		<div class="flex items-center ml-5">
			<!-- Connection Status -->
			<p class={`font-bold ${$ConnectionStatusStore ? 'text-green-100' : 'text-red-100'}`}>
				{$ConnectionStatusStore ? 'Online' : 'Disconnected'}
			</p>
			<p class="text-gray-300 mx-8">|</p>

			<!-- Conditionally Render Connected To -->
			{#if $ConnectionStatusStore}
				<p>Connected to</p>
				<p class={`font-bold text-green-200 ml-1`}>Birchwood Health</p>
				<p class="text-gray-300 mx-8">|</p>
			{/if}

			<!-- Synced Nodes -->
			<p>Synced to</p>
			<p class={`mx-1 font-bold ${$ConnectionStatusStore ? 'text-green-200' : 'text-red-200'}`}>
				{$ConnectionStatusStore ? '20/20' : '0/20'}
			</p>
			<p>nodes</p>
		</div>

		<!-- Right Section -->
		<div class="flex items-center">
			<p class="mr-1">Last Edited @</p>
			<p class="font-bold text-white">5:15:21 PM EST</p>
		</div>
	</div>
</div>
