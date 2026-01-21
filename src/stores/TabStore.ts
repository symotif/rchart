// src/stores/TabStore.ts
import { writable, get } from 'svelte/store';

// Define the type for a tab
export type Tab = {
	id: string;
	title: string;
	path: string; // The route path for this tab
};

// Store to manage tabs
const TabStore = writable<Tab[]>([
	{ id: 'dashboard', title: 'Dashboard', path: '/' } // Default tab
]);

// Active tab store to track the currently active tab
const ActiveTabStore = writable<string>('dashboard');

// Navigation history for back/forward
const HistoryStore = writable<string[]>(['/']); // Start with dashboard
const HistoryIndexStore = writable<number>(0);

// Function to add a tab
function addTab(tab: Tab): void {
	TabStore.update((tabs) => {
		const existingTab = tabs.find((t) => t.id === tab.id);
		if (!existingTab) {
			return [...tabs, tab];
		}
		return tabs;
	});

	// Set this tab as active
	ActiveTabStore.set(tab.id);
}

// Function to set a tab as active and add to history
function setActiveTab(tabId: string): void {
	ActiveTabStore.set(tabId);
}

// Add a path to navigation history (called when navigating)
function addToHistory(path: string): void {
	const history = get(HistoryStore);
	const currentIndex = get(HistoryIndexStore);

	// If we're not at the end of history, truncate forward history
	const newHistory = history.slice(0, currentIndex + 1);

	// Don't add duplicate consecutive entries
	if (newHistory[newHistory.length - 1] !== path) {
		newHistory.push(path);
		HistoryStore.set(newHistory);
		HistoryIndexStore.set(newHistory.length - 1);
	}
}

// Go back in history
function goBack(): string | null {
	const history = get(HistoryStore);
	const currentIndex = get(HistoryIndexStore);

	if (currentIndex > 0) {
		const newIndex = currentIndex - 1;
		HistoryIndexStore.set(newIndex);
		return history[newIndex];
	}
	return null;
}

// Go forward in history
function goForward(): string | null {
	const history = get(HistoryStore);
	const currentIndex = get(HistoryIndexStore);

	if (currentIndex < history.length - 1) {
		const newIndex = currentIndex + 1;
		HistoryIndexStore.set(newIndex);
		return history[newIndex];
	}
	return null;
}

// Check if can go back
function canGoBack(): boolean {
	return get(HistoryIndexStore) > 0;
}

// Check if can go forward
function canGoForward(): boolean {
	const history = get(HistoryStore);
	const currentIndex = get(HistoryIndexStore);
	return currentIndex < history.length - 1;
}

// Function to remove a tab and return the path to navigate to (or 'no-tabs' if all tabs removed)
function removeTab(tabId: string): string | null {
	const tabs = get(TabStore);
	const tabIndex = tabs.findIndex((tab) => tab.id === tabId);
	const isActive = get(ActiveTabStore) === tabId;
	const tabToRemove = tabs.find((tab) => tab.id === tabId);

	// Remove the tab
	TabStore.update((tabs) => tabs.filter((tab) => tab.id !== tabId));

	// Remove this tab's path from history
	if (tabToRemove) {
		HistoryStore.update((history) => {
			const filtered = history.filter((path) => path !== tabToRemove.path);
			// If no history left, clear it
			if (filtered.length === 0) {
				return [];
			}
			return filtered;
		});

		// Adjust history index if needed
		const newHistory = get(HistoryStore);
		const currentIndex = get(HistoryIndexStore);
		if (currentIndex >= newHistory.length) {
			HistoryIndexStore.set(Math.max(0, newHistory.length - 1));
		}
	}

	// If we removed the active tab, activate another one or signal no tabs
	if (isActive) {
		const remainingTabs = tabs.filter((tab) => tab.id !== tabId);
		if (remainingTabs.length > 0) {
			// Prefer the tab to the left, or the first tab
			const newActiveIndex = Math.max(0, tabIndex - 1);
			const newActiveTab = remainingTabs[newActiveIndex] || remainingTabs[0];
			ActiveTabStore.set(newActiveTab.id);
			return newActiveTab.path;
		} else {
			// No tabs left
			ActiveTabStore.set('');
			return 'no-tabs';
		}
	}

	return null;
}

// Get a tab by ID
function getTab(tabId: string): Tab | undefined {
	return get(TabStore).find((tab) => tab.id === tabId);
}

// Get tab by path
function getTabByPath(path: string): Tab | undefined {
	return get(TabStore).find((tab) => tab.path === path);
}

export {
	TabStore,
	ActiveTabStore,
	HistoryStore,
	HistoryIndexStore,
	addTab,
	setActiveTab,
	addToHistory,
	goBack,
	goForward,
	canGoBack,
	canGoForward,
	removeTab,
	getTab,
	getTabByPath
};
