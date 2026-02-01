<script lang="ts">
	import type { Todo, Goal, Diagnosis } from '$lib/types/patient';

	let {
		todos,
		goals,
		diagnoses
	}: {
		todos: Todo[];
		goals: Goal[];
		diagnoses: Diagnosis[];
	} = $props();

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	function getDiagnosisName(diagnosisId: number | null): string | null {
		if (!diagnosisId) return null;
		const diagnosis = diagnoses.find((d) => d.id === diagnosisId);
		return diagnosis?.name ?? null;
	}

	function getPriorityColor(priority: string | null): string {
		switch (priority?.toLowerCase()) {
			case 'high':
				return 'border-l-red-500';
			case 'normal':
				return 'border-l-blue-500';
			case 'low':
				return 'border-l-gray-400';
			default:
				return 'border-l-gray-300';
		}
	}

	function getStatusIcon(status: string | null): string {
		switch (status?.toLowerCase()) {
			case 'completed':
				return 'fa-check-circle text-green-500';
			case 'in_progress':
				return 'fa-spinner text-blue-500';
			default:
				return 'fa-circle text-gray-400';
		}
	}
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 h-full flex flex-col">
	<h3 class="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-3 flex items-center gap-2">
		<i class="fa-solid fa-list-check text-purple-500"></i>
		To-Dos & Goals
	</h3>

	<div class="flex-1 overflow-y-auto space-y-4">
		<!-- To-Dos Section -->
		<div>
			<h4 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2 uppercase tracking-wide">
				To-Do Items
			</h4>
			{#if todos.length === 0}
				<p class="text-sm text-gray-500 dark:text-gray-400 italic">No pending to-dos</p>
			{:else}
				<div class="space-y-2">
					{#each todos as todo}
						<div
							class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg border-l-4 {getPriorityColor(todo.priority)}"
						>
							<div class="flex items-start gap-2">
								<i class="fa-solid {getStatusIcon(todo.status)} mt-0.5"></i>
								<div class="flex-1 min-w-0">
									<p class="text-sm text-gray-800 dark:text-gray-200">{todo.description}</p>
									<div class="flex items-center gap-2 mt-1">
										{#if todo.due_date}
											<span class="text-xs text-gray-500 dark:text-gray-400">
												<i class="fa-solid fa-calendar mr-1"></i>
												{formatDate(todo.due_date)}
											</span>
										{/if}
										{#if getDiagnosisName(todo.diagnosis_id)}
											<span class="text-xs px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded">
												{getDiagnosisName(todo.diagnosis_id)}
											</span>
										{/if}
									</div>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<!-- Goals Section -->
		<div>
			<h4 class="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2 uppercase tracking-wide">
				Patient Goals
			</h4>
			{#if goals.length === 0}
				<p class="text-sm text-gray-500 dark:text-gray-400 italic">No goals set</p>
			{:else}
				<div class="space-y-2">
					{#each goals as goal}
						<div class="p-3 bg-gradient-to-r from-green-50 to-blue-50 dark:from-green-900/20 dark:to-blue-900/20 rounded-lg">
							<p class="text-sm text-gray-800 dark:text-gray-200 mb-2">{goal.description}</p>

							<!-- Progress bar -->
							<div class="relative">
								<div class="h-2 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
									<div
										class="h-full bg-gradient-to-r from-green-400 to-blue-500 transition-all duration-300"
										style="width: {goal.progress ?? 0}%"
									></div>
								</div>
								<div class="flex items-center justify-between mt-1">
									<span class="text-xs text-gray-500 dark:text-gray-400">
										{goal.progress ?? 0}% complete
									</span>
									{#if goal.target_date}
										<span class="text-xs text-gray-500 dark:text-gray-400">
											Target: {formatDate(goal.target_date)}
										</span>
									{/if}
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>
