<script lang="ts">
	import type { TimelineEvent } from '$lib/types/patient';

	let { events }: { events: TimelineEvent[] } = $props();

	const sortedEvents = $derived(
		[...events].sort((a, b) => new Date(a.event_date).getTime() - new Date(b.event_date).getTime())
	);

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function getEventIcon(eventType: string, icon: string | null): string {
		if (icon) return icon;
		const icons: Record<string, string> = {
			medication_start: 'fa-pills',
			medication_end: 'fa-prescription-bottle',
			diagnosis: 'fa-stethoscope',
			hospitalization: 'fa-hospital',
			surgery: 'fa-scalpel',
			birthday: 'fa-cake-candles',
			lab_result: 'fa-flask',
			vaccination: 'fa-syringe',
			encounter: 'fa-user-doctor'
		};
		return icons[eventType] ?? 'fa-circle';
	}

	function getEventColor(eventType: string, color: string | null): string {
		if (color) return `bg-${color}-500`;
		const colors: Record<string, string> = {
			medication_start: 'bg-green-500',
			medication_end: 'bg-red-500',
			diagnosis: 'bg-purple-500',
			hospitalization: 'bg-orange-500',
			surgery: 'bg-red-600',
			birthday: 'bg-pink-500',
			lab_result: 'bg-yellow-500',
			vaccination: 'bg-blue-500',
			encounter: 'bg-blue-400'
		};
		return colors[eventType] ?? 'bg-gray-500';
	}

	function getBorderColor(eventType: string, color: string | null): string {
		if (color) return `border-${color}-500`;
		const colors: Record<string, string> = {
			medication_start: 'border-green-500',
			medication_end: 'border-red-500',
			diagnosis: 'border-purple-500',
			hospitalization: 'border-orange-500',
			surgery: 'border-red-600',
			birthday: 'border-pink-500',
			lab_result: 'border-yellow-500',
			vaccination: 'border-blue-500',
			encounter: 'border-blue-400'
		};
		return colors[eventType] ?? 'border-gray-500';
	}
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full">
	<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
		<i class="fa-solid fa-timeline text-indigo-500"></i>
		Timeline
	</h3>

	{#if sortedEvents.length === 0}
		<p class="text-sm text-gray-500 dark:text-gray-400 italic">No timeline events recorded</p>
	{:else}
		<div class="overflow-x-auto pb-2">
			<div class="relative">
				<!-- Prominent timeline line -->
				<div class="absolute top-6 left-0 right-0 h-1 bg-gradient-to-r from-indigo-300 via-indigo-400 to-indigo-300 dark:from-indigo-600 dark:via-indigo-500 dark:to-indigo-600 rounded-full" style="margin-left: 24px; margin-right: 24px;"></div>

				<div class="flex gap-4 min-w-max relative">
					{#each sortedEvents as event, index}
						<div class="flex-shrink-0 w-48 relative">
							<!-- Date marker with circle on the line -->
							<div class="text-xs text-gray-500 dark:text-gray-400 mb-1 font-medium text-center">
								{formatDate(event.event_date)}
							</div>

							<!-- Circle marker on the timeline line -->
							<div class="flex justify-center mb-3">
								<div class="w-3 h-3 rounded-full bg-indigo-500 dark:bg-indigo-400 border-2 border-white dark:border-gray-800 shadow-sm z-10"></div>
							</div>

							<!-- Event card -->
							<div
								class="bg-gray-50 dark:bg-gray-700 rounded-lg p-3 border-l-4 {getBorderColor(event.event_type, event.color)} min-h-[80px]"
							>
								<div class="flex items-center gap-2 mb-2">
									<span
										class="w-6 h-6 rounded-full flex items-center justify-center text-white text-xs {getEventColor(event.event_type, event.color)}"
									>
										<i class="fa-solid {getEventIcon(event.event_type, event.icon)}"></i>
									</span>
									<span class="text-xs font-medium text-gray-600 dark:text-gray-300 capitalize">
										{event.event_type.replace(/_/g, ' ')}
									</span>
								</div>
								<p class="text-sm text-gray-700 dark:text-gray-200">
									{event.description}
								</p>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>
