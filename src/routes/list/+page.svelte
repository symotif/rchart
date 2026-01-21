<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { addTab } from '../../stores/TabStore';
	import { invoke } from '@tauri-apps/api/tauri';
	let loading = $state(true);
	let error = $state('');

	interface Patient {
		id: number | null;
		first_name: string;
		last_name: string;
		dob: string;
		sex: string;
		gender: string | null;
		address: string | null;
		phone: string | null;
		email: string | null;
	}

	let patients = $state<Patient[]>([]);

	function calculateAge(dob: string): number {
		const birthDate = new Date(dob);
		const today = new Date();
		let age = today.getFullYear() - birthDate.getFullYear();
		const monthDiff = today.getMonth() - birthDate.getMonth();
		if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
			age--;
		}
		return age;
	}

	function formatDob(dob: string): string {
		const date = new Date(dob);
		return `${date.getMonth() + 1}/${date.getDate()}/${date.getFullYear() % 100}`;
	}

	async function loadPatients() {
		try {
			loading = true;
			error = '';

			// First, try to seed test data if database is empty
			const seedResult = await invoke<string>('db_seed_test_data');
			console.log('Seed result:', seedResult);

			// Now fetch all patients
			patients = await invoke<Patient[]>('db_get_all_patients');
			console.log('Loaded patients:', patients);
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('Failed to load patients:', e);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		loadPatients();
	});

	function onPatientRowClick(patientId: string, patientName: string): void {
		const newTab = {
			id: `patient-${patientId}`,
			title: patientName,
			path: `/patient/${patientId}`
		};
		addTab(newTab);
		goto(`/patient/${patientId}`);
	}
</script>

<div class="">
	{#if loading}
		<div class="absolute left-20 top-20 mx-5 my-8 text-gray-600">
			Loading patients...
		</div>
	{:else if error}
		<div class="absolute left-20 top-20 mx-5 my-8 text-red-600">
			Error: {error}
		</div>
	{:else}
		<table class="overflow-hidden absolute left-20 top-20 mx-5 my-8 rounded-lg shadow w-[92%]">
			<thead class="bg-gray-50 border-b-2 border-gray-200">
				<tr>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">#</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">pic</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">First Name</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">Last Name</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">DOB</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">Age</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">Sex</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">Gender</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">Address</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">Phone Number</th>
					<th class="p-3 text-sm font-semibold tracking-wide text-left">Email</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-100">
				{#each patients as patient}
					<tr
						onclick={() =>
							onPatientRowClick(
								String(patient.id),
								`${patient.first_name} ${patient.last_name}`
							)}
						class="bg-white cursor-pointer hover:bg-gray-100 transition-colors"
					>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.id?.toString().padStart(3, '0') ?? '---'}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"></td>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.first_name}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.last_name}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{formatDob(patient.dob)}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{calculateAge(patient.dob)}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.sex}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.gender ?? '-'}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.address ?? '-'}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.phone ?? '-'}</td
						>
						<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
							>{patient.email ?? '-'}</td
						>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>
