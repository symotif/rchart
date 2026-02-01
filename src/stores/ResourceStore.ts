import { writable, derived, get } from 'svelte/store';
import type { ResourceExtension, ResourceTab, Drug, DrugCategory } from '$lib/types/resource';

// Available resource extensions
const RESOURCE_EXTENSIONS: ResourceExtension[] = [
	{
		id: 'drug-library',
		name: 'drug-library',
		displayName: 'Drug Library',
		description: 'Comprehensive medication reference with FDA indications, dosing, formulations, and pharmacology.',
		icon: 'fa-pills',
		iconColor: 'bg-emerald-500',
		type: 'drug-library',
		isEnabled: true,
		requiresAuth: false
	},
	{
		id: 'openevidence',
		name: 'openevidence',
		displayName: 'OpenEvidence',
		description: 'AI-powered clinical answers from peer-reviewed medical literature.',
		icon: 'fa-magnifying-glass-chart',
		iconColor: 'bg-blue-500',
		type: 'openevidence',
		url: 'https://www.openevidence.com',
		isEnabled: true,
		requiresAuth: true,
		isAuthenticated: false
	},
	{
		id: 'uptodate',
		name: 'uptodate',
		displayName: 'UpToDate',
		description: 'Evidence-based clinical decision support resource.',
		icon: 'fa-book-medical',
		iconColor: 'bg-orange-500',
		type: 'uptodate',
		url: 'https://www.uptodate.com',
		isEnabled: true,
		requiresAuth: true,
		isAuthenticated: false
	}
];

