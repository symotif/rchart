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

	// stores
	import { SideBarStore, setTab } from '../stores/SideBarStore';
	import { ThemeStore } from '../stores/ThemeStore';

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
	<section class="flex flex-col absolute top-0 left-20 h-30 w-full pt-3 px-5 bg-white dark:bg-gray-800">
		<!-- the top part of the bar -->
		<div class="flex flex-row">
			<!-- search box -->
			<form class="w-full max-w-md">
				<div class="relative flex items-center text-gray-400 focus-within:text-gray-500 dark:text-gray-300 dark:focus-within:text-gray-200">
					<i class="fa-solid fa-magnifying-glass w-5 h-5 absolute ml-3 pointer-events-none"></i>
					<input
						type="text"
						name="search"
						placeholder="Search"
						autocomplete="off"
						class="w-full pr-3 pl-10 py-2 font-semibold bg-gray-100 dark:bg-gray-700 placeholder-gray-500 dark:placeholder-gray-400 text-block dark:text-gray-100
								rounded-2xl border-none ring-2 ring-gray-300 dark:ring-gray-600 focus:ring-gray-500 dark:focus:ring-gray-400
								focus:ring-2"
					/>
				</div>
			</form>

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
		{@render children()}
	</main>

	<!-- message center -->
	<MessageCenter />

	<!-- status bar -->
	<StatusBar />
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
