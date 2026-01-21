<script lang="ts">
	let { title, defaultOpen = false, children }: { title: string; defaultOpen?: boolean; children: any } = $props();

	let isOpen = $state(defaultOpen);

	function toggle() {
		isOpen = !isOpen;
	}
</script>

<div class="border border-gray-200 dark:border-gray-600 rounded-lg mb-2">
	<button
		class="w-full flex items-center justify-between p-3 text-left bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-t-lg transition-colors"
		onclick={toggle}
		aria-expanded={isOpen}
	>
		<span class="font-medium text-gray-700 dark:text-gray-200">{title}</span>
		<i
			class="fa-solid fa-chevron-down text-gray-500 dark:text-gray-400 transition-transform duration-200"
			class:rotate-180={isOpen}
		></i>
	</button>

	{#if isOpen}
		<div class="p-3 border-t border-gray-200 dark:border-gray-600">
			{@render children()}
		</div>
	{/if}
</div>
