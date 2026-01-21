// src/stores/ThemeStore.ts
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark';

// Get initial theme from localStorage or system preference
function getInitialTheme(): Theme {
	if (browser) {
		const stored = localStorage.getItem('theme') as Theme | null;
		if (stored === 'light' || stored === 'dark') {
			return stored;
		}
		// Check system preference
		if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
			return 'dark';
		}
	}
	return 'light';
}

// Create the theme store
const ThemeStore = writable<Theme>(getInitialTheme());

// Function to toggle theme
function toggleTheme(): void {
	ThemeStore.update((current) => {
		const newTheme = current === 'light' ? 'dark' : 'light';
		if (browser) {
			localStorage.setItem('theme', newTheme);
			updateDocumentClass(newTheme);
		}
		return newTheme;
	});
}

// Function to set theme explicitly
function setTheme(theme: Theme): void {
	ThemeStore.set(theme);
	if (browser) {
		localStorage.setItem('theme', theme);
		updateDocumentClass(theme);
	}
}

// Update the document class for Tailwind dark mode
function updateDocumentClass(theme: Theme): void {
	if (browser) {
		if (theme === 'dark') {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}
}

// Initialize document class on load
if (browser) {
	updateDocumentClass(getInitialTheme());
}

export { ThemeStore, toggleTheme, setTheme };
export type { Theme };
