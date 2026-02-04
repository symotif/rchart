<script lang="ts">
	import { onMount } from 'svelte';
	import { flip } from 'svelte/animate';
	import { fade } from 'svelte/transition';
	import type { TaskCategory } from '$lib/types/user';
	import { TASK_CATEGORY_CONFIG } from '$lib/types/user';
	import {
		activeTasks,
		completedTasksList,
		showStarredOnly,
		addTask,
		deleteTask,
		toggleTaskCompleted,
		toggleTaskStarred,
		toggleTaskPinned,
		toggleStarredFilter,
		updateTaskTitle,
		initTasksWithDefaults,
		reorderTasks
	} from '../../stores/TaskStore';

	// Add task form state
	let showAddForm = $state(false);
	let newTaskTitle = $state('');
	let newTaskCategory = $state<TaskCategory>('other');

	// Archive expanded state
	let showArchive = $state(false);

	// Edit state
	let editingTaskId = $state<string | null>(null);
	let editingTitle = $state('');

	// Drag and drop state
	let draggedTaskId = $state<string | null>(null);
	let dragOverTaskId = $state<string | null>(null);

	onMount(() => {
		initTasksWithDefaults();
	});

	function handleAddTask() {
		if (newTaskTitle.trim()) {
			addTask(newTaskTitle.trim(), newTaskCategory);
			newTaskTitle = '';
			newTaskCategory = 'other';
			showAddForm = false;
		}
	}

	function handleAddKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			handleAddTask();
		} else if (e.key === 'Escape') {
			showAddForm = false;
			newTaskTitle = '';
		}
	}

	// Edit handlers
	function startEditing(taskId: string, currentTitle: string) {
		editingTaskId = taskId;
		editingTitle = currentTitle;
	}

	function saveEdit() {
		if (editingTaskId && editingTitle.trim()) {
			updateTaskTitle(editingTaskId, editingTitle.trim());
		}
		editingTaskId = null;
		editingTitle = '';
	}

	function cancelEdit() {
		editingTaskId = null;
		editingTitle = '';
	}

	function handleEditKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			saveEdit();
		} else if (e.key === 'Escape') {
			cancelEdit();
		}
	}

	function handleDoubleClick(e: MouseEvent, taskId: string, currentTitle: string) {
		e.preventDefault();
		e.stopPropagation();
		startEditing(taskId, currentTitle);
	}

	// Drag and drop handlers
	function handleDragStart(e: DragEvent, taskId: string) {
		if (editingTaskId) return;
		draggedTaskId = taskId;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', taskId);
		}
	}

	function handleDragOver(e: DragEvent, taskId: string) {
		e.preventDefault();
		if (draggedTaskId && draggedTaskId !== taskId) {
			dragOverTaskId = taskId;
		}
	}

	function handleDragLeave() {
		dragOverTaskId = null;
	}

	function handleDrop(e: DragEvent, targetTaskId: string) {
		e.preventDefault();
		if (draggedTaskId && draggedTaskId !== targetTaskId) {
			const tasks = $activeTasks;
			const draggedIndex = tasks.findIndex((t) => t.id === draggedTaskId);
			const targetIndex = tasks.findIndex((t) => t.id === targetTaskId);

			if (draggedIndex !== -1 && targetIndex !== -1) {
				const taskIds = tasks.map((t) => t.id);
				const [removed] = taskIds.splice(draggedIndex, 1);
				taskIds.splice(targetIndex, 0, removed);
				reorderTasks(taskIds);
			}
		}
		draggedTaskId = null;
		dragOverTaskId = null;
	}

	function handleDragEnd() {
		draggedTaskId = null;
		dragOverTaskId = null;
	}

	function getCategoryConfig(category: TaskCategory) {
		return TASK_CATEGORY_CONFIG[category];
	}

	function handleCheckboxClick(e: MouseEvent, taskId: string) {
		e.preventDefault();
		e.stopPropagation();
		toggleTaskCompleted(taskId);
	}

	function handleStarClick(e: MouseEvent, taskId: string) {
		e.preventDefault();
		e.stopPropagation();
		toggleTaskStarred(taskId);
	}

	function handlePinClick(e: MouseEvent, taskId: string) {
		e.preventDefault();
		e.stopPropagation();
		toggleTaskPinned(taskId);
	}

	function handleDeleteClick(e: MouseEvent, taskId: string) {
		e.preventDefault();
		e.stopPropagation();
		deleteTask(taskId);
	}
</script>

