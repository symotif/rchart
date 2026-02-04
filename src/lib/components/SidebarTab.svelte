<script lang="ts">
	import { goto } from '$app/navigation';
	import { addTab, setActiveTab, getTabByPath, addToHistory } from '../../stores/TabStore';

	let { label, icon, path, isActive }: { label: string; icon: string; path: string; isActive: boolean } = $props();

	function handleClick(e: MouseEvent) {
		e.preventDefault();

		// Check if tab already exists for this path
		const existingTab = getTabByPath(path);
		if (existingTab) {
			setActiveTab(existingTab.id);
			addToHistory(path);
		} else {
			// Create a new tab for this sidebar item
			const tabId = label.toLowerCase().replace(/\s+/g, '-');
			addTab({
				id: tabId,
				title: label,
				path: path
			});
			addToHistory(path);
		}
		goto(path);
	}
</script>

<div class="group relative">
	<!-- icon -->
	<a href={path} aria-label={label} onclick={handleClick}>
		<div
			class={`flex items-center justify-center h-20 w-20 transition-all duration-300 ease-linear
					${isActive ? 'bg-gray-300 dark:bg-gray-600 text-gray-500 dark:text-gray-200' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-500 dark:hover:bg-gray-600 hover:text-white'}`}
		>
			<i class={`${icon} fa-solid text-2xl`}></i>
		</div>
	</a>

	<!-- tooltip -->
	<div
		class="absolute w-auto z-40 p-2 m-2 min-2-max left-20 top-5 rounded-md
                text-white bg-gray-500 dark:bg-gray-700 text-xs font-bold
                transition-all duration-100 scale-0 origin-left group-hover:scale-100 group-hover:block"
	>
		{label}
	</div>
</div>
