// TypeScript interfaces for user/provider data

export interface User {
	id: number | null;
	username: string;
	password_hash: string;
	first_name: string;
	last_name: string;
	degree_type: string | null;
	specialty: string | null;
	subspecialty: string | null;
	npi_number: string | null;
	photo_url: string | null;
	bio: string | null;
}

export interface UserEducation {
	id: number | null;
	user_id: number;
	education_type: string;
	institution: string;
	degree: string | null;
	field_of_study: string | null;
	start_year: number | null;
	end_year: number | null;
}

export interface UserBadge {
	id: number | null;
	user_id: number;
	badge_name: string;
	badge_type: string;
	description: string | null;
	icon: string | null;
	color: string | null;
	awarded_date: string | null;
}

export interface UserSettings {
	id: number | null;
	user_id: number;
	language: string | null;
	notifications_enabled: boolean | null;
	email_notifications: boolean | null;
	sms_notifications: boolean | null;
	two_factor_enabled: boolean | null;
	zen_mode_default: boolean | null;
}

export interface UserFullData {
	user: User;
	education: UserEducation[];
	badges: UserBadge[];
	settings: UserSettings;
}

// Education type display names
export const EDUCATION_TYPE_NAMES: Record<string, string> = {
	'Medical School': 'Medical School',
	'Residency': 'Residency',
	'Fellowship': 'Fellowship',
	'Undergraduate': 'Undergraduate',
	'Graduate': 'Graduate School',
	'Other': 'Other Training'
};

// Badge type display info
export const BADGE_TYPE_INFO: Record<string, { label: string; bgClass: string }> = {
	certification: { label: 'Certification', bgClass: 'bg-blue-100 dark:bg-blue-900' },
	award: { label: 'Award', bgClass: 'bg-amber-100 dark:bg-amber-900' },
	honor: { label: 'Honor', bgClass: 'bg-green-100 dark:bg-green-900' },
	achievement: { label: 'Achievement', bgClass: 'bg-purple-100 dark:bg-purple-900' }
};

// Language options
export const LANGUAGE_OPTIONS = [
	{ code: 'en', name: 'English' },
	{ code: 'es', name: 'Espa\u00f1ol' },
	{ code: 'fr', name: 'Fran\u00e7ais' },
	{ code: 'de', name: 'Deutsch' },
	{ code: 'zh', name: '\u4e2d\u6587' },
	{ code: 'ja', name: '\u65e5\u672c\u8a9e' },
	{ code: 'ko', name: '\ud55c\uad6d\uc5b4' }
];

// User online status types
export type UserPresenceStatus = 'online' | 'busy' | 'away' | 'dnd' | 'invisible' | 'offline';

export interface UserStatus {
	status: UserPresenceStatus;
	statusMessage: string | null;
	expiresAt: Date | null; // null means indefinite
	isInvisible: boolean; // true means appear offline but actually online
}

// Status duration presets (in minutes, null = indefinite)
export type StatusDuration = 15 | 30 | 60 | 120 | 240 | 480 | null;

export const STATUS_DURATION_OPTIONS: { value: StatusDuration; label: string }[] = [
	{ value: 15, label: '15 minutes' },
	{ value: 30, label: '30 minutes' },
	{ value: 60, label: '1 hour' },
	{ value: 120, label: '2 hours' },
	{ value: 240, label: '4 hours' },
	{ value: 480, label: '8 hours' },
	{ value: null, label: 'Until I change it' }
];

// Status configuration for UI display
export const USER_STATUS_CONFIG: Record<
	UserPresenceStatus,
	{
		label: string;
		description: string;
		color: string;
		ringColor: string;
		bgColor: string;
		icon: string;
	}
> = {
	online: {
		label: 'Online',
		description: 'Available and ready to chat',
		color: 'text-green-500',
		ringColor: 'ring-green-500',
		bgColor: 'bg-green-500',
		icon: 'fa-circle'
	},
	busy: {
		label: 'Busy',
		description: 'Working, may respond slowly',
		color: 'text-yellow-500',
		ringColor: 'ring-yellow-500',
		bgColor: 'bg-yellow-500',
		icon: 'fa-clock'
	},
	away: {
		label: 'Away',
		description: 'Stepped away temporarily',
		color: 'text-gray-400',
		ringColor: 'ring-gray-400',
		bgColor: 'bg-gray-400',
		icon: 'fa-moon'
	},
	dnd: {
		label: 'Do Not Disturb',
		description: 'No notifications or interruptions',
		color: 'text-red-500',
		ringColor: 'ring-red-500',
		bgColor: 'bg-red-500',
		icon: 'fa-minus-circle'
	},
	invisible: {
		label: 'Invisible',
		description: 'Appear offline but stay connected',
		color: 'text-gray-400',
		ringColor: 'ring-gray-400',
		bgColor: 'bg-gray-400',
		icon: 'fa-eye-slash'
	},
	offline: {
		label: 'Offline',
		description: 'Not connected',
		color: 'text-gray-400',
		ringColor: 'ring-gray-400',
		bgColor: 'bg-gray-400',
		icon: 'fa-circle-xmark'
	}
};

