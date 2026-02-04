<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/tauri';
	import { addTab } from '../../../stores/TabStore';
	import type { Patient } from '$lib/types/patient';

	let saving = $state(false);
	let error = $state<string | null>(null);

	// Form state
	let formData = $state<Partial<Patient>>({
		first_name: '',
		last_name: '',
		dob: '',
		sex: '',
		gender: null,
		address: null,
		phone: null,
		email: null,
		photo_url: null,
		ai_summary: null,
		preferred_pharmacy: null,
		insurance_provider: null,
		insurance_policy_number: null,
		insurance_group_number: null
	});

	// Validation
	let formErrors = $state<Record<string, string>>({});

	function validateForm(): boolean {
		const errors: Record<string, string> = {};

		if (!formData.first_name?.trim()) {
			errors.first_name = 'First name is required';
		}

		if (!formData.last_name?.trim()) {
			errors.last_name = 'Last name is required';
		}

		if (!formData.dob) {
			errors.dob = 'Date of birth is required';
		} else {
			const dobDate = new Date(formData.dob);
			const today = new Date();
			if (dobDate > today) {
				errors.dob = 'Date of birth cannot be in the future';
			}
		}

		if (!formData.sex) {
			errors.sex = 'Sex is required';
		}

		// Validate email format if provided
		if (formData.email && formData.email.trim()) {
			const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
			if (!emailRegex.test(formData.email)) {
				errors.email = 'Invalid email format';
			}
		}

		// Validate phone format if provided
		if (formData.phone && formData.phone.trim()) {
			const phoneRegex = /^[\d\s\-\(\)\+]+$/;
			if (!phoneRegex.test(formData.phone)) {
				errors.phone = 'Invalid phone format';
			}
		}

		formErrors = errors;
		return Object.keys(errors).length === 0;
	}

	async function handleSubmit() {
		if (!validateForm()) return;

		saving = true;
		error = null;

		try {
			const patient: Patient = {
				id: null,
				first_name: formData.first_name!.trim(),
				last_name: formData.last_name!.trim(),
				dob: formData.dob!,
				sex: formData.sex!,
				gender: formData.gender?.trim() || null,
				address: formData.address?.trim() || null,
				phone: formData.phone?.trim() || null,
				email: formData.email?.trim() || null,
				photo_url: null,
				ai_summary: null,
				preferred_pharmacy: formData.preferred_pharmacy?.trim() || null,
				insurance_provider: formData.insurance_provider?.trim() || null,
				insurance_policy_number: formData.insurance_policy_number?.trim() || null,
				insurance_group_number: formData.insurance_group_number?.trim() || null
			};

			const newPatientId = await invoke<number>('db_create_patient', { patient });

			// Navigate to the new patient's page
			const tab = {
				id: `patient-${newPatientId}`,
				title: `${patient.first_name} ${patient.last_name}`,
				path: `/patient/${newPatientId}`
			};
			addTab(tab);
			goto(tab.path);
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			saving = false;
		}
	}

	function handleCancel() {
		goto('/list');
	}

	function formatPhoneNumber(value: string): string {
		// Remove non-digits
		const digits = value.replace(/\D/g, '');
		// Format as (XXX) XXX-XXXX
		if (digits.length >= 10) {
			return `(${digits.slice(0, 3)}) ${digits.slice(3, 6)}-${digits.slice(6, 10)}`;
		}
		return value;
	}

	function handlePhoneInput(event: Event) {
		const input = event.target as HTMLInputElement;
		const formatted = formatPhoneNumber(input.value);
		formData.phone = formatted;
	}
</script>

