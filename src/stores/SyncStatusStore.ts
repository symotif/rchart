import { writable, derived, get } from 'svelte/store';
import type { SyncState, ConnectionMode, SyncStatus, Institution, NodeConnection } from '../lib/types/sync';
import { DEFAULT_SYNC_STATE } from '../lib/types/sync';

// Main sync state store
const syncState = writable<SyncState>(DEFAULT_SYNC_STATE);

// Derived stores for convenience
export const isOnline = derived(syncState, ($state) => $state.isOnline);
export const connectionMode = derived(syncState, ($state) => $state.mode);
export const overallStatus = derived(syncState, ($state) => $state.overallStatus);
export const isSyncing = derived(syncState, ($state) => $state.isSyncing);
export const lastEdited = derived(syncState, ($state) => $state.lastEdited);
export const institution = derived(syncState, ($state) => $state.institution);
export const localNodesStatus = derived(syncState, ($state) => $state.localNodes);
export const centralDbStatus = derived(syncState, ($state) => $state.centralDb);

// Actions

/**
 * Update online status based on network connectivity
 */
export function setOnlineStatus(online: boolean): void {
	syncState.update((state) => {
		const newState = { ...state, isOnline: online };
		// Recalculate overall status
		newState.overallStatus = calculateOverallStatus(newState);
		return newState;
	});
}

/**
 * Set the connection mode
 */
export function setConnectionMode(mode: ConnectionMode): void {
	syncState.update((state) => {
		const newState = { ...state, mode };

		// Update enabled flags based on mode
		newState.localNodes.enabled = mode === 'local_nodes' || mode === 'both';
		newState.centralDb.enabled = mode === 'central_db' || mode === 'both';

		// If offline mode, reset connections
		if (mode === 'offline') {
			newState.localNodes.status = 'disconnected';
			newState.centralDb.status = 'disconnected';
		}

		newState.overallStatus = calculateOverallStatus(newState);
		return newState;
	});
}

/**
 * Set the current institution
 */
export function setInstitution(inst: Institution | null): void {
	syncState.update((state) => ({
		...state,
		institution: inst
	}));
}

/**
 * Update local nodes connection status
 */
export function updateLocalNodesStatus(
	connectedNodes: number,
	totalNodes: number,
	status: SyncStatus
): void {
	syncState.update((state) => {
		const newState = {
			...state,
			localNodes: {
				...state.localNodes,
				connectedNodes,
				totalNodes,
				status
			}
		};
		newState.overallStatus = calculateOverallStatus(newState);
		return newState;
	});
}

/**
 * Update central database connection status
 */
export function updateCentralDbStatus(status: SyncStatus): void {
	syncState.update((state) => {
		const newState = {
			...state,
			centralDb: {
				...state.centralDb,
				status,
				connection: state.centralDb.connection
					? { ...state.centralDb.connection, connected: status === 'connected' }
					: null
			}
		};
		newState.overallStatus = calculateOverallStatus(newState);
		return newState;
	});
}

/**
 * Set syncing state (shows syncing animation)
 */
export function setSyncing(syncing: boolean): void {
	syncState.update((state) => {
		const newState = { ...state, isSyncing: syncing };
		if (syncing) {
			// When syncing starts, set status to syncing for enabled connections
			if (state.localNodes.enabled) {
				newState.localNodes = { ...newState.localNodes, status: 'syncing' };
			}
			if (state.centralDb.enabled) {
				newState.centralDb = { ...newState.centralDb, status: 'syncing' };
			}
			newState.overallStatus = 'syncing';
		}
		return newState;
	});
}

/**
 * Update last edited timestamp - called on every CRUD operation
 */
export function updateLastEdited(): void {
	syncState.update((state) => ({
		...state,
		lastEdited: new Date()
	}));
}

/**
 * Get formatted last edited time
 */
export function getFormattedLastEdited(): string {
	const state = get(syncState);
	if (!state.lastEdited) return 'Never';

	return state.lastEdited.toLocaleTimeString('en-US', {
		hour: 'numeric',
		minute: '2-digit',
		second: '2-digit',
		hour12: true,
		timeZoneName: 'short'
	});
}

/**
 * Calculate overall status based on connection mode and individual statuses
 */
function calculateOverallStatus(state: SyncState): SyncStatus {
	if (!state.isOnline && state.mode !== 'offline') {
		return 'disconnected';
	}

	if (state.mode === 'offline') {
		return 'disconnected';
	}

	if (state.isSyncing) {
		return 'syncing';
	}

	// Check based on mode
	if (state.mode === 'local_nodes') {
		return state.localNodes.status;
	}

	if (state.mode === 'central_db') {
		return state.centralDb.status;
	}

	if (state.mode === 'both') {
		// Both must be connected for green status
		if (state.localNodes.status === 'connected' && state.centralDb.status === 'connected') {
			return 'connected';
		}
		// If one is error, show error
		if (state.localNodes.status === 'error' || state.centralDb.status === 'error') {
			return 'error';
		}
		// If one is syncing, show syncing
		if (state.localNodes.status === 'syncing' || state.centralDb.status === 'syncing') {
			return 'syncing';
		}
		// Otherwise disconnected
		return 'disconnected';
	}

	return 'disconnected';
}

/**
 * Initialize connection check with ping
 */
export async function checkInternetConnection(): Promise<boolean> {
	try {
		// Try to ping a reliable endpoint
		const controller = new AbortController();
		const timeout = setTimeout(() => controller.abort(), 5000);

		const response = await fetch('https://www.google.com/generate_204', {
			method: 'HEAD',
			mode: 'no-cors',
			signal: controller.signal
		});

		clearTimeout(timeout);
		setOnlineStatus(true);
		return true;
	} catch {
		setOnlineStatus(false);
		return false;
	}
}

/**
 * Start periodic internet connection checks
 */
export function startConnectionMonitoring(intervalMs: number = 30000): () => void {
	// Initial check
	checkInternetConnection();

	// Set up browser online/offline events
	const handleOnline = () => setOnlineStatus(true);
	const handleOffline = () => setOnlineStatus(false);

	if (typeof window !== 'undefined') {
		window.addEventListener('online', handleOnline);
		window.addEventListener('offline', handleOffline);

		// Initial state from browser
		setOnlineStatus(navigator.onLine);
	}

	// Periodic check
	const interval = setInterval(checkInternetConnection, intervalMs);

	// Return cleanup function
	return () => {
		clearInterval(interval);
		if (typeof window !== 'undefined') {
			window.removeEventListener('online', handleOnline);
			window.removeEventListener('offline', handleOffline);
		}
	};
}

// Export the main store
export const SyncStatusStore = syncState;
