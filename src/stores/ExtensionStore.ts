import { writable, derived } from 'svelte/store';
import type { Extension, ExtensionFilter, ExtensionCategory } from '$lib/types/extension';

// Demo extensions data
const DEMO_EXTENSIONS: Extension[] = [
	// Telehealth
	{
		id: 'telehealth-pro',
		name: 'telehealth-pro',
		displayName: 'Telehealth Pro',
		description: 'Complete telehealth solution with HD video, screen sharing, and virtual waiting room.',
		longDescription: 'Telehealth Pro provides a comprehensive virtual care platform integrated directly into your workflow. Features include HD video consultations, screen sharing for reviewing results, virtual waiting rooms, appointment scheduling, and automatic visit documentation.',
		version: '2.4.1',
		author: { name: 'MedConnect', organization: 'MedConnect Inc.', verified: true },
		category: 'telehealth',
		tags: ['video', 'virtual-care', 'telemedicine', 'remote'],
		icon: 'fa-video',
		iconColor: 'bg-blue-500',
		downloadCount: 45230,
		rating: 4.8,
		ratingCount: 1256,
		isInstalled: true,
		isEnabled: true,
		isOfficial: false,
		isFeatured: true,
		lastUpdated: '2025-01-15',
		permissions: ['camera', 'microphone', 'notifications']
	},
	{
		id: 'async-telehealth',
		name: 'async-telehealth',
		displayName: 'Async Care Messenger',
		description: 'Asynchronous telehealth with secure messaging, photo uploads, and AI triage.',
		version: '1.2.0',
		author: { name: 'HealthChat', verified: true },
		category: 'telehealth',
		tags: ['messaging', 'async', 'triage', 'photos'],
		icon: 'fa-comments-medical',
		iconColor: 'bg-teal-500',
		downloadCount: 12450,
		rating: 4.5,
		ratingCount: 342,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2025-01-10'
	},

	// Specialty - TMS
	{
		id: 'tms-suite',
		name: 'tms-suite',
		displayName: 'TMS Treatment Suite',
		description: 'Complete Transcranial Magnetic Stimulation management with treatment protocols, session tracking, and outcome measures.',
		longDescription: 'The TMS Treatment Suite is designed specifically for psychiatry practices offering TMS therapy. Track treatment sessions, manage protocols (including theta burst), monitor PHQ-9/GAD-7 scores over time, and generate insurance documentation automatically.',
		version: '3.1.0',
		author: { name: 'NeuroTech Solutions', organization: 'NeuroTech Solutions LLC', verified: true },
		category: 'specialty',
		tags: ['psychiatry', 'tms', 'neuromodulation', 'depression', 'treatment-resistant'],
		icon: 'fa-bolt',
		iconColor: 'bg-purple-500',
		downloadCount: 3420,
		rating: 4.9,
		ratingCount: 89,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: true,
		lastUpdated: '2025-01-18',
		permissions: ['patient-data', 'scheduling']
	},

	// Specialty - Cardiology
	{
		id: 'cardio-tools',
		name: 'cardio-tools',
		displayName: 'Cardiology Toolkit',
		description: 'ECG interpretation, cardiac risk calculators, and echo report integration.',
		version: '2.0.3',
		author: { name: 'HeartLogic', organization: 'HeartLogic Medical', verified: true },
		category: 'specialty',
		tags: ['cardiology', 'ecg', 'echo', 'risk-calculator'],
		icon: 'fa-heart-pulse',
		iconColor: 'bg-red-500',
		downloadCount: 8920,
		rating: 4.7,
		ratingCount: 234,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-12-20'
	},

	// Specialty - Dermatology
	{
		id: 'derm-imaging',
		name: 'derm-imaging',
		displayName: 'DermView Imaging',
		description: 'Dermatoscopy image capture, AI-assisted lesion analysis, and photo comparison over time.',
		version: '1.5.2',
		author: { name: 'SkinAI', verified: true },
		category: 'specialty',
		tags: ['dermatology', 'imaging', 'ai', 'lesions', 'melanoma'],
		icon: 'fa-microscope',
		iconColor: 'bg-amber-500',
		downloadCount: 5670,
		rating: 4.6,
		ratingCount: 178,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2025-01-05',
		permissions: ['camera', 'patient-data']
	},

	// Specialty - Pain Management
	{
		id: 'pain-tracker',
		name: 'pain-tracker',
		displayName: 'Pain Management Suite',
		description: 'Comprehensive pain assessment tools, opioid risk calculators, and treatment tracking.',
		version: '2.2.1',
		author: { name: 'PainCare Analytics', verified: true },
		category: 'specialty',
		tags: ['pain', 'opioids', 'pdmp', 'assessment', 'chronic-pain'],
		icon: 'fa-hand-dots',
		iconColor: 'bg-orange-600',
		downloadCount: 4350,
		rating: 4.4,
		ratingCount: 156,
		isInstalled: true,
		isEnabled: true,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-11-28'
	},

	// Specialty - Oncology
	{
		id: 'onco-protocols',
		name: 'onco-protocols',
		displayName: 'Oncology Protocol Manager',
		description: 'Chemotherapy protocols, regimen calculators, and toxicity grading tools.',
		version: '4.0.0',
		author: { name: 'OncoSystems', organization: 'OncoSystems Inc.', verified: true },
		category: 'specialty',
		tags: ['oncology', 'chemotherapy', 'protocols', 'toxicity', 'cancer'],
		icon: 'fa-ribbon',
		iconColor: 'bg-pink-500',
		downloadCount: 2890,
		rating: 4.8,
		ratingCount: 67,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2025-01-12'
	},

	// Workflow
	{
		id: 'smart-inbox',
		name: 'smart-inbox',
		displayName: 'Smart Inbox',
		description: 'AI-powered inbox management that prioritizes messages and suggests quick responses.',
		version: '1.8.0',
		author: { name: 'WorkflowAI', verified: true },
		category: 'workflow',
		tags: ['inbox', 'ai', 'productivity', 'messages'],
		icon: 'fa-inbox',
		iconColor: 'bg-green-500',
		downloadCount: 23400,
		rating: 4.3,
		ratingCount: 567,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: true,
		lastUpdated: '2025-01-08'
	},
	{
		id: 'task-automator',
		name: 'task-automator',
		displayName: 'Clinical Task Automator',
		description: 'Automate repetitive tasks with customizable rules and triggers.',
		version: '2.1.0',
		author: { name: 'AutoMed', verified: false },
		category: 'workflow',
		tags: ['automation', 'tasks', 'rules', 'triggers'],
		icon: 'fa-robot',
		iconColor: 'bg-slate-500',
		downloadCount: 8900,
		rating: 4.1,
		ratingCount: 234,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-12-15'
	},

	// Integration
	{
		id: 'epic-connect',
		name: 'epic-connect',
		displayName: 'Epic MyChart Connect',
		description: 'Bidirectional integration with Epic MyChart for seamless data exchange.',
		version: '3.2.1',
		author: { name: 'InteropHealth', organization: 'InteropHealth Systems', verified: true },
		category: 'integration',
		tags: ['epic', 'mychart', 'hl7', 'fhir', 'interoperability'],
		icon: 'fa-link',
		iconColor: 'bg-orange-500',
		downloadCount: 15600,
		rating: 4.6,
		ratingCount: 423,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2025-01-02'
	},
	{
		id: 'lab-connect',
		name: 'lab-connect',
		displayName: 'LabConnect Universal',
		description: 'Connect to 500+ lab systems for electronic ordering and results.',
		version: '5.0.2',
		author: { name: 'LabLink', verified: true },
		category: 'integration',
		tags: ['labs', 'orders', 'results', 'quest', 'labcorp'],
		icon: 'fa-flask',
		iconColor: 'bg-cyan-600',
		downloadCount: 34500,
		rating: 4.7,
		ratingCount: 892,
		isInstalled: true,
		isEnabled: true,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-12-28'
	},

	// AI
	{
		id: 'clinical-copilot',
		name: 'clinical-copilot',
		displayName: 'Clinical Copilot AI',
		description: 'AI-assisted clinical decision support with evidence-based recommendations.',
		longDescription: 'Clinical Copilot uses advanced AI to provide real-time clinical decision support. Get medication interaction alerts, diagnostic suggestions based on symptoms and labs, and evidence-based treatment recommendations with cited sources.',
		version: '2.0.0',
		author: { name: 'MedAI Labs', organization: 'MedAI Labs', verified: true },
		category: 'ai',
		tags: ['ai', 'clinical-decision-support', 'diagnosis', 'recommendations'],
		icon: 'fa-brain',
		iconColor: 'bg-indigo-500',
		downloadCount: 28900,
		rating: 4.5,
		ratingCount: 678,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: true,
		lastUpdated: '2025-01-20',
		permissions: ['patient-data', 'internet']
	},
	{
		id: 'scribe-ai',
		name: 'scribe-ai',
		displayName: 'AI Medical Scribe',
		description: 'Ambient AI documentation that listens to visits and generates notes automatically.',
		version: '1.9.5',
		author: { name: 'ScribeGenius', verified: true },
		category: 'ai',
		tags: ['ai', 'documentation', 'scribe', 'voice', 'ambient'],
		icon: 'fa-microphone',
		iconColor: 'bg-violet-500',
		downloadCount: 19200,
		rating: 4.4,
		ratingCount: 445,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: true,
		lastUpdated: '2025-01-14',
		permissions: ['microphone', 'patient-data']
	},

	// Documentation
	{
		id: 'smart-templates',
		name: 'smart-templates',
		displayName: 'Smart Templates Pro',
		description: 'Dynamic note templates with auto-population and smart phrases.',
		version: '3.5.0',
		author: { name: 'DocuMed', verified: true },
		category: 'documentation',
		tags: ['templates', 'notes', 'smart-phrases', 'documentation'],
		icon: 'fa-file-lines',
		iconColor: 'bg-yellow-500',
		downloadCount: 42100,
		rating: 4.6,
		ratingCount: 1123,
		isInstalled: true,
		isEnabled: true,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-12-10'
	},

	// Billing
	{
		id: 'code-assist',
		name: 'code-assist',
		displayName: 'Coding Assistant',
		description: 'AI-powered ICD-10 and CPT code suggestions based on documentation.',
		version: '2.3.0',
		author: { name: 'RevCycle Pro', verified: true },
		category: 'billing',
		tags: ['coding', 'icd-10', 'cpt', 'billing', 'revenue'],
		icon: 'fa-barcode',
		iconColor: 'bg-emerald-500',
		downloadCount: 18700,
		rating: 4.5,
		ratingCount: 389,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2025-01-06'
	},

	// Patient Engagement
	{
		id: 'patient-portal-plus',
		name: 'patient-portal-plus',
		displayName: 'Patient Portal Plus',
		description: 'Enhanced patient portal with appointment booking, bill pay, and health tracking.',
		version: '4.1.0',
		author: { name: 'EngageCare', organization: 'EngageCare Health', verified: true },
		category: 'patient-engagement',
		tags: ['portal', 'patient', 'booking', 'engagement'],
		icon: 'fa-mobile-screen',
		iconColor: 'bg-pink-500',
		downloadCount: 31200,
		rating: 4.4,
		ratingCount: 756,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-12-22'
	},
	{
		id: 'health-reminders',
		name: 'health-reminders',
		displayName: 'Preventive Care Reminders',
		description: 'Automated patient outreach for screenings, vaccinations, and wellness visits.',
		version: '1.4.2',
		author: { name: 'PreventiveHealth', verified: false },
		category: 'patient-engagement',
		tags: ['reminders', 'preventive', 'outreach', 'screenings'],
		icon: 'fa-bell',
		iconColor: 'bg-rose-500',
		downloadCount: 7800,
		rating: 4.2,
		ratingCount: 198,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-11-15'
	},

	// Analytics
	{
		id: 'quality-dashboard',
		name: 'quality-dashboard',
		displayName: 'Quality Metrics Dashboard',
		description: 'Track HEDIS, MIPS, and custom quality measures with real-time dashboards.',
		version: '2.8.0',
		author: { name: 'QualityFirst Analytics', organization: 'QualityFirst', verified: true },
		category: 'analytics',
		tags: ['quality', 'hedis', 'mips', 'metrics', 'dashboards'],
		icon: 'fa-gauge-high',
		iconColor: 'bg-cyan-500',
		downloadCount: 11300,
		rating: 4.7,
		ratingCount: 287,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-12-18'
	},

	// Institutional
	{
		id: 'mayo-clinic-bundle',
		name: 'mayo-clinic-bundle',
		displayName: 'Mayo Clinic Care Standards',
		description: 'Official Mayo Clinic extension with approved protocols, order sets, and documentation standards.',
		longDescription: 'This official Mayo Clinic extension brings evidence-based care standards directly into your workflow. Includes curated order sets, clinical pathways, documentation templates, and integration with Mayo Clinic Proceedings guidelines.',
		version: '5.2.0',
		author: { name: 'Mayo Clinic', organization: 'Mayo Clinic', verified: true, avatarUrl: '/mayo-logo.png' },
		category: 'institutional',
		tags: ['mayo', 'protocols', 'order-sets', 'institutional', 'guidelines'],
		icon: 'fa-hospital',
		iconColor: 'bg-blue-600',
		downloadCount: 8900,
		rating: 4.9,
		ratingCount: 234,
		isInstalled: false,
		isEnabled: false,
		isOfficial: true,
		isFeatured: true,
		lastUpdated: '2025-01-19'
	},
	{
		id: 'cleveland-clinic-pack',
		name: 'cleveland-clinic-pack',
		displayName: 'Cleveland Clinic Standards',
		description: 'Cleveland Clinic approved workflows, templates, and best practice alerts.',
		version: '3.1.0',
		author: { name: 'Cleveland Clinic', organization: 'Cleveland Clinic', verified: true },
		category: 'institutional',
		tags: ['cleveland', 'protocols', 'workflows', 'institutional'],
		icon: 'fa-building-columns',
		iconColor: 'bg-emerald-600',
		downloadCount: 6700,
		rating: 4.8,
		ratingCount: 178,
		isInstalled: false,
		isEnabled: false,
		isOfficial: true,
		isFeatured: false,
		lastUpdated: '2025-01-10'
	},
	{
		id: 'academic-research',
		name: 'academic-research',
		displayName: 'Academic Research Tools',
		description: 'Clinical trial management, research protocol tracking, and IRB documentation.',
		version: '2.0.1',
		author: { name: 'AcademicMed', organization: 'Academic Medical Systems', verified: true },
		category: 'institutional',
		tags: ['research', 'clinical-trials', 'irb', 'academic'],
		icon: 'fa-graduation-cap',
		iconColor: 'bg-slate-600',
		downloadCount: 3400,
		rating: 4.5,
		ratingCount: 89,
		isInstalled: false,
		isEnabled: false,
		isOfficial: false,
		isFeatured: false,
		lastUpdated: '2024-12-05'
	}
];

