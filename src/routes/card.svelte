<script lang="ts">
	let { data }: { data: { time: string; name: string; age: string; sex: string } } = $props();
</script>

<div class="bg-red-100 dark:bg-red-900/40 p-3 rounded-lg border-6 shadow text-black dark:text-gray-100 flex-shrink-0">
	<div class="flex flex-row justify-between items-center gap-3 flex-wrap">
		<p class="font-bold text-base whitespace-nowrap">{data.time} pm</p>

		<div class="flex flex-col min-w-0">
			<h3 class="text-sm font-medium truncate">{data.name}</h3>
			<p class="text-xs text-gray-600 dark:text-gray-300">{data.age}, {data.sex}</p>
		</div>
		<p class="text-sm">MDD</p>
		<div class="text-xs text-gray-600 dark:text-gray-300 min-w-0">
			<p class="truncate">Birchwood Health</p>
			<p class="truncate">Cardiology Clinic</p>
		</div>

		<div class="flex gap-2 flex-shrink-0">
			<button class="bg-blue-400 dark:bg-blue-600 text-white rounded-xl font-bold px-2 py-1 text-sm"><i>Edit</i></button>
			<button class="bg-green-500 dark:bg-green-600 text-white rounded-xl font-bold px-2 py-1 text-sm">Start</button>
		</div>
	</div>
</div>
