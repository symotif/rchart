<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { CalendarEvent } from '$lib/types/calendar';

	type ViewMode = 'month' | 'week';

	interface Props {
		events?: CalendarEvent[];
		selectedDate?: Date;
		viewMode?: ViewMode;
		onDateSelect?: (date: Date) => void;
		onEventClick?: (event: CalendarEvent) => void;
		onEventCreate?: (date: Date) => void;
	}

	let {
		events = [],
		selectedDate = new Date(),
		viewMode = 'month',
		onDateSelect = () => {},
		onEventClick = () => {},
		onEventCreate = () => {}
	}: Props = $props();

	const DAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
	const MONTHS = [
		'January', 'February', 'March', 'April', 'May', 'June',
		'July', 'August', 'September', 'October', 'November', 'December'
	];

	let currentDate = $state(new Date(selectedDate));
	let internalViewMode = $state<ViewMode>(viewMode);

	// Computed values
	let currentMonth = $derived(currentDate.getMonth());
	let currentYear = $derived(currentDate.getFullYear());
	let monthDays = $derived(getMonthDays(currentYear, currentMonth));
	let weekDays = $derived(getWeekDays(currentDate));

	function getMonthDays(year: number, month: number): Date[] {
		const days: Date[] = [];
		const firstDay = new Date(year, month, 1);
		const lastDay = new Date(year, month + 1, 0);

		// Add days from previous month to fill first week
		const startPadding = firstDay.getDay();
		for (let i = startPadding - 1; i >= 0; i--) {
			days.push(new Date(year, month, -i));
		}

		// Add all days of current month
		for (let i = 1; i <= lastDay.getDate(); i++) {
			days.push(new Date(year, month, i));
		}

		// Add days from next month to complete last week
		const endPadding = 6 - lastDay.getDay();
		for (let i = 1; i <= endPadding; i++) {
			days.push(new Date(year, month + 1, i));
		}

		return days;
	}

	function getWeekDays(date: Date): Date[] {
		const days: Date[] = [];
		const dayOfWeek = date.getDay();
		const startOfWeek = new Date(date);
		startOfWeek.setDate(date.getDate() - dayOfWeek);

		for (let i = 0; i < 7; i++) {
			const day = new Date(startOfWeek);
			day.setDate(startOfWeek.getDate() + i);
			days.push(day);
		}

		return days;
	}

	function getEventsForDate(date: Date): CalendarEvent[] {
		return events.filter(event => {
			const eventDate = new Date(event.date);
			return eventDate.getFullYear() === date.getFullYear() &&
				eventDate.getMonth() === date.getMonth() &&
				eventDate.getDate() === date.getDate();
		});
	}

	function isToday(date: Date): boolean {
		const today = new Date();
		return date.getFullYear() === today.getFullYear() &&
			date.getMonth() === today.getMonth() &&
			date.getDate() === today.getDate();
	}

	function isSelected(date: Date): boolean {
		return date.getFullYear() === selectedDate.getFullYear() &&
			date.getMonth() === selectedDate.getMonth() &&
			date.getDate() === selectedDate.getDate();
	}

	function isCurrentMonth(date: Date): boolean {
		return date.getMonth() === currentMonth;
	}

	function navigatePrev() {
		if (internalViewMode === 'month') {
			currentDate = new Date(currentYear, currentMonth - 1, 1);
		} else {
			const newDate = new Date(currentDate);
			newDate.setDate(currentDate.getDate() - 7);
			currentDate = newDate;
		}
	}

	function navigateNext() {
		if (internalViewMode === 'month') {
			currentDate = new Date(currentYear, currentMonth + 1, 1);
		} else {
			const newDate = new Date(currentDate);
			newDate.setDate(currentDate.getDate() + 7);
			currentDate = newDate;
		}
	}

	function goToToday() {
		currentDate = new Date();
		onDateSelect(new Date());
	}

	function handleDateClick(date: Date) {
		onDateSelect(date);
	}

	function handleEventClick(event: CalendarEvent, e: MouseEvent) {
		e.stopPropagation();
		onEventClick(event);
	}

	function handleWheel(e: WheelEvent) {
		if (Math.abs(e.deltaY) > 30) {
			if (e.deltaY > 0) {
				navigateNext();
			} else {
				navigatePrev();
			}
		}
	}

	function getEventColor(type: string): string {
		const colors: Record<string, string> = {
			'encounter': 'bg-blue-500',
			'appointment': 'bg-green-500',
			'follow-up': 'bg-purple-500',
			'telehealth': 'bg-cyan-500',
			'consultation': 'bg-orange-500',
			'procedure': 'bg-red-500',
			'default': 'bg-gray-500'
		};
		return colors[type] || colors['default'];
	}

	function formatTime(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit', hour12: true });
	}
</script>

