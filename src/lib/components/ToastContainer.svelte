<script lang="ts">
	import { ToastStore, type Toast } from '../../stores/ToastStore';
	import { fly } from 'svelte/transition';

	function getToastStyles(type: Toast['type']) {
		switch (type) {
			case 'success':
				return {
					bg: 'bg-green-500',
					icon: 'fa-check-circle',
					iconColor: 'text-white'
				};
			case 'error':
				return {
					bg: 'bg-red-500',
					icon: 'fa-exclamation-circle',
					iconColor: 'text-white'
				};
			case 'warning':
				return {
					bg: 'bg-yellow-500',
					icon: 'fa-exclamation-triangle',
					iconColor: 'text-white'
				};
			case 'info':
			default:
				return {
					bg: 'bg-blue-500',
					icon: 'fa-info-circle',
					iconColor: 'text-white'
				};
		}
	}

	function dismiss(id: string) {
		ToastStore.remove(id);
	}
</script>

<div class="fixed bottom-10 right-4 z-[200] flex flex-col gap-2 pointer-events-none">
	{#each $ToastStore as toast (toast.id)}
		<div
			transition:fly={{ x: 300, duration: 300 }}
			class="pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg {getToastStyles(toast.type).bg} text-white min-w-[300px] max-w-md"
		>
			<i class="fa-solid {getToastStyles(toast.type).icon} text-lg flex-shrink-0"></i>
			<p class="flex-1 text-sm font-medium">{toast.message}</p>
			<button
				onclick={() => dismiss(toast.id)}
				class="p-1 hover:bg-white/20 rounded transition-colors flex-shrink-0"
				aria-label="Dismiss"
			>
				<i class="fa-solid fa-times text-sm"></i>
			</button>
		</div>
	{/each}
</div>