// Default user status
export const DEFAULT_USER_STATUS: UserStatus = {
	status: 'online',
	statusMessage: null,
	expiresAt: null,
	isInvisible: false
};

// ==========================================
// Provider Task Types (Dashboard "My Tasks")
// ==========================================

export type TaskCategory = 'paperwork' | 'refills' | 'follow_up' | 'documentation' | 'labs' | 'prescriptions' | 'calls' | 'other';

export interface ProviderTask {
	id: string;
	title: string;
	category: TaskCategory;
	starred: boolean;
	pinned: boolean;
	completed: boolean;
	sortOrder: number;
	createdAt: string;
}

// Task category configuration with colors and icons
export const TASK_CATEGORY_CONFIG: Record<
	TaskCategory,
	{
		label: string;
		bgLight: string;
		bgDark: string;
		icon: string;
		iconColor: string;
		checkboxBg: string;
		checkboxBorder: string;
		checkboxHover: string;
	}
> = {
	paperwork: {
		label: 'Paperwork',
		bgLight: 'bg-blue-50',
		bgDark: 'dark:bg-blue-900/30',
		icon: 'fa-file-alt',
		iconColor: 'text-blue-500',
		checkboxBg: 'bg-blue-500 border-blue-500',
		checkboxBorder: 'border-blue-400',
		checkboxHover: 'hover:border-blue-500'
	},
	refills: {
		label: 'Refills',
		bgLight: 'bg-green-50',
		bgDark: 'dark:bg-green-900/30',
		icon: 'fa-prescription-bottle',
		iconColor: 'text-green-500',
		checkboxBg: 'bg-green-500 border-green-500',
		checkboxBorder: 'border-green-400',
		checkboxHover: 'hover:border-green-500'
	},
	follow_up: {
		label: 'Follow-up',
		bgLight: 'bg-yellow-50',
		bgDark: 'dark:bg-yellow-900/30',
		icon: 'fa-phone',
		iconColor: 'text-yellow-500',
		checkboxBg: 'bg-yellow-500 border-yellow-500',
		checkboxBorder: 'border-yellow-400',
		checkboxHover: 'hover:border-yellow-500'
	},
	documentation: {
		label: 'Documentation',
		bgLight: 'bg-purple-50',
		bgDark: 'dark:bg-purple-900/30',
		icon: 'fa-pen-to-square',
		iconColor: 'text-purple-500',
		checkboxBg: 'bg-purple-500 border-purple-500',
		checkboxBorder: 'border-purple-400',
		checkboxHover: 'hover:border-purple-500'
	},
	labs: {
		label: 'Labs',
		bgLight: 'bg-red-50',
		bgDark: 'dark:bg-red-900/30',
		icon: 'fa-vial',
		iconColor: 'text-red-500',
		checkboxBg: 'bg-red-500 border-red-500',
		checkboxBorder: 'border-red-400',
		checkboxHover: 'hover:border-red-500'
	},
	prescriptions: {
		label: 'Prescriptions',
		bgLight: 'bg-indigo-50',
		bgDark: 'dark:bg-indigo-900/30',
		icon: 'fa-prescription',
		iconColor: 'text-indigo-500',
		checkboxBg: 'bg-indigo-500 border-indigo-500',
		checkboxBorder: 'border-indigo-400',
		checkboxHover: 'hover:border-indigo-500'
	},
	calls: {
		label: 'Calls',
		bgLight: 'bg-teal-50',
		bgDark: 'dark:bg-teal-900/30',
		icon: 'fa-phone-volume',
		iconColor: 'text-teal-500',
		checkboxBg: 'bg-teal-500 border-teal-500',
		checkboxBorder: 'border-teal-400',
		checkboxHover: 'hover:border-teal-500'
	},
	other: {
		label: 'Other',
		bgLight: 'bg-gray-50',
		bgDark: 'dark:bg-gray-700/50',
		icon: 'fa-tasks',
		iconColor: 'text-gray-500',
		checkboxBg: 'bg-gray-500 border-gray-500',
		checkboxBorder: 'border-gray-400',
		checkboxHover: 'hover:border-gray-500'
	}
};