<div class="absolute left-20 top-24 right-0 bottom-5 px-5 py-8 overflow-y-auto">
	<div class="max-w-3xl mx-auto">
		<!-- Header -->
		<div class="mb-6">
			<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">New Patient</h1>
			<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
				Enter the patient's information to create a new record.
			</p>
		</div>

		{#if error}
			<div class="mb-6 bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 rounded-lg p-4">
				<p class="text-red-600 dark:text-red-400">
					<i class="fa-solid fa-exclamation-circle mr-2"></i>
					Error: {error}
				</p>
			</div>
		{/if}

		<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
			<!-- Basic Information -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-6">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
					<i class="fa-solid fa-user mr-2 text-blue-500"></i>
					Basic Information
				</h2>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<!-- First Name -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							First Name <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={formData.first_name}
							placeholder="Enter first name"
							class="w-full px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent
								{formErrors.first_name ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
						/>
						{#if formErrors.first_name}
							<p class="mt-1 text-xs text-red-500">{formErrors.first_name}</p>
						{/if}
					</div>

					<!-- Last Name -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Last Name <span class="text-red-500">*</span>
						</label>
						<input
							type="text"
							bind:value={formData.last_name}
							placeholder="Enter last name"
							class="w-full px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent
								{formErrors.last_name ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
						/>
						{#if formErrors.last_name}
							<p class="mt-1 text-xs text-red-500">{formErrors.last_name}</p>
						{/if}
					</div>

					<!-- Date of Birth -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Date of Birth <span class="text-red-500">*</span>
						</label>
						<input
							type="date"
							bind:value={formData.dob}
							class="w-full px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent
								{formErrors.dob ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
						/>
						{#if formErrors.dob}
							<p class="mt-1 text-xs text-red-500">{formErrors.dob}</p>
						{/if}
					</div>

					<!-- Sex -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Sex at Birth <span class="text-red-500">*</span>
						</label>
						<select
							bind:value={formData.sex}
							class="w-full px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent
								{formErrors.sex ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
						>
							<option value="">Select sex</option>
							<option value="M">Male</option>
							<option value="F">Female</option>
							<option value="I">Intersex</option>
							<option value="U">Unknown</option>
						</select>
						{#if formErrors.sex}
							<p class="mt-1 text-xs text-red-500">{formErrors.sex}</p>
						{/if}
					</div>

					<!-- Gender -->
					<div class="md:col-span-2">
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Gender Identity (Optional)
						</label>
						<input
							type="text"
							bind:value={formData.gender}
							placeholder="Enter gender identity if different from sex"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>
				</div>
			</div>

			<!-- Contact Information -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-6">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
					<i class="fa-solid fa-address-book mr-2 text-green-500"></i>
					Contact Information
				</h2>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<!-- Phone -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Phone Number
						</label>
						<input
							type="tel"
							value={formData.phone || ''}
							oninput={handlePhoneInput}
							placeholder="(555) 123-4567"
							class="w-full px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent
								{formErrors.phone ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
						/>
						{#if formErrors.phone}
							<p class="mt-1 text-xs text-red-500">{formErrors.phone}</p>
						{/if}
					</div>

					<!-- Email -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Email Address
						</label>
						<input
							type="email"
							bind:value={formData.email}
							placeholder="patient@example.com"
							class="w-full px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent
								{formErrors.email ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}"
						/>
						{#if formErrors.email}
							<p class="mt-1 text-xs text-red-500">{formErrors.email}</p>
						{/if}
					</div>

					<!-- Address -->
					<div class="md:col-span-2">
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Address
						</label>
						<textarea
							bind:value={formData.address}
							rows={2}
							placeholder="Enter street address, city, state, ZIP"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
						></textarea>
					</div>
				</div>
			</div>

			<!-- Insurance Information -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-6">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
					<i class="fa-solid fa-id-card mr-2 text-purple-500"></i>
					Insurance Information
				</h2>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<!-- Insurance Provider -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Insurance Provider
						</label>
						<input
							type="text"
							bind:value={formData.insurance_provider}
							placeholder="e.g., Blue Cross Blue Shield"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>

					<!-- Policy Number -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Policy Number
						</label>
						<input
							type="text"
							bind:value={formData.insurance_policy_number}
							placeholder="Enter policy number"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>

					<!-- Group Number -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Group Number
						</label>
						<input
							type="text"
							bind:value={formData.insurance_group_number}
							placeholder="Enter group number"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>

					<!-- Preferred Pharmacy -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Preferred Pharmacy
						</label>
						<input
							type="text"
							bind:value={formData.preferred_pharmacy}
							placeholder="e.g., CVS Pharmacy - Main St"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>
				</div>
			</div>

			<!-- Form Actions -->
			<div class="flex justify-end gap-3">
				<button
					type="button"
					onclick={handleCancel}
					disabled={saving}
					class="px-6 py-2.5 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors disabled:opacity-50"
				>
					Cancel
				</button>
				<button
					type="submit"
					disabled={saving}
					class="px-6 py-2.5 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors disabled:opacity-50 flex items-center gap-2"
				>
					{#if saving}
						<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
						Creating...
					{:else}
						<i class="fa-solid fa-plus"></i>
						Create Patient
					{/if}
				</button>
			</div>
		</form>
	</div>
</div>
