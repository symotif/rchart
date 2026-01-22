<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Patient, Encounter } from '$lib/types/patient';

	type NoteData = {
		noteContent: string;
		chiefComplaint: string;
		noteType: string;
		cosigner: { name: string; initials: string } | null;
	};

	let {
		patient,
		encounter = null,
		patientId,
		saving = false,
		onSign = (_data: NoteData) => {},
		onSave = (_data: NoteData) => {}
	}: {
		patient: Patient | null;
		encounter: Encounter | null;
		patientId: number;
		saving?: boolean;
		onSign?: (data: NoteData) => void;
		onSave?: (data: NoteData) => void;
	} = $props();

	// Note content state
	let noteContent = $state(encounter?.note_content || '');
	let chiefComplaint = $state(encounter?.chief_complaint || '');
	let noteType = $state(encounter?.encounter_type || 'Progress Note');
	let selectedTemplate = $state('');
	let cosigner = $state<{ name: string; initials: string } | null>(null);

	// Rich text editor state
	let editorRef = $state<HTMLDivElement | null>(null);
	let currentFont = $state('Arial');
	let currentFontSize = $state('14');
	let currentColor = $state('#000000');

	// Note types
	const noteTypes = [
		'Progress Note',
		'Clinic Visit',
		'Procedure Note',
		'Discharge Summary',
		'H&P',
		'Consult Note',
		'Operative Note',
		'Follow-up Note'
	];

	// Note templates
	const templates = [
		{ id: 'soap', name: 'SOAP Note', content: `SUBJECTIVE:\n\n\nOBJECTIVE:\n\n\nASSESSMENT:\n\n\nPLAN:\n\n` },
		{ id: 'hp', name: 'H&P Template', content: `CHIEF COMPLAINT:\n\n\nHISTORY OF PRESENT ILLNESS:\n\n\nPAST MEDICAL HISTORY:\n\n\nMEDICATIONS:\n\n\nALLERGIES:\n\n\nSOCIAL HISTORY:\n\n\nFAMILY HISTORY:\n\n\nREVIEW OF SYSTEMS:\n\n\nPHYSICAL EXAMINATION:\n\n\nASSESSMENT AND PLAN:\n\n` },
		{ id: 'procedure', name: 'Procedure Note', content: `PROCEDURE:\n\n\nINDICATION:\n\n\nCONSENT:\n\n\nANESTHESIA:\n\n\nFINDINGS:\n\n\nSPECIMENS:\n\n\nCOMPLICATIONS:\n\n\nDISPOSITION:\n\n` },
		{ id: 'discharge', name: 'Discharge Summary', content: `ADMISSION DATE:\n\n\nDISCHARGE DATE:\n\n\nADMITTING DIAGNOSIS:\n\n\nDISCHARGE DIAGNOSIS:\n\n\nHOSPITAL COURSE:\n\n\nDISCHARGE MEDICATIONS:\n\n\nFOLLOW-UP:\n\n\nDISCHARGE INSTRUCTIONS:\n\n` },
		{ id: 'followup', name: 'Follow-up Note', content: `FOLLOW-UP FOR:\n\n\nINTERVAL HISTORY:\n\n\nCURRENT MEDICATIONS:\n\n\nEXAMINATION:\n\n\nASSESSMENT:\n\n\nPLAN:\n\n\nRETURN:\n\n` }
	];

	// Sample cosigners
	const availableCosigners = [
		{ name: 'Dr. Sarah Chen', initials: 'SC' },
		{ name: 'Dr. Michael Ross', initials: 'MR' },
		{ name: 'Dr. Emily Watson', initials: 'EW' },
		{ name: 'Dr. James Park', initials: 'JP' }
	];

	let showCosignerDropdown = $state(false);

	// Fonts available
	const fonts = ['Arial', 'Times New Roman', 'Georgia', 'Courier New', 'Verdana', 'Helvetica'];
	const fontSizes = ['10', '12', '14', '16', '18', '20', '24', '28', '32'];

	function execCommand(command: string, value: string | null = null) {
		document.execCommand(command, false, value);
		editorRef?.focus();
	}

	function applyTemplate() {
		const template = templates.find((t) => t.id === selectedTemplate);
		if (template && editorRef) {
			editorRef.innerHTML = template.content.replace(/\n/g, '<br>');
		}
	}

	function handlePrint() {
		window.print();
	}

	function handleBillingView() {
		// Placeholder for billing view functionality
		alert('Billing view will be implemented');
	}

	function handleAIScribe() {
		// Placeholder for AI scribe functionality
		alert('AI Scribe feature coming soon');
	}

	function selectCosigner(signer: { name: string; initials: string }) {
		cosigner = signer;
		showCosignerDropdown = false;
	}

	function removeCosigner() {
		cosigner = null;
	}

	function getNoteData(): NoteData {
		return {
			noteContent: editorRef?.innerHTML || '',
			chiefComplaint,
			noteType,
			cosigner
		};
	}

	function handleSign() {
		onSign(getNoteData());
	}

	function handleSave() {
		onSave(getNoteData());
	}

	function goBackToPatient() {
		goto(`/patient/${patientId}`);
	}

	// Initialize editor content when component mounts
	$effect(() => {
		if (editorRef && encounter?.note_content) {
			editorRef.innerHTML = encounter.note_content;
		}
	});
