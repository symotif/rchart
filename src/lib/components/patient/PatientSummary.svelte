<script lang="ts">
	import type { Patient } from '$lib/types/patient';

	let { patient }: { patient: Patient } = $props();

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

	function getInitials(firstName: string, lastName: string): string {
		return `${firstName.charAt(0)}${lastName.charAt(0)}`.toUpperCase();
	}
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full">
	<div class="flex items-start gap-4">
		<!-- Patient Photo (circle) -->
		<div class="flex-shrink-0">
			{#if patient.photo_url}
				<img
					src={patient.photo_url}
					alt="{patient.first_name} {patient.last_name}"
					class="w-20 h-20 rounded-full object-cover border-2 border-gray-200 dark:border-gray-600"
				/>
			{:else}
				<!-- Placeholder with initials -->
				<div
					class="w-20 h-20 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white text-2xl font-bold border-2 border-gray-200 dark:border-gray-600"
				>
					{getInitials(patient.first_name, patient.last_name)}
				</div>
			{/if}
		</div>

		<!-- Name and Age -->
		<div class="flex-1 min-w-0">
			<h2 class="text-xl font-bold text-gray-800 dark:text-gray-100 truncate">
				{patient.first_name} {patient.last_name}
			</h2>
			<p class="text-gray-500 dark:text-gray-400">
				{calculateAge(patient.dob)} years old &bull; {patient.sex === 'M' ? 'Male' : patient.sex === 'F' ? 'Female' : patient.sex}
			</p>
			<p class="text-xs text-gray-400 dark:text-gray-500">
				ID: {patient.id?.toString().padStart(3, '0')}
			</p>
		</div>
	</div>

	<!-- AI Summary -->
	<div class="mt-4 p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-100 dark:border-blue-800">
		<div class="flex items-start gap-2">
			<i class="fa-solid fa-robot text-blue-500 dark:text-blue-400 mt-0.5"></i>
			<p class="text-sm text-gray-700 dark:text-gray-300 italic">
				{patient.ai_summary ?? 'Patient summary will be generated based on clinical history and recent visits.'}
			</p>
		</div>
	</div>
</div>
