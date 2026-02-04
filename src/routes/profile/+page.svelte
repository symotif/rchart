<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { UserFullData, UserEducation, UserBadge, UserPresenceStatus, StatusDuration } from '$lib/types/user';
	import { EDUCATION_TYPE_NAMES, BADGE_TYPE_INFO, USER_STATUS_CONFIG, STATUS_DURATION_OPTIONS } from '$lib/types/user';
	import {
		currentStatus,
		statusMessage,
		statusExpiresAt,
		setUserStatus,
		getTimeRemaining,
		initUserStatus
	} from '../../stores/UserStatusStore';

	let userData: UserFullData | null = $state(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let isEditing = $state(false);

	// Status editing state
	let showStatusPicker = $state(false);
	let selectedStatus = $state<UserPresenceStatus>('online');
	let selectedDuration = $state<StatusDuration>(null);
	let customStatusMessage = $state('');
	let timeRemainingText = $state<string | null>(null);

	// Update time remaining every second
	let timeRemainingInterval: ReturnType<typeof setInterval> | null = null;

	function updateTimeRemaining() {
		const remaining = getTimeRemaining();
		if (remaining) {
			if (remaining.minutes > 0) {
				timeRemainingText = `${remaining.minutes}m ${remaining.seconds}s remaining`;
			} else {
				timeRemainingText = `${remaining.seconds}s remaining`;
			}
		} else {
			timeRemainingText = null;
		}
	}

	function openStatusPicker() {
		selectedStatus = $currentStatus;
		customStatusMessage = $statusMessage ?? '';
		selectedDuration = null;
		showStatusPicker = true;
	}

	function closeStatusPicker() {
		showStatusPicker = false;
	}

	function applyStatus() {
		setUserStatus(selectedStatus, {
			message: customStatusMessage || null,
			durationMinutes: selectedDuration
		});
		showStatusPicker = false;
	}

	function quickSetStatus(status: UserPresenceStatus) {
		setUserStatus(status);
	}

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
		initUserStatus();
		await loadUserData();

		// Update time remaining every second
		timeRemainingInterval = setInterval(updateTimeRemaining, 1000);
		updateTimeRemaining();
	});

	onDestroy(() => {
		if (timeRemainingInterval) {
			clearInterval(timeRemainingInterval);
		}
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

<div class="absolute left-20 top-24 right-0 bottom-5 px-5 py-8 overflow-y-auto">
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

			<!-- Online Status Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
					<i class="fa-solid fa-signal mr-2 text-green-500"></i>Online Status
				</h2>

				<!-- Current Status Display -->
				<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg mb-4">
					<div class="flex items-center gap-3">
						<span class={`w-4 h-4 rounded-full ${USER_STATUS_CONFIG[$currentStatus].bgColor}`}></span>
						<div>
							<p class="font-medium text-gray-900 dark:text-gray-100">
								{USER_STATUS_CONFIG[$currentStatus].label}
							</p>
							{#if $statusMessage}
								<p class="text-sm text-gray-500 dark:text-gray-400 italic">
									"{$statusMessage}"
								</p>
							{/if}
							{#if timeRemainingText}
								<p class="text-xs text-gray-400 dark:text-gray-500">
									<i class="fa-solid fa-clock mr-1"></i>{timeRemainingText}
								</p>
							{/if}
						</div>
					</div>
					<button
						onclick={openStatusPicker}
						class="px-4 py-2 text-sm font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors"
					>
						<i class="fa-solid fa-pen mr-2"></i>Change
					</button>
				</div>

				<!-- Quick Status Buttons -->
				<div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-2">
					{#each Object.entries(USER_STATUS_CONFIG) as [status, config]}
						<button
							onclick={() => quickSetStatus(status as UserPresenceStatus)}
							class={`p-3 rounded-lg border-2 transition-all flex flex-col items-center gap-1 ${
								$currentStatus === status
									? `border-current ${config.color} bg-opacity-10`
									: 'border-gray-200 dark:border-gray-600 hover:border-gray-300 dark:hover:border-gray-500'
							}`}
							style={$currentStatus === status ? `background-color: ${config.bgColor.replace('bg-', '')}20` : ''}
						>
							<span class={`w-3 h-3 rounded-full ${config.bgColor}`}></span>
							<span class="text-xs font-medium text-gray-700 dark:text-gray-300">{config.label}</span>
						</button>
					{/each}
				</div>

				<p class="text-xs text-gray-400 dark:text-gray-500 mt-3">
					<i class="fa-solid fa-info-circle mr-1"></i>
					Your status is visible to other team members when they view the chat roster
				</p>
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

<!-- Status Picker Modal -->
{#if showStatusPicker}
	<div class="fixed inset-0 bg-black/50 z-[200] flex items-center justify-center" onclick={closeStatusPicker}>
		<div
			class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl p-6 max-w-md w-full mx-4"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="flex items-center justify-between mb-4">
				<h2 class="text-lg font-bold text-gray-900 dark:text-white">Set Your Status</h2>
				<button
					onclick={closeStatusPicker}
					class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
				>
					<i class="fa-solid fa-times"></i>
				</button>
			</div>

			<!-- Status Options -->
			<div class="space-y-2 mb-4">
				{#each Object.entries(USER_STATUS_CONFIG) as [status, config]}
					<button
						onclick={() => (selectedStatus = status as UserPresenceStatus)}
						class={`w-full p-3 rounded-lg border-2 transition-all flex items-center gap-3 text-left ${
							selectedStatus === status
								? `border-blue-500 bg-blue-50 dark:bg-blue-900/30`
								: 'border-gray-200 dark:border-gray-600 hover:border-gray-300 dark:hover:border-gray-500'
						}`}
					>
						<span class={`w-4 h-4 rounded-full ${config.bgColor}`}></span>
						<div class="flex-1">
							<p class="font-medium text-gray-900 dark:text-gray-100">{config.label}</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">{config.description}</p>
						</div>
						{#if selectedStatus === status}
							<i class="fa-solid fa-check text-blue-500"></i>
						{/if}
					</button>
				{/each}
			</div>

			<!-- Status Message -->
			<div class="mb-4">
				<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
					Status Message (optional)
				</label>
				<input
					type="text"
					bind:value={customStatusMessage}
					placeholder="e.g., In a meeting until 3pm"
					maxlength="100"
					class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
				/>
				<p class="text-xs text-gray-400 mt-1">{customStatusMessage.length}/100</p>
			</div>

			<!-- Duration Selector -->
			<div class="mb-6">
				<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
					Clear status after
				</label>
				<div class="grid grid-cols-2 gap-2">
					{#each STATUS_DURATION_OPTIONS as option}
						<button
							onclick={() => (selectedDuration = option.value)}
							class={`px-3 py-2 text-sm rounded-lg border transition-all ${
								selectedDuration === option.value
									? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300'
									: 'border-gray-200 dark:border-gray-600 hover:border-gray-300 dark:hover:border-gray-500 text-gray-700 dark:text-gray-300'
							}`}
						>
							{option.label}
						</button>
					{/each}
				</div>
			</div>

			<!-- Action Buttons -->
			<div class="flex gap-3">
				<button
					onclick={closeStatusPicker}
					class="flex-1 px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={applyStatus}
					class="flex-1 px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
				>
					Set Status
				</button>
			</div>
		</div>
	</div>
{/if}
