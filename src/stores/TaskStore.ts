import { writable, derived, get } from 'svelte/store';
import type { ProviderTask, TaskCategory } from '../lib/types/user';

// Main tasks store
const tasksStore = writable<ProviderTask[]>([]);

// Filter state
const showStarredOnlyStore = writable<boolean>(false);

// Derived store for active (non-completed) tasks - filtered and sorted
export const activeTasks = derived(
	[tasksStore, showStarredOnlyStore],
	([$tasks, $showStarredOnly]) => {
		let filtered = $tasks.filter((t) => !t.completed);
		if ($showStarredOnly) {
			filtered = filtered.filter((t) => t.starred);
		}

		// Sort: pinned first, then by sortOrder
		return filtered.sort((a, b) => {
			if (a.pinned && !b.pinned) return -1;
			if (!a.pinned && b.pinned) return 1;
			return a.sortOrder - b.sortOrder;
		});
	}
);

// Derived store for completed tasks
export const completedTasksList = derived(
	[tasksStore, showStarredOnlyStore],
	([$tasks, $showStarredOnly]) => {
		let filtered = $tasks.filter((t) => t.completed);
		if ($showStarredOnly) {
			filtered = filtered.filter((t) => t.starred);
		}
		return filtered.sort((a, b) => a.sortOrder - b.sortOrder);
	}
);

// Legacy: filtered tasks (all tasks) for backwards compatibility
export const filteredTasks = derived(
	[tasksStore, showStarredOnlyStore],
	([$tasks, $showStarredOnly]) => {
		let filtered = $showStarredOnly ? $tasks.filter((t) => t.starred) : $tasks;

		// Sort: pinned first, then by sortOrder
		return filtered.sort((a, b) => {
			if (a.pinned && !b.pinned) return -1;
			if (!a.pinned && b.pinned) return 1;
			return a.sortOrder - b.sortOrder;
		});
	}
);

// Derived stores for stats
export const totalTasks = derived(tasksStore, ($tasks) => $tasks.length);
export const completedTasks = derived(tasksStore, ($tasks) => $tasks.filter((t) => t.completed).length);
export const starredTasks = derived(tasksStore, ($tasks) => $tasks.filter((t) => t.starred).length);

// Export filter store
export const showStarredOnly = showStarredOnlyStore;

/**
 * Generate a unique ID for tasks
 */
