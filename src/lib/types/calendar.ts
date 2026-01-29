// TypeScript interfaces for calendar data

export interface CalendarEvent {
	id: number;
	title: string;
	date: string; // ISO datetime string
	endDate?: string;
	type: EventType;
	patientId?: number;
	patientName?: string;
	encounterId?: number;
	appointmentId?: number;
	description?: string;
	location?: string;
	provider?: string;
	status?: EventStatus;
	color?: string;
	// Additional appointment details
	chiefComplaint?: string;
	aiContextSummary?: string; // AI-generated context about patient's recent care
	durationMinutes?: number;
	patientDob?: string;
	patientPhone?: string;
	lastVisitDate?: string;
	lastVisitReason?: string;
}

export type EventType =
	| 'encounter'
	| 'appointment'
	| 'follow-up'
	| 'telehealth'
	| 'consultation'
	| 'procedure'
	| 'meeting'
	| 'blocked'
	| 'other';

export type EventStatus =
	| 'scheduled'
	| 'confirmed'
	| 'in-progress'
	| 'completed'
	| 'cancelled'
	| 'no-show';

export type ViewMode = 'month' | 'week' | 'day';

// Event type configuration
export const EVENT_TYPE_CONFIG: Record<EventType, { label: string; color: string; icon: string }> = {
	encounter: { label: 'Encounter', color: 'bg-blue-500', icon: 'fa-user-doctor' },
	appointment: { label: 'Appointment', color: 'bg-green-500', icon: 'fa-calendar-check' },
	'follow-up': { label: 'Follow-up', color: 'bg-purple-500', icon: 'fa-rotate' },
	telehealth: { label: 'Telehealth', color: 'bg-cyan-500', icon: 'fa-video' },
	consultation: { label: 'Consultation', color: 'bg-orange-500', icon: 'fa-comments' },
	procedure: { label: 'Procedure', color: 'bg-red-500', icon: 'fa-syringe' },
	meeting: { label: 'Meeting', color: 'bg-indigo-500', icon: 'fa-users' },
	blocked: { label: 'Blocked', color: 'bg-gray-500', icon: 'fa-ban' },
	other: { label: 'Other', color: 'bg-gray-400', icon: 'fa-calendar' }
};

// Event status configuration
export const EVENT_STATUS_CONFIG: Record<EventStatus, { label: string; color: string }> = {
	scheduled: { label: 'Scheduled', color: 'text-blue-600' },
	confirmed: { label: 'Confirmed', color: 'text-green-600' },
	'in-progress': { label: 'In Progress', color: 'text-yellow-600' },
	completed: { label: 'Completed', color: 'text-gray-600' },
	cancelled: { label: 'Cancelled', color: 'text-red-600' },
	'no-show': { label: 'No Show', color: 'text-orange-600' }
};