// Sample drug data for the drug library
const DEMO_DRUGS: Drug[] = [
	{
		id: 'sertraline',
		genericName: 'Sertraline',
		brandNames: ['Zoloft'],
		drugClass: 'Antidepressant',
		subClass: 'SSRI',
		fdaApprovalDate: '1991-12-30',
		fdaIndications: [
			'Major Depressive Disorder (MDD)',
			'Obsessive-Compulsive Disorder (OCD)',
			'Panic Disorder',
			'Post-Traumatic Stress Disorder (PTSD)',
			'Social Anxiety Disorder',
			'Premenstrual Dysphoric Disorder (PMDD)'
		],
		offLabelUses: ['Generalized Anxiety Disorder', 'Premature Ejaculation', 'Binge Eating Disorder'],
		blackBoxWarnings: [
			{
				title: 'Suicidality Risk',
				description: 'Antidepressants increased the risk of suicidal thinking and behavior in children, adolescents, and young adults in short-term studies. Monitor closely for clinical worsening and emergence of suicidal thoughts and behaviors.'
			}
		],
		formulations: [
			{ form: 'Tablet', strengths: ['25mg', '50mg', '100mg'], route: 'Oral' },
			{ form: 'Oral Solution', strengths: ['20mg/mL'], route: 'Oral' }
		],
		dosing: [
			{
				indication: 'Major Depressive Disorder',
				adultDose: '50mg once daily',
				pediatricDose: 'Not approved for pediatric MDD',
				maxDose: '200mg/day',
				frequency: 'Once daily',
				notes: 'May increase by 25-50mg at intervals of at least 1 week'
			},
			{
				indication: 'OCD (Adults)',
				adultDose: '50mg once daily',
				maxDose: '200mg/day',
				frequency: 'Once daily',
				notes: 'May increase by 50mg at intervals of at least 1 week'
			},
			{
				indication: 'OCD (Pediatric 6-12 years)',
				adultDose: 'N/A',
				pediatricDose: '25mg once daily',
				maxDose: '200mg/day',
				frequency: 'Once daily',
				notes: 'May increase by 25-50mg at intervals of at least 1 week'
			},
			{
				indication: 'Panic Disorder',
				adultDose: '25mg once daily initially, then 50mg daily',
				maxDose: '200mg/day',
				frequency: 'Once daily'
			},
			{
				indication: 'PTSD',
				adultDose: '25mg once daily initially, then 50mg daily',
				maxDose: '200mg/day',
				frequency: 'Once daily'
			}
		],
		tapering: [
			{
				indication: 'Discontinuation',
				protocol: 'Reduce dose by 25-50mg every 1-2 weeks',
				duration: '2-4 weeks minimum',
				notes: 'Watch for discontinuation syndrome: dizziness, nausea, headache, paresthesias, irritability'
			}
		],
		pharmacokinetics: {
			absorption: 'Well absorbed; food increases absorption and decreases time to peak',
			distribution: 'Highly protein bound (98%)',
			proteinBinding: '98%',
			metabolism: 'Hepatic via CYP3A4, CYP2C19, CYP2D6',
			halfLife: '26 hours (66 hours for active metabolite desmethylsertraline)',
			elimination: 'Renal (40-45%) and fecal (40-45%)',
			bioavailability: '44%',
			onsetOfAction: '1-4 weeks for therapeutic effect',
			peakPlasma: '4.5-8.4 hours',
			durationOfAction: '24 hours'
		},
		pharmacodynamics: {
			mechanismOfAction: 'Selective serotonin reuptake inhibitor (SSRI). Blocks presynaptic serotonin transporter (SERT), increasing serotonin concentrations in the synaptic cleft.',
			primaryEffects: ['Serotonin reuptake inhibition'],
			secondaryEffects: ['Weak dopamine reuptake inhibition', 'Sigma-1 receptor agonism'],
			receptorBinding: ['SERT (Ki = 0.29 nM)', 'DAT (Ki = 25 nM)', 'Sigma-1 receptor']
		},
		contraindications: [
			{
				condition: 'MAO inhibitor use within 14 days',
				severity: 'absolute',
				description: 'Risk of serotonin syndrome; do not use within 14 days of MAOI'
			},
			{
				condition: 'Pimozide use',
				severity: 'absolute',
				description: 'Increased pimozide levels and risk of QT prolongation'
			},
			{
				condition: 'Hypersensitivity to sertraline',
				severity: 'absolute',
				description: 'Prior allergic reaction to sertraline'
			}
		],
		interactions: [
			{
				drug: 'MAO Inhibitors',
				severity: 'major',
				description: 'Risk of serotonin syndrome',
				management: 'Contraindicated; allow 14-day washout'
			},
			{
				drug: 'Pimozide',
				severity: 'major',
				description: 'Increased pimozide levels, QT prolongation',
				management: 'Contraindicated'
			},
			{
				drug: 'Warfarin',
				severity: 'moderate',
				description: 'May increase INR and bleeding risk',
				management: 'Monitor INR closely'
			},
			{
				drug: 'NSAIDs/Aspirin',
				severity: 'moderate',
				description: 'Increased bleeding risk',
				management: 'Use with caution; consider gastroprotection'
			},
			{
				drug: 'Tramadol',
				severity: 'moderate',
				description: 'Increased risk of serotonin syndrome and seizures',
				management: 'Use with caution'
			}
		],
		adverseEffects: [
			{ effect: 'Nausea', frequency: 'common', description: 'Usually transient' },
			{ effect: 'Diarrhea', frequency: 'common' },
			{ effect: 'Headache', frequency: 'common' },
			{ effect: 'Insomnia', frequency: 'common' },
			{ effect: 'Dry mouth', frequency: 'common' },
			{ effect: 'Sexual dysfunction', frequency: 'common', description: 'Decreased libido, anorgasmia, delayed ejaculation' },
			{ effect: 'Dizziness', frequency: 'common' },
			{ effect: 'Fatigue', frequency: 'common' },
			{ effect: 'Hyponatremia', frequency: 'uncommon', description: 'Especially in elderly' },
			{ effect: 'Serotonin syndrome', frequency: 'rare', description: 'With serotonergic drugs' },
			{ effect: 'QT prolongation', frequency: 'rare' },
			{ effect: 'Bleeding', frequency: 'uncommon', description: 'GI or other bleeding' }
		],
		pregnancyCategory: 'C',
		lactationSafety: 'Limited data; considered relatively safe',
		monitoring: [
			'Suicidal ideation (especially early treatment and dose changes)',
			'Blood pressure',
			'Weight',
			'Sodium levels (especially in elderly)',
			'Signs of bleeding',
			'Sexual function'
		],
		patientCounseling: [
			'Take at the same time each day',
			'May take 2-4 weeks to see full effect',
			'Do not stop abruptly; taper under medical supervision',
			'Avoid alcohol',
			'Report suicidal thoughts immediately',
			'May cause drowsiness; use caution when driving'
		],
		storage: 'Store at room temperature 15-30°C (59-86°F)',
		lastUpdated: '2025-01-15'
	},
	{
		id: 'bupropion',
		genericName: 'Bupropion',
		brandNames: ['Wellbutrin', 'Wellbutrin SR', 'Wellbutrin XL', 'Zyban', 'Forfivo XL'],
		drugClass: 'Antidepressant',
		subClass: 'NDRI',
		fdaApprovalDate: '1985-12-30',
		fdaIndications: [
			'Major Depressive Disorder (MDD)',
			'Seasonal Affective Disorder (SAD)',
			'Smoking Cessation (Zyban)'
		],
		offLabelUses: ['ADHD', 'Sexual dysfunction (SSRI-induced)', 'Weight loss', 'Bipolar depression (adjunct)'],
		blackBoxWarnings: [
			{
				title: 'Suicidality Risk',
				description: 'Antidepressants increased the risk of suicidal thinking and behavior in children, adolescents, and young adults.'
			},
			{
				title: 'Neuropsychiatric Events (Zyban)',
				description: 'Serious neuropsychiatric events have been reported in patients taking bupropion for smoking cessation.'
			}
		],
		formulations: [
			{ form: 'Tablet IR', strengths: ['75mg', '100mg'], route: 'Oral' },
			{ form: 'Tablet SR', strengths: ['100mg', '150mg', '200mg'], route: 'Oral' },
			{ form: 'Tablet XL', strengths: ['150mg', '300mg', '450mg'], route: 'Oral' }
		],
		dosing: [
			{
				indication: 'MDD (IR)',
				adultDose: '100mg twice daily initially',
				maxDose: '450mg/day (150mg/dose max)',
				frequency: 'Two to three times daily',
				notes: 'Allow at least 6 hours between doses'
			},
			{
				indication: 'MDD (SR)',
				adultDose: '150mg once daily initially',
				maxDose: '400mg/day (200mg/dose max)',
				frequency: 'Twice daily',
				notes: 'Allow at least 8 hours between doses'
			},
			{
				indication: 'MDD (XL)',
				adultDose: '150mg once daily initially',
				maxDose: '450mg/day',
				frequency: 'Once daily in the morning',
				notes: 'May increase to 300mg after 4 days'
			},
			{
				indication: 'Smoking Cessation',
				adultDose: '150mg daily for 3 days, then 150mg twice daily',
				maxDose: '300mg/day',
				frequency: 'Twice daily',
				notes: 'Start 1-2 weeks before quit date; treat for 7-12 weeks'
			}
		],
		tapering: [
			{
				indication: 'Discontinuation',
				protocol: 'Generally can be stopped without taper due to mechanism',
				duration: '1-2 weeks if concerned',
				notes: 'Lower risk of discontinuation syndrome compared to SSRIs'
			}
		],
		pharmacokinetics: {
			absorption: 'Rapidly absorbed',
			distribution: 'Widely distributed',
			proteinBinding: '84%',
			metabolism: 'Hepatic via CYP2B6 to active metabolites (hydroxybupropion, threohydrobupropion, erythrohydrobupropion)',
			halfLife: '21 hours (hydroxybupropion: 20 hours)',
			elimination: 'Renal (87%) and fecal (10%)',
			bioavailability: '5-20% (high first-pass metabolism)',
			onsetOfAction: '1-4 weeks',
			peakPlasma: 'IR: 2 hours, SR: 3 hours, XL: 5 hours'
		},
		pharmacodynamics: {
			mechanismOfAction: 'Norepinephrine-dopamine reuptake inhibitor (NDRI). Inhibits neuronal uptake of norepinephrine and dopamine; does not inhibit MAO or serotonin reuptake.',
			primaryEffects: ['Norepinephrine reuptake inhibition', 'Dopamine reuptake inhibition'],
			secondaryEffects: ['Nicotinic acetylcholine receptor antagonism'],
			receptorBinding: ['NET', 'DAT', 'nAChR (antagonist)']
		},
		contraindications: [
			{
				condition: 'Seizure disorder',
				severity: 'absolute',
				description: 'Dose-dependent seizure risk'
			},
			{
				condition: 'Bulimia or anorexia nervosa',
				severity: 'absolute',
				description: 'Higher incidence of seizures'
			},
			{
				condition: 'Abrupt discontinuation of alcohol or sedatives',
				severity: 'absolute',
				description: 'Increases seizure risk'
			},
			{
				condition: 'MAO inhibitor use within 14 days',
				severity: 'absolute',
				description: 'Risk of hypertensive crisis'
			}
		],
		interactions: [
			{
				drug: 'MAO Inhibitors',
				severity: 'major',
				description: 'Risk of hypertensive reactions',
				management: 'Contraindicated; allow 14-day washout'
			},
			{
				drug: 'CYP2B6 inhibitors (e.g., ticlopidine)',
				severity: 'moderate',
				description: 'May increase bupropion levels',
				management: 'Monitor for toxicity'
			},
			{
				drug: 'CYP2D6 substrates',
				severity: 'moderate',
				description: 'Bupropion inhibits CYP2D6',
				management: 'May need dose reduction of CYP2D6 substrates'
			},
			{
				drug: 'Drugs that lower seizure threshold',
				severity: 'moderate',
				description: 'Additive seizure risk',
				management: 'Use with caution'
			}
		],
		adverseEffects: [
			{ effect: 'Insomnia', frequency: 'common' },
			{ effect: 'Dry mouth', frequency: 'common' },
			{ effect: 'Headache', frequency: 'common' },
			{ effect: 'Nausea', frequency: 'common' },
			{ effect: 'Agitation', frequency: 'common' },
			{ effect: 'Tachycardia', frequency: 'uncommon' },
			{ effect: 'Seizures', frequency: 'rare', description: 'Dose-dependent; 0.4% at 450mg/day' },
			{ effect: 'Psychosis', frequency: 'rare' }
		],
		pregnancyCategory: 'C',
		lactationSafety: 'Present in breast milk; use caution',
		monitoring: [
			'Blood pressure',
			'Suicidal ideation',
			'Seizure activity',
			'Weight',
			'Psychiatric symptoms'
		],
		patientCounseling: [
			'Do not crush or chew SR/XL formulations',
			'Take in the morning to avoid insomnia',
			'Do not exceed recommended doses (seizure risk)',
			'Avoid alcohol',
			'Report mood changes or suicidal thoughts'
		],
		storage: 'Store at room temperature 20-25°C (68-77°F)',
		lastUpdated: '2025-01-15'
	},
	{
		id: 'quetiapine',
		genericName: 'Quetiapine',
		brandNames: ['Seroquel', 'Seroquel XR'],
		drugClass: 'Antipsychotic',
		subClass: 'Second-generation (atypical)',
		fdaApprovalDate: '1997-09-26',
		fdaIndications: [
			'Schizophrenia',
			'Bipolar I Disorder (manic/mixed/depressive episodes)',
			'Bipolar II Disorder (depressive episodes)',
			'Major Depressive Disorder (adjunct)',
		],
		offLabelUses: ['Insomnia', 'Generalized Anxiety Disorder', 'PTSD', 'Delirium'],
		blackBoxWarnings: [
			{
				title: 'Increased Mortality in Elderly with Dementia-Related Psychosis',
				description: 'Elderly patients with dementia-related psychosis treated with antipsychotic drugs are at an increased risk of death. Quetiapine is not approved for this use.'
			},
			{
				title: 'Suicidality Risk',
				description: 'Increased risk of suicidal thinking and behavior in children, adolescents, and young adults taking antidepressants.'
			}
		],
		formulations: [
			{ form: 'Tablet IR', strengths: ['25mg', '50mg', '100mg', '200mg', '300mg', '400mg'], route: 'Oral' },
			{ form: 'Tablet XR', strengths: ['50mg', '150mg', '200mg', '300mg', '400mg'], route: 'Oral' }
		],
		dosing: [
			{
				indication: 'Schizophrenia (IR)',
				adultDose: '25mg twice daily, titrate to 300-400mg/day',
				maxDose: '800mg/day',
				frequency: 'Two to three times daily',
				notes: 'Titrate by 25-50mg twice daily'
			},
			{
				indication: 'Bipolar Mania (IR)',
				adultDose: '50mg twice daily on day 1, titrate to 400-800mg/day',
				maxDose: '800mg/day',
				frequency: 'Twice daily'
			},
			{
				indication: 'Bipolar Depression (XR)',
				adultDose: '50mg at bedtime on day 1, titrate to 300mg/day',
				maxDose: '300mg/day',
				frequency: 'Once daily at bedtime'
			},
			{
				indication: 'MDD Adjunct (XR)',
				adultDose: '50mg at bedtime on days 1-2, then 150mg',
				maxDose: '300mg/day',
				frequency: 'Once daily at bedtime'
			}
		],
		tapering: [
			{
				indication: 'Discontinuation',
				protocol: 'Reduce dose by 25-50% every 1-2 weeks',
				duration: '2-4 weeks',
				notes: 'Gradual taper to avoid withdrawal symptoms (nausea, insomnia, restlessness)'
			}
		],
		pharmacokinetics: {
			absorption: 'Rapidly absorbed; food increases Cmax by 25%',
			distribution: 'Widely distributed (Vd = 10 L/kg)',
			proteinBinding: '83%',
			metabolism: 'Hepatic via CYP3A4 to active metabolite norquetiapine',
			halfLife: '6 hours (IR), 7 hours (XR); norquetiapine: 12 hours',
			elimination: 'Renal (73%) and fecal (20%)',
			bioavailability: '100% (relative to oral solution)',
			onsetOfAction: 'Sedation immediate; antipsychotic effect 1-2 weeks',
			peakPlasma: 'IR: 1.5 hours, XR: 6 hours'
		},
		pharmacodynamics: {
			mechanismOfAction: 'Atypical antipsychotic with antagonist activity at serotonin 5-HT2A, dopamine D2, histamine H1, and adrenergic alpha-1 receptors. Norquetiapine has norepinephrine reuptake inhibition.',
			primaryEffects: ['D2 receptor antagonism', '5-HT2A receptor antagonism'],
			secondaryEffects: ['H1 antagonism (sedation)', 'Alpha-1 antagonism (orthostasis)', 'NET inhibition via norquetiapine'],
			receptorBinding: ['5-HT2A (Ki = 31 nM)', 'D2 (Ki = 160 nM)', 'H1 (Ki = 11 nM)', 'Alpha-1 (Ki = 8 nM)']
		},
		contraindications: [
			{
				condition: 'Hypersensitivity to quetiapine',
				severity: 'absolute',
				description: 'Prior allergic reaction'
			}
		],
		interactions: [
			{
				drug: 'CYP3A4 inhibitors (ketoconazole, etc.)',
				severity: 'major',
				description: 'Increased quetiapine levels',
				management: 'Reduce quetiapine dose to 1/6 of normal'
			},
			{
				drug: 'CYP3A4 inducers (phenytoin, carbamazepine)',
				severity: 'major',
				description: 'Decreased quetiapine levels',
				management: 'May need 5-fold increase in quetiapine dose'
			},
			{
				drug: 'CNS depressants',
				severity: 'moderate',
				description: 'Additive sedation',
				management: 'Use with caution'
			},
			{
				drug: 'Antihypertensives',
				severity: 'moderate',
				description: 'Additive hypotensive effects',
				management: 'Monitor blood pressure'
			}
		],
		adverseEffects: [
			{ effect: 'Somnolence', frequency: 'common', description: 'Very common, especially at initiation' },
			{ effect: 'Dry mouth', frequency: 'common' },
			{ effect: 'Dizziness', frequency: 'common' },
			{ effect: 'Weight gain', frequency: 'common' },
			{ effect: 'Orthostatic hypotension', frequency: 'common', description: 'Especially during titration' },
			{ effect: 'Constipation', frequency: 'common' },
			{ effect: 'Elevated triglycerides', frequency: 'common' },
			{ effect: 'Hyperglycemia', frequency: 'uncommon' },
			{ effect: 'Tardive dyskinesia', frequency: 'uncommon' },
			{ effect: 'QT prolongation', frequency: 'uncommon' },
			{ effect: 'Cataracts', frequency: 'rare' },
			{ effect: 'Neuroleptic malignant syndrome', frequency: 'rare' }
		],
		pregnancyCategory: 'C',
		lactationSafety: 'Present in breast milk; not recommended',
		monitoring: [
			'Fasting glucose and lipids at baseline, 12 weeks, then annually',
			'Weight and BMI at each visit',
			'Blood pressure',
			'Extrapyramidal symptoms',
			'Tardive dyskinesia (AIMS exam)',
			'Eye exam (cataracts) at baseline and every 6 months'
		],
		patientCounseling: [
			'Take XR formulation without food or with light meal',
			'Do not crush or chew XR tablets',
			'Rise slowly to prevent dizziness',
			'May cause drowsiness; avoid driving until effects known',
			'Report fever, muscle rigidity, or confusion immediately',
			'Avoid alcohol'
		],
		storage: 'Store at room temperature 25°C (77°F)',
		lastUpdated: '2025-01-15'
	},
	{
		id: 'lorazepam',
		genericName: 'Lorazepam',
		brandNames: ['Ativan'],
		drugClass: 'Anxiolytic',
		subClass: 'Benzodiazepine',
		controlledSubstance: { schedule: 'IV' },
		fdaApprovalDate: '1977-09-30',
		fdaIndications: [
			'Anxiety Disorders',
			'Anxiety associated with depressive symptoms',
			'Insomnia due to anxiety',
			'Status epilepticus (injection)',
			'Preoperative sedation (injection)'
		],
		offLabelUses: ['Alcohol withdrawal', 'Chemotherapy-induced nausea', 'Agitation', 'Catatonia'],
		blackBoxWarnings: [
			{
				title: 'Concomitant Use with Opioids',
				description: 'Concomitant use of benzodiazepines and opioids may result in profound sedation, respiratory depression, coma, and death.'
			},
			{
				title: 'Risks from Abuse, Misuse, and Addiction',
				description: 'Risks of abuse, misuse, addiction, and physical dependence. Assess risk prior to prescribing and monitor regularly.'
			}
		],
		formulations: [
			{ form: 'Tablet', strengths: ['0.5mg', '1mg', '2mg'], route: 'Oral' },
			{ form: 'Oral Solution', strengths: ['2mg/mL'], route: 'Oral' },
			{ form: 'Injection', strengths: ['2mg/mL', '4mg/mL'], route: 'IV/IM' }
		],
		dosing: [
			{
				indication: 'Anxiety (Oral)',
				adultDose: '2-3mg/day in divided doses',
				maxDose: '10mg/day',
				frequency: 'Two to three times daily',
				renalAdjustment: 'Use lower doses in renal impairment',
				hepaticAdjustment: 'Use lower doses in hepatic impairment',
				notes: 'Start with 1-2mg/day in elderly'
			},
			{
				indication: 'Insomnia (Oral)',
				adultDose: '2-4mg at bedtime',
				maxDose: '4mg/day',
				frequency: 'Once daily at bedtime'
			},
			{
				indication: 'Status Epilepticus (IV)',
				adultDose: '4mg IV over 2 minutes',
				pediatricDose: '0.05-0.1mg/kg IV (max 4mg/dose)',
				maxDose: '8mg total',
				frequency: 'May repeat once after 10-15 minutes'
			}
		],
		tapering: [
			{
				indication: 'Chronic use discontinuation',
				protocol: 'Reduce dose by 0.5mg every 1-2 weeks, or by 10-25% every 1-2 weeks',
				duration: '8-12 weeks for long-term use',
				notes: 'Slower taper for longer duration of use. Watch for rebound anxiety, insomnia, seizures.'
			}
		],
		pharmacokinetics: {
			absorption: 'Well absorbed orally; IM absorption complete',
			distribution: 'Vd = 1.3 L/kg',
			proteinBinding: '85%',
			metabolism: 'Hepatic glucuronidation (no CYP450 involvement)',
			halfLife: '10-20 hours',
			elimination: 'Renal (88%)',
			bioavailability: '90%',
			onsetOfAction: 'Oral: 20-30 minutes, IV: 1-5 minutes, IM: 15-30 minutes',
			peakPlasma: '2 hours (oral)',
			durationOfAction: '6-8 hours'
		},
		pharmacodynamics: {
			mechanismOfAction: 'Enhances the effect of GABA at the GABA-A receptor, increasing chloride ion conductance and resulting in neuronal hyperpolarization.',
			primaryEffects: ['Anxiolytic', 'Sedative', 'Anticonvulsant', 'Muscle relaxant', 'Amnestic'],
			receptorBinding: ['GABA-A receptor (positive allosteric modulator)']
		},
		contraindications: [
			{
				condition: 'Acute narrow-angle glaucoma',
				severity: 'absolute',
				description: 'May precipitate acute glaucoma'
			},
			{
				condition: 'Severe respiratory insufficiency',
				severity: 'absolute',
				description: 'Risk of respiratory depression'
			},
			{
				condition: 'Sleep apnea syndrome',
				severity: 'relative',
				description: 'May worsen apnea'
			}
		],
		interactions: [
			{
				drug: 'Opioids',
				severity: 'major',
				description: 'Risk of profound sedation, respiratory depression, death',
				management: 'Avoid combination if possible; use lowest doses if necessary'
			},
			{
				drug: 'CNS depressants (alcohol, sedatives)',
				severity: 'major',
				description: 'Additive CNS depression',
				management: 'Avoid combination'
			},
			{
				drug: 'Valproate',
				severity: 'moderate',
				description: 'Increased lorazepam concentrations',
				management: 'Reduce lorazepam dose by 50%'
			}
		],
		adverseEffects: [
			{ effect: 'Sedation', frequency: 'common' },
			{ effect: 'Dizziness', frequency: 'common' },
			{ effect: 'Weakness', frequency: 'common' },
			{ effect: 'Unsteadiness', frequency: 'common' },
			{ effect: 'Cognitive impairment', frequency: 'common' },
			{ effect: 'Paradoxical reactions', frequency: 'uncommon', description: 'Agitation, aggression in some patients' },
			{ effect: 'Respiratory depression', frequency: 'rare', description: 'Higher risk with IV use' },
			{ effect: 'Dependence', frequency: 'common', description: 'With prolonged use' }
		],
		pregnancyCategory: 'D',
		lactationSafety: 'Present in breast milk; not recommended',
		monitoring: [
			'Respiratory status (especially with IV use)',
			'Mental status',
			'Signs of abuse or dependence',
			'Effectiveness of therapy'
		],
		patientCounseling: [
			'Avoid alcohol and other CNS depressants',
			'Do not drive or operate machinery until effects known',
			'Do not stop abruptly after prolonged use',
			'Risk of dependence with prolonged use',
			'Store securely due to controlled substance status'
		],
		storage: 'Store at room temperature 20-25°C (68-77°F). Protect from light. Refrigerate injection.',
		lastUpdated: '2025-01-15'
	},
	{
		id: 'lithium',
		genericName: 'Lithium Carbonate / Lithium Citrate',
		brandNames: ['Lithobid', 'Eskalith', 'Lithium Citrate'],
		drugClass: 'Mood Stabilizer',
		subClass: 'Alkali metal ion',
		fdaApprovalDate: '1970-04-06',
		fdaIndications: [
			'Bipolar I Disorder (manic episodes)',
			'Bipolar I Disorder (maintenance)'
		],
		offLabelUses: ['Treatment-resistant depression (augmentation)', 'Cluster headaches', 'Neutropenia'],
		blackBoxWarnings: [
			{
				title: 'Lithium Toxicity',
				description: 'Lithium toxicity is closely related to serum lithium concentrations and can occur at doses close to therapeutic levels. Facilities for prompt and accurate serum lithium determinations should be available before initiating therapy.'
			}
		],
		formulations: [
			{ form: 'Capsule', strengths: ['150mg', '300mg', '600mg'], route: 'Oral' },
			{ form: 'Tablet', strengths: ['300mg'], route: 'Oral' },
			{ form: 'Extended-release Tablet', strengths: ['300mg', '450mg'], route: 'Oral' },
			{ form: 'Oral Solution (citrate)', strengths: ['8mEq/5mL (equivalent to 300mg carbonate)'], route: 'Oral' }
		],
		dosing: [
			{
				indication: 'Acute Mania',
				adultDose: '900-1800mg/day in divided doses',
				maxDose: '2400mg/day',
				frequency: 'Two to three times daily (IR), twice daily (ER)',
				notes: 'Target serum level 0.8-1.2 mEq/L for acute mania'
			},
			{
				indication: 'Maintenance',
				adultDose: '900-1200mg/day',
				maxDose: '2400mg/day',
				frequency: 'Two to three times daily (IR), twice daily (ER)',
				renalAdjustment: 'Reduce dose and monitor levels closely in renal impairment',
				notes: 'Target serum level 0.6-1.0 mEq/L for maintenance'
			}
		],
		tapering: [
			{
				indication: 'Discontinuation',
				protocol: 'Reduce dose gradually over 2-4 weeks',
				duration: '2-4 weeks minimum',
				notes: 'Abrupt discontinuation increases relapse risk. Some experts recommend even slower tapers (months).'
			}
		],
		pharmacokinetics: {
			absorption: 'Completely absorbed from GI tract',
			distribution: 'Distributes in total body water; crosses BBB',
			proteinBinding: 'Not protein bound',
			metabolism: 'Not metabolized',
			halfLife: '18-24 hours (longer in elderly)',
			elimination: 'Renal (95% unchanged)',
			bioavailability: '95-100%',
			onsetOfAction: '5-7 days for initial effect, 2-3 weeks for full effect',
			peakPlasma: '0.5-2 hours (IR), 4-6 hours (ER)'
		},
		pharmacodynamics: {
			mechanismOfAction: 'Mechanism not fully understood. Affects multiple neurotransmitter systems including serotonin and norepinephrine. Modulates intracellular signaling cascades (inositol phosphate, GSK-3, CREB).',
			primaryEffects: ['Mood stabilization', 'Antimanic', 'Neuroprotective'],
			secondaryEffects: ['Reduces suicide risk', 'May enhance neurogenesis']
		},
		contraindications: [
			{
				condition: 'Significant renal impairment',
				severity: 'relative',
				description: 'Lithium is renally eliminated; use with extreme caution'
			},
			{
				condition: 'Severe cardiovascular disease',
				severity: 'relative',
				description: 'May cause ECG changes'
			},
			{
				condition: 'Dehydration/sodium depletion',
				severity: 'relative',
				description: 'Increases lithium levels and toxicity risk'
			}
		],
		interactions: [
			{
				drug: 'NSAIDs',
				severity: 'major',
				description: 'Decrease lithium clearance by 20-60%',
				management: 'Monitor lithium levels; may need dose reduction'
			},
			{
				drug: 'ACE inhibitors / ARBs',
				severity: 'major',
				description: 'Decrease lithium clearance',
				management: 'Monitor lithium levels closely'
			},
			{
				drug: 'Thiazide diuretics',
				severity: 'major',
				description: 'Decrease lithium clearance',
				management: 'May need 50% lithium dose reduction; monitor levels'
			},
			{
				drug: 'Carbamazepine',
				severity: 'moderate',
				description: 'Additive neurotoxicity risk',
				management: 'Monitor for signs of toxicity'
			}
		],
		adverseEffects: [
			{ effect: 'Tremor', frequency: 'common', description: 'Fine tremor of hands' },
			{ effect: 'Polyuria/Polydipsia', frequency: 'common', description: 'Nephrogenic DI' },
			{ effect: 'Weight gain', frequency: 'common' },
			{ effect: 'Nausea/Diarrhea', frequency: 'common', description: 'Usually transient' },
			{ effect: 'Hypothyroidism', frequency: 'common', description: '20-30% of patients' },
			{ effect: 'Cognitive dulling', frequency: 'uncommon' },
			{ effect: 'Acne', frequency: 'uncommon' },
			{ effect: 'Psoriasis exacerbation', frequency: 'uncommon' },
			{ effect: 'Cardiac conduction abnormalities', frequency: 'uncommon' },
			{ effect: 'Chronic kidney disease', frequency: 'uncommon', description: 'With long-term use' },
			{ effect: 'Lithium toxicity', frequency: 'rare', description: 'Tremor, ataxia, confusion, seizures' }
		],
		pregnancyCategory: 'D',
		lactationSafety: 'Contraindicated; lithium freely enters breast milk',
		monitoring: [
			'Serum lithium levels: Weekly during initiation, then every 1-3 months',
			'Renal function (BUN, creatinine) every 2-3 months initially, then every 6-12 months',
			'Thyroid function (TSH) every 6 months',
			'ECG at baseline in patients >40 or with cardiac history',
			'Calcium and parathyroid function annually',
			'Weight',
			'Signs of toxicity: tremor, GI symptoms, ataxia, confusion'
		],
		patientCounseling: [
			'Maintain adequate fluid and sodium intake',
			'Take with food to reduce GI upset',
			'Consistent dosing times are important',
			'Signs of toxicity: severe tremor, vomiting, diarrhea, confusion, drowsiness',
			'Avoid NSAIDs unless approved by physician',
			'Inform all healthcare providers about lithium therapy',
			'Women of childbearing potential need reliable contraception'
		],
		storage: 'Store at room temperature 15-30°C (59-86°F)',
		lastUpdated: '2025-01-15'
	}
];

