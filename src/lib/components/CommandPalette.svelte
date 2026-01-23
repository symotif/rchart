<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { invoke } from '@tauri-apps/api/tauri';
	import { SearchStore, type SearchMode } from '../../stores/SearchStore';
	import { addTab } from '../../stores/TabStore';
	import type { SearchResult } from '$lib/types/search';
	import type { Patient } from '$lib/types/patient';
	import {
		SEARCH_RESULT_ICONS,
		SEARCH_RESULT_COLORS,
		SEARCH_RESULT_LABELS
	} from '$lib/types/search';

	// Debounce timer
	let debounceTimer: ReturnType<typeof setTimeout>;
	const DEBOUNCE_MS = 200;

	// Input ref for focus
	let inputRef: HTMLInputElement;

	// Computed values from store
	let isOpen = $derived($SearchStore.isOpen);
	let query = $derived($SearchStore.query);
	let mode = $derived($SearchStore.mode);
	let results = $derived($SearchStore.results);
	let localResults = $derived($SearchStore.localResults);
	let isLoading = $derived($SearchStore.isLoading);
	let selectedIndex = $derived($SearchStore.selectedIndex);
	let recentSearches = $derived($SearchStore.recentSearches);
	let recentPatients = $derived($SearchStore.recentPatients);

	// Get current route context
	let currentRoute = $derived($page.url.pathname);
	let isOnPatientList = $derived(currentRoute === '/list');
	let isOnPatientPage = $derived(currentRoute.startsWith('/patient/') && !currentRoute.includes('/note/'));
	let currentPatientId = $derived(() => {
		const match = currentRoute.match(/\/patient\/(\d+)/);
		return match ? parseInt(match[1], 10) : null;
	});

	// Total items for navigation
	let totalItems = $derived(
		mode === 'local'
			? localResults.length
			: results.length + (query ? 0 : recentPatients.length)
	);

	// Handle keyboard shortcut to open
	function handleGlobalKeydown(event: KeyboardEvent) {
		// Cmd+K or Ctrl+K to open global search
		if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
			event.preventDefault();
			const patientId = currentPatientId();
			if (isOnPatientPage && patientId) {
				SearchStore.open('patient', patientId);
			} else {
				SearchStore.open('global');
			}
		}

		// Escape to close
		if (event.key === 'Escape' && isOpen) {
			event.preventDefault();
			SearchStore.close();
		}
	}

	// Handle input changes with debounce
	function handleInput(event: Event) {
		const value = (event.target as HTMLInputElement).value;
		SearchStore.setQuery(value);

		// Clear previous timer
		clearTimeout(debounceTimer);

		if (!value.trim()) {
			SearchStore.setResults([]);
			SearchStore.setLocalResults([]);
			return;
		}

		// Debounce the search
		debounceTimer = setTimeout(() => {
			performSearch(value);
		}, DEBOUNCE_MS);
	}

	// Perform the actual search
	async function performSearch(searchQuery: string) {
		if (!searchQuery.trim()) return;

		SearchStore.setLoading(true);

		try {
			if (mode === 'local') {
				// Quick patient search for patient list filtering
				const patients = await invoke<Patient[]>('db_quick_search_patients', {
					query: searchQuery,
					limit: 50
				});
				SearchStore.setLocalResults(patients);
			} else if (mode === 'patient' && $SearchStore.patientId) {
				// Search within a specific patient
				const searchResults = await invoke<SearchResult[]>('db_search_patient_data', {
					patientId: $SearchStore.patientId,
					query: searchQuery,
					limit: 20
				});
				SearchStore.setResults(searchResults);
			} else {
				// Global search
				const searchResults = await invoke<SearchResult[]>('db_global_search', {
					query: searchQuery,
					limit: 20
				});
				SearchStore.setResults(searchResults);
			}
		} catch (error) {
			console.error('Search error:', error);
			SearchStore.setResults([]);
			SearchStore.setLocalResults([]);
		} finally {
			SearchStore.setLoading(false);
		}
	}

	// Handle keyboard navigation
	function handleKeydown(event: KeyboardEvent) {
		if (!isOpen) return;

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				SearchStore.selectNext(totalItems - 1);
				break;
			case 'ArrowUp':
				event.preventDefault();
				SearchStore.selectPrevious();
				break;
			case 'Enter':
				event.preventDefault();
				selectCurrentItem();
				break;
			case 'Tab':
				// Toggle between local and global mode
				event.preventDefault();
				if (mode === 'local') {
					SearchStore.setMode('global');
				} else if (mode === 'global') {
					SearchStore.setMode('local');
				}
				break;
		}
	}

	// Select the current highlighted item
	function selectCurrentItem() {
		if (mode === 'local' && localResults.length > 0) {
			const patient = localResults[selectedIndex];
			if (patient) {
				navigateToPatient(patient);
			}
		} else if (results.length > 0) {
			const result = results[selectedIndex];
			if (result) {
				navigateToResult(result);
			}
		} else if (!query && recentPatients.length > 0) {
			const patient = recentPatients[selectedIndex];
			if (patient) {
				navigateToPatient(patient);
			}
		}
	}

	// Navigate to a search result
	function navigateToResult(result: SearchResult) {
		SearchStore.addRecentSearch(query);
		SearchStore.close();

		switch (result.result_type) {
			case 'patient':
				goto(`/patient/${result.id}`);
				addTab({
					id: `patient-${result.id}`,
					title: result.title,
					path: `/patient/${result.id}`
				});
				break;
			case 'encounter':
				if (result.patient_id) {
					goto(`/patient/${result.patient_id}/note/${result.id}`);
					addTab({
						id: `note-${result.id}-${result.patient_id}`,
						title: `Note - ${result.title}`,
						path: `/patient/${result.patient_id}/note/${result.id}`
					});
				}
				break;
			case 'diagnosis':
			case 'medication':
			case 'lab':
				// Navigate to patient page for these types
				if (result.patient_id) {
					goto(`/patient/${result.patient_id}`);
					addTab({
						id: `patient-${result.patient_id}`,
						title: `Patient`,
						path: `/patient/${result.patient_id}`
					});
				}
				break;
		}
	}

	// Navigate to a patient
	function navigateToPatient(patient: Patient) {
		SearchStore.addRecentPatient(patient);
		SearchStore.addRecentSearch(query);
		SearchStore.close();

		const patientId = patient.id;
		const name = `${patient.first_name} ${patient.last_name}`;
		goto(`/patient/${patientId}`);
		addTab({
			id: `patient-${patientId}`,
			title: name,
			path: `/patient/${patientId}`
		});
	}

	// Focus input when opened
	$effect(() => {
		if (isOpen && inputRef) {
			setTimeout(() => inputRef?.focus(), 50);
		}
	});

	onMount(() => {
		window.addEventListener('keydown', handleGlobalKeydown);
		return () => {
			window.removeEventListener('keydown', handleGlobalKeydown);
			clearTimeout(debounceTimer);
		};
	});
