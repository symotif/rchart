import { writable, derived } from 'svelte/store';
import type { SearchResult } from '$lib/types/search';
import type { Patient } from '$lib/types/patient';

// Search mode: local (filter current list), patient (search within patient), global (full DB search)
export type SearchMode = 'local' | 'patient' | 'global';

interface SearchState {
	isOpen: boolean;
	query: string;
	mode: SearchMode;
	patientId: number | null;
	results: SearchResult[];
	localResults: Patient[];
	isLoading: boolean;
	selectedIndex: number;
	recentSearches: string[];
	recentPatients: Patient[];
}

const initialState: SearchState = {
	isOpen: false,
	query: '',
	mode: 'local',
	patientId: null,
	results: [],
	localResults: [],
	isLoading: false,
	selectedIndex: 0,
	recentSearches: [],
	recentPatients: []
};

// Load recent data from localStorage
function loadRecentData(): Partial<SearchState> {
	if (typeof window === 'undefined') return {};
	try {
		const recentSearches = JSON.parse(localStorage.getItem('rchart_recent_searches') || '[]');
		const recentPatients = JSON.parse(localStorage.getItem('rchart_recent_patients') || '[]');
		return { recentSearches, recentPatients };
	} catch {
		return {};
	}
}

function createSearchStore() {
	const { subscribe, set, update } = writable<SearchState>({
		...initialState,
		...loadRecentData()
	});

	return {
		subscribe,

		// Open the search palette
		open: (mode: SearchMode = 'local', patientId: number | null = null) => {
			update((state) => ({
				...state,
				isOpen: true,
				mode,
				patientId,
				query: '',
				results: [],
				localResults: [],
				selectedIndex: 0
			}));
		},

		// Close the search palette
		close: () => {
			update((state) => ({
				...state,
				isOpen: false,
				query: '',
				results: [],
				localResults: [],
				selectedIndex: 0
			}));
		},

		// Toggle the search palette
		toggle: (mode: SearchMode = 'local', patientId: number | null = null) => {
			update((state) => {
				if (state.isOpen) {
					return { ...state, isOpen: false };
				}
				return {
					...state,
					isOpen: true,
					mode,
					patientId,
					query: '',
					results: [],
					localResults: [],
					selectedIndex: 0
				};
			});
		},

		// Set the search query
		setQuery: (query: string) => {
			update((state) => ({
				...state,
				query,
				selectedIndex: 0
			}));
		},

		// Set search mode
		setMode: (mode: SearchMode) => {
			update((state) => ({
				...state,
				mode,
				results: [],
				localResults: [],
				selectedIndex: 0
			}));
		},

		// Set loading state
		setLoading: (isLoading: boolean) => {
			update((state) => ({ ...state, isLoading }));
		},

		// Set global search results
		setResults: (results: SearchResult[]) => {
			update((state) => ({
				...state,
				results,
				isLoading: false
			}));
		},

		// Set local (patient list) search results
		setLocalResults: (localResults: Patient[]) => {
			update((state) => ({
				...state,
				localResults,
				isLoading: false
			}));
		},

		// Move selection up
		selectPrevious: () => {
			update((state) => ({
				...state,
				selectedIndex: Math.max(0, state.selectedIndex - 1)
			}));
		},

		// Move selection down
		selectNext: (maxIndex: number) => {
			update((state) => ({
				...state,
				selectedIndex: Math.min(maxIndex, state.selectedIndex + 1)
			}));
		},

		// Add to recent searches
		addRecentSearch: (query: string) => {
			if (!query.trim()) return;
			update((state) => {
				const recentSearches = [
					query,
					...state.recentSearches.filter((s) => s !== query)
				].slice(0, 10);
				// Persist to localStorage
				if (typeof window !== 'undefined') {
					localStorage.setItem('rchart_recent_searches', JSON.stringify(recentSearches));
				}
				return { ...state, recentSearches };
			});
		},

		// Add to recent patients (for quick access)
		addRecentPatient: (patient: Patient) => {
			update((state) => {
				const recentPatients = [
					patient,
					...state.recentPatients.filter((p) => p.id !== patient.id)
				].slice(0, 10);
				// Persist to localStorage
				if (typeof window !== 'undefined') {
					localStorage.setItem('rchart_recent_patients', JSON.stringify(recentPatients));
				}
				return { ...state, recentPatients };
			});
		},

		// Clear recent searches
		clearRecentSearches: () => {
			update((state) => {
				if (typeof window !== 'undefined') {
					localStorage.removeItem('rchart_recent_searches');
				}
				return { ...state, recentSearches: [] };
			});
		},

		// Reset the store
		reset: () => {
			set({ ...initialState, ...loadRecentData() });
		}
	};
}

export const SearchStore = createSearchStore();

// Derived store for checking if there are any results to display
export const hasResults = derived(SearchStore, ($search) => {
	return $search.results.length > 0 || $search.localResults.length > 0;
});

// Derived store for total result count
export const totalResults = derived(SearchStore, ($search) => {
	return $search.results.length + $search.localResults.length;
});
