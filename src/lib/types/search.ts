// TypeScript interfaces for search functionality

export interface SearchResult {
	result_type: 'patient' | 'encounter' | 'diagnosis' | 'medication' | 'lab';
	id: number;
	patient_id: number | null;
	title: string;
	subtitle: string | null;
	snippet: string | null;
	rank: number;
}

export interface SearchContext {
	mode: 'local' | 'patient' | 'global';
	patientId?: number;
	route: string;
}

// Icons for different result types
export const SEARCH_RESULT_ICONS: Record<string, string> = {
	patient: 'fa-user',
	encounter: 'fa-file-medical',
	diagnosis: 'fa-stethoscope',
	medication: 'fa-pills',
	lab: 'fa-flask'
};

// Colors for different result types
export const SEARCH_RESULT_COLORS: Record<string, string> = {
	patient: 'text-blue-500',
	encounter: 'text-green-500',
	diagnosis: 'text-purple-500',
	medication: 'text-orange-500',
	lab: 'text-cyan-500'
};

// Labels for different result types
export const SEARCH_RESULT_LABELS: Record<string, string> = {
	patient: 'Patient',
	encounter: 'Note',
	diagnosis: 'Diagnosis',
	medication: 'Medication',
	lab: 'Lab Result'
};
