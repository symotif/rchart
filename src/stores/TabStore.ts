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

// Function to set a tab as active
function setActiveTab(tabId: string): void {
	ActiveTabStore.set(tabId);
}

// Function to remove a tab and return the path to navigate to
function removeTab(tabId: string): string | null {
	const tabs = get(TabStore);
	const tabIndex = tabs.findIndex((tab) => tab.id === tabId);
	const isActive = get(ActiveTabStore) === tabId;

	// Don't remove the last tab (Dashboard)
	if (tabs.length <= 1) {
		return null;
	}

	// Remove the tab
	TabStore.update((tabs) => tabs.filter((tab) => tab.id !== tabId));

	// If we removed the active tab, activate another one
	if (isActive) {
		const remainingTabs = tabs.filter((tab) => tab.id !== tabId);
		if (remainingTabs.length > 0) {
			// Prefer the tab to the left, or the first tab
			const newActiveIndex = Math.max(0, tabIndex - 1);
			const newActiveTab = remainingTabs[newActiveIndex] || remainingTabs[0];
			ActiveTabStore.set(newActiveTab.id);
			return newActiveTab.path;
		}
	}

	return null;
}

// Get a tab by ID
function getTab(tabId: string): Tab | undefined {
	return get(TabStore).find((tab) => tab.id === tabId);
}

export { TabStore, ActiveTabStore, addTab, setActiveTab, removeTab, getTab };
