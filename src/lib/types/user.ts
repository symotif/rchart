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
