<script lang="ts">
	import { goto } from '$app/navigation';
	import { addTab } from '../../../stores/TabStore';
	import type { CalendarEvent, EventType, EventStatus } from '$lib/types/calendar';
	import { EVENT_TYPE_CONFIG, EVENT_STATUS_CONFIG } from '$lib/types/calendar';

	interface Props {
		event: CalendarEvent | null;
		isOpen: boolean;
		isCreating?: boolean;
		selectedDate?: Date;
		onClose: () => void;
		onSave: (event: CalendarEvent) => void;
		onDelete: (eventId: number) => void;
	}

	let {
		event,
		isOpen,
		isCreating = false,
		selectedDate = new Date(),
		onClose,
		onSave,
		onDelete
	}: Props = $props();

	let isEditing = $state(false);

	// Form state
	let formData = $state<Partial<CalendarEvent>>({});

	$effect(() => {
		if (event) {
			formData = { ...event };
			isEditing = false;
		} else if (isCreating) {
			const dateStr = selectedDate.toISOString().slice(0, 16);
			formData = {
				title: '',
				date: dateStr,
				type: 'encounter',
				status: 'scheduled',
				patientName: '',
				description: '',
				location: ''
			};
			isEditing = true;
		}
	});

	function handleSave() {
		if (!formData.title && !formData.patientName) {
			return;
		}
		onSave(formData as CalendarEvent);
		onClose();
	}

	function handleDelete() {
		if (event?.id && confirm('Are you sure you want to delete this event?')) {
			onDelete(event.id);
			onClose();
		}
	}

	function handleStartEncounter() {
		if (event?.patientId) {
			const tab = {
				id: `note-new-${event.patientId}`,
				title: `New Note - ${event.patientName}`,
				path: `/patient/${event.patientId}/note/new`
			};
			addTab(tab);
			goto(tab.path);
			onClose();
		}
	}

	function handleViewPatient() {
		if (event?.patientId) {
			const tab = {
				id: `patient-${event.patientId}`,
				title: event.patientName || 'Patient',
				path: `/patient/${event.patientId}`
			};
			addTab(tab);
			goto(tab.path);
			onClose();
		}
	}

	function formatDateTime(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleString('en-US', {
			weekday: 'long',
			month: 'long',
			day: 'numeric',
			year: 'numeric',
			hour: 'numeric',
			minute: '2-digit',
			hour12: true
		});
	}

	function formatInputDateTime(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toISOString().slice(0, 16);
	}