<div class="flex flex-col h-full">
	<!-- Header with title, add button, and filter -->
	<div class="flex items-center justify-between mb-3">
		<h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 flex items-center gap-2">
			<i class="fa-solid fa-list-check text-purple-500"></i>
			My Tasks
		</h2>
		<div class="flex items-center gap-1">
			<!-- Add task button -->
			<button
				onclick={() => (showAddForm = !showAddForm)}
				class="p-1.5 rounded-lg transition-colors {showAddForm
					? 'bg-purple-100 dark:bg-purple-900/50 text-purple-500'
					: 'text-gray-400 hover:text-purple-500 hover:bg-gray-100 dark:hover:bg-gray-700'}"
				title="Add task"
			>
				<i class="fa-solid fa-plus"></i>
			</button>
			<!-- Star filter button -->
			<button
				onclick={() => toggleStarredFilter()}
				class="p-1.5 rounded-lg transition-all duration-200 {$showStarredOnly
					? 'bg-yellow-100 dark:bg-yellow-900/50'
					: 'hover:bg-gray-100 dark:hover:bg-gray-700'}"
				title={$showStarredOnly ? 'Show all tasks' : 'Show starred only'}
			>
				<i
					class="fa-star transition-all duration-200 {$showStarredOnly
						? 'fa-solid text-yellow-500'
						: 'fa-regular text-gray-400 hover:text-yellow-500'}"
				></i>
			</button>
		</div>
	</div>

	<!-- Add task form (inline, shown when clicking +) -->
	{#if showAddForm}
		<div class="mb-3 p-2 bg-gray-50 dark:bg-gray-700/50 rounded-lg space-y-2" transition:fade={{ duration: 150 }}>
			<input
				type="text"
				bind:value={newTaskTitle}
				onkeydown={handleAddKeydown}
				placeholder="Task description..."
				class="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-lg
					bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200
					focus:ring-2 focus:ring-purple-500 focus:border-transparent"
			/>
			<div class="flex items-center gap-2">
				<select
					bind:value={newTaskCategory}
					class="flex-1 px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg
						bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200
						focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				>
					{#each Object.entries(TASK_CATEGORY_CONFIG) as [key, config]}
						<option value={key}>{config.label}</option>
					{/each}
				</select>
				<button
					onclick={handleAddTask}
					disabled={!newTaskTitle.trim()}
					class="px-3 py-1.5 text-sm bg-purple-500 text-white rounded-lg hover:bg-purple-600
						disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
				>
					Add
				</button>
				<button
					onclick={() => {
						showAddForm = false;
						newTaskTitle = '';
					}}
					class="px-2 py-1.5 text-sm text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 transition-colors"
				>
					<i class="fa-solid fa-xmark"></i>
				</button>
			</div>
		</div>
	{/if}

	<!-- Active Task list -->
	<ul class="space-y-2 flex-1 overflow-y-auto">
		{#each $activeTasks as task (task.id)}
			{@const config = getCategoryConfig(task.category)}
			<li
				animate:flip={{ duration: 300 }}
				draggable={editingTaskId !== task.id}
				ondragstart={(e) => handleDragStart(e, task.id)}
				ondragover={(e) => handleDragOver(e, task.id)}
				ondragleave={handleDragLeave}
				ondrop={(e) => handleDrop(e, task.id)}
				ondragend={handleDragEnd}
				ondblclick={(e) => handleDoubleClick(e, task.id, task.title)}
				class="group p-2.5 {config.bgLight} {config.bgDark} rounded-lg text-sm transition-all duration-200
					{editingTaskId !== task.id ? 'cursor-grab active:cursor-grabbing' : ''}
					{draggedTaskId === task.id ? 'opacity-50 scale-95' : ''}
					{dragOverTaskId === task.id ? 'ring-2 ring-purple-400 ring-offset-1' : ''}"
			>
				<div class="flex items-start gap-2">
					<!-- Checkbox -->
					<button
						onclick={(e) => handleCheckboxClick(e, task.id)}
						class="relative flex items-center justify-center mt-0.5 cursor-pointer"
						type="button"
					>
						<div
							class="w-5 h-5 border-2 rounded-md transition-all flex items-center justify-center {config.checkboxBorder} {config.checkboxHover}"
						>
						</div>
					</button>

					<!-- Task content -->
					<div class="flex-1 min-w-0">
						{#if editingTaskId === task.id}
							<!-- Edit mode -->
							<input
								type="text"
								bind:value={editingTitle}
								onkeydown={handleEditKeydown}
								onblur={saveEdit}
								class="w-full px-2 py-1 text-sm border border-purple-400 rounded bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:ring-2 focus:ring-purple-500 focus:outline-none"
								autofocus
							/>
						{:else}
							<div class="flex items-center gap-1.5">
								{#if task.pinned}
									<i class="fa-solid fa-thumbtack text-purple-500 text-xs" title="Pinned"></i>
								{/if}
								{#if task.starred}
									<i class="fa-solid fa-star text-yellow-500 text-xs" title="Starred"></i>
								{/if}
								<span class="text-gray-700 dark:text-gray-300">
									{task.title}
								</span>
							</div>
							<!-- Category tag -->
							<div class="flex items-center gap-1 mt-1">
								<span
									class="inline-flex items-center gap-1 text-xs px-1.5 py-0.5 rounded bg-white/60 dark:bg-gray-800/60 text-gray-600 dark:text-gray-400"
								>
									<i class="fa-solid {config.icon} {config.iconColor}"></i>
									{config.label}
								</span>
							</div>
						{/if}
					</div>

					<!-- Action buttons (visible on hover) -->
					{#if editingTaskId !== task.id}
						<div class="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
							<!-- Star button -->
							<button
								onclick={(e) => handleStarClick(e, task.id)}
								class="p-1 rounded hover:bg-white/50 dark:hover:bg-gray-600/50 transition-colors"
								title={task.starred ? 'Unstar' : 'Star'}
								type="button"
							>
								<i
									class="fa-star text-xs transition-colors duration-200 {task.starred
										? 'fa-solid text-yellow-500'
										: 'fa-regular text-gray-400 hover:text-yellow-500'}"
								></i>
							</button>

							<!-- Pin button -->
							<button
								onclick={(e) => handlePinClick(e, task.id)}
								class="p-1 rounded hover:bg-white/50 dark:hover:bg-gray-600/50 transition-colors"
								title={task.pinned ? 'Unpin' : 'Pin to top'}
								type="button"
							>
								<i
									class="fa-solid fa-thumbtack text-xs transition-colors duration-200 {task.pinned
										? 'text-purple-500'
										: 'text-gray-400 hover:text-purple-500'}"
								></i>
							</button>

							<!-- Delete button -->
							<button
								onclick={(e) => handleDeleteClick(e, task.id)}
								class="p-1 rounded hover:bg-red-100 dark:hover:bg-red-900/50 transition-colors"
								title="Delete task"
								type="button"
							>
								<i class="fa-solid fa-trash-can text-xs text-gray-400 hover:text-red-500"></i>
							</button>
						</div>
					{/if}
				</div>
			</li>
		{/each}

		{#if $activeTasks.length === 0}
			<li class="text-center text-gray-400 dark:text-gray-500 py-4 text-sm">
				{#if $showStarredOnly}
					No starred tasks
				{:else}
					No active tasks
				{/if}
			</li>
		{/if}
	</ul>

	<!-- Archived/Completed Tasks Section -->
	{#if $completedTasksList.length > 0}
		<div class="mt-3 pt-3 border-t border-gray-200 dark:border-gray-700">
			<button
				onclick={() => (showArchive = !showArchive)}
				class="w-full flex items-center justify-between text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 transition-colors"
				type="button"
			>
				<span class="flex items-center gap-2">
					<i class="fa-solid fa-box-archive"></i>
					Completed ({$completedTasksList.length})
				</span>
				<i class="fa-solid fa-chevron-down text-xs transition-transform duration-200 {showArchive ? 'rotate-180' : ''}"></i>
			</button>

			{#if showArchive}
				<ul class="mt-2 space-y-1.5" transition:fade={{ duration: 150 }}>
					{#each $completedTasksList as task (task.id)}
						{@const config = getCategoryConfig(task.category)}
						<li
							class="group p-2 {config.bgLight} {config.bgDark} rounded-lg text-sm opacity-60"
						>
							<div class="flex items-start gap-2">
								<!-- Checkbox (checked) -->
								<button
									onclick={(e) => handleCheckboxClick(e, task.id)}
									class="relative flex items-center justify-center mt-0.5 cursor-pointer"
									type="button"
								>
									<div
										class="w-5 h-5 border-2 rounded-md transition-all flex items-center justify-center {config.checkboxBg}"
									>
										<i class="fa-solid fa-check text-white text-xs"></i>
									</div>
								</button>

								<!-- Task content -->
								<div class="flex-1 min-w-0">
									<span class="text-gray-500 dark:text-gray-400 line-through">
										{task.title}
									</span>
								</div>

								<!-- Delete button -->
								<button
									onclick={(e) => handleDeleteClick(e, task.id)}
									class="p-1 rounded hover:bg-red-100 dark:hover:bg-red-900/50 transition-colors opacity-0 group-hover:opacity-100"
									title="Delete task"
									type="button"
								>
									<i class="fa-solid fa-trash-can text-xs text-gray-400 hover:text-red-500"></i>
								</button>
							</div>
						</li>
					{/each}
				</ul>
			{/if}
		</div>
	{/if}
</div>
