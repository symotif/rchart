<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { ThemeStore, toggleTheme } from '../../stores/ThemeStore';
	import type { UserFullData, UserSettings } from '$lib/types/user';
	import { LANGUAGE_OPTIONS } from '$lib/types/user';

	let userData: UserFullData | null = $state(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let saveSuccess = $state(false);

	// Password change form
	let showPasswordForm = $state(false);
	let passwordForm = $state({
		currentPassword: '',
		newPassword: '',
		confirmPassword: ''
	});
	let passwordError = $state<string | null>(null);
	let passwordSuccess = $state(false);

	// Username change form
	let showUsernameForm = $state(false);
	let newUsername = $state('');
	let usernameError = $state<string | null>(null);
	let usernameSuccess = $state(false);

	// 2FA setup
	let show2FASetup = $state(false);

	onMount(async () => {
		await loadUserData();
	});

	async function loadUserData() {
		loading = true;
		error = null;
		try {
			await invoke('db_seed_user_data');
			const data = await invoke<UserFullData | null>('db_get_current_user');
			userData = data;
			if (data) {
				newUsername = data.user.username;
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	async function updateSettings(updates: Partial<UserSettings>) {
		if (!userData) return;

		try {
			const updatedSettings: UserSettings = {
				...userData.settings,
				...updates
			};

			await invoke('db_update_user_settings', { settings: updatedSettings });
			userData = {
				...userData,
				settings: updatedSettings
			};
			showSaveSuccess();
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		}
	}

	function showSaveSuccess() {
		saveSuccess = true;
		setTimeout(() => {
			saveSuccess = false;
		}, 2000);
	}

	async function handleLanguageChange(event: Event) {
		const select = event.target as HTMLSelectElement;
		await updateSettings({ language: select.value });
	}

	async function handleNotificationToggle(type: 'notifications_enabled' | 'email_notifications' | 'sms_notifications') {
		if (!userData) return;
		const currentValue = userData.settings[type] ?? false;
		await updateSettings({ [type]: !currentValue });
	}

	async function handleChangePassword() {
		passwordError = null;
		passwordSuccess = false;

		if (!passwordForm.newPassword) {
			passwordError = 'Please enter a new password';
			return;
		}

		if (passwordForm.newPassword.length < 8) {
			passwordError = 'Password must be at least 8 characters';
			return;
		}

		if (passwordForm.newPassword !== passwordForm.confirmPassword) {
			passwordError = 'Passwords do not match';
			return;
		}

		if (!userData) return;

		try {
			// In a real app, you'd hash the password on the backend
			await invoke('db_update_password', {
				userId: userData.user.id,
				newPasswordHash: `hashed_${passwordForm.newPassword}`
			});
			passwordSuccess = true;
			passwordForm = { currentPassword: '', newPassword: '', confirmPassword: '' };
			setTimeout(() => {
				showPasswordForm = false;
				passwordSuccess = false;
			}, 2000);
		} catch (e) {
			passwordError = e instanceof Error ? e.message : String(e);
		}
	}

	async function handleChangeUsername() {
		usernameError = null;
		usernameSuccess = false;

		if (!newUsername.trim()) {
			usernameError = 'Username cannot be empty';
			return;
		}

		if (newUsername.length < 3) {
			usernameError = 'Username must be at least 3 characters';
			return;
		}

		if (!userData) return;

		try {
			const updatedUser = {
				...userData.user,
				username: newUsername.trim()
			};
			await invoke('db_update_user', { user: updatedUser });
			userData = {
				...userData,
				user: updatedUser
			};
			usernameSuccess = true;
			setTimeout(() => {
				showUsernameForm = false;
				usernameSuccess = false;
			}, 2000);
		} catch (e) {
			usernameError = e instanceof Error ? e.message : String(e);
		}
	}

	async function handle2FAToggle() {
		if (!userData) return;
		const currentValue = userData.settings.two_factor_enabled ?? false;

		if (!currentValue) {
			// Enabling 2FA - show setup modal
			show2FASetup = true;
		} else {
			// Disabling 2FA
			await updateSettings({ two_factor_enabled: false });
		}
	}

	async function complete2FASetup() {
		await updateSettings({ two_factor_enabled: true });
		show2FASetup = false;
	}

	function handleLogout() {
		// In a real app, this would clear auth tokens, session, etc.
		// For now, just show a placeholder action
		alert('Logout functionality will clear your session and return to the login screen.');
	}
</script>

<div class="absolute left-20 top-20 right-0 bottom-5 px-5 py-8 overflow-y-auto">
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
	{:else}
		<div class="max-w-2xl space-y-6">
			<!-- Save Success Toast -->
			{#if saveSuccess}
				<div class="fixed top-24 right-10 bg-green-500 text-white px-4 py-2 rounded-lg shadow-lg flex items-center gap-2 animate-fade-in z-50">
					<i class="fa-solid fa-check"></i>
					Settings saved
				</div>
			{/if}

			<h1 class="text-3xl font-bold text-gray-800 dark:text-gray-100">Settings</h1>

			<!-- Appearance Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">
					<i class="fa-solid fa-palette mr-2 text-purple-500"></i>Appearance
				</h2>

				<div class="space-y-4">
					<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
						<div>
							<p class="font-medium text-gray-800 dark:text-gray-100">Dark Mode</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Switch between light and dark themes
							</p>
						</div>
						<button
							onclick={toggleTheme}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
								{$ThemeStore === 'dark' ? 'bg-blue-600' : 'bg-gray-300'}"
							role="switch"
							aria-checked={$ThemeStore === 'dark'}
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
									{$ThemeStore === 'dark' ? 'translate-x-6' : 'translate-x-1'}"
							></span>
						</button>
					</div>

					<!-- Zen Mode Default -->
					<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
						<div>
							<p class="font-medium text-gray-800 dark:text-gray-100">
								<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 inline mr-1 text-pink-500" viewBox="0 0 24 24" fill="currentColor">
									<path d="M12 2C12 2 9.5 5.5 9.5 8.5C9.5 10.5 10.5 12 12 12C13.5 12 14.5 10.5 14.5 8.5C14.5 5.5 12 2 12 2Z"/>
									<path d="M7 8C7 8 4.5 10 4.5 12.5C4.5 14.5 6 16 7.5 16C8.5 16 9.5 15.5 10 14.5" opacity="0.8"/>
									<path d="M17 8C17 8 19.5 10 19.5 12.5C19.5 14.5 18 16 16.5 16C15.5 16 14.5 15.5 14 14.5" opacity="0.8"/>
									<path d="M12 12V22" stroke="currentColor" stroke-width="2" fill="none"/>
								</svg>
								Zen Mode by Default
							</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Start all new notes in distraction-free Zen mode
							</p>
						</div>
						<button
							onclick={() => updateSettings({ zen_mode_default: !(userData?.settings.zen_mode_default ?? false) })}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-pink-500 focus:ring-offset-2
								{userData?.settings.zen_mode_default ? 'bg-pink-500' : 'bg-gray-300'}"
							role="switch"
							aria-checked={userData?.settings.zen_mode_default ?? false}
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
									{userData?.settings.zen_mode_default ? 'translate-x-6' : 'translate-x-1'}"
							></span>
						</button>
					</div>
				</div>
			</div>

			<!-- Language Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">
					<i class="fa-solid fa-globe mr-2 text-blue-500"></i>Language
				</h2>

				<div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
					<label for="language-select" class="block font-medium text-gray-800 dark:text-gray-100 mb-2">
						Display Language
					</label>
					<select
						id="language-select"
						onchange={handleLanguageChange}
						value={userData?.settings.language ?? 'en'}
						class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					>
						{#each LANGUAGE_OPTIONS as lang}
							<option value={lang.code}>{lang.name}</option>
						{/each}
					</select>
					<p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
						Changes the language of the interface
					</p>
				</div>
			</div>

			<!-- Account Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">
					<i class="fa-solid fa-user-gear mr-2 text-green-500"></i>Account
				</h2>

				<div class="space-y-4">
					<!-- Username -->
					<div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
						<div class="flex items-center justify-between">
							<div>
								<p class="font-medium text-gray-800 dark:text-gray-100">Username</p>
								<p class="text-sm text-gray-500 dark:text-gray-400">
									{userData?.user.username ?? 'Not set'}
								</p>
							</div>
							<button
								onclick={() => (showUsernameForm = !showUsernameForm)}
								class="px-4 py-2 text-sm font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors"
							>
								Change
							</button>
						</div>

						{#if showUsernameForm}
							<div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-600 space-y-3">
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
										New Username
									</label>
									<input
										type="text"
										bind:value={newUsername}
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
									/>
								</div>
								{#if usernameError}
									<p class="text-sm text-red-600 dark:text-red-400">{usernameError}</p>
								{/if}
								{#if usernameSuccess}
									<p class="text-sm text-green-600 dark:text-green-400">Username updated successfully!</p>
								{/if}
								<div class="flex gap-2">
									<button
										onclick={handleChangeUsername}
										class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
									>
										Save Username
									</button>
									<button
										onclick={() => {
											showUsernameForm = false;
											newUsername = userData?.user.username ?? '';
										}}
										class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-lg transition-colors"
									>
										Cancel
									</button>
								</div>
							</div>
						{/if}
					</div>

					<!-- Password -->
					<div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
						<div class="flex items-center justify-between">
							<div>
								<p class="font-medium text-gray-800 dark:text-gray-100">Password</p>
								<p class="text-sm text-gray-500 dark:text-gray-400">
									Last changed: Unknown
								</p>
							</div>
							<button
								onclick={() => (showPasswordForm = !showPasswordForm)}
								class="px-4 py-2 text-sm font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors"
							>
								Change Password
							</button>
						</div>

						{#if showPasswordForm}
							<div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-600 space-y-3">
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
										Current Password
									</label>
									<input
										type="password"
										bind:value={passwordForm.currentPassword}
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
									/>
								</div>
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
										New Password
									</label>
									<input
										type="password"
										bind:value={passwordForm.newPassword}
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
									/>
								</div>
								<div>
									<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
										Confirm New Password
									</label>
									<input
										type="password"
										bind:value={passwordForm.confirmPassword}
										class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
									/>
								</div>
								{#if passwordError}
									<p class="text-sm text-red-600 dark:text-red-400">{passwordError}</p>
								{/if}
								{#if passwordSuccess}
									<p class="text-sm text-green-600 dark:text-green-400">Password changed successfully!</p>
								{/if}
								<div class="flex gap-2">
									<button
										onclick={handleChangePassword}
										class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
									>
										Update Password
									</button>
									<button
										onclick={() => {
											showPasswordForm = false;
											passwordForm = { currentPassword: '', newPassword: '', confirmPassword: '' };
											passwordError = null;
										}}
										class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-600 rounded-lg transition-colors"
									>
										Cancel
									</button>
								</div>
							</div>
						{/if}
					</div>
				</div>
			</div>

			<!-- Notifications Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">
					<i class="fa-solid fa-bell mr-2 text-amber-500"></i>Notifications
				</h2>

				<div class="space-y-4">
					<!-- Master notifications toggle -->
					<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
						<div>
							<p class="font-medium text-gray-800 dark:text-gray-100">Enable Notifications</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Receive alerts and updates
							</p>
						</div>
						<button
							onclick={() => handleNotificationToggle('notifications_enabled')}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
								{userData?.settings.notifications_enabled ? 'bg-blue-600' : 'bg-gray-300'}"
							role="switch"
							aria-checked={userData?.settings.notifications_enabled ?? false}
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
									{userData?.settings.notifications_enabled ? 'translate-x-6' : 'translate-x-1'}"
							></span>
						</button>
					</div>

					<!-- Email notifications -->
					<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg {!(userData?.settings.notifications_enabled) ? 'opacity-50' : ''}">
						<div>
							<p class="font-medium text-gray-800 dark:text-gray-100">Email Notifications</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Receive notifications via email
							</p>
						</div>
						<button
							onclick={() => handleNotificationToggle('email_notifications')}
							disabled={!(userData?.settings.notifications_enabled)}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed
								{userData?.settings.email_notifications ? 'bg-blue-600' : 'bg-gray-300'}"
							role="switch"
							aria-checked={userData?.settings.email_notifications ?? false}
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
									{userData?.settings.email_notifications ? 'translate-x-6' : 'translate-x-1'}"
							></span>
						</button>
					</div>

					<!-- SMS notifications -->
					<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg {!(userData?.settings.notifications_enabled) ? 'opacity-50' : ''}">
						<div>
							<p class="font-medium text-gray-800 dark:text-gray-100">SMS Notifications</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Receive notifications via text message
							</p>
						</div>
						<button
							onclick={() => handleNotificationToggle('sms_notifications')}
							disabled={!(userData?.settings.notifications_enabled)}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed
								{userData?.settings.sms_notifications ? 'bg-blue-600' : 'bg-gray-300'}"
							role="switch"
							aria-checked={userData?.settings.sms_notifications ?? false}
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
									{userData?.settings.sms_notifications ? 'translate-x-6' : 'translate-x-1'}"
							></span>
						</button>
					</div>
				</div>
			</div>

			<!-- Security Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">
					<i class="fa-solid fa-shield-halved mr-2 text-green-500"></i>Security
				</h2>

				<div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
					<div class="flex items-center justify-between">
						<div>
							<p class="font-medium text-gray-800 dark:text-gray-100">Two-Factor Authentication</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								{userData?.settings.two_factor_enabled
									? 'Enabled - Your account is protected with 2FA'
									: 'Add an extra layer of security to your account'}
							</p>
						</div>
						<button
							onclick={handle2FAToggle}
							class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
								{userData?.settings.two_factor_enabled ? 'bg-green-600' : 'bg-gray-300'}"
							role="switch"
							aria-checked={userData?.settings.two_factor_enabled ?? false}
						>
							<span
								class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
									{userData?.settings.two_factor_enabled ? 'translate-x-6' : 'translate-x-1'}"
							></span>
						</button>
					</div>

					{#if userData?.settings.two_factor_enabled}
						<div class="mt-3 flex items-center gap-2 text-sm text-green-600 dark:text-green-400">
							<i class="fa-solid fa-check-circle"></i>
							<span>Two-factor authentication is active</span>
						</div>
					{/if}
				</div>
			</div>

			<!-- Logout Section -->
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">
					<i class="fa-solid fa-right-from-bracket mr-2 text-red-500"></i>Session
				</h2>

				<div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
					<div class="flex items-center justify-between">
						<div>
							<p class="font-medium text-gray-800 dark:text-gray-100">Log Out</p>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Sign out of your account on this device
							</p>
						</div>
						<button
							onclick={handleLogout}
							class="px-4 py-2 text-sm font-medium text-white bg-red-600 hover:bg-red-700 rounded-lg transition-colors"
						>
							Log Out
						</button>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<!-- 2FA Setup Modal -->
{#if show2FASetup}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
		<div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
			<h3 class="text-xl font-bold text-gray-900 dark:text-gray-100 mb-4">
				Set Up Two-Factor Authentication
			</h3>

			<div class="space-y-4">
				<p class="text-gray-600 dark:text-gray-400">
					Scan the QR code below with your authenticator app (Google Authenticator, Authy, etc.)
				</p>

				<!-- Placeholder QR code -->
				<div class="flex justify-center">
					<div class="w-48 h-48 bg-gray-200 dark:bg-gray-700 rounded-lg flex items-center justify-center">
						<div class="text-center text-gray-500 dark:text-gray-400">
							<i class="fa-solid fa-qrcode text-6xl mb-2"></i>
							<p class="text-sm">QR Code Placeholder</p>
						</div>
					</div>
				</div>

				<p class="text-sm text-gray-500 dark:text-gray-400 text-center">
					Or enter this code manually: <code class="bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded">XXXX-XXXX-XXXX-XXXX</code>
				</p>

				<div>
					<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
						Enter verification code
					</label>
					<input
						type="text"
						placeholder="000000"
						maxlength="6"
						class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent text-center text-2xl tracking-widest"
					/>
				</div>

				<div class="flex gap-3 pt-2">
					<button
						onclick={() => (show2FASetup = false)}
						class="flex-1 px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
					>
						Cancel
					</button>
					<button
						onclick={complete2FASetup}
						class="flex-1 px-4 py-2 text-sm font-medium text-white bg-green-600 hover:bg-green-700 rounded-lg transition-colors"
					>
						Enable 2FA
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.animate-fade-in {
		animation: fade-in 0.3s ease-out;
	}
</style>
