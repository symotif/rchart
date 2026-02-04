/**
 * @deprecated Use SyncStatusStore instead
 * This file is kept for backwards compatibility
 */
import { derived } from 'svelte/store';
import { isOnline, setOnlineStatus } from './SyncStatusStore';

// Re-export isOnline as ConnectionStatusStore for backwards compatibility
export const ConnectionStatusStore = isOnline;

// Wrapper for setConnectionStatus that uses the new store
export function setConnectionStatus(value: boolean): void {
	setOnlineStatus(value);
}