<div class="calendar-wrapper h-full flex flex-col select-none" onwheel={handleWheel}>
	<!-- Header -->
	<div class="calendar-header flex items-center justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
		<div class="flex items-center gap-2">
			<button
				onclick={goToToday}
				class="px-3 py-1.5 text-sm font-medium border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200 transition-colors"
			>
				Today
			</button>
			<div class="flex items-center">
				<button
					onclick={navigatePrev}
					class="p-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-full transition-colors"
					aria-label="Previous"
				>
					<svg class="w-5 h-5 text-gray-600 dark:text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
					</svg>
				</button>
				<button
					onclick={navigateNext}
					class="p-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-full transition-colors"
					aria-label="Next"
				>
					<svg class="w-5 h-5 text-gray-600 dark:text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
					</svg>
				</button>
			</div>
			<h2 class="text-lg font-semibold text-gray-800 dark:text-gray-100 ml-2">
				{MONTHS[currentMonth]} {currentYear}
			</h2>
		</div>

		<!-- View Toggle -->
		<div class="flex items-center bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5">
			<button
				onclick={() => internalViewMode = 'month'}
				class="px-3 py-1 text-sm font-medium rounded-md transition-colors {internalViewMode === 'month' ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm' : 'text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white'}"
			>
				Month
			</button>
			<button
				onclick={() => internalViewMode = 'week'}
				class="px-3 py-1 text-sm font-medium rounded-md transition-colors {internalViewMode === 'week' ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm' : 'text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white'}"
			>
				Week
			</button>
		</div>
	</div>

	<!-- Day Headers -->
	<div class="grid grid-cols-7 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
		{#each DAYS as day}
			<div class="py-2 text-center text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
				{day}
			</div>
		{/each}
	</div>

	<!-- Calendar Grid -->
	{#if internalViewMode === 'month'}
		<div class="calendar-grid grid grid-cols-7 flex-1 overflow-hidden">
			{#each monthDays as date, i}
				{@const dayEvents = getEventsForDate(date)}
				{@const maxVisibleEvents = 3}
				<button
					onclick={() => handleDateClick(date)}
					ondblclick={() => onEventCreate(date)}
					class="calendar-cell relative border-b border-r border-gray-100 dark:border-gray-700 p-1 text-left overflow-hidden transition-colors hover:bg-gray-50 dark:hover:bg-gray-700/50
						{!isCurrentMonth(date) ? 'bg-gray-50 dark:bg-gray-800/50' : 'bg-white dark:bg-gray-800'}
						{isSelected(date) ? 'ring-2 ring-inset ring-blue-500' : ''}"
				>
					<!-- Date Number -->
					<div class="flex justify-end mb-1">
						<span
							class="w-6 h-6 flex items-center justify-center text-sm rounded-full
								{isToday(date) ? 'bg-blue-500 text-white font-semibold' : ''}
								{!isCurrentMonth(date) ? 'text-gray-400 dark:text-gray-500' : 'text-gray-700 dark:text-gray-200'}"
						>
							{date.getDate()}
						</span>
					</div>

					<!-- Events -->
					<div class="space-y-0.5 overflow-hidden">
						{#each dayEvents.slice(0, maxVisibleEvents) as event}
							<button
								onclick={(e) => handleEventClick(event, e)}
								class="event-dot w-full text-left px-1.5 py-0.5 text-xs rounded truncate text-white {getEventColor(event.type)} hover:opacity-80 transition-opacity"
								title="{formatTime(event.date)} - {event.title}"
							>
								{#if internalViewMode === 'month'}
									<span class="font-medium">{formatTime(event.date)}</span>
									<span class="ml-1 truncate">{event.patientName || event.title}</span>
								{/if}
							</button>
						{/each}
						{#if dayEvents.length > maxVisibleEvents}
							<div class="text-xs text-gray-500 dark:text-gray-400 px-1.5 font-medium">
								+{dayEvents.length - maxVisibleEvents} more
							</div>
						{/if}
					</div>
				</button>
			{/each}
		</div>
	{:else}
		<!-- Week View -->
		<div class="calendar-grid grid grid-cols-7 flex-1 overflow-hidden">
			{#each weekDays as date}
				{@const dayEvents = getEventsForDate(date)}
				<button
					onclick={() => handleDateClick(date)}
					ondblclick={() => onEventCreate(date)}
					class="calendar-cell relative border-r border-gray-100 dark:border-gray-700 p-2 text-left overflow-hidden transition-colors hover:bg-gray-50 dark:hover:bg-gray-700/50 bg-white dark:bg-gray-800
						{isSelected(date) ? 'ring-2 ring-inset ring-blue-500' : ''}"
				>
					<!-- Date Header for Week View -->
					<div class="text-center mb-2 pb-2 border-b border-gray-100 dark:border-gray-700">
						<div class="text-xs text-gray-500 dark:text-gray-400 uppercase">{DAYS[date.getDay()]}</div>
						<div
							class="w-8 h-8 mx-auto flex items-center justify-center text-lg rounded-full mt-1
								{isToday(date) ? 'bg-blue-500 text-white font-semibold' : 'text-gray-700 dark:text-gray-200'}"
						>
							{date.getDate()}
						</div>
					</div>

					<!-- Events for Week View -->
					<div class="space-y-1 overflow-y-auto max-h-full">
						{#each dayEvents as event}
							<button
								onclick={(e) => handleEventClick(event, e)}
								class="w-full text-left p-2 text-xs rounded {getEventColor(event.type)} text-white hover:opacity-80 transition-opacity"
							>
								<div class="font-semibold">{formatTime(event.date)}</div>
								<div class="truncate">{event.patientName || event.title}</div>
								{#if event.type}
									<div class="text-white/80 truncate capitalize">{event.type}</div>
								{/if}
							</button>
						{/each}
					</div>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.calendar-wrapper {
		min-height: 0;
	}

	.calendar-grid {
		min-height: 0;
	}

	.calendar-cell {
		min-height: 0;
		min-width: 0;
	}

	/* Fixed cell heights based on row count */
	.calendar-grid {
		grid-auto-rows: 1fr;
	}

	.event-dot {
		display: block;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
</style>
