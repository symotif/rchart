// TypeScript interfaces for patient data

export interface Patient {
	id: number | null;
	first_name: string;
	last_name: string;
	dob: string;
	sex: string;
	gender: string | null;
	address: string | null;
	phone: string | null;
	email: string | null;
	photo_url: string | null;
	ai_summary: string | null;
	preferred_pharmacy: string | null;
	insurance_provider: string | null;
	insurance_policy_number: string | null;
	insurance_group_number: string | null;
}

// Diagnosis category types for the pie chart
export type DiagnosisCategory =
	| 'cardiac'    // Red
	| 'pulm'       // Blue
	| 'gi'         // Brown
	| 'neuro'      // Orange
	| 'psych'      // Purple
	| 'renal'      // Yellow
	| 'endocrine'  // Orange
	| 'obgyn'      // Pink
	| 'oncology'   // Lime green
	| 'heme'       // Maroon
	| 'msk'        // Gray
	| 'immune'     // Green
	| 'social';    // Beige

// Category color mapping
export const CATEGORY_COLORS: Record<DiagnosisCategory, string> = {
	cardiac: '#ef4444',    // Red
	pulm: '#3b82f6',       // Blue
	gi: '#92400e',         // Brown
	neuro: '#f97316',      // Orange
	psych: '#a855f7',      // Purple
	renal: '#eab308',      // Yellow
	endocrine: '#fb923c',  // Light orange
	obgyn: '#ec4899',      // Pink
	oncology: '#84cc16',   // Lime green
	heme: '#7f1d1d',       // Maroon
	msk: '#6b7280',        // Gray
	immune: '#22c55e',     // Green
	social: '#d4a574'      // Beige
};

// Category display names
export const CATEGORY_NAMES: Record<DiagnosisCategory, string> = {
	cardiac: 'Cardiac',
	pulm: 'Pulmonary',
	gi: 'GI',
	neuro: 'Neuro',
	psych: 'Psych',
	renal: 'Renal/GU',
	endocrine: 'Endocrine',
	obgyn: 'OB/GYN',
	oncology: 'Oncology',
	heme: 'Heme',
	msk: 'MSK',
	immune: 'Immune',
	social: 'Social'
};

export interface Diagnosis {
	id: number | null;
	patient_id: number;
	name: string;
	icd_code: string | null;
	onset_date: string | null;
	status: string | null;
	category: DiagnosisCategory | null;
}

export interface Medication {
	id: number | null;
	patient_id: number;
	name: string;
	dose: string | null;
	frequency: string | null;
	route: string | null;
	start_date: string | null;
	end_date: string | null;
	status: string | null;
}

export interface DiagnosisWithMedications {
	diagnosis: Diagnosis;
	medication_ids: number[];
}

export interface Vital {
	id: number | null;
	patient_id: number;
	vital_type: string;
	value: number;
	value_secondary: number | null;
	unit: string;
	recorded_at: string;
}

export interface Lab {
	id: number | null;
	patient_id: number;
	name: string;
	value: number;
	unit: string | null;
	reference_range_low: number | null;
	reference_range_high: number | null;
	is_abnormal: boolean | null;
	recorded_at: string;
}

export interface ClinicalScore {
	id: number | null;
	patient_id: number;
	score_type: string;
	score: number;
	max_score: number | null;
	interpretation: string | null;
	recorded_at: string;
}

export interface Encounter {
	id: number | null;
	patient_id: number;
	encounter_date: string;
	encounter_type: string;
	chief_complaint: string | null;
	summary: string | null;
	note_content: string | null;
	provider: string | null;
	location: string | null;
}

export interface Allergy {
	id: number | null;
	patient_id: number;
	allergen: string;
	reaction: string | null;
	severity: string | null;
}

export interface Vaccination {
	id: number | null;
	patient_id: number;
	vaccine_name: string;
	date_given: string;
}

export interface SocialHistory {
	id: number | null;
	patient_id: number;
	category: string;
	detail: string;
	status: string | null;
}

export interface FamilyHistory {
	id: number | null;
	patient_id: number;
	relation: string;
	condition: string;
	age_at_onset: number | null;
}

export interface Todo {
	id: number | null;
	patient_id: number;
	diagnosis_id: number | null;
	description: string;
	due_date: string | null;
	priority: string | null;
	status: string | null;
}

export interface Goal {
	id: number | null;
	patient_id: number;
	description: string;
	target_date: string | null;
	status: string | null;
	progress: number | null;
}

export interface TimelineEvent {
	id: number | null;
	patient_id: number;
	event_type: string;
	description: string;
	event_date: string;
	icon: string | null;
	color: string | null;
}

export interface PatientFullData {
	patient: Patient;
	diagnoses: DiagnosisWithMedications[];
	medications: Medication[];
	vitals: Vital[];
	labs: Lab[];
	clinical_scores: ClinicalScore[];
	encounters: Encounter[];
	allergies: Allergy[];
	vaccinations: Vaccination[];
	social_history: SocialHistory[];
	family_history: FamilyHistory[];
	todos: Todo[];
	goals: Goal[];
	timeline_events: TimelineEvent[];
}
