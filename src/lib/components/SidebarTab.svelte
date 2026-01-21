<script lang="ts">
	let { label, icon, path, isActive }: { label: string; icon: string; path: string; isActive: boolean } = $props();
</script>

<div class="group relative">
	<!-- icon -->
	<a href={path} aria-label={label}>
		<div
			class={`flex items-center justify-center h-20 w-20 transition-all duration-300 ease-linear
					${isActive ? 'bg-gray-300 dark:bg-gray-600 text-gray-500 dark:text-gray-200' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-500 dark:hover:bg-gray-600 hover:text-white'}`}
		>
			<i class={`${icon} fa-solid h-9 w-9`}></i>
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
