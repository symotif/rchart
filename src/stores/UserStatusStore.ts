import { writable, derived, get } from 'svelte/store';
import type { UserStatus, UserPresenceStatus, StatusDuration } from '../lib/types/user';
import { DEFAULT_USER_STATUS, USER_STATUS_CONFIG } from '../lib/types/user';

// Main user status store
const userStatusStore = writable<UserStatus>(DEFAULT_USER_STATUS);

// Timer for auto-expiring status
let statusExpiryTimer: ReturnType<typeof setTimeout> | null = null;

// Derived stores
export const currentStatus = derived(userStatusStore, ($status) => $status.status);
export const statusMessage = derived(userStatusStore, ($status) => $status.statusMessage);
export const isInvisible = derived(userStatusStore, ($status) => $status.isInvisible);
export const statusExpiresAt = derived(userStatusStore, ($status) => $status.expiresAt);

// Get the apparent status (what others see)
export const apparentStatus = derived(userStatusStore, ($status) => {
	if ($status.isInvisible || $status.status === 'invisible') {
		return 'offline';
	}
	return $status.status;
});

// Get status display config
export const statusConfig = derived(currentStatus, ($status) => USER_STATUS_CONFIG[$status]);

// Get apparent status config (for display to others)
export const apparentStatusConfig = derived(apparentStatus, ($status) => USER_STATUS_CONFIG[$status]);

/**
 * Set user status with optional duration and message
 */
export function setUserStatus(
	status: UserPresenceStatus,
	options?: {
		message?: string | null;
		durationMinutes?: StatusDuration;
	}
): void {
	// Clear any existing timer
	if (statusExpiryTimer) {
		clearTimeout(statusExpiryTimer);
		statusExpiryTimer = null;
	}

	const isInvisible = status === 'invisible';
	const expiresAt = options?.durationMinutes
		? new Date(Date.now() + options.durationMinutes * 60 * 1000)
		: null;

	userStatusStore.set({
		status,
		statusMessage: options?.message ?? null,
		expiresAt,
		isInvisible
	});

	// Set up expiry timer if duration specified
	if (options?.durationMinutes) {
		statusExpiryTimer = setTimeout(() => {
			// Revert to online when status expires
			setUserStatus('online');
		}, options.durationMinutes * 60 * 1000);
	}

	// Persist to localStorage for session persistence
	saveStatusToStorage();
}

/**
 * Set just the status message without changing status
 */
export function setStatusMessage(message: string | null): void {
	userStatusStore.update((state) => ({
		...state,
		statusMessage: message
	}));
	saveStatusToStorage();
}

/**
 * Clear status message
 */
export function clearStatusMessage(): void {
	setStatusMessage(null);
}

/**
 * Quick status setters
 */
export function setOnline(message?: string): void {
	setUserStatus('online', { message });
}

export function setBusy(durationMinutes?: StatusDuration, message?: string): void {
	setUserStatus('busy', { durationMinutes, message });
}

export function setAway(durationMinutes?: StatusDuration, message?: string): void {
	setUserStatus('away', { durationMinutes, message });
}

export function setDoNotDisturb(durationMinutes?: StatusDuration, message?: string): void {
	setUserStatus('dnd', { durationMinutes, message });
}

export function setInvisible(message?: string): void {
	setUserStatus('invisible', { message });
}

export function setOffline(): void {
	setUserStatus('offline');
}

/**
 * Get time remaining until status expires
 */
export function getTimeRemaining(): { minutes: number; seconds: number } | null {
	const state = get(userStatusStore);
	if (!state.expiresAt) return null;

	const remaining = state.expiresAt.getTime() - Date.now();
	if (remaining <= 0) return null;

	return {
		minutes: Math.floor(remaining / 60000),
		seconds: Math.floor((remaining % 60000) / 1000)
	};
}

/**
 * Cancel the current status timer and make it indefinite
 */
export function clearStatusExpiry(): void {
	if (statusExpiryTimer) {
		clearTimeout(statusExpiryTimer);
		statusExpiryTimer = null;
	}
	userStatusStore.update((state) => ({
		...state,
		expiresAt: null
	}));
	saveStatusToStorage();
}

/**
 * Save status to localStorage
 */
function saveStatusToStorage(): void {
	if (typeof localStorage === 'undefined') return;

	const state = get(userStatusStore);
	localStorage.setItem(
		'userStatus',
		JSON.stringify({
			...state,
			expiresAt: state.expiresAt?.toISOString() ?? null
		})
	);
}

/**
 * Load status from localStorage
 */
export function loadStatusFromStorage(): void {
	if (typeof localStorage === 'undefined') return;

	try {
		const saved = localStorage.getItem('userStatus');
		if (saved) {
			const parsed = JSON.parse(saved);
			const expiresAt = parsed.expiresAt ? new Date(parsed.expiresAt) : null;

			// Check if saved status has expired
			if (expiresAt && expiresAt.getTime() < Date.now()) {
				// Status expired, reset to online
				setUserStatus('online');
				return;
			}

			// Restore the saved status
			userStatusStore.set({
				status: parsed.status,
				statusMessage: parsed.statusMessage,
				expiresAt,
				isInvisible: parsed.isInvisible
			});

			// Re-set timer if there's an expiry
			if (expiresAt) {
				const remaining = expiresAt.getTime() - Date.now();
				if (remaining > 0) {
					statusExpiryTimer = setTimeout(() => {
						setUserStatus('online');
					}, remaining);
				}
			}
		}
	} catch {
		// If parsing fails, just use defaults
		console.warn('Failed to load user status from storage');
	}
}

/**
 * Initialize the store - call this on app startup
 */
export function initUserStatus(): void {
	loadStatusFromStorage();
}

// Export the main store
export const UserStatusStore = userStatusStore;
