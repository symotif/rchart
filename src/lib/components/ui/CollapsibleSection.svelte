<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		defaultOpen = false,
		children,
		headerActions
	}: {
		title: string;
		defaultOpen?: boolean;
		children: Snippet;
		headerActions?: Snippet;
	} = $props();

	let isOpen = $state(defaultOpen);

	function toggle() {
		isOpen = !isOpen;
	}
</script>

<div class="border border-gray-200 dark:border-gray-600 rounded-lg mb-2">
	<div class="flex items-center bg-gray-50 dark:bg-gray-700 rounded-t-lg">
		<button
			class="flex-1 flex items-center justify-between p-3 text-left hover:bg-gray-100 dark:hover:bg-gray-600 rounded-tl-lg transition-colors"
			onclick={toggle}
			aria-expanded={isOpen}
		>
			<span class="font-medium text-gray-700 dark:text-gray-200">{title}</span>
			<i
				class="fa-solid fa-chevron-down text-gray-500 dark:text-gray-400 transition-transform duration-200"
				class:rotate-180={isOpen}
			></i>
		</button>
		{#if headerActions}
			<div class="pr-3" onclick={(e) => e.stopPropagation()}>
				{@render headerActions()}
			</div>
		{/if}
	</div>

	{#if isOpen}
		<div class="p-3 border-t border-gray-200 dark:border-gray-600">
			{@render children()}
		</div>
	{/if}
</div>
