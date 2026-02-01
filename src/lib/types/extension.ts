// Extension types for the marketplace

export type ExtensionCategory =
	| 'telehealth'
	| 'specialty'
	| 'workflow'
	| 'integration'
	| 'analytics'
	| 'documentation'
	| 'billing'
	| 'patient-engagement'
	| 'ai'
	| 'institutional';

export interface ExtensionAuthor {
	name: string;
	organization?: string;
	verified: boolean;
	avatarUrl?: string;
}

export interface Extension {
	id: string;
	name: string;
	displayName: string;
	description: string;
	longDescription?: string;
	version: string;
	author: ExtensionAuthor;
	category: ExtensionCategory;
	tags: string[];
	icon: string;
	iconColor: string;
	downloadCount: number;
	rating: number;
	ratingCount: number;
	isInstalled: boolean;
	isEnabled: boolean;
	isOfficial: boolean;
	isFeatured: boolean;
	lastUpdated: string;
	changelog?: string;
	screenshots?: string[];
	dependencies?: string[];
	permissions?: string[];
}

export interface ExtensionFilter {
	search: string;
	category: ExtensionCategory | 'all';
	showInstalled: boolean;
	sortBy: 'popular' | 'rating' | 'recent' | 'name';
}

export const CATEGORY_INFO: Record<ExtensionCategory, { label: string; icon: string; color: string }> = {
	telehealth: { label: 'Telehealth', icon: 'fa-video', color: 'text-blue-500' },
	specialty: { label: 'Specialty', icon: 'fa-stethoscope', color: 'text-purple-500' },
	workflow: { label: 'Workflow', icon: 'fa-diagram-project', color: 'text-green-500' },
	integration: { label: 'Integration', icon: 'fa-plug', color: 'text-orange-500' },
	analytics: { label: 'Analytics', icon: 'fa-chart-line', color: 'text-cyan-500' },
	documentation: { label: 'Documentation', icon: 'fa-file-medical', color: 'text-yellow-500' },
	billing: { label: 'Billing', icon: 'fa-credit-card', color: 'text-emerald-500' },
	'patient-engagement': { label: 'Patient Engagement', icon: 'fa-users', color: 'text-pink-500' },
	ai: { label: 'AI & ML', icon: 'fa-brain', color: 'text-indigo-500' },
	institutional: { label: 'Institutional', icon: 'fa-building', color: 'text-slate-500' }
};
