export interface PatientList {
	id: number | null;
	user_id: number;
	name: string;
	description: string | null;
	color: string | null;
	icon: string | null;
	is_default: boolean;
	sort_order: number;
}

export interface PatientListMember {
	id: number | null;
	list_id: number;
	patient_id: number;
	added_at: string | null;
	notes: string | null;
}

export interface PatientListColumn {
	id: number | null;
	list_id: number;
	column_key: string;
	column_label: string;
	column_type: string | null;
	is_visible: boolean;
	sort_order: number;
	width: number | null;
}

export interface PatientListWithPatients {
	list: PatientList;
	columns: PatientListColumn[];
	patients: import('./patient').Patient[];
}

// Available column options for creating new lists
export const AVAILABLE_COLUMNS: { key: string; label: string; type: string; defaultVisible: boolean }[] = [
	{ key: 'name', label: 'Name', type: 'text', defaultVisible: true },
	{ key: 'dob', label: 'DOB', type: 'date', defaultVisible: true },
	{ key: 'age', label: 'Age', type: 'computed', defaultVisible: false },
	{ key: 'sex', label: 'Sex', type: 'text', defaultVisible: true },
	{ key: 'gender', label: 'Gender', type: 'text', defaultVisible: false },
	{ key: 'phone', label: 'Phone', type: 'text', defaultVisible: true },
	{ key: 'email', label: 'Email', type: 'text', defaultVisible: false },
	{ key: 'address', label: 'Address', type: 'text', defaultVisible: false },
	{ key: 'last_visit', label: 'Last Visit', type: 'date', defaultVisible: true },
	{ key: 'next_appointment', label: 'Next Appt', type: 'date', defaultVisible: true },
	{ key: 'primary_diagnosis', label: 'Primary Dx', type: 'text', defaultVisible: true },
	{ key: 'insurance_provider', label: 'Insurance', type: 'text', defaultVisible: false },
	{ key: 'preferred_pharmacy', label: 'Pharmacy', type: 'text', defaultVisible: false },
	{ key: 'room', label: 'Room', type: 'text', defaultVisible: false },
	{ key: 'attending', label: 'Attending', type: 'text', defaultVisible: false },
	{ key: 'notes', label: 'Notes', type: 'text', defaultVisible: false },
];

// Color options for lists
export const LIST_COLORS = [
	{ value: '#3B82F6', label: 'Blue' },
	{ value: '#10B981', label: 'Green' },
	{ value: '#EF4444', label: 'Red' },
	{ value: '#F59E0B', label: 'Yellow' },
	{ value: '#8B5CF6', label: 'Purple' },
	{ value: '#EC4899', label: 'Pink' },
	{ value: '#06B6D4', label: 'Cyan' },
	{ value: '#6B7280', label: 'Gray' },
];

// Icon options for lists
export const LIST_ICONS = [
	{ value: 'fa-list', label: 'List' },
	{ value: 'fa-stethoscope', label: 'Stethoscope' },
	{ value: 'fa-hospital', label: 'Hospital' },
	{ value: 'fa-user-doctor', label: 'Doctor' },
	{ value: 'fa-brain', label: 'Brain' },
	{ value: 'fa-heart-pulse', label: 'Heart' },
	{ value: 'fa-hand-dots', label: 'Pain' },
	{ value: 'fa-syringe', label: 'Syringe' },
	{ value: 'fa-pills', label: 'Pills' },
	{ value: 'fa-users', label: 'Team' },
	{ value: 'fa-clipboard-list', label: 'Clipboard' },
	{ value: 'fa-star', label: 'Star' },
];
