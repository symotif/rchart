<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { UserFullData, UserEducation, UserBadge } from '$lib/types/user';
	import { EDUCATION_TYPE_NAMES, BADGE_TYPE_INFO } from '$lib/types/user';

	let userData: UserFullData | null = $state(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let isEditing = $state(false);

	// Edit form state
	let editForm = $state({
		first_name: '',
		last_name: '',
		degree_type: '',
		specialty: '',
		subspecialty: '',
		npi_number: '',
		bio: ''
	});

	onMount(async () => {
		await loadUserData();
	});

	async function loadUserData() {
		loading = true;
		error = null;
		try {
			// First try to seed user data if it doesn't exist
			await invoke('db_seed_user_data');
			// Then get the current user
			const data = await invoke<UserFullData | null>('db_get_current_user');
			userData = data;
			if (data) {
				editForm = {
					first_name: data.user.first_name,
					last_name: data.user.last_name,
					degree_type: data.user.degree_type ?? '',
					specialty: data.user.specialty ?? '',
					subspecialty: data.user.subspecialty ?? '',
					npi_number: data.user.npi_number ?? '',
					bio: data.user.bio ?? ''
				};
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	function startEditing() {
		isEditing = true;
	}

	function cancelEditing() {
		if (userData) {
			editForm = {
				first_name: userData.user.first_name,
				last_name: userData.user.last_name,
				degree_type: userData.user.degree_type ?? '',
				specialty: userData.user.specialty ?? '',
				subspecialty: userData.user.subspecialty ?? '',
				npi_number: userData.user.npi_number ?? '',
				bio: userData.user.bio ?? ''
			};
		}
		isEditing = false;
	}

	async function saveProfile() {
		if (!userData) return;

		try {
			const updatedUser = {
				...userData.user,
				first_name: editForm.first_name,
				last_name: editForm.last_name,
				degree_type: editForm.degree_type || null,
				specialty: editForm.specialty || null,
				subspecialty: editForm.subspecialty || null,
				npi_number: editForm.npi_number || null,
				bio: editForm.bio || null
			};

			await invoke('db_update_user', { user: updatedUser });
			await loadUserData();
			isEditing = false;
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		}
	}

	function getEducationIcon(type: string): string {
		switch (type) {
			case 'Medical School':
				return 'fa-user-doctor';
			case 'Residency':
				return 'fa-hospital';
			case 'Fellowship':
				return 'fa-graduation-cap';
			case 'Undergraduate':
				return 'fa-university';
			default:
				return 'fa-book';
		}
	}

	function getBadgeIcon(icon: string | null): string {
		if (!icon) return 'fa-award';
		// Map Heroicon-style names to Font Awesome
		const iconMap: Record<string, string> = {
			'shield-check': 'fa-shield',
			'academic-cap': 'fa-graduation-cap',
			star: 'fa-star',
			beaker: 'fa-flask',
			heart: 'fa-heart'
		};
		return iconMap[icon] ?? `fa-${icon}`;
	}

	function sortEducation(education: UserEducation[]): UserEducation[] {
		const typeOrder = ['Fellowship', 'Residency', 'Medical School', 'Graduate', 'Undergraduate', 'Other'];
		return [...education].sort((a, b) => {
			const aIndex = typeOrder.indexOf(a.education_type);
			const bIndex = typeOrder.indexOf(b.education_type);
			if (aIndex !== bIndex) return aIndex - bIndex;
			return (b.end_year ?? 0) - (a.end_year ?? 0);
		});
	}
</script>

<div class="absolute left-20 top-20 right-10 bottom-5 mx-5 my-8 overflow-y-auto">
	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
		</div>
	{:else if error}
		<div class="bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 rounded-lg p-4">
			<p class="text-red-600 dark:text-red-400">Error: {error}</p>
			<button onclick={loadUserData} class="mt-2 text-sm text-red-600 dark:text-red-400 underline">
				Try again
			</button>
		</div>
	{:else if userData}
		<div class="max-w-4xl space-y-6">
			<!-- Profile Header -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<div class="flex items-start gap-6">
					<!-- Profile Photo -->
					<div class="flex-shrink-0">
						<div
							class="w-32 h-32 rounded-full bg-gradient-to-br from-purple-500 to-blue-500 flex items-center justify-center text-white text-4xl font-bold shadow-lg"
						>
							{userData.user.first_name[0]}{userData.user.last_name[0]}
						</div>
					</div>

					<!-- Basic Info -->
					<div class="flex-1">
						{#if isEditing}
							<div class="space-y-4">
								<div class="grid grid-cols-2 gap-4">
									<div>
										<label
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
										>
											First Name
										</label>
										<input
											type="text"
											bind:value={editForm.first_name}
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
										/>
									</div>
									<div>
										<label
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
										>
											Last Name
										</label>
										<input
											type="text"
											bind:value={editForm.last_name}
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
										/>
									</div>
								</div>
								<div class="grid grid-cols-3 gap-4">
									<div>
										<label
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
										>
											Degree
										</label>
										<input
											type="text"
											bind:value={editForm.degree_type}
											placeholder="MD, DO, etc."
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
										/>
									</div>
									<div>
										<label
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
										>
											Specialty
										</label>
										<input
											type="text"
											bind:value={editForm.specialty}
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
										/>
									</div>
									<div>
										<label
											class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
										>
											Subspecialty
										</label>
										<input
											type="text"
											bind:value={editForm.subspecialty}
											class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
										/>
									</div>
								</div>
								<div>
									<label
										class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
									>
										NPI Number
									</label>
									<input
										type="text"
										bind:value={editForm.npi_number}
										placeholder="10-digit NPI"
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
									/>
								</div>
							</div>
						{:else}
							<div class="flex items-start justify-between">
								<div>
									<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
										{userData.user.first_name}
										{userData.user.last_name}{userData.user.degree_type
											? `, ${userData.user.degree_type}`
											: ''}
									</h1>
									<p class="text-lg text-purple-600 dark:text-purple-400 font-medium">
										{userData.user.specialty ?? 'Specialty not set'}
										{#if userData.user.subspecialty}
											<span class="text-gray-500 dark:text-gray-400">
												&bull; {userData.user.subspecialty}
											</span>
										{/if}
									</p>
									{#if userData.user.npi_number}
										<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
											<i class="fa-solid fa-id-card mr-1"></i>
											NPI: {userData.user.npi_number}
										</p>
									{/if}
								</div>
								<button
									onclick={startEditing}
									class="px-4 py-2 text-sm font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors"
								>
									<i class="fa-solid fa-pen mr-2"></i>Edit Profile
								</button>
							</div>
						{/if}
					</div>
				</div>

				{#if isEditing}
					<div class="mt-4 flex justify-end gap-3">
						<button
							onclick={cancelEditing}
							class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
						>
							Cancel
						</button>
						<button
							onclick={saveProfile}
							class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
						>
							Save Changes
						</button>
					</div>
				{/if}
			</div>

			<!-- Bio Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3">
					<i class="fa-solid fa-user mr-2 text-purple-500"></i>About
				</h2>
				{#if isEditing}
					<textarea
						bind:value={editForm.bio}
						rows={4}
						placeholder="Tell patients about yourself, your approach to care, and interests..."
						class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
					></textarea>
				{:else}
					<p class="text-gray-700 dark:text-gray-300 leading-relaxed">
						{userData.user.bio ?? 'No bio added yet. Click Edit Profile to add one.'}
					</p>
				{/if}
			</div>

			<!-- Badges Section -->
			{#if userData.badges.length > 0}
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
					<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
						<i class="fa-solid fa-award mr-2 text-amber-500"></i>Certifications & Awards
					</h2>
					<div class="grid grid-cols-2 md:grid-cols-3 gap-4">
						{#each userData.badges as badge}
							<div
								class="p-4 rounded-lg border-2 transition-transform hover:scale-105"
								style="border-color: {badge.color ?? '#6b7280'}; background-color: {badge.color ?? '#6b7280'}10"
							>
								<div class="flex items-center gap-3">
									<div
										class="w-10 h-10 rounded-full flex items-center justify-center text-white"
										style="background-color: {badge.color ?? '#6b7280'}"
									>
										<i class="fa-solid {getBadgeIcon(badge.icon)}"></i>
									</div>
									<div class="flex-1 min-w-0">
										<p class="font-semibold text-gray-900 dark:text-gray-100 text-sm truncate">
											{badge.badge_name}
										</p>
										{#if badge.description}
											<p class="text-xs text-gray-500 dark:text-gray-400 truncate">
												{badge.description}
											</p>
										{/if}
									</div>
								</div>
								{#if badge.awarded_date}
									<p class="text-xs text-gray-400 dark:text-gray-500 mt-2">
										{new Date(badge.awarded_date).getFullYear()}
									</p>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Education Section -->
			{#if userData.education.length > 0}
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
					<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
						<i class="fa-solid fa-graduation-cap mr-2 text-blue-500"></i>Education & Training
					</h2>
					<div class="space-y-4">
						{#each sortEducation(userData.education) as edu}
							<div
								class="flex items-start gap-4 p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg"
							>
								<div
									class="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-900/50 flex items-center justify-center text-blue-600 dark:text-blue-400 flex-shrink-0"
								>
									<i class="fa-solid {getEducationIcon(edu.education_type)}"></i>
								</div>
								<div class="flex-1">
									<div class="flex items-start justify-between">
										<div>
											<p class="font-semibold text-gray-900 dark:text-gray-100">
												{edu.education_type}
												{#if edu.degree}
													<span class="text-gray-500 dark:text-gray-400 font-normal">
														- {edu.degree}
													</span>
												{/if}
											</p>
											<p class="text-purple-600 dark:text-purple-400 font-medium">
												{edu.institution}
											</p>
											{#if edu.field_of_study}
												<p class="text-sm text-gray-500 dark:text-gray-400">
													{edu.field_of_study}
												</p>
											{/if}
										</div>
										{#if edu.start_year && edu.end_year}
											<span
												class="text-sm text-gray-500 dark:text-gray-400 flex-shrink-0"
											>
												{edu.start_year} - {edu.end_year}
											</span>
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	{:else}
		<div class="bg-yellow-50 dark:bg-yellow-900/30 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
			<p class="text-yellow-600 dark:text-yellow-400">No user profile found. Please set up your account.</p>
		</div>
	{/if}
</div>
