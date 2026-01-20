<script lang="ts">
	let { data }: { data: { time: string; name: string; age: string; sex: string } } = $props();
</script>

<div class="bg-red-100 p-5 rounded-lg border-6 shadow text-black">
	<div class="flex flex-row justify-between items-center gap-5">
		<p class="font-bold text-lg">{data.time} pm</p>

		<div class="flex flex-col">
			<h3>{data.name}</h3>
			<p>{data.age}, {data.sex}</p>
		</div>
        <p>MDD</p>
		<div class="text-sm">
			<p>Birchwood Health</p>
			<p>Cardiology Clinic</p>
		</div>

		<button class="bg-blue-400 text-white rounded-xl font-bold p-2"><i>Edit</i></button>
		<button class="bg-green-500 text-white rounded-xl font-bold p-2">Start</button>
	</div>
</div>
