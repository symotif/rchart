<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { setActiveTab } from '../../../stores/TabStore';

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

	let patient = $state<Patient | null>(null);
	let loading = $state(true);
	let error = $state('');

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
		return `${date.getMonth() + 1}/${date.getDate()}/${date.getFullYear()}`;
	}

	async function loadPatient(id: string) {
		try {
			loading = true;
			error = '';
			const patientId = parseInt(id, 10);
			const result = await invoke<Patient | null>('db_get_patient', { id: patientId });
			patient = result;

			// Set this tab as active
			setActiveTab(`patient-${id}`);
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('Failed to load patient:', e);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		const id = $page.params.id;
		if (id) {
			loadPatient(id);
		}
	});
</script>

<div class="absolute left-20 top-20 right-10 bottom-5 mx-5 my-8">
	{#if loading}
		<div class="text-gray-600">Loading patient...</div>
	{:else if error}
		<div class="text-red-600">Error: {error}</div>
	{:else if patient}
		<div class="bg-white rounded-lg shadow-lg p-6">
			<div class="border-b pb-4 mb-4">
				<h1 class="text-3xl font-bold text-gray-800">
					{patient.first_name} {patient.last_name}
				</h1>
				<p class="text-gray-500">Patient ID: {patient.id?.toString().padStart(3, '0')}</p>
			</div>

			<div class="grid grid-cols-2 gap-6">
				<div class="space-y-4">
					<h2 class="text-xl font-semibold text-gray-700">Demographics</h2>

					<div class="grid grid-cols-2 gap-4">
						<div>
							<label class="text-sm text-gray-500">Date of Birth</label>
							<p class="text-gray-800">{formatDob(patient.dob)}</p>
						</div>
						<div>
							<label class="text-sm text-gray-500">Age</label>
							<p class="text-gray-800">{calculateAge(patient.dob)} years</p>
						</div>
						<div>
							<label class="text-sm text-gray-500">Sex</label>
							<p class="text-gray-800">{patient.sex}</p>
						</div>
						<div>
							<label class="text-sm text-gray-500">Gender</label>
							<p class="text-gray-800">{patient.gender ?? '-'}</p>
						</div>
					</div>
				</div>

				<div class="space-y-4">
					<h2 class="text-xl font-semibold text-gray-700">Contact Information</h2>

					<div class="space-y-3">
						<div>
							<label class="text-sm text-gray-500">Address</label>
							<p class="text-gray-800">{patient.address ?? '-'}</p>
						</div>
						<div>
							<label class="text-sm text-gray-500">Phone</label>
							<p class="text-gray-800">{patient.phone ?? '-'}</p>
						</div>
						<div>
							<label class="text-sm text-gray-500">Email</label>
							<p class="text-gray-800">{patient.email ?? '-'}</p>
						</div>
					</div>
				</div>
			</div>

			<div class="mt-8 pt-4 border-t">
				<h2 class="text-xl font-semibold text-gray-700 mb-4">Quick Actions</h2>
				<div class="flex gap-3">
					<button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors">
						New Appointment
					</button>
					<button class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 transition-colors">
						Add Note
					</button>
					<button class="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600 transition-colors">
						View History
					</button>
				</div>
			</div>
		</div>
	{:else}
		<div class="text-gray-600">Patient not found</div>
	{/if}
</div>
