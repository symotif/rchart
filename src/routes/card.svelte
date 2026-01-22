<script lang="ts">
	import { goto } from '$app/navigation';
	import { addTab } from '../stores/TabStore';

	let { data }: { data: { time: string; name: string; age: string; sex: string; patientId?: number; photoUrl?: string } } = $props();

	function handleStart() {
		const patientId = data.patientId || 1;
		const tab = {
			id: `note-new-${patientId}`,
			title: `New Note - ${data.name}`,
			path: `/patient/${patientId}/note/new`
		};
		addTab(tab);
		goto(tab.path);
	}

	function handleEdit() {
		const patientId = data.patientId || 1;
		const tab = {
			id: `patient-${patientId}`,
			title: data.name,
			path: `/patient/${patientId}`
		};
		addTab(tab);
		goto(tab.path);
	}

	// Get initials from name for avatar fallback
	function getInitials(name: string): string {
		return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2);
	}

	// Generate a consistent color based on patient ID
	function getAvatarColor(id: number): string {
		const colors = [
			'bg-blue-500', 'bg-green-500', 'bg-purple-500', 'bg-pink-500',
			'bg-indigo-500', 'bg-teal-500', 'bg-orange-500', 'bg-cyan-500'
		];
		return colors[id % colors.length];
	}
</script>

<div class="bg-white dark:bg-gray-700 p-3 rounded-lg shadow-md text-black dark:text-gray-100 flex-shrink-0 border border-gray-200 dark:border-gray-600 hover:shadow-lg transition-shadow">
	<div class="flex flex-row justify-between items-center gap-3 flex-wrap">
		<p class="font-bold text-base whitespace-nowrap">{data.time} pm</p>

		<!-- Patient Avatar and Info -->
		<div class="flex items-center gap-2 min-w-0">
			{#if data.photoUrl}
				<img
					src={data.photoUrl}
					alt={data.name}
					class="w-10 h-10 rounded-full object-cover border-2 border-white dark:border-gray-700 shadow-sm"
				/>
			{:else}
				<div class="w-10 h-10 rounded-full {getAvatarColor(data.patientId || 1)} flex items-center justify-center text-white font-bold text-sm border-2 border-white dark:border-gray-700 shadow-sm">
					{getInitials(data.name)}
				</div>
			{/if}
			<div class="flex flex-col min-w-0">
				<h3 class="text-sm font-medium truncate">{data.name}</h3>
				<p class="text-xs text-gray-600 dark:text-gray-300">{data.age}, {data.sex}</p>
			</div>
		</div>

		<p class="text-sm">MDD</p>
		<div class="text-xs text-gray-600 dark:text-gray-300 min-w-0">
			<p class="truncate">Birchwood Health</p>
			<p class="truncate">Cardiology Clinic</p>
		</div>

		<div class="flex gap-2 flex-shrink-0">
			<button
				onclick={handleEdit}
				class="bg-blue-400 dark:bg-blue-600 text-white rounded-xl font-bold px-2 py-1 text-sm hover:bg-blue-500 dark:hover:bg-blue-700 transition-colors"
			>
				<i class="fa-solid fa-pen-to-square mr-1"></i>Edit
			</button>
			<button
				onclick={handleStart}
				class="bg-green-500 dark:bg-green-600 text-white rounded-xl font-bold px-2 py-1 text-sm hover:bg-green-600 dark:hover:bg-green-700 transition-colors"
			>
				<i class="fa-solid fa-play mr-1"></i>Start
			</button>
		</div>
	</div>
</div>
