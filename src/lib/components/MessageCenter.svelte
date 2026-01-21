<script lang="ts">
	let isOpen = $state(false);

	const toggleDrawer = () => {
		isOpen = !isOpen;
	};
</script>

<!-- Invisible click-away area (no shadow) -->
{#if isOpen}
	<div class="fixed inset-0 z-40" onclick={toggleDrawer}></div>
{/if}

<!-- Drawer -->
<div
	class="fixed right-0 top-20 h-full my-8 pb-5 bg-gray-100 dark:bg-gray-800 shadow-sm rounded-lg transition-transform duration-300 flex flex-row"
	class:translate-x-[83%]={!isOpen}
>
	<!-- Pull Tab -->
	<div class="flex flex-col justify-center items-center p-0 m-0 w-0">
		<div
			class="relative -rotate-90 px-4 py-2 mr-8 bg-gray-100 dark:bg-gray-800 rounded-lg cursor-pointer whitespace-nowrap"
			onclick={toggleDrawer}
		>
			<span class="text-m font-bold text-gray-800 dark:text-gray-100">Message Center</span>
		</div>
	</div>

	<!-- Drawer Contents -->
	<div class="flex flex-row py-4 px-4 mx-2 h-full ">
		<!-- Messages -->
		<div class="flex flex-col">
			<h2 class="text-lg font-semibold text-gray-800 dark:text-gray-100">Message Center</h2>

			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">New Message</p>
		</div>
	</div>
</div>
