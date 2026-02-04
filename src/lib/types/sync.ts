// TypeScript interfaces for sync and connection management

export type ConnectionMode = 'local_nodes' | 'central_db' | 'both' | 'offline';

export type SyncStatus = 'connected' | 'disconnected' | 'syncing' | 'error';

export interface NodeConnection {
	id: string;
	name: string;
	ipAddress: string;
	connected: boolean;
	lastSeen: Date | null;
}

export interface CentralDbConnection {
	host: string;
	port: number;
	database: string;
	connected: boolean;
	lastSync: Date | null;
}

export interface Institution {
	id: number;
	name: string;
	logoUrl: string | null;
	department: string | null;
}

export interface SyncState {
	// Connection mode
	mode: ConnectionMode;

	// Internet connectivity
	isOnline: boolean;

	// Local node sync
	localNodes: {
		enabled: boolean;
		connectedNodes: number;
		totalNodes: number;
		nodes: NodeConnection[];
		status: SyncStatus;
	};

	// Central PostgreSQL sync
	centralDb: {
		enabled: boolean;
		connection: CentralDbConnection | null;
		status: SyncStatus;
	};

	// Current institution
	institution: Institution | null;

	// Last database edit timestamp
	lastEdited: Date | null;

	// Overall sync status
	overallStatus: SyncStatus;

	// Is currently syncing
	isSyncing: boolean;
}

// Default sync state
export const DEFAULT_SYNC_STATE: SyncState = {
	mode: 'offline',
	isOnline: false,
	localNodes: {
		enabled: false,
		connectedNodes: 0,
		totalNodes: 0,
		nodes: [],
		status: 'disconnected'
	},
	centralDb: {
		enabled: false,
		connection: null,
		status: 'disconnected'
	},
	institution: null,
	lastEdited: null,
	overallStatus: 'disconnected',
	isSyncing: false
};

// Connection mode display names
export const CONNECTION_MODE_NAMES: Record<ConnectionMode, string> = {
	local_nodes: 'Local Network Only',
	central_db: 'Central Database Only',
	both: 'Full Sync (Nodes + Central)',
	offline: 'Offline Mode'
};

// Status colors for UI
export const STATUS_COLORS: Record<SyncStatus, { bg: string; text: string; border: string }> = {
	connected: {
		bg: 'bg-gradient-to-r from-green-600 via-green-400 to-green-600',
		text: 'text-green-100',
		border: 'border-green-400'
	},
	disconnected: {
		bg: 'bg-gradient-to-r from-gray-500 via-gray-400 to-gray-500',
		text: 'text-gray-100',
		border: 'border-gray-400'
	},
	syncing: {
		bg: 'bg-gradient-to-r from-yellow-500 via-yellow-400 to-yellow-500',
		text: 'text-yellow-100',
		border: 'border-yellow-400'
	},
	error: {
		bg: 'bg-gradient-to-r from-red-500 via-red-400 to-red-500',
		text: 'text-red-100',
		border: 'border-red-400'
	}
};