function createResourceStore() {
	const extensions = writable<ResourceExtension[]>(RESOURCE_EXTENSIONS);
	const tabs = writable<ResourceTab[]>([
		{ id: 'drug-library-tab', extensionId: 'drug-library', title: 'Drug Library', isActive: true }
	]);
	const activeTabId = writable<string>('drug-library-tab');

	// Drug library state
	const drugs = writable<Drug[]>(DEMO_DRUGS);
	const drugSearch = writable<string>('');
	const drugCategory = writable<DrugCategory>('all');
	const selectedDrugId = writable<string | null>(null);

	// Filtered drugs
	const filteredDrugs = derived([drugs, drugSearch, drugCategory], ([$drugs, $search, $category]) => {
		let result = [...$drugs];

		if ($search) {
			const searchLower = $search.toLowerCase();
			result = result.filter(
				(drug) =>
					drug.genericName.toLowerCase().includes(searchLower) ||
					drug.brandNames.some(b => b.toLowerCase().includes(searchLower)) ||
					drug.drugClass.toLowerCase().includes(searchLower) ||
					drug.subClass?.toLowerCase().includes(searchLower)
			);
		}

		if ($category !== 'all') {
			const categoryMap: Record<DrugCategory, string[]> = {
				'all': [],
				'antidepressants': ['Antidepressant'],
				'antipsychotics': ['Antipsychotic'],
				'anxiolytics': ['Anxiolytic'],
				'mood-stabilizers': ['Mood Stabilizer'],
				'stimulants': ['Stimulant'],
				'hypnotics': ['Hypnotic', 'Sedative'],
				'anticonvulsants': ['Anticonvulsant'],
				'analgesics': ['Analgesic', 'Opioid'],
				'antihypertensives': ['Antihypertensive'],
				'antibiotics': ['Antibiotic'],
				'other': []
			};

			const classesForCategory = categoryMap[$category];
			if (classesForCategory && classesForCategory.length > 0) {
				result = result.filter(drug =>
					classesForCategory.some(c => drug.drugClass.toLowerCase().includes(c.toLowerCase()))
				);
			}
		}

		return result.sort((a, b) => a.genericName.localeCompare(b.genericName));
	});

	const selectedDrug = derived([drugs, selectedDrugId], ([$drugs, $id]) => {
		if (!$id) return null;
		return $drugs.find(d => d.id === $id) || null;
	});

	return {
		// Subscriptions
		extensions: { subscribe: extensions.subscribe },
		tabs: { subscribe: tabs.subscribe },
		activeTabId: { subscribe: activeTabId.subscribe },
		drugs: { subscribe: drugs.subscribe },
		drugSearch: { subscribe: drugSearch.subscribe },
		drugCategory: { subscribe: drugCategory.subscribe },
		filteredDrugs,
		selectedDrug,
		selectedDrugId: { subscribe: selectedDrugId.subscribe },

		// Extension actions
		getExtension: (extensionId: string): ResourceExtension | undefined => {
			let found: ResourceExtension | undefined;
			extensions.subscribe(exts => {
				found = exts.find(e => e.id === extensionId);
			})();
			return found;
		},

		setExtensionAuth: (extensionId: string, isAuthenticated: boolean) => {
			extensions.update(exts =>
				exts.map(e => e.id === extensionId ? { ...e, isAuthenticated } : e)
			);
		},

		// Tab actions
		openTab: (extensionId: string) => {
			const ext = get(extensions).find(e => e.id === extensionId);
			if (!ext) return;

			const existingTab = get(tabs).find(t => t.extensionId === extensionId);
			if (existingTab) {
				activeTabId.set(existingTab.id);
				return;
			}

			const newTab: ResourceTab = {
				id: `${extensionId}-tab-${Date.now()}`,
				extensionId,
				title: ext.displayName,
				isActive: true
			};

			tabs.update(t => [...t.map(tab => ({ ...tab, isActive: false })), newTab]);
			activeTabId.set(newTab.id);
		},

		closeTab: (tabId: string) => {
			const currentTabs = get(tabs);
			const tabIndex = currentTabs.findIndex(t => t.id === tabId);
			const isActive = get(activeTabId) === tabId;

			tabs.update(t => t.filter(tab => tab.id !== tabId));

			if (isActive) {
				const remaining = currentTabs.filter(t => t.id !== tabId);
				if (remaining.length > 0) {
					const newActiveIndex = Math.max(0, tabIndex - 1);
					activeTabId.set(remaining[newActiveIndex].id);
				} else {
					activeTabId.set('');
				}
			}
		},

		setActiveTab: (tabId: string) => {
			activeTabId.set(tabId);
			tabs.update(t => t.map(tab => ({ ...tab, isActive: tab.id === tabId })));
		},

		getActiveExtension: (): ResourceExtension | null => {
			const tabId = get(activeTabId);
			const tab = get(tabs).find(t => t.id === tabId);
			if (!tab) return null;
			return get(extensions).find(e => e.id === tab.extensionId) || null;
		},

		// Drug library actions
		setDrugSearch: (search: string) => {
			drugSearch.set(search);
		},

		setDrugCategory: (category: DrugCategory) => {
			drugCategory.set(category);
		},

		selectDrug: (drugId: string | null) => {
			selectedDrugId.set(drugId);
		},

		getDrugById: (drugId: string): Drug | undefined => {
			return get(drugs).find(d => d.id === drugId);
		}
	};
}

export const ResourceStore = createResourceStore();