function createExtensionStore() {
	const extensions = writable<Extension[]>(DEMO_EXTENSIONS);
	const filter = writable<ExtensionFilter>({
		search: '',
		category: 'all',
		showInstalled: false,
		sortBy: 'popular'
	});

	const filteredExtensions = derived([extensions, filter], ([$extensions, $filter]) => {
		let result = [...$extensions];

		// Filter by search
		if ($filter.search) {
			const searchLower = $filter.search.toLowerCase();
			result = result.filter(
				(ext) =>
					ext.displayName.toLowerCase().includes(searchLower) ||
					ext.description.toLowerCase().includes(searchLower) ||
					ext.author.name.toLowerCase().includes(searchLower) ||
					ext.author.organization?.toLowerCase().includes(searchLower) ||
					ext.tags.some((tag) => tag.toLowerCase().includes(searchLower))
			);
		}

		// Filter by category
		if ($filter.category !== 'all') {
			result = result.filter((ext) => ext.category === $filter.category);
		}

		// Filter by installed
		if ($filter.showInstalled) {
			result = result.filter((ext) => ext.isInstalled);
		}

		// Sort
		switch ($filter.sortBy) {
			case 'popular':
				result.sort((a, b) => b.downloadCount - a.downloadCount);
				break;
			case 'rating':
				result.sort((a, b) => b.rating - a.rating);
				break;
			case 'recent':
				result.sort((a, b) => new Date(b.lastUpdated).getTime() - new Date(a.lastUpdated).getTime());
				break;
			case 'name':
				result.sort((a, b) => a.displayName.localeCompare(b.displayName));
				break;
		}

		return result;
	});

	const installedExtensions = derived(extensions, ($extensions) =>
		$extensions.filter((ext) => ext.isInstalled)
	);

	const featuredExtensions = derived(extensions, ($extensions) =>
		$extensions.filter((ext) => ext.isFeatured)
	);

	return {
		subscribe: extensions.subscribe,
		filter,
		filteredExtensions,
		installedExtensions,
		featuredExtensions,

		setSearch: (search: string) => {
			filter.update((f) => ({ ...f, search }));
		},

		setCategory: (category: ExtensionCategory | 'all') => {
			filter.update((f) => ({ ...f, category }));
		},

		setShowInstalled: (showInstalled: boolean) => {
			filter.update((f) => ({ ...f, showInstalled }));
		},

		setSortBy: (sortBy: ExtensionFilter['sortBy']) => {
			filter.update((f) => ({ ...f, sortBy }));
		},

		install: (extensionId: string) => {
			extensions.update((exts) =>
				exts.map((ext) =>
					ext.id === extensionId ? { ...ext, isInstalled: true, isEnabled: true } : ext
				)
			);
		},

		uninstall: (extensionId: string) => {
			extensions.update((exts) =>
				exts.map((ext) =>
					ext.id === extensionId ? { ...ext, isInstalled: false, isEnabled: false } : ext
				)
			);
		},

		toggleEnabled: (extensionId: string) => {
			extensions.update((exts) =>
				exts.map((ext) =>
					ext.id === extensionId ? { ...ext, isEnabled: !ext.isEnabled } : ext
				)
			);
		},

		getById: (extensionId: string): Extension | undefined => {
			let found: Extension | undefined;
			extensions.subscribe((exts) => {
				found = exts.find((ext) => ext.id === extensionId);
			})();
			return found;
		}
	};
}

export const ExtensionStore = createExtensionStore();