</script>

<div class="flex flex-col h-full bg-white dark:bg-gray-800 rounded-lg shadow-lg overflow-hidden">
	<!-- Header -->
	<div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
		<div class="flex items-center gap-4">
			<button
				class="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors"
				onclick={goBackToPatient}
			>
				<i class="fa-solid fa-arrow-left mr-2"></i>
				Back
			</button>
			{#if patient}
				<div>
					<h1 class="text-xl font-bold text-gray-800 dark:text-gray-100">
						{patient.first_name} {patient.last_name}
					</h1>
					<p class="text-sm text-gray-500 dark:text-gray-400">
						{encounter ? 'Edit Note' : 'New Note'}
					</p>
				</div>
			{/if}
		</div>

		<!-- Note Type Selector -->
		<div class="flex items-center gap-3">
			<label class="text-sm text-gray-600 dark:text-gray-400">Note Type:</label>
			<select
				bind:value={noteType}
				class="px-3 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-sm text-gray-800 dark:text-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
			>
				{#each noteTypes as type}
					<option value={type}>{type}</option>
				{/each}
			</select>
		</div>
	</div>

	<!-- Chief Complaint -->
	<div class="p-4 border-b border-gray-200 dark:border-gray-700">
		<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
			Chief Complaint
		</label>
		<input
			type="text"
			bind:value={chiefComplaint}
			placeholder="Enter chief complaint..."
			class="w-full px-3 py-2 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg text-gray-800 dark:text-gray-200 focus:outline-none focus:ring-2 focus:ring-yellow-500"
		/>
	</div>

	<!-- Toolbar -->
	<div class="flex flex-wrap items-center gap-2 p-3 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
		<!-- AI Scribe Button -->
		<button
			onclick={handleAIScribe}
			class="flex items-center gap-2 px-3 py-1.5 bg-purple-500 text-white rounded-lg hover:bg-purple-600 transition-colors text-sm"
		>
			<i class="fa-solid fa-microphone"></i>
			AI Scribe
		</button>

		<!-- Template Selector -->
		<div class="flex items-center gap-2">
			<select
				bind:value={selectedTemplate}
				class="px-2 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded text-sm text-gray-800 dark:text-gray-200"
			>
				<option value="">Select Template...</option>
				{#each templates as template}
					<option value={template.id}>{template.name}</option>
				{/each}
			</select>
			<button
				onclick={applyTemplate}
				disabled={!selectedTemplate}
				class="px-2 py-1.5 bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-200 rounded text-sm hover:bg-gray-300 dark:hover:bg-gray-500 disabled:opacity-50 disabled:cursor-not-allowed"
			>
				Apply
			</button>
		</div>

		<div class="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1"></div>

		<!-- Font Family -->
		<select
			bind:value={currentFont}
			onchange={() => execCommand('fontName', currentFont)}
			class="px-2 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded text-sm text-gray-800 dark:text-gray-200"
		>
			{#each fonts as font}
				<option value={font}>{font}</option>
			{/each}
		</select>

		<!-- Font Size -->
		<select
			bind:value={currentFontSize}
			onchange={() => execCommand('fontSize', currentFontSize)}
			class="px-2 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded text-sm text-gray-800 dark:text-gray-200 w-16"
		>
			{#each fontSizes as size}
				<option value={size}>{size}</option>
			{/each}
		</select>

		<div class="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1"></div>

		<!-- Text Formatting -->
		<button
			onclick={() => execCommand('bold')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Bold"
		>
			<i class="fa-solid fa-bold text-gray-700 dark:text-gray-300"></i>
		</button>
		<button
			onclick={() => execCommand('italic')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Italic"
		>
			<i class="fa-solid fa-italic text-gray-700 dark:text-gray-300"></i>
		</button>
		<button
			onclick={() => execCommand('underline')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Underline"
		>
			<i class="fa-solid fa-underline text-gray-700 dark:text-gray-300"></i>
		</button>
		<button
			onclick={() => execCommand('strikeThrough')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Strikethrough"
		>
			<i class="fa-solid fa-strikethrough text-gray-700 dark:text-gray-300"></i>
		</button>

		<div class="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1"></div>

		<!-- Text Color -->
		<div class="flex items-center gap-1">
			<label class="text-xs text-gray-500 dark:text-gray-400">Color:</label>
			<input
				type="color"
				bind:value={currentColor}
				onchange={() => execCommand('foreColor', currentColor)}
				class="w-8 h-8 rounded cursor-pointer border border-gray-300 dark:border-gray-600"
			/>
		</div>

		<!-- Highlight -->
		<button
			onclick={() => execCommand('hiliteColor', '#ffff00')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Highlight"
		>
			<i class="fa-solid fa-highlighter text-yellow-500"></i>
		</button>

		<div class="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1"></div>

		<!-- Lists -->
		<button
			onclick={() => execCommand('insertUnorderedList')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Bullet List"
		>
			<i class="fa-solid fa-list-ul text-gray-700 dark:text-gray-300"></i>
		</button>
		<button
			onclick={() => execCommand('insertOrderedList')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Numbered List"
		>
			<i class="fa-solid fa-list-ol text-gray-700 dark:text-gray-300"></i>
		</button>

		<div class="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1"></div>

		<!-- Alignment -->
		<button
			onclick={() => execCommand('justifyLeft')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Align Left"
		>
			<i class="fa-solid fa-align-left text-gray-700 dark:text-gray-300"></i>
		</button>
		<button
			onclick={() => execCommand('justifyCenter')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Align Center"
		>
			<i class="fa-solid fa-align-center text-gray-700 dark:text-gray-300"></i>
		</button>
		<button
			onclick={() => execCommand('justifyRight')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Align Right"
		>
			<i class="fa-solid fa-align-right text-gray-700 dark:text-gray-300"></i>
		</button>

		<div class="w-px h-6 bg-gray-300 dark:bg-gray-600 mx-1"></div>

		<!-- Undo/Redo -->
		<button
			onclick={() => execCommand('undo')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Undo"
		>
			<i class="fa-solid fa-rotate-left text-gray-700 dark:text-gray-300"></i>
		</button>
		<button
			onclick={() => execCommand('redo')}
			class="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors"
			title="Redo"
		>
			<i class="fa-solid fa-rotate-right text-gray-700 dark:text-gray-300"></i>
		</button>
	</div>

	<!-- Editor Area -->
	<div class="flex-1 p-4 overflow-auto">
		<div
			bind:this={editorRef}
			contenteditable="true"
			class="min-h-full p-4 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800 dark:text-gray-200"
			style="font-family: {currentFont}; font-size: {currentFontSize}px;"
		></div>
	</div>

	<!-- Footer Actions -->
	<div class="flex items-center justify-between p-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
		<!-- Cosigner Section -->
		<div class="flex items-center gap-3">
			<span class="text-sm text-gray-600 dark:text-gray-400">Cosigner:</span>
			{#if cosigner}
				<div class="flex items-center gap-2">
					<div class="w-8 h-8 rounded-full bg-blue-500 text-white flex items-center justify-center text-sm font-medium">
						{cosigner.initials}
					</div>
					<span class="text-sm text-gray-700 dark:text-gray-300">{cosigner.name}</span>
					<button
						onclick={removeCosigner}
						class="text-gray-400 hover:text-red-500 transition-colors"
						title="Remove cosigner"
						aria-label="Remove cosigner"
					>
						<i class="fa-solid fa-times"></i>
					</button>
				</div>
			{:else}
				<div class="relative">
					<button
						onclick={() => (showCosignerDropdown = !showCosignerDropdown)}
						class="px-3 py-1.5 bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded-lg text-sm hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
					>
						<i class="fa-solid fa-plus mr-1"></i>
						Add Cosigner
					</button>
					{#if showCosignerDropdown}
						<div class="absolute bottom-full left-0 mb-2 w-48 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg z-10">
							{#each availableCosigners as signer}
								<button
									onclick={() => selectCosigner(signer)}
									class="w-full px-3 py-2 text-left text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors flex items-center gap-2"
								>
									<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-xs font-medium">
										{signer.initials}
									</div>
									{signer.name}
								</button>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Action Buttons -->
		<div class="flex items-center gap-3">
			<button
				onclick={handlePrint}
				disabled={saving}
				class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			>
				<i class="fa-solid fa-print mr-2"></i>
				Print
			</button>
			<button
				onclick={handleBillingView}
				disabled={saving}
				class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			>
				<i class="fa-solid fa-file-invoice-dollar mr-2"></i>
				Billing View
			</button>
			<button
				onclick={handleSave}
				disabled={saving}
				class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			>
				{#if saving}
					<i class="fa-solid fa-spinner fa-spin mr-2"></i>
					Saving...
				{:else}
					<i class="fa-solid fa-save mr-2"></i>
					Save Draft
				{/if}
			</button>
			<button
				onclick={handleSign}
				disabled={saving}
				class="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			>
				{#if saving}
					<i class="fa-solid fa-spinner fa-spin mr-2"></i>
					Signing...
				{:else}
					<i class="fa-solid fa-signature mr-2"></i>
					Sign Note
				{/if}
			</button>
		</div>
	</div>
</div>
