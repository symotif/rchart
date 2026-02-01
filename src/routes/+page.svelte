<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';
	import { addTab } from '../stores/TabStore';
	import Calendar from '$lib/components/calendar/Calendar.svelte';
	import EventModal from '$lib/components/calendar/EventModal.svelte';
	import DashboardStats from '$lib/components/dashboard/DashboardStats.svelte';
	import type { CalendarEvent } from '$lib/types/calendar';

	// Panel sizes (persisted in state)
	let leftPanelWidth = $state(420);
	let rightPanelWidth = $state(220);
	let calendarHeight = $state(380);

	// Resize state
	let isResizingLeft = $state(false);
	let isResizingRight = $state(false);
	let isResizingCalendar = $state(false);

	// Calendar state
	let selectedDate = $state(new Date());
	let calendarEvents = $state<CalendarEvent[]>([]);
	let selectedEvent = $state<CalendarEvent | null>(null);
	let isEventModalOpen = $state(false);
	let isCreatingEvent = $state(false);

	// Schedule state (appointments for selected day)
	let dayAppointments = $derived(getAppointmentsForDay(selectedDate));

	function startResizeLeft(e: MouseEvent) {
		e.preventDefault();
		isResizingLeft = true;
		const startX = e.clientX;
		const startWidth = leftPanelWidth;

		function onMouseMove(e: MouseEvent) {
			const delta = e.clientX - startX;
			leftPanelWidth = Math.max(280, Math.min(600, startWidth + delta));
		}

		function onMouseUp() {
			isResizingLeft = false;
			document.removeEventListener('mousemove', onMouseMove);
			document.removeEventListener('mouseup', onMouseUp);
		}

		document.addEventListener('mousemove', onMouseMove);
		document.addEventListener('mouseup', onMouseUp);
	}

	function startResizeRight(e: MouseEvent) {
		e.preventDefault();
		isResizingRight = true;
		const startX = e.clientX;
		const startWidth = rightPanelWidth;

		function onMouseMove(e: MouseEvent) {
			const delta = startX - e.clientX;
			rightPanelWidth = Math.max(150, Math.min(400, startWidth + delta));
		}

		function onMouseUp() {
			isResizingRight = false;
			document.removeEventListener('mousemove', onMouseMove);
			document.removeEventListener('mouseup', onMouseUp);
		}

		document.addEventListener('mousemove', onMouseMove);
		document.addEventListener('mouseup', onMouseUp);
	}

	function startResizeCalendar(e: MouseEvent) {
		e.preventDefault();
		isResizingCalendar = true;
		const startY = e.clientY;
		const startHeight = calendarHeight;

		function onMouseMove(e: MouseEvent) {
			const delta = e.clientY - startY;
			calendarHeight = Math.max(250, Math.min(600, startHeight + delta));
		}

		function onMouseUp() {
			isResizingCalendar = false;
			document.removeEventListener('mousemove', onMouseMove);
			document.removeEventListener('mouseup', onMouseUp);
		}

		document.addEventListener('mousemove', onMouseMove);
		document.addEventListener('mouseup', onMouseUp);
	}

	onMount(async () => {
		await loadCalendarData();
	});

	async function loadCalendarData() {
		try {
			// Seed test data first
			await invoke('db_seed_test_data');

			// Get all appointments
			const appointments = await invoke<any[]>('db_get_all_appointments');

			// Get all patients to fetch their encounters
			const patients = await invoke<any[]>('db_get_all_patients');

			// Convert appointments to calendar events
			const events: CalendarEvent[] = appointments.map((apt, idx) => ({
				id: apt.id || idx + 1000,
				title: apt.reason || 'Appointment',
				date: apt.appointment_time,
				type: 'appointment' as const,
				patientId: apt.patient_id,
				patientName: apt.patient_name,
				status: apt.status || 'scheduled',
				location: 'Main Clinic'
			}));

			// Also load encounters from patient data
			for (const patient of patients) {
				try {
					const patientData = await invoke<any>('db_get_patient_full', { id: patient.id });
					if (patientData?.encounters) {
						for (const enc of patientData.encounters) {
							events.push({
								id: enc.id,
								title: enc.chief_complaint || enc.encounter_type,
								date: enc.encounter_date + 'T10:00:00',
								type: 'encounter' as const,
								patientId: patient.id,
								patientName: `${patient.first_name} ${patient.last_name}`,
								encounterId: enc.id,
								description: enc.summary,
								location: enc.location,
								provider: enc.provider,
								status: 'completed'
							});
						}
					}
				} catch (e) {
					// Patient may not have detail data yet
				}
			}

			// Always add today's appointments with rich data (merge with any DB data)
			const today = new Date();
			const todayStart = new Date(today.getFullYear(), today.getMonth(), today.getDate());

			// Check if we already have today's appointments
			const hasTodayAppointments = events.some(e => {
				const eventDate = new Date(e.date);
				return eventDate.getFullYear() === today.getFullYear() &&
					eventDate.getMonth() === today.getMonth() &&
					eventDate.getDate() === today.getDate();
			});

			if (!hasTodayAppointments || events.length < 3) {
				const sampleEvents: CalendarEvent[] = [
					{
						id: 1001,
						title: 'Follow-up: Depression Management',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 9, 0).toISOString(),
						type: 'follow-up',
						patientId: 1,
						patientName: 'Logan Nguyen',
						patientDob: '1998-09-15',
						patientPhone: '(555) 123-4567',
						status: 'scheduled',
						location: 'Room 101',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 30,
						chiefComplaint: 'Feeling more anxious lately, sleep issues returning',
						aiContextSummary: 'Currently managing MDD with sertraline 100mg. PHQ-9 improved from 18 to 12 over 3 months. Last visit discussed adding therapy referral.',
						lastVisitDate: '2025-01-15',
						lastVisitReason: 'Depression follow-up'
					},
					{
						id: 1002,
						title: 'Hypertension Check',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 9, 30).toISOString(),
						type: 'appointment',
						patientId: 2,
						patientName: 'Sarah Johnson',
						patientDob: '1985-03-22',
						patientPhone: '(555) 234-5678',
						status: 'confirmed',
						location: 'Room 102',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 20,
						chiefComplaint: 'Blood pressure monitoring, occasional headaches',
						aiContextSummary: 'On lisinopril 10mg for hypertension diagnosed 6 months ago. BP trending down nicely (last reading 132/84). Due for metabolic panel.',
						lastVisitDate: '2025-01-08',
						lastVisitReason: 'BP follow-up'
					},
					{
						id: 1003,
						title: 'Medication Review',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 10, 30).toISOString(),
						type: 'follow-up',
						patientId: 3,
						patientName: 'Michael Chen',
						patientDob: '1990-07-10',
						patientPhone: '(555) 345-6789',
						status: 'scheduled',
						location: 'Room 103',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 30,
						chiefComplaint: 'Review diabetes medications, A1c recheck',
						aiContextSummary: 'Type 2 DM on metformin 1000mg BID. A1c was 7.8% three months ago, target <7%. Also managing mild anxiety with hydroxyzine PRN.',
						lastVisitDate: '2024-12-20',
						lastVisitReason: 'Diabetes management'
					},
					{
						id: 1004,
						title: 'Post-Hospitalization Follow-up',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 11, 0).toISOString(),
						type: 'encounter',
						patientId: 4,
						patientName: 'Emily Davis',
						patientDob: '1978-12-01',
						patientPhone: '(555) 456-7890',
						status: 'confirmed',
						location: 'Room 101',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 45,
						chiefComplaint: 'Follow-up after CHF exacerbation hospitalization',
						aiContextSummary: 'Recently discharged (Jan 20) after CHF exacerbation. Started on new diuretic regimen. Needs weight monitoring and medication reconciliation.',
						lastVisitDate: '2025-01-20',
						lastVisitReason: 'Hospital discharge'
					},
					{
						id: 1005,
						title: 'Anxiety Management',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 14, 0).toISOString(),
						type: 'telehealth',
						patientId: 5,
						patientName: 'James Wilson',
						patientDob: '2000-05-18',
						patientPhone: '(555) 567-8901',
						status: 'scheduled',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 30,
						chiefComplaint: 'GAD follow-up, considering SSRI',
						aiContextSummary: 'Young adult with GAD, currently on buspirone 10mg TID with partial response. Discussed starting SSRI at last visit, patient was hesitant.',
						lastVisitDate: '2025-01-10',
						lastVisitReason: 'Anxiety follow-up'
					},
					{
						id: 1006,
						title: 'New Patient - Insomnia',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 15, 0).toISOString(),
						type: 'consultation',
						patientId: 6,
						patientName: 'Patricia Martinez',
						patientDob: '1965-08-14',
						patientPhone: '(555) 678-9012',
						status: 'scheduled',
						location: 'Room 102',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 45,
						chiefComplaint: 'Chronic insomnia, tried OTC sleep aids without success',
						aiContextSummary: 'New patient transfer. Records show chronic insomnia for 2+ years, history of depression (in remission). Previous trials of melatonin and diphenhydramine.',
						lastVisitDate: undefined,
						lastVisitReason: undefined
					},
					// Tomorrow's appointments
					{
						id: 1007,
						title: 'Bipolar Disorder Follow-up',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate() + 1, 9, 0).toISOString(),
						type: 'follow-up',
						patientId: 7,
						patientName: 'Robert Kim',
						patientDob: '1982-04-25',
						status: 'scheduled',
						location: 'Room 101',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 30,
						chiefComplaint: 'Mood stability check, lithium level review',
						aiContextSummary: 'Bipolar I on lithium 900mg daily. Stable for 8 months. Due for lithium level and thyroid panel.'
					},
					{
						id: 1008,
						title: 'ADHD Medication Check',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate() + 1, 10, 30).toISOString(),
						type: 'telehealth',
						patientId: 8,
						patientName: 'Amanda Foster',
						patientDob: '1995-11-30',
						status: 'confirmed',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 20,
						chiefComplaint: 'ADHD medication refill, discuss dosage',
						aiContextSummary: 'Adult ADHD on Adderall XR 20mg. Good response but reports afternoon crash. May need dose adjustment or booster.'
					},
					// Day after tomorrow
					{
						id: 1009,
						title: 'Annual Wellness Visit',
						date: new Date(today.getFullYear(), today.getMonth(), today.getDate() + 2, 9, 30).toISOString(),
						type: 'encounter',
						patientId: 9,
						patientName: 'Thomas Wright',
						patientDob: '1958-06-12',
						status: 'scheduled',
						location: 'Room 103',
						provider: 'Dr. Madeline Chu',
						durationMinutes: 45,
						chiefComplaint: 'Annual physical, preventive care',
						aiContextSummary: 'Healthy 66-year-old male. Due for colonoscopy (last one 8 years ago). Needs flu vaccine and shingles vaccine discussion.'
					}
				];

				// Filter out any duplicates by checking patient IDs that might already exist
				const existingPatientIds = new Set(events.map(e => e.patientId));
				const newEvents = sampleEvents.filter(e => !existingPatientIds.has(e.patientId));
				events.push(...newEvents);
			}

			calendarEvents = events;
		} catch (e) {
			console.error('Failed to load calendar data:', e);
			// Use sample data as fallback with rich appointment info
			const today = new Date();
			calendarEvents = [
				{
					id: 1001,
					title: 'Follow-up: Depression Management',
					date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 9, 0).toISOString(),
					type: 'follow-up',
					patientId: 1,
					patientName: 'Logan Nguyen',
					patientDob: '1998-09-15',
					patientPhone: '(555) 123-4567',
					status: 'scheduled',
					location: 'Room 101',
					provider: 'Dr. Madeline Chu',
					durationMinutes: 30,
					chiefComplaint: 'Feeling more anxious lately, sleep issues returning',
					aiContextSummary: 'Currently managing MDD with sertraline 100mg. PHQ-9 improved from 18 to 12 over 3 months. Last visit discussed adding therapy referral.',
					lastVisitDate: '2025-01-15',
					lastVisitReason: 'Depression follow-up'
				},
				{
					id: 1002,
					title: 'Hypertension Check',
					date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 9, 30).toISOString(),
					type: 'appointment',
					patientId: 2,
					patientName: 'Sarah Johnson',
					patientDob: '1985-03-22',
					patientPhone: '(555) 234-5678',
					status: 'confirmed',
					location: 'Room 102',
					provider: 'Dr. Madeline Chu',
					durationMinutes: 20,
					chiefComplaint: 'Blood pressure monitoring, occasional headaches',
					aiContextSummary: 'On lisinopril 10mg for hypertension diagnosed 6 months ago. BP trending down nicely (last reading 132/84). Due for metabolic panel.',
					lastVisitDate: '2025-01-08',
					lastVisitReason: 'BP follow-up'
				},
				{
					id: 1003,
					title: 'Medication Review',
					date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 10, 30).toISOString(),
					type: 'follow-up',
					patientId: 3,
					patientName: 'Michael Chen',
					patientDob: '1990-07-10',
					patientPhone: '(555) 345-6789',
					status: 'scheduled',
					location: 'Room 103',
					provider: 'Dr. Madeline Chu',
					durationMinutes: 30,
					chiefComplaint: 'Review diabetes medications, A1c recheck',
					aiContextSummary: 'Type 2 DM on metformin 1000mg BID. A1c was 7.8% three months ago, target <7%. Also managing mild anxiety with hydroxyzine PRN.',
					lastVisitDate: '2024-12-20',
					lastVisitReason: 'Diabetes management'
				},
				{
					id: 1004,
					title: 'Post-Hospitalization Follow-up',
					date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 11, 0).toISOString(),
					type: 'encounter',
					patientId: 4,
					patientName: 'Emily Davis',
					patientDob: '1978-12-01',
					patientPhone: '(555) 456-7890',
					status: 'confirmed',
					location: 'Room 101',
					provider: 'Dr. Madeline Chu',
					durationMinutes: 45,
					chiefComplaint: 'Follow-up after CHF exacerbation hospitalization',
					aiContextSummary: 'Recently discharged (Jan 20) after CHF exacerbation. Started on new diuretic regimen. Needs weight monitoring and medication reconciliation.',
					lastVisitDate: '2025-01-20',
					lastVisitReason: 'Hospital discharge'
				},
				{
					id: 1005,
					title: 'Anxiety Management',
					date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 14, 0).toISOString(),
					type: 'telehealth',
					patientId: 5,
					patientName: 'James Wilson',
					patientDob: '2000-05-18',
					patientPhone: '(555) 567-8901',
					status: 'scheduled',
					provider: 'Dr. Madeline Chu',
					durationMinutes: 30,
					chiefComplaint: 'GAD follow-up, considering SSRI',
					aiContextSummary: 'Young adult with GAD, currently on buspirone 10mg TID with partial response. Discussed starting SSRI at last visit, patient was hesitant.',
					lastVisitDate: '2025-01-10',
					lastVisitReason: 'Anxiety follow-up'
				},
				{
					id: 1006,
					title: 'New Patient - Insomnia',
					date: new Date(today.getFullYear(), today.getMonth(), today.getDate(), 15, 0).toISOString(),
					type: 'consultation',
					patientId: 6,
					patientName: 'Patricia Martinez',
					patientDob: '1965-08-14',
					patientPhone: '(555) 678-9012',
					status: 'scheduled',
					location: 'Room 102',
					provider: 'Dr. Madeline Chu',
					durationMinutes: 45,
					chiefComplaint: 'Chronic insomnia, tried OTC sleep aids without success',
					aiContextSummary: 'New patient transfer. Records show chronic insomnia for 2+ years, history of depression (in remission). Previous trials of melatonin and diphenhydramine.'
				}
			];
		}
	}

	function getAppointmentsForDay(date: Date): CalendarEvent[] {
		return calendarEvents
			.filter(event => {
				const eventDate = new Date(event.date);
				return eventDate.getFullYear() === date.getFullYear() &&
					eventDate.getMonth() === date.getMonth() &&
					eventDate.getDate() === date.getDate();
			})
			.sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime());
	}

	function handleDateSelect(date: Date) {
		selectedDate = date;
	}

	function handleEventClick(event: CalendarEvent) {
		selectedEvent = event;
		isCreatingEvent = false;
		isEventModalOpen = true;
	}

	function handleEventCreate(date: Date) {
		selectedDate = date;
		selectedEvent = null;
		isCreatingEvent = true;
		isEventModalOpen = true;
	}

	function handleEventSave(event: CalendarEvent) {
		if (isCreatingEvent) {
			// Add new event
			const newEvent = {
				...event,
				id: Date.now()
			};
			calendarEvents = [...calendarEvents, newEvent];
		} else {
			// Update existing event
			calendarEvents = calendarEvents.map(e =>
				e.id === event.id ? event : e
			);
		}
		isEventModalOpen = false;
	}

	function handleEventDelete(eventId: number) {
		calendarEvents = calendarEvents.filter(e => e.id !== eventId);
		isEventModalOpen = false;
	}

	function handleCloseModal() {
		isEventModalOpen = false;
		selectedEvent = null;
		isCreatingEvent = false;
	}

	function formatTime(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit', hour12: true });
	}

	function formatDateHeader(date: Date): string {
		return date.toLocaleDateString('en-US', { weekday: 'long', month: 'short', day: 'numeric' });
	}

	function getEventTypeColor(type: string): string {
		const colors: Record<string, string> = {
			'encounter': 'bg-blue-500',
			'appointment': 'bg-green-500',
			'follow-up': 'bg-purple-500',
			'telehealth': 'bg-cyan-500',
			'consultation': 'bg-orange-500',
			'procedure': 'bg-red-500'
		};
		return colors[type] || 'bg-gray-500';
	}

	function getAvatarColor(id: number): string {
		const colors = [
			'bg-blue-500', 'bg-green-500', 'bg-purple-500', 'bg-pink-500',
			'bg-indigo-500', 'bg-teal-500', 'bg-orange-500', 'bg-cyan-500'
		];
		return colors[(id || 0) % colors.length];
	}

	function getInitials(name: string): string {
		return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2);
	}

	function handleStartEncounter(event: CalendarEvent) {
		if (event.patientId) {
			const tab = {
				id: `note-new-${event.patientId}`,
				title: `New Note - ${event.patientName}`,
				path: `/patient/${event.patientId}/note/new`
			};
			addTab(tab);
			goto(tab.path);
		}
	}

	function handleViewPatient(event: CalendarEvent) {
		if (event.patientId) {
			const tab = {
				id: `patient-${event.patientId}`,
				title: event.patientName || 'Patient',
				path: `/patient/${event.patientId}`
			};
			addTab(tab);
			goto(tab.path);
		}
	}

	function handleNewPatient() {
		const tab = {
			id: 'new-patient',
			title: 'New Patient',
			path: '/patient/new'
		};
		addTab(tab);
		goto(tab.path);
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="an ehr for the world" />
</svelte:head>

<!-- page content -->
<div
	class="dashboard-container absolute top-20 left-20 right-10 bottom-10 my-4 ml-5 mr-3 flex flex-row"
	class:resizing={isResizingLeft || isResizingRight || isResizingCalendar}
>
	<!-- LEFT PANEL: Schedule/Appointments -->
	<div class="panel bg-white dark:bg-gray-800 rounded-lg shadow-lg mt-5 overflow-hidden flex flex-col" style="width: {leftPanelWidth}px; flex-shrink: 0;">
		<div class="p-4 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
			<h1 class="text-2xl font-bold text-gray-800 dark:text-gray-100">Schedule</h1>
			<p class="text-sm text-gray-500 dark:text-gray-400">Dr. Madeline Chu, MD</p>
			<p class="text-lg font-semibold text-gray-700 dark:text-gray-200 mt-2">
				{formatDateHeader(selectedDate)}
			</p>
		</div>

		<div class="flex-1 overflow-y-auto p-4 space-y-3">
			{#if dayAppointments.length === 0}
				<div class="text-center py-8 text-gray-500 dark:text-gray-400">
					<i class="fa-solid fa-calendar-xmark text-4xl mb-3 opacity-50"></i>
					<p>No appointments scheduled</p>
					<button
						onclick={() => handleEventCreate(selectedDate)}
						class="mt-3 text-sm text-blue-600 dark:text-blue-400 hover:underline"
					>
						+ Add appointment
					</button>
				</div>
			{:else}
				{#each dayAppointments as appointment}
					<button
						onclick={() => handleEventClick(appointment)}
						class="appointment-card w-full text-left bg-gray-50 dark:bg-gray-700 rounded-lg p-3 border border-gray-200 dark:border-gray-600 hover:shadow-md hover:border-blue-300 dark:hover:border-blue-600 transition-all"
					>
						<div class="flex items-start gap-3">
							<!-- Time -->
							<div class="flex-shrink-0 text-center">
								<div class="text-sm font-bold text-gray-800 dark:text-gray-100">
									{formatTime(appointment.date)}
								</div>
								<div class="w-2 h-2 rounded-full mx-auto mt-1 {getEventTypeColor(appointment.type)}"></div>
							</div>

							<!-- Patient Info -->
							<div class="flex-1 min-w-0">
								<div class="flex items-center gap-2">
									<div class="w-8 h-8 rounded-full {getAvatarColor(appointment.patientId || 0)} flex items-center justify-center text-white text-xs font-bold flex-shrink-0">
										{getInitials(appointment.patientName || 'NA')}
									</div>
									<div class="min-w-0">
										<h3 class="font-medium text-gray-900 dark:text-gray-100 truncate">
											{appointment.patientName}
										</h3>
										<p class="text-xs text-gray-500 dark:text-gray-400 truncate">
											{appointment.title}
										</p>
									</div>
								</div>
								{#if appointment.chiefComplaint}
									<p class="text-xs text-blue-600 dark:text-blue-400 mt-1 truncate">
										<i class="fa-solid fa-comment-medical mr-1"></i>{appointment.chiefComplaint}
									</p>
								{:else if appointment.location}
									<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
										<i class="fa-solid fa-location-dot mr-1"></i>{appointment.location}
									</p>
								{/if}
							</div>

							<!-- Chevron indicator -->
							<div class="flex-shrink-0 self-center">
								<i class="fa-solid fa-chevron-right text-gray-400 text-sm"></i>
							</div>
						</div>
					</button>
				{/each}
			{/if}
		</div>
	</div>

	<!-- LEFT RESIZE HANDLE -->
	<div
		class="resize-handle-vertical"
		class:active={isResizingLeft}
		onmousedown={startResizeLeft}
		role="separator"
		aria-orientation="vertical"
		tabindex="0"
	></div>

	<!-- MIDDLE PANEL: Calendar + Stats -->
	<div class="flex flex-col flex-1 min-w-0 mt-4 mx-2">
		<!-- Calendar -->
		<div class="panel bg-white dark:bg-gray-800 rounded-lg shadow-lg overflow-hidden" style="height: {calendarHeight}px; flex-shrink: 0;">
			<Calendar
				events={calendarEvents}
				{selectedDate}
				onDateSelect={handleDateSelect}
				onEventClick={handleEventClick}
				onEventCreate={handleEventCreate}
			/>
		</div>

		<!-- CALENDAR/STATS RESIZE HANDLE -->
		<div
			class="resize-handle-horizontal"
			class:active={isResizingCalendar}
			onmousedown={startResizeCalendar}
			role="separator"
			aria-orientation="horizontal"
			tabindex="0"
		></div>

		<!-- Dashboard Stats -->
		<div class="panel bg-white dark:bg-gray-800 rounded-lg shadow-lg flex-1 overflow-auto min-h-0">
			<DashboardStats
				todayAppointments={dayAppointments.length}
				completedAppointments={dayAppointments.filter(a => a.status === 'completed').length}
				telehealthCount={calendarEvents.filter(e => e.type === 'telehealth').length}
			/>
		</div>
	</div>

	<!-- RIGHT RESIZE HANDLE -->
	<div
		class="resize-handle-vertical"
		class:active={isResizingRight}
		onmousedown={startResizeRight}
		role="separator"
		aria-orientation="vertical"
		tabindex="0"
	></div>

	<!-- RIGHT PANEL: Tasks -->
	<div class="panel bg-white dark:bg-gray-800 rounded-lg shadow-lg mt-4 overflow-auto" style="width: {rightPanelWidth}px; flex-shrink: 0;">
		<div class="p-4">
			<h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-3">My Tasks</h2>
			<ul class="space-y-2">
				<li class="p-2 bg-blue-50 dark:bg-blue-900/30 rounded-lg text-sm text-gray-700 dark:text-gray-300 flex items-center gap-2">
					<input type="checkbox" class="rounded text-blue-500" />
					<span>Review patient charts</span>
				</li>
				<li class="p-2 bg-yellow-50 dark:bg-yellow-900/30 rounded-lg text-sm text-gray-700 dark:text-gray-300 flex items-center gap-2">
					<input type="checkbox" class="rounded text-yellow-500" />
					<span>Follow up with lab results</span>
				</li>
				<li class="p-2 bg-green-50 dark:bg-green-900/30 rounded-lg text-sm text-gray-700 dark:text-gray-300 flex items-center gap-2">
					<input type="checkbox" class="rounded text-green-500" />
					<span>Complete documentation</span>
				</li>
				<li class="p-2 bg-purple-50 dark:bg-purple-900/30 rounded-lg text-sm text-gray-700 dark:text-gray-300 flex items-center gap-2">
					<input type="checkbox" class="rounded text-purple-500" />
					<span>Sign prescriptions</span>
				</li>
			</ul>

			<h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mt-6 mb-3">Quick Actions</h2>
			<div class="space-y-2">
				<button
					onclick={() => handleEventCreate(selectedDate)}
					class="w-full text-left p-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors flex items-center gap-2"
				>
					<i class="fa-solid fa-plus w-4"></i>
					New Appointment
				</button>
				<button
					onclick={handleNewPatient}
					class="w-full text-left p-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors flex items-center gap-2"
				>
					<i class="fa-solid fa-user-plus w-4"></i>
					New Patient
				</button>
				<button class="w-full text-left p-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors flex items-center gap-2">
					<i class="fa-solid fa-file-medical w-4"></i>
					New Note
				</button>
			</div>
		</div>
	</div>
</div>

<!-- Event Modal -->
<EventModal
	event={selectedEvent}
	isOpen={isEventModalOpen}
	isCreating={isCreatingEvent}
	{selectedDate}
	onClose={handleCloseModal}
	onSave={handleEventSave}
	onDelete={handleEventDelete}
/>

<style>
	.dashboard-container {
		gap: 0;
	}

	.dashboard-container.resizing {
		user-select: none;
		cursor: col-resize;
	}

	.panel {
		position: relative;
	}

	.resize-handle-vertical {
		width: 8px;
		cursor: col-resize;
		background: transparent;
		transition: background-color 0.2s;
		flex-shrink: 0;
		z-index: 10;
	}

	.resize-handle-vertical:hover,
	.resize-handle-vertical.active {
		background: rgba(59, 130, 246, 0.4);
	}

	.resize-handle-horizontal {
		height: 8px;
		cursor: row-resize;
		background: transparent;
		transition: background-color 0.2s;
		flex-shrink: 0;
		z-index: 10;
		margin: 2px 0;
	}

	.resize-handle-horizontal:hover,
	.resize-handle-horizontal.active {
		background: rgba(59, 130, 246, 0.4);
	}

	.appointment-card {
		cursor: pointer;
	}
</style>
