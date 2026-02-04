/**
 * Database service wrapper that tracks CRUD operations
 * and updates the last edited timestamp in SyncStatusStore
 */

import { invoke } from '@tauri-apps/api/tauri';
import { updateLastEdited, setSyncing } from '../../stores/SyncStatusStore';

// Type for Tauri command names
type TauriCommand = string;

// Commands that are considered "write" operations (CRUD minus Read)
const WRITE_COMMANDS = new Set([
	// Patient operations
	'db_create_patient',
	'db_update_patient',
	'db_delete_patient',

	// Appointment operations
	'db_create_appointment',
	'db_update_appointment',
	'db_delete_appointment',

	// User operations
	'db_update_user',
	'db_update_user_settings',
	'db_update_password',
	'db_seed_user_data',

	// Encounter operations
	'db_create_encounter',
	'db_update_encounter',
	'db_delete_encounter',

	// Patient list operations
	'db_create_patient_list',
	'db_update_patient_list',
	'db_delete_patient_list',
	'db_add_patient_to_list',
	'db_remove_patient_from_list',

	// Prescription/medication operations
	'db_create_prescriptions',
	'db_update_prescription',
	'db_delete_prescription',

	// Diagnosis operations
	'db_create_diagnosis',
	'db_update_diagnosis',
	'db_delete_diagnosis',

	// Allergy operations
	'db_create_allergy',
	'db_update_allergy',
	'db_delete_allergy',

	// Vaccination operations
	'db_create_vaccination',
	'db_update_vaccination',
	'db_delete_vaccination',

	// Social history operations
	'db_create_social_history',
	'db_update_social_history',
	'db_delete_social_history',

	// Family history operations
	'db_create_family_history',
	'db_update_family_history',
	'db_delete_family_history',

	// Vitals operations
	'db_create_vitals',
	'db_update_vitals',
	'db_delete_vitals',

	// Labs operations
	'db_create_lab',
	'db_update_lab',
	'db_delete_lab',

	// Clinical scores
	'db_create_clinical_score',
	'db_update_clinical_score',
	'db_delete_clinical_score',

	// Todos/goals
	'db_create_todo',
	'db_update_todo',
	'db_delete_todo',
	'db_create_goal',
	'db_update_goal',
	'db_delete_goal',

	// Messages
	'db_create_message',
	'db_update_message',
	'db_delete_message',
	'db_create_user_message',
	'db_update_user_message',
	'db_delete_user_message',

	// Timeline events
	'db_create_timeline_event',
	'db_update_timeline_event',
	'db_delete_timeline_event'
]);

/**
 * Wrapper around Tauri invoke that tracks database operations
 * and updates the last edited timestamp for write operations
 */
export async function dbInvoke<T>(command: TauriCommand, args?: Record<string, unknown>): Promise<T> {
	const isWriteOperation = WRITE_COMMANDS.has(command);

	try {
		// If it's a write operation, show syncing indicator briefly
		if (isWriteOperation) {
			setSyncing(true);
		}

		// Execute the actual Tauri command
		const result = await invoke<T>(command, args);

		// If it was a write operation, update the last edited timestamp
		if (isWriteOperation) {
			updateLastEdited();
		}

		return result;
	} finally {
		// Always stop syncing indicator
		if (isWriteOperation) {
			// Small delay to make the sync animation visible
			setTimeout(() => setSyncing(false), 300);
		}
	}
}

/**
 * Helper to check if a command is a write operation
 */
export function isWriteCommand(command: string): boolean {
	return WRITE_COMMANDS.has(command);
}

/**
 * Register a custom write command (for extensions or new commands)
 */
export function registerWriteCommand(command: string): void {
	WRITE_COMMANDS.add(command);
}