</script>

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 bg-black/50 z-[100]"
		onclick={() => SearchStore.close()}
		role="button"
		tabindex="-1"
		onkeydown={(e) => e.key === 'Escape' && SearchStore.close()}
	></div>

	<!-- Command Palette -->
	<div class="fixed inset-0 z-[101] flex items-start justify-center pt-[15vh] pointer-events-none">
		<div
			class="w-full max-w-2xl bg-white dark:bg-gray-800 rounded-xl shadow-2xl overflow-hidden pointer-events-auto"
			onkeydown={handleKeydown}
			role="dialog"
			aria-modal="true"
		>
			<!-- Search Input -->
			<div class="flex items-center px-4 border-b border-gray-200 dark:border-gray-700">
				<i class="fa-solid fa-magnifying-glass text-gray-400 mr-3"></i>
				<input
					bind:this={inputRef}
					type="text"
					value={query}
					oninput={handleInput}
					placeholder={mode === 'local'
						? 'Search patients...'
						: mode === 'patient'
							? 'Search within patient...'
							: 'Search everything... (Cmd+K)'}
					class="flex-1 py-4 bg-transparent text-gray-900 dark:text-gray-100 placeholder-gray-400 focus:outline-none text-lg"
				/>
				{#if isLoading}
					<div class="animate-spin rounded-full h-5 w-5 border-b-2 border-blue-500"></div>
				{/if}
				<!-- Mode indicator -->
				<div class="ml-3 flex items-center gap-2">
					<button
						onclick={() => SearchStore.setMode('local')}
						class="px-2 py-1 text-xs rounded {mode === 'local'
							? 'bg-blue-500 text-white'
							: 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400'}"
					>
						Local
					</button>
					<button
						onclick={() => SearchStore.setMode('global')}
						class="px-2 py-1 text-xs rounded {mode === 'global'
							? 'bg-blue-500 text-white'
							: 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400'}"
					>
						Global
					</button>
				</div>
			</div>

			<!-- Results -->
			<div class="max-h-[60vh] overflow-y-auto">
				{#if mode === 'local' && localResults.length > 0}
					<!-- Local patient results -->
					<div class="p-2">
						<div class="px-3 py-2 text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">
							Patients ({localResults.length})
						</div>
						{#each localResults as patient, index}
							<button
								onclick={() => navigateToPatient(patient)}
								class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors
									{selectedIndex === index
									? 'bg-blue-50 dark:bg-blue-900/30'
									: 'hover:bg-gray-50 dark:hover:bg-gray-700'}"
							>
								<div
									class="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400 flex items-center justify-center font-medium"
								>
									{patient.first_name[0]}{patient.last_name[0]}
								</div>
								<div class="flex-1 min-w-0">
									<p class="font-medium text-gray-900 dark:text-gray-100">
										{patient.first_name} {patient.last_name}
									</p>
									<p class="text-sm text-gray-500 dark:text-gray-400 truncate">
										{patient.dob} {patient.phone ? `• ${patient.phone}` : ''}
									</p>
								</div>
								<kbd
									class="hidden sm:inline-block px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 rounded text-gray-500"
								>
									Enter
								</kbd>
							</button>
						{/each}
					</div>
				{:else if results.length > 0}
					<!-- Global/patient search results -->
					<div class="p-2">
						{#each results as result, index}
							<button
								onclick={() => navigateToResult(result)}
								class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors
									{selectedIndex === index
									? 'bg-blue-50 dark:bg-blue-900/30'
									: 'hover:bg-gray-50 dark:hover:bg-gray-700'}"
							>
								<div
									class="w-10 h-10 rounded-lg bg-gray-100 dark:bg-gray-700 flex items-center justify-center"
								>
									<i
										class="fa-solid {SEARCH_RESULT_ICONS[result.result_type]} {SEARCH_RESULT_COLORS[
											result.result_type
										]}"
									></i>
								</div>
								<div class="flex-1 min-w-0">
									<div class="flex items-center gap-2">
										<p class="font-medium text-gray-900 dark:text-gray-100 truncate">
											{result.title}
										</p>
										<span
											class="text-xs px-1.5 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400"
										>
											{SEARCH_RESULT_LABELS[result.result_type]}
										</span>
									</div>
									{#if result.subtitle}
										<p class="text-sm text-gray-500 dark:text-gray-400 truncate">
											{result.subtitle}
										</p>
									{/if}
									{#if result.snippet}
										<p class="text-sm text-gray-400 dark:text-gray-500 truncate">
											{@html result.snippet}
										</p>
									{/if}
								</div>
							</button>
						{/each}
					</div>
				{:else if !query && recentPatients.length > 0}
					<!-- Recent patients when no query -->
					<div class="p-2">
						<div class="px-3 py-2 text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">
							Recent Patients
						</div>
						{#each recentPatients as patient, index}
							<button
								onclick={() => navigateToPatient(patient)}
								class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors
									{selectedIndex === index
									? 'bg-blue-50 dark:bg-blue-900/30'
									: 'hover:bg-gray-50 dark:hover:bg-gray-700'}"
							>
								<div
									class="w-10 h-10 rounded-full bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 flex items-center justify-center font-medium"
								>
									{patient.first_name[0]}{patient.last_name[0]}
								</div>
								<div class="flex-1 min-w-0">
									<p class="font-medium text-gray-900 dark:text-gray-100">
										{patient.first_name} {patient.last_name}
									</p>
									<p class="text-sm text-gray-500 dark:text-gray-400">
										{patient.dob}
									</p>
								</div>
								<i class="fa-solid fa-clock text-gray-300 dark:text-gray-600"></i>
							</button>
						{/each}
					</div>
				{:else if query && !isLoading}
					<!-- No results -->
					<div class="p-8 text-center text-gray-500 dark:text-gray-400">
						<i class="fa-solid fa-search text-4xl mb-3 opacity-50"></i>
						<p>No results found for "{query}"</p>
						<p class="text-sm mt-1">Try a different search term</p>
					</div>
				{:else if !query}
					<!-- Empty state with hints -->
					<div class="p-4 space-y-3">
						<div class="px-3 py-2 text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">
							Quick Actions
						</div>
						<div class="space-y-1">
							<div class="flex items-center gap-3 px-3 py-2 text-gray-600 dark:text-gray-400">
								<i class="fa-solid fa-user-plus w-5"></i>
								<span>Type a patient name to search</span>
							</div>
							<div class="flex items-center gap-3 px-3 py-2 text-gray-600 dark:text-gray-400">
								<i class="fa-solid fa-keyboard w-5"></i>
								<span>Press <kbd class="px-1.5 py-0.5 bg-gray-100 dark:bg-gray-700 rounded text-xs">Tab</kbd> to switch modes</span>
							</div>
							<div class="flex items-center gap-3 px-3 py-2 text-gray-600 dark:text-gray-400">
								<i class="fa-solid fa-arrow-up w-5"></i>
								<span>Use arrow keys to navigate</span>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div
				class="flex items-center justify-between px-4 py-2 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 text-xs text-gray-500 dark:text-gray-400"
			>
				<div class="flex items-center gap-4">
					<span><kbd class="px-1.5 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">↑↓</kbd> Navigate</span>
					<span><kbd class="px-1.5 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">Enter</kbd> Select</span>
					<span><kbd class="px-1.5 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">Tab</kbd> Toggle Mode</span>
				</div>
				<span><kbd class="px-1.5 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">Esc</kbd> Close</span>
			</div>
		</div>
	</div>
{/if}
