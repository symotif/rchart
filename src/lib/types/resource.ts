// Resource types for the resources page

export type ResourceExtensionType = 'drug-library' | 'openevidence' | 'uptodate' | 'external';

export interface ResourceExtension {
	id: string;
	name: string;
	displayName: string;
	description: string;
	icon: string;
	iconColor: string;
	type: ResourceExtensionType;
	url?: string; // For external/embedded resources
	isEnabled: boolean;
	requiresAuth: boolean;
	isAuthenticated?: boolean;
}

export interface ResourceTab {
	id: string;
	extensionId: string;
	title: string;
	isActive: boolean;
}

// Drug Library types
export interface DrugFormulation {
	form: string; // tablet, capsule, solution, injection, etc.
	strengths: string[];
	route: string; // oral, IV, IM, topical, etc.
}

export interface DrugDosing {
	indication: string;
	adultDose: string;
	pediatricDose?: string;
	renalAdjustment?: string;
	hepaticAdjustment?: string;
	maxDose?: string;
	frequency: string;
	notes?: string;
}

export interface DrugTapering {
	indication: string;
	protocol: string;
	duration: string;
	notes?: string;
}

export interface DrugPharmacokinetics {
	absorption?: string;
	distribution?: string;
	proteinBinding?: string;
	metabolism?: string;
	halfLife: string;
	elimination?: string;
	bioavailability?: string;
	onsetOfAction?: string;
	peakPlasma?: string;
	durationOfAction?: string;
}

export interface DrugPharmacodynamics {
	mechanismOfAction: string;
	primaryEffects?: string[];
	secondaryEffects?: string[];
	receptorBinding?: string[];
}

export interface DrugInteraction {
	drug: string;
	severity: 'major' | 'moderate' | 'minor';
	description: string;
	management?: string;
}

export interface DrugContraindication {
	condition: string;
	severity: 'absolute' | 'relative';
	description: string;
}

export interface DrugAdverseEffect {
	effect: string;
	frequency: 'common' | 'uncommon' | 'rare' | 'very-rare';
	description?: string;
}

export interface DrugBlackBoxWarning {
	title: string;
	description: string;
}

export interface Drug {
	id: string;
	genericName: string;
	brandNames: string[];
	drugClass: string;
	subClass?: string;
	controlledSubstance?: {
		schedule: 'I' | 'II' | 'III' | 'IV' | 'V';
		deaNumber?: string;
	};

	// FDA Information
	fdaApprovalDate?: string;
	fdaIndications: string[];
	offLabelUses?: string[];
	blackBoxWarnings?: DrugBlackBoxWarning[];

	// Formulations
	formulations: DrugFormulation[];

	// Dosing
	dosing: DrugDosing[];

	// Tapering protocols
	tapering?: DrugTapering[];

	// Pharmacology
	pharmacokinetics: DrugPharmacokinetics;
	pharmacodynamics: DrugPharmacodynamics;

	// Safety
	contraindications: DrugContraindication[];
	interactions: DrugInteraction[];
	adverseEffects: DrugAdverseEffect[];

	// Pregnancy/Lactation
	pregnancyCategory?: string;
	lactationSafety?: string;

	// Additional info
	monitoring?: string[];
	patientCounseling?: string[];
	storage?: string;

	// Metadata
	lastUpdated: string;
}

export interface DrugSearchResult {
	drug: Drug;
	matchType: 'generic' | 'brand' | 'class';
	matchedTerm: string;
}

export type DrugCategory =
	| 'all'
	| 'antidepressants'
	| 'antipsychotics'
	| 'anxiolytics'
	| 'mood-stabilizers'
	| 'stimulants'
	| 'hypnotics'
	| 'anticonvulsants'
	| 'analgesics'
	| 'antihypertensives'
	| 'antibiotics'
	| 'other';

export const DRUG_CATEGORY_INFO: Record<DrugCategory, { label: string; icon: string }> = {
	'all': { label: 'All Medications', icon: 'fa-pills' },
	'antidepressants': { label: 'Antidepressants', icon: 'fa-brain' },
	'antipsychotics': { label: 'Antipsychotics', icon: 'fa-shield-halved' },
	'anxiolytics': { label: 'Anxiolytics', icon: 'fa-cloud' },
	'mood-stabilizers': { label: 'Mood Stabilizers', icon: 'fa-scale-balanced' },
	'stimulants': { label: 'Stimulants', icon: 'fa-bolt' },
	'hypnotics': { label: 'Hypnotics/Sedatives', icon: 'fa-moon' },
	'anticonvulsants': { label: 'Anticonvulsants', icon: 'fa-wave-square' },
	'analgesics': { label: 'Analgesics', icon: 'fa-hand-dots' },
	'antihypertensives': { label: 'Antihypertensives', icon: 'fa-heart-pulse' },
	'antibiotics': { label: 'Antibiotics', icon: 'fa-bacterium' },
	'other': { label: 'Other', icon: 'fa-capsules' }
};