</script>

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
		onclick={onClose}
		role="dialog"
		aria-modal="true"
	>
		<!-- Modal -->
		<div
			class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-full max-w-md overflow-hidden"
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div class="flex items-center justify-between px-5 py-4 border-b border-gray-200 dark:border-gray-700">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
					{isCreating ? 'New Event' : isEditing ? 'Edit Event' : 'Event Details'}
				</h2>
				<button
					onclick={onClose}
					class="p-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-full transition-colors"
				>
					<svg class="w-5 h-5 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Content -->
			<div class="p-5 space-y-4 max-h-[60vh] overflow-y-auto">
				{#if isEditing}
					<!-- Edit/Create Form -->
					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Patient Name
						</label>
						<input
							type="text"
							bind:value={formData.patientName}
							placeholder="Enter patient name"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Title/Reason
						</label>
						<input
							type="text"
							bind:value={formData.title}
							placeholder="e.g., Follow-up visit"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Date & Time
						</label>
						<input
							type="datetime-local"
							value={formData.date ? formatInputDateTime(formData.date) : ''}
							onchange={(e) => formData.date = (e.target as HTMLInputElement).value}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>

					<div class="grid grid-cols-2 gap-4">
						<div>
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								Type
							</label>
							<select
								bind:value={formData.type}
								class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
							>
								{#each Object.entries(EVENT_TYPE_CONFIG) as [key, config]}
									<option value={key}>{config.label}</option>
								{/each}
							</select>
						</div>

						<div>
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								Status
							</label>
							<select
								bind:value={formData.status}
								class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
							>
								{#each Object.entries(EVENT_STATUS_CONFIG) as [key, config]}
									<option value={key}>{config.label}</option>
								{/each}
							</select>
						</div>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Location
						</label>
						<input
							type="text"
							bind:value={formData.location}
							placeholder="e.g., Room 101"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							Notes
						</label>
						<textarea
							bind:value={formData.description}
							rows={3}
							placeholder="Additional notes..."
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
						></textarea>
					</div>
				{:else if event}
					<!-- View Mode -->
					<div class="space-y-4">
						<!-- Event Type Badge -->
						{#if event.type}
							<div class="flex items-center justify-between">
								<div>
									<span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium text-white {EVENT_TYPE_CONFIG[event.type]?.color || 'bg-gray-500'}">
										<i class="fa-solid {EVENT_TYPE_CONFIG[event.type]?.icon || 'fa-calendar'}"></i>
										{EVENT_TYPE_CONFIG[event.type]?.label || event.type}
									</span>
									{#if event.status}
										<span class="ml-2 text-sm {EVENT_STATUS_CONFIG[event.status]?.color || 'text-gray-600'}">
											{EVENT_STATUS_CONFIG[event.status]?.label || event.status}
										</span>
									{/if}
								</div>
								{#if event.durationMinutes}
									<span class="text-sm text-gray-500 dark:text-gray-400">
										{event.durationMinutes} min
									</span>
								{/if}
							</div>
						{/if}

						<!-- Patient/Title -->
						<div>
							<h3 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
								{event.patientName || event.title}
							</h3>
							{#if event.patientDob}
								<p class="text-sm text-gray-500 dark:text-gray-400">
									DOB: {event.patientDob}
									{#if event.patientPhone}
										<span class="ml-2">| {event.patientPhone}</span>
									{/if}
								</p>
							{/if}
							{#if event.patientName && event.title}
								<p class="text-gray-600 dark:text-gray-400 mt-1">{event.title}</p>
							{/if}
						</div>

						<!-- Chief Complaint -->
						{#if event.chiefComplaint}
							<div class="p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-200 dark:border-blue-800">
								<h4 class="text-xs font-semibold text-blue-700 dark:text-blue-300 uppercase tracking-wide mb-1">
									Chief Complaint
								</h4>
								<p class="text-sm text-blue-900 dark:text-blue-100">{event.chiefComplaint}</p>
							</div>
						{/if}

						<!-- AI Context Summary -->
						{#if event.aiContextSummary}
							<div class="p-3 bg-purple-50 dark:bg-purple-900/30 rounded-lg border border-purple-200 dark:border-purple-800">
								<h4 class="text-xs font-semibold text-purple-700 dark:text-purple-300 uppercase tracking-wide mb-1 flex items-center gap-1">
									<i class="fa-solid fa-wand-magic-sparkles"></i>
									Patient Context
								</h4>
								<p class="text-sm text-purple-900 dark:text-purple-100">{event.aiContextSummary}</p>
							</div>
						{/if}

						<!-- Date/Time -->
						<div class="flex items-center gap-2 text-gray-600 dark:text-gray-400">
							<i class="fa-solid fa-clock w-5"></i>
							<span>{formatDateTime(event.date)}</span>
						</div>

						<!-- Location -->
						{#if event.location}
							<div class="flex items-center gap-2 text-gray-600 dark:text-gray-400">
								<i class="fa-solid fa-location-dot w-5"></i>
								<span>{event.location}</span>
							</div>
						{/if}

						<!-- Provider -->
						{#if event.provider}
							<div class="flex items-center gap-2 text-gray-600 dark:text-gray-400">
								<i class="fa-solid fa-user-doctor w-5"></i>
								<span>{event.provider}</span>
							</div>
						{/if}

						<!-- Last Visit Info -->
						{#if event.lastVisitDate}
							<div class="flex items-center gap-2 text-gray-600 dark:text-gray-400">
								<i class="fa-solid fa-history w-5"></i>
								<span>Last visit: {event.lastVisitDate}{event.lastVisitReason ? ` - ${event.lastVisitReason}` : ''}</span>
							</div>
						{/if}

						<!-- Description -->
						{#if event.description}
							<div class="pt-2 border-t border-gray-200 dark:border-gray-700">
								<p class="text-sm text-gray-600 dark:text-gray-400">{event.description}</p>
							</div>
						{/if}

						<!-- Patient Actions -->
						{#if event.patientId}
							<div class="pt-3 border-t border-gray-200 dark:border-gray-700 flex gap-2">
								<button
									onclick={handleViewPatient}
									class="flex-1 px-3 py-2 text-sm font-medium text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/30 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors"
								>
									<i class="fa-solid fa-user mr-1.5"></i>View Patient
								</button>
								<button
									onclick={handleStartEncounter}
									class="flex-1 px-3 py-2 text-sm font-medium text-white bg-green-600 rounded-lg hover:bg-green-700 transition-colors"
								>
									<i class="fa-solid fa-play mr-1.5"></i>Start Encounter
								</button>
							</div>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="flex items-center justify-between px-5 py-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
				{#if isEditing}
					<div>
						{#if event && !isCreating}
							<button
								onclick={handleDelete}
								class="px-3 py-2 text-sm font-medium text-red-600 hover:bg-red-50 dark:hover:bg-red-900/30 rounded-lg transition-colors"
							>
								<i class="fa-solid fa-trash mr-1.5"></i>Delete
							</button>
						{/if}
					</div>
					<div class="flex gap-2">
						<button
							onclick={() => isCreating ? onClose() : (isEditing = false)}
							class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
						>
							Cancel
						</button>
						<button
							onclick={handleSave}
							class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
						>
							{isCreating ? 'Create' : 'Save'}
						</button>
					</div>
				{:else}
					<button
						onclick={handleDelete}
						class="px-3 py-2 text-sm font-medium text-red-600 hover:bg-red-50 dark:hover:bg-red-900/30 rounded-lg transition-colors"
					>
						<i class="fa-solid fa-trash mr-1.5"></i>Delete
					</button>
					<button
						onclick={() => isEditing = true}
						class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
					>
						<i class="fa-solid fa-pen mr-1.5"></i>Edit
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}