function generateId(): string {
	return `task_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
}

/**
 * Add a new task
 */
export function addTask(title: string, category: TaskCategory): ProviderTask {
	const tasks = get(tasksStore);
	const maxSortOrder = tasks.length > 0 ? Math.max(...tasks.map((t) => t.sortOrder)) : 0;

	const newTask: ProviderTask = {
		id: generateId(),
		title,
		category,
		starred: false,
		pinned: false,
		completed: false,
		sortOrder: maxSortOrder + 1,
		createdAt: new Date().toISOString()
	};

	tasksStore.update((tasks) => [...tasks, newTask]);
	saveTasksToStorage();

	return newTask;
}

/**
 * Delete a task
 */
export function deleteTask(taskId: string): void {
	tasksStore.update((tasks) => tasks.filter((t) => t.id !== taskId));
	saveTasksToStorage();
}

/**
 * Toggle task completion
 */
export function toggleTaskCompleted(taskId: string): void {
	tasksStore.update((tasks) =>
		tasks.map((t) => (t.id === taskId ? { ...t, completed: !t.completed } : t))
	);
	saveTasksToStorage();
}

/**
 * Toggle task starred
 */
export function toggleTaskStarred(taskId: string): void {
	tasksStore.update((tasks) =>
		tasks.map((t) => (t.id === taskId ? { ...t, starred: !t.starred } : t))
	);
	saveTasksToStorage();
}

/**
 * Toggle task pinned
 */
export function toggleTaskPinned(taskId: string): void {
	tasksStore.update((tasks) =>
		tasks.map((t) => (t.id === taskId ? { ...t, pinned: !t.pinned } : t))
	);
	saveTasksToStorage();
}

/**
 * Update task title
 */
export function updateTaskTitle(taskId: string, title: string): void {
	tasksStore.update((tasks) =>
		tasks.map((t) => (t.id === taskId ? { ...t, title } : t))
	);
	saveTasksToStorage();
}

/**
 * Update task category
 */
export function updateTaskCategory(taskId: string, category: TaskCategory): void {
	tasksStore.update((tasks) =>
		tasks.map((t) => (t.id === taskId ? { ...t, category } : t))
	);
	saveTasksToStorage();
}

/**
 * Reorder tasks (for drag and drop)
 */
export function reorderTasks(taskIds: string[]): void {
	tasksStore.update((tasks) => {
		const taskMap = new Map(tasks.map((t) => [t.id, t]));
		return taskIds.map((id, index) => {
			const task = taskMap.get(id);
			if (task) {
				return { ...task, sortOrder: index };
			}
			return task!;
		}).filter(Boolean);
	});
	saveTasksToStorage();
}

/**
 * Move task to a specific position
 */
export function moveTask(taskId: string, newIndex: number): void {
	const tasks = get(filteredTasks);
	const currentIndex = tasks.findIndex((t) => t.id === taskId);

	if (currentIndex === -1 || currentIndex === newIndex) return;

	const taskIds = tasks.map((t) => t.id);
	const [removed] = taskIds.splice(currentIndex, 1);
	taskIds.splice(newIndex, 0, removed);

	reorderTasks(taskIds);
}

/**
 * Toggle starred filter
 */
export function toggleStarredFilter(): void {
	showStarredOnlyStore.update((v) => !v);
}

/**
 * Set starred filter
 */
export function setStarredFilter(value: boolean): void {
	showStarredOnlyStore.set(value);
}

/**
 * Clear all completed tasks
 */
export function clearCompletedTasks(): void {
	tasksStore.update((tasks) => tasks.filter((t) => !t.completed));
	saveTasksToStorage();
}

/**
 * Save tasks to localStorage
 */
function saveTasksToStorage(): void {
	if (typeof localStorage === 'undefined') return;

	const tasks = get(tasksStore);
	localStorage.setItem('providerTasks', JSON.stringify(tasks));
}

/**
 * Load tasks from localStorage
 */
export function loadTasksFromStorage(): void {
	if (typeof localStorage === 'undefined') return;

	try {
		const saved = localStorage.getItem('providerTasks');
		if (saved) {
			const tasks = JSON.parse(saved) as ProviderTask[];
			tasksStore.set(tasks);
		}
	} catch {
		console.warn('Failed to load tasks from storage');
	}
}

/**
 * Initialize the store with default tasks (for demo)
 */
export function initTasksWithDefaults(): void {
	loadTasksFromStorage();

	// If no tasks exist, add some defaults
	const tasks = get(tasksStore);
	if (tasks.length === 0) {
		const defaults: { title: string; category: TaskCategory; starred?: boolean; pinned?: boolean }[] = [
			// Pinned urgent tasks
			{ title: 'Call insurance re: Smith prior auth', category: 'calls', pinned: true, starred: true },
			{ title: 'Sign controlled substance Rx for Martinez', category: 'prescriptions', pinned: true },

			// Starred important tasks
			{ title: 'Review Johnson lab results - abnormal CBC', category: 'labs', starred: true },
			{ title: 'Complete disability paperwork for Williams', category: 'paperwork', starred: true },

			// Regular tasks - Paperwork
			{ title: 'FMLA form for Thompson', category: 'paperwork' },
			{ title: 'Prior auth form - MRI lumbar spine', category: 'paperwork' },
			{ title: 'DME order form - CPAP for Davis', category: 'paperwork' },

			// Regular tasks - Refills
			{ title: 'Refill lisinopril 10mg - Anderson', category: 'refills' },
			{ title: 'Refill metformin 500mg - Garcia', category: 'refills' },
			{ title: 'Refill atorvastatin 40mg - Brown', category: 'refills' },
			{ title: 'Refill amlodipine 5mg - Wilson', category: 'refills' },

			// Regular tasks - Follow-ups
			{ title: 'Follow up with Chen - post-op day 7', category: 'follow_up' },
			{ title: 'Call patient re: colonoscopy scheduling', category: 'follow_up' },
			{ title: 'Check in with Taylor - new anxiety med', category: 'follow_up' },

			// Regular tasks - Documentation
			{ title: 'Complete chart note - Robinson visit', category: 'documentation' },
			{ title: 'Finish H&P for Lee admission', category: 'documentation' },
			{ title: 'Update problem list - Harris', category: 'documentation' },

			// Regular tasks - Labs
			{ title: 'Review lipid panel - Moore', category: 'labs' },
			{ title: 'Check HbA1c results - Jackson', category: 'labs' },
			{ title: 'Review thyroid panel - White', category: 'labs' },

			// Regular tasks - Prescriptions
			{ title: 'New Rx - omeprazole for Clark', category: 'prescriptions' },
			{ title: 'Adjust insulin dose - Rodriguez', category: 'prescriptions' },
			{ title: 'E-prescribe antibiotics - Lewis', category: 'prescriptions' },

			// Regular tasks - Calls
			{ title: 'Return call - Walker questions about surgery', category: 'calls' },
			{ title: 'Call specialist re: Hall referral', category: 'calls' },
			{ title: 'Pharmacy callback - drug interaction question', category: 'calls' },

			// Regular tasks - Other
			{ title: 'Review radiology report - Young', category: 'other' },
			{ title: 'Staff meeting prep - quality metrics', category: 'other' },
			{ title: 'CME - complete required modules', category: 'other' }
		];

		defaults.forEach(({ title, category, starred, pinned }) => {
			const task = addTask(title, category);
			if (starred) toggleTaskStarred(task.id);
			if (pinned) toggleTaskPinned(task.id);
		});
	}
}

// Export the main store for direct access if needed
export const TaskStore = tasksStore;
