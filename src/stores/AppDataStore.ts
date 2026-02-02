// src/stores/AppDataStore.ts
// Global store for preloaded app data to avoid loading delays

import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { Patient } from '$lib/types/patient';
import type { PatientList, PatientListWithPatients } from '$lib/types/patientList';

export interface AppointmentWithPatient {
	id: number;
	patient_id: number;
	patient_name: string;
	appointment_time: string;
	duration_minutes: number;
	reason: string | null;
	status: string | null;
}

export interface UserFullData {
	user: {
		id: number;
		username: string;
		first_name: string;
		last_name: string;
		degree_type: string | null;
		specialty: string | null;
		subspecialty: string | null;
		npi_number: string | null;
		photo_url: string | null;
		bio: string | null;
	};
	education: Array<{
		id: number;
		education_type: string;
		institution: string;
		degree: string | null;
		field_of_study: string | null;
		start_year: number | null;
		end_year: number | null;
	}>;
	badges: Array<{
		id: number;
		badge_name: string;
		badge_type: string;
		description: string | null;
		icon: string | null;
		color: string | null;
		awarded_date: string | null;
	}>;
	settings: {
		language: string;
		notifications_enabled: boolean;
		email_notifications: boolean;
		sms_notifications: boolean;
		two_factor_enabled: boolean;
		zen_mode_default: boolean;
	} | null;
}

export interface AppData {
	patients: Patient[];
	patientLists: PatientList[];
	patientListsWithPatients: Map<number, PatientListWithPatients>;
	appointments: AppointmentWithPatient[];
	currentUser: UserFullData | null;
	isLoaded: boolean;
	isLoading: boolean;
	loadingStep: string;
	loadingProgress: number;
	error: string | null;
}

const initialState: AppData = {
	patients: [],
	patientLists: [],
	patientListsWithPatients: new Map(),
	appointments: [],
	currentUser: null,
	isLoaded: false,
	isLoading: false,
	loadingStep: '',
	loadingProgress: 0,
	error: null
};

function createAppDataStore() {
	const { subscribe, set, update } = writable<AppData>(initialState);

	return {
		subscribe,

		async initialize() {
			update((state) => ({
				...state,
				isLoading: true,
				loadingStep: 'Initializing...',
				loadingProgress: 0,
				error: null
			}));

			try {
				// Step 1: Seed user data
				update((state) => ({
					...state,
					loadingStep: 'Setting up user profile...',
					loadingProgress: 10
				}));
				await invoke('db_seed_user_data');

				// Step 2: Seed test patients if needed
				update((state) => ({
					...state,
					loadingStep: 'Loading patient records...',
					loadingProgress: 20
				}));
				await invoke('db_seed_test_data');

				// Step 3: Load all patients
				update((state) => ({
					...state,
					loadingStep: 'Preparing patient data...',
					loadingProgress: 35
				}));
				const patients = await invoke<Patient[]>('db_get_all_patients');

				// Step 4: Seed patient lists
				update((state) => ({
					...state,
					loadingStep: 'Organizing patient lists...',
					loadingProgress: 50
				}));
				await invoke('db_seed_patient_lists', { userId: 1 });

				// Step 5: Load patient lists
				update((state) => ({
					...state,
					loadingStep: 'Loading patient lists...',
					loadingProgress: 60
				}));
				const patientLists = await invoke<PatientList[]>('db_get_patient_lists', { userId: 1 });

				// Step 6: Load each patient list with patients
				update((state) => ({
					...state,
					loadingStep: 'Loading list details...',
					loadingProgress: 70
				}));
				const patientListsWithPatients = new Map<number, PatientListWithPatients>();
				for (const list of patientLists) {
					if (list.id) {
						const listData = await invoke<PatientListWithPatients | null>('db_get_patient_list', {
							listId: list.id
						});
						if (listData) {
							patientListsWithPatients.set(list.id, listData);
						}
					}
				}

				// Step 7: Load appointments
				update((state) => ({
					...state,
					loadingStep: 'Loading appointments...',
					loadingProgress: 85
				}));
				const appointments = await invoke<AppointmentWithPatient[]>('db_get_all_appointments');

				// Step 8: Load current user
				update((state) => ({
					...state,
					loadingStep: 'Finalizing...',
					loadingProgress: 95
				}));
				const currentUser = await invoke<UserFullData | null>('db_get_current_user');

				// Done!
				update((state) => ({
					...state,
					patients,
					patientLists,
					patientListsWithPatients,
					appointments,
					currentUser,
					isLoaded: true,
					isLoading: false,
					loadingStep: 'Ready!',
					loadingProgress: 100
				}));
			} catch (e) {
				update((state) => ({
					...state,
					isLoading: false,
					error: e instanceof Error ? e.message : String(e)
				}));
			}
		},

		// Get a specific patient list with patients
		getPatientListWithPatients(listId: number): PatientListWithPatients | undefined {
			const state = get({ subscribe });
			return state.patientListsWithPatients.get(listId);
		},

		// Refresh patients
		async refreshPatients() {
			const patients = await invoke<Patient[]>('db_get_all_patients');
			update((state) => ({ ...state, patients }));
		},

		// Refresh patient lists
		async refreshPatientLists() {
			const patientLists = await invoke<PatientList[]>('db_get_patient_lists', { userId: 1 });
			const patientListsWithPatients = new Map<number, PatientListWithPatients>();
			for (const list of patientLists) {
				if (list.id) {
					const listData = await invoke<PatientListWithPatients | null>('db_get_patient_list', {
						listId: list.id
					});
					if (listData) {
						patientListsWithPatients.set(list.id, listData);
					}
				}
			}
			update((state) => ({ ...state, patientLists, patientListsWithPatients }));
		},

		// Refresh appointments
		async refreshAppointments() {
			const appointments = await invoke<AppointmentWithPatient[]>('db_get_all_appointments');
			update((state) => ({ ...state, appointments }));
		},

		// Reset store
		reset() {
			set(initialState);
		}
	};
}

export const AppDataStore = createAppDataStore();
