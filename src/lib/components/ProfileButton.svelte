<script lang="ts">
	import { goto } from '$app/navigation';
	import { setTab } from '../../stores/SideBarStore';
	import { addTab } from '../../stores/TabStore';
	import { institution, isOnline } from '../../stores/SyncStatusStore';

	// State for institution switcher modal
	let showInstitutionSwitcher = $state(false);

	function openProfile() {
		const tab = {
			id: 'profile',
			title: 'Profile',
			path: '/profile'
		};
		addTab(tab);
		setTab(-1);
		goto('/profile');
	}

	function openInstitutionSwitcher() {
		showInstitutionSwitcher = true;
		// In the future, this will open a modal to switch institutions/departments
		console.log('Opening institution switcher');
	}

	function closeInstitutionSwitcher() {
		showInstitutionSwitcher = false;
	}
</script>

<div class="group mb-4">
	<div
		class="relative flex items-center justify-center mt-3 transform transition-all duration-200 ease-in-out group-hover:scale-110"
	>
		<!-- profile button -->
		<button onclick={openProfile} class="relative">
			<img
				class="w-16 h-16 rounded-full border-2 border-blue-400"
				src="./src/lib/img/doctor_headshot.jpg"
				alt="profile-pic"
				draggable="false"
			/>
			<!-- Online indicator -->
			<span
				class={`w-4 h-4 rounded-full absolute bottom-0 left-0 border-2 border-white dark:border-gray-800 ${
					$isOnline ? 'bg-green-500' : 'bg-gray-400'
				}`}
			></span>
		</button>

		<!-- institution badge - separate button -->
		<button
			onclick={openInstitutionSwitcher}
			class="absolute bottom-0 right-0 transform transition-all duration-150 hover:scale-110 active:scale-95"
			title={$institution ? `${$institution.name}${$institution.department ? ` - ${$institution.department}` : ''}\nClick to switch` : 'Select institution'}
		>
			{#if $institution?.logoUrl}
				<img
					class="w-8 h-8 rounded-full border-2 border-white dark:border-gray-800 shadow-md"
					src={$institution.logoUrl}
					alt={$institution.name}
					draggable="false"
				/>
			{:else}
				<div class="w-8 h-8 rounded-full border-2 border-white dark:border-gray-800 shadow-md bg-gray-200 dark:bg-gray-600 flex items-center justify-center">
					<i class="fa-solid fa-building text-gray-500 dark:text-gray-300 text-xs"></i>
				</div>
			{/if}
			<!-- Small indicator showing it's clickable -->
			<span class="absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full bg-blue-500 border border-white dark:border-gray-800 flex items-center justify-center">
				<i class="fa-solid fa-exchange-alt text-white text-[6px]"></i>
			</span>
		</button>
	</div>

	<!-- tooltip for profile -->
	<span
		class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
            shadow-md text-white bg-gray-600 text-xs font-bold
            transition-all duration-100 scale-0 origin-left group-hover:scale-100"
	>
		profile
	</span>
</div>

<!-- Institution Switcher Modal -->
{#if showInstitutionSwitcher}
	<div class="fixed inset-0 bg-black/50 z-[200] flex items-center justify-center" onclick={closeInstitutionSwitcher}>
		<div
			class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl p-6 max-w-md w-full mx-4"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="flex items-center justify-between mb-4">
				<h2 class="text-lg font-bold text-gray-900 dark:text-white">Switch Institution</h2>
				<button
					onclick={closeInstitutionSwitcher}
					class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
				>
					<i class="fa-solid fa-times"></i>
				</button>
			</div>

			<p class="text-gray-500 dark:text-gray-400 text-sm mb-4">
				Select an institution or department to connect to.
			</p>

			<!-- Current institution -->
			{#if $institution}
				<div class="mb-4 p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-200 dark:border-blue-800">
					<div class="flex items-center gap-3">
						{#if $institution.logoUrl}
							<img src={$institution.logoUrl} alt="" class="w-10 h-10 rounded-full" />
						{:else}
							<div class="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-800 flex items-center justify-center">
								<i class="fa-solid fa-building text-blue-500 dark:text-blue-300"></i>
							</div>
						{/if}
						<div>
							<p class="font-medium text-gray-900 dark:text-white">{$institution.name}</p>
							{#if $institution.department}
								<p class="text-sm text-gray-500 dark:text-gray-400">{$institution.department}</p>
							{/if}
						</div>
						<span class="ml-auto text-xs bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300 px-2 py-1 rounded">
							Current
						</span>
					</div>
				</div>
			{/if}

			<!-- Placeholder for institution list - will be populated when connected to central DB -->
			<div class="space-y-2">
				<p class="text-center text-gray-400 dark:text-gray-500 py-8">
					<i class="fa-solid fa-plug mb-2 text-2xl"></i>
					<br />
					Connect to central database to view available institutions
				</p>
			</div>

			<div class="mt-6 flex justify-end">
				<button
					onclick={closeInstitutionSwitcher}
					class="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
				>
					Close
				</button>
			</div>
		</div>
	</div>
{/if}
