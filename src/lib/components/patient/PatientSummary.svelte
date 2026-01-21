<script lang="ts">
	import type { Patient, DiagnosisWithMedications, ClinicalScore } from '$lib/types/patient';
	import AvatarWithDiagnosisRing from './AvatarWithDiagnosisRing.svelte';

	let { patient, diagnoses, scores = [] }: { patient: Patient; diagnoses: DiagnosisWithMedications[]; scores?: ClinicalScore[] } = $props();

	let showPatientInfo = $state(false);

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

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function togglePatientInfo() {
		showPatientInfo = !showPatientInfo;
	}
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4">
	<div class="flex items-center gap-6">
		<!-- Patient Photo with Diagnosis Ring -->
		<div class="flex-shrink-0">
			<AvatarWithDiagnosisRing {patient} {diagnoses} {scores} />
		</div>

		<!-- Name, Age, ID (clickable for patient info popup) -->
		<div class="flex-shrink-0 relative">
			<button
				onclick={togglePatientInfo}
				class="text-left hover:bg-gray-100 dark:hover:bg-gray-700 p-2 -m-2 rounded-lg transition-colors"
				title="Click to view patient details"
			>
				<h2 class="text-xl font-bold text-gray-800 dark:text-gray-100">
					{patient.first_name} {patient.last_name}
				</h2>
				<p class="text-gray-500 dark:text-gray-400">
					{calculateAge(patient.dob)} years old &bull; {patient.sex === 'M' ? 'Male' : patient.sex === 'F' ? 'Female' : patient.sex}
					<span class="text-xs text-gray-400 dark:text-gray-500 ml-2">ID: {patient.id?.toString().padStart(3, '0')}</span>
				</p>
			</button>

			<!-- Patient Info Popup -->
			{#if showPatientInfo}
				<!-- Click-away overlay -->
				<div class="fixed inset-0 z-40" onclick={togglePatientInfo} role="presentation"></div>

				<div class="absolute top-full left-0 mt-2 w-80 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-600 z-50 p-4">
					<!-- Close button -->
					<button
						onclick={togglePatientInfo}
						class="absolute top-2 right-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
						aria-label="Close"
					>
						<i class="fa-solid fa-times"></i>
					</button>

					<h3 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-3">
						Patient Information
					</h3>

					<div class="space-y-3">
						<!-- Demographics -->
						<div>
							<h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1">
								Demographics
							</h4>
							<div class="text-sm text-gray-700 dark:text-gray-300 space-y-1">
								<p><span class="font-medium">Name:</span> {patient.first_name} {patient.last_name}</p>
								<p><span class="font-medium">DOB:</span> {formatDate(patient.dob)} ({calculateAge(patient.dob)} years old)</p>
								<p><span class="font-medium">Sex:</span> {patient.sex === 'M' ? 'Male' : patient.sex === 'F' ? 'Female' : patient.sex}</p>
								{#if patient.gender}
									<p><span class="font-medium">Gender:</span> {patient.gender}</p>
								{/if}
								<p><span class="font-medium">ID:</span> {patient.id?.toString().padStart(3, '0')}</p>
							</div>
						</div>

						<!-- Contact Info -->
						<div class="border-t border-gray-200 dark:border-gray-600 pt-3">
							<h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1">
								Contact
							</h4>
							<div class="text-sm text-gray-700 dark:text-gray-300 space-y-1">
								{#if patient.address}
									<p><span class="font-medium">Address:</span> {patient.address}</p>
								{/if}
								{#if patient.phone}
									<p><span class="font-medium">Phone:</span> {patient.phone}</p>
								{/if}
								{#if patient.email}
									<p><span class="font-medium">Email:</span> {patient.email}</p>
								{/if}
								{#if !patient.address && !patient.phone && !patient.email}
									<p class="text-gray-400 dark:text-gray-500 italic">No contact info on file</p>
								{/if}
							</div>
						</div>

						<!-- Pharmacy -->
						<div class="border-t border-gray-200 dark:border-gray-600 pt-3">
							<h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1">
								<i class="fa-solid fa-prescription-bottle-medical mr-1"></i>
								Preferred Pharmacy
							</h4>
							<p class="text-sm text-gray-700 dark:text-gray-300">
								{patient.preferred_pharmacy ?? 'Not specified'}
							</p>
						</div>

						<!-- Insurance -->
						<div class="border-t border-gray-200 dark:border-gray-600 pt-3">
							<h4 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1">
								<i class="fa-solid fa-id-card mr-1"></i>
								Insurance
							</h4>
							{#if patient.insurance_provider}
								<div class="text-sm text-gray-700 dark:text-gray-300 space-y-1">
									<p><span class="font-medium">Provider:</span> {patient.insurance_provider}</p>
									{#if patient.insurance_policy_number}
										<p><span class="font-medium">Policy #:</span> {patient.insurance_policy_number}</p>
									{/if}
									{#if patient.insurance_group_number}
										<p><span class="font-medium">Group #:</span> {patient.insurance_group_number}</p>
									{/if}
								</div>
							{:else}
								<p class="text-sm text-gray-400 dark:text-gray-500 italic">No insurance on file</p>
							{/if}
						</div>
					</div>
				</div>
			{/if}
		</div>

		<!-- AI Summary (horizontal, takes remaining space) -->
		<div class="flex-1 ml-4 p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-100 dark:border-blue-800">
			<div class="flex items-start gap-2">
				<i class="fa-solid fa-robot text-blue-500 dark:text-blue-400 mt-0.5 flex-shrink-0"></i>
				<p class="text-sm text-gray-700 dark:text-gray-300 italic">
					{patient.ai_summary ?? 'Patient summary will be generated based on clinical history and recent visits.'}
				</p>
			</div>
		</div>
	</div>
</div>
