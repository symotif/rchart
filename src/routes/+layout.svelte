<script lang="ts">
	import '../app.css';
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';

	// components
	import TabList from '../lib/components/TabList.svelte';
	import MessageCenter from '../lib/components/MessageCenter.svelte';
	import SidebarTab from '../lib/components/SidebarTab.svelte';
	import ProfileButton from '$lib/components/ProfileButton.svelte';
	import StatusBar from '$lib/components/StatusBar.svelte';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import ToastContainer from '$lib/components/ToastContainer.svelte';
	import StartupLoader from '$lib/components/StartupLoader.svelte';

	// stores
	import { SideBarStore, setTab } from '../stores/SideBarStore';
	import { ThemeStore } from '../stores/ThemeStore';
	import { TabStore } from '../stores/TabStore';
	import { SearchStore } from '../stores/SearchStore';
	import { AppDataStore } from '../stores/AppDataStore';

	// App version
	const appVersion = '0.0.1';

	// Check if there are no tabs
	let hasTabs = $derived($TabStore.length > 0);

	// Check if app data is loaded
	let appDataLoaded = $derived($AppDataStore.isLoaded);

	// Apply theme class on mount and when theme changes
	onMount(() => {
		const unsubscribe = ThemeStore.subscribe((theme) => {
			if (theme === 'dark') {
				document.documentElement.classList.add('dark');
			} else {
				document.documentElement.classList.remove('dark');
			}
		});
		return unsubscribe;
	});

	let { children }: { children: Snippet } = $props();

	const sideBarTabInfo = [
		{ label: 'Dashboard', icon: 'fa-calendar-days', path: '/' },
		{ label: 'Patient List', icon: 'fa-list', path: '/list' },
		{ label: 'Team', icon: 'fa-people-group', path: '/team' },
		{ label: 'Resources', icon: 'fa-book-medical', path: '/resources' },
		{ label: 'Extensions', icon: 'fa-puzzle-piece', path: '/extensions' }
	];
</script>

<div class="overscroll-none bg-gray-300 dark:bg-gray-900 app-container">
	<!-- top bar -->
	<section class="flex flex-col absolute top-0 left-20 h-30 w-full pt-6 px-5 bg-white dark:bg-gray-800 z-40">
		<!-- the top part of the bar -->
		<div class="flex flex-row">
			<!-- search box (opens command palette) -->
			<button
				onclick={() => SearchStore.open('global')}
				class="w-full max-w-md"
			>
				<div class="relative flex items-center text-gray-400 focus-within:text-gray-500 dark:text-gray-300 dark:focus-within:text-gray-200">
					<i class="fa-solid fa-magnifying-glass w-5 h-5 absolute ml-3 pointer-events-none"></i>
					<div
						class="w-full pr-3 pl-10 py-2 font-semibold bg-gray-100 dark:bg-gray-700 placeholder-gray-500 dark:placeholder-gray-400 text-gray-500 dark:text-gray-400
								rounded-2xl border-none ring-2 ring-gray-300 dark:ring-gray-600 hover:ring-gray-400 dark:hover:ring-gray-500
								text-left cursor-pointer transition-all flex items-center justify-between"
					>
						<span>Search...</span>
						<kbd class="hidden sm:inline-flex items-center gap-1 px-2 py-0.5 text-xs bg-gray-200 dark:bg-gray-600 rounded text-gray-500 dark:text-gray-400">
							<span class="text-base">⌘</span>K
						</kbd>
					</div>
				</div>
			</button>

			<!-- three top buttons -->
			<div
				class="fixed right-0 mr-10 bg-gray-300 dark:bg-gray-700 px-3 pt-3 pb-2 rounded-xl flex flex-row gap-3"
			>
				<!-- notifications -->
				<button
					onclick={() => {
						setTab(-1);
					}}><i class="fa-solid fa-bell h-6 w-6 text-gray-500 dark:text-gray-300"></i></button
				>

				<!-- statistics -->
				<button
					onclick={() => {
						setTab(-1);
					}}
					><a href="/stats"
						><i class="fa-solid fa-chart-simple h-6 w-6 text-gray-500 dark:text-gray-300"></i></a
					></button
				>

				<!-- settings -->
				<button
					onclick={() => {
						setTab(-1);
					}}
					><a href="/options"><i class="fa-solid fa-gear h-6 w-6 text-gray-500 dark:text-gray-300"></i></a
					></button
				>
			</div>
		</div>

		<!-- the bottom part of the bar is just tabs -->
		<TabList />
	</section>

	<!-- side bar -->
	<section class="fixed top-0 left-0 h-screen w-20 m-0 p-0 flex flex-col gap-0 bg-white dark:bg-gray-800 z-50">
		<!-- profile button-->
		<ProfileButton />

		<!-- sidebar tabs -->
		{#each sideBarTabInfo as { label, icon, path }, index}
			<button
				onclick={() => {
					setTab(index);
				}}
			>
				<SidebarTab {label} {icon} {path} isActive={index == $SideBarStore} />
			</button>
		{/each}
	</section>

	<!-- page contents -->
	<main>
		{#if hasTabs}
			{@render children()}
		{:else}
			<!-- No tabs - show version screen -->
			<div class="absolute inset-0 left-20 top-24 flex items-center justify-center">
				<p class="text-gray-400 dark:text-gray-500 text-lg font-light">rchart version {appVersion}</p>
			</div>
		{/if}
	</main>

	<!-- message center -->
	<MessageCenter />

	<!-- status bar -->
	<StatusBar />

	<!-- command palette (global search) -->
	<CommandPalette />

	<!-- toast notifications -->
	<ToastContainer />

	<!-- startup loader overlay -->
	<StartupLoader />
</div>

<style>
	.app-container {
		margin: 0;
		height: 100vh;
		width: 100%;
		overflow: hidden;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		width: 100%;
		max-width: 1024px;
		margin: 0 auto;
		box-sizing: border-box;
	}
</style>
