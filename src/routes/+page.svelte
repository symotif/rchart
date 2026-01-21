<script lang="ts">
	import { onMount } from 'svelte';
	import Greet from '../lib/greet.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Card from './card.svelte';
	import Calendar from './calendar.svelte';

	// Panel sizes (persisted in state)
	let leftPanelWidth = $state(480);
	let rightPanelWidth = $state(250);
	let calendarHeight = $state(350);

	// Resize state
	let isResizingLeft = $state(false);
	let isResizingRight = $state(false);
	let isResizingCalendar = $state(false);

	function startResizeLeft(e: MouseEvent) {
		e.preventDefault();
		isResizingLeft = true;
		const startX = e.clientX;
		const startWidth = leftPanelWidth;

		function onMouseMove(e: MouseEvent) {
			const delta = e.clientX - startX;
			leftPanelWidth = Math.max(200, Math.min(800, startWidth + delta));
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
			rightPanelWidth = Math.max(150, Math.min(500, startWidth + delta));
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
			calendarHeight = Math.max(200, Math.min(600, startHeight + delta));
		}

		function onMouseUp() {
			isResizingCalendar = false;
			document.removeEventListener('mousemove', onMouseMove);
			document.removeEventListener('mouseup', onMouseUp);
		}

		document.addEventListener('mousemove', onMouseMove);
		document.addEventListener('mouseup', onMouseUp);
	}

	let appointments = $state([
		{ name: 'Logan', age: '24', sex: 'Male', time: '12:30' },
		{ name: 'John', age: '14', sex: 'Female', time: '1:30' },
		{ name: 'Max', age: '50', sex: 'Male', time: '2:00' },
		{ name: 'Sam', age: '22', sex: 'Nonbinary', time: '2:30' },
		{ name: 'Ben', age: '23', sex: 'Male', time: '3:30' }
	]);

	async function get_month() {
		month = await invoke('get_month');
	}

	// calendar code:
	const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
	const monthNames = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	];

	let headers = $state([]);
	const now = new Date();
	let year = $state(now.getFullYear());
	let month = $state(now.getMonth());
	let eventText = $state('Click an item or date');

	let days = $state([]); //	The days to display in each box
	let items = $state([]);

	function randInt(max: number) {
		return Math.floor(Math.random() * max) + 1;
	}

	//	The Calendar Component just displays stuff in a row & column. It has no knowledge of dates.
	//	The items[] below are placed (by you) in a specified row & column of the calendar.
	//	You need to call findRowCol() to calc the row/col based on each items start date. Each date box has a Date() property.
	//	And, if an item overlaps rows, then you need to add a 2nd item on the subsequent row.

	function initMonthItems() {
		let y = year;
		let m = month;
		let d1 = new Date(y, m, randInt(7) + 7);
		items = [
			{
				title: '11:00 Task Early in month',
				className: 'task--primary',
				date: new Date(y, m, randInt(6)),
				len: randInt(4) + 1
			},
			{ title: '7:30 Wk 2 tasks', className: 'task--warning', date: d1, len: randInt(4) + 2 },
			{
				title: 'Overlapping Stuff (isBottom:true)',
				date: d1,
				className: 'task--info',
				len: 4,
				isBottom: true
			},
			{
				title: '10:00 More Stuff to do',
				date: new Date(y, m, randInt(7) + 14),
				className: 'task--info',
				len: randInt(4) + 1,
				detailHeader: 'Difficult',
				detailContent: 'But not especially so'
			},
			{
				title: 'All day task',
				date: new Date(y, m, randInt(7) + 21),
				className: 'task--danger',
				len: 1,
				vlen: 2
			}
		];

		//This is where you calc the row/col to put each dated item
		for (let i of items) {
			let rc = findRowCol(i.date);
			if (rc == null) {
				console.log('didn`t find date for ', i);
				console.log(i.date);
				console.log(days);
				i.startCol = i.startRow = 0;
			} else {
				i.startCol = rc.col;
				i.startRow = rc.row;
			}
		}
	}

	// Initialize content on mount
	onMount(() => {
		initContent();
	});

	// choose what date/day gets displayed in each date box.
	function initContent() {
		headers = dayNames;
		initMonth();
		initMonthItems();
	}

	function initMonth() {
		days = [];
		let monthAbbrev = monthNames[month].slice(0, 3);
		let nextMonthAbbrev = monthNames[(month + 1) % 12].slice(0, 3);
		//	find the last Monday of the previous month
		const firstDay = new Date(year, month, 1).getDay();
		//console.log('fd='+firstDay+' '+dayNames[firstDay]);
		const daysInThisMonth = new Date(year, month + 1, 0).getDate();
		const daysInLastMonth = new Date(year, month, 0).getDate();
		const prevMonth = month == 0 ? 11 : month - 1;

		//	show the days before the start of this month (disabled) - always less than 7
		for (let i = daysInLastMonth - firstDay; i < daysInLastMonth; i++) {
			let d = new Date(prevMonth == 11 ? year - 1 : year, prevMonth, i + 1);
			days.push({ name: '' + (i + 1), enabled: false, date: d });
		}
		//	show the days in this month (enabled) - always 28 - 31
		for (let i = 0; i < daysInThisMonth; i++) {
			let d = new Date(year, month, i + 1);
			if (i == 0) days.push({ name: monthAbbrev + ' ' + (i + 1), enabled: true, date: d });
			else days.push({ name: '' + (i + 1), enabled: true, date: d });
			//console.log('i='+i+'  dt is '+d+' date() is '+d.getDate());
		}
		//	show any days to fill up the last row (disabled) - always less than 7
		for (let i = 0; days.length % 7; i++) {
			let d = new Date(month == 11 ? year + 1 : year, (month + 1) % 12, i + 1);
			if (i == 0)
				days.push({ name: nextMonthAbbrev + ' ' + (i + 1), enabled: false, date: d });
			else days.push({ name: '' + (i + 1), enabled: false, date: d });
		}
	}

	function findRowCol(dt: Date) {
		for (let i = 0; i < days.length; i++) {
			let d = days[i].date;
			if (
				d.getFullYear() === dt.getFullYear() &&
				d.getMonth() === dt.getMonth() &&
				d.getDate() === dt.getDate()
			)
				return { row: Math.floor(i / 7) + 2, col: (i % 7) + 1 };
		}
		return null;
	}

	function itemClick(detail: any) {
		eventText = 'itemClick ' + JSON.stringify(detail) + ' localtime=' + detail.date.toString();
	}
	function dayClick(detail: any) {
		eventText = 'onDayClick ' + JSON.stringify(detail) + ' localtime=' + detail.date.toString();
	}
	function headerClick(detail: any) {
		eventText = 'onHheaderClick ' + JSON.stringify(detail);
	}
	function next() {
		month++;
		if (month == 12) {
			year++;
			month = 0;
		}
		initContent();
	}
	function prev() {
		if (month == 0) {
			month = 11;
			year--;
		} else {
			month--;
		}
		initContent();
	}
</script>

<svelte:head>
	<script src="https://d3js.org/d3.v7.min.js"></script>
	<title>Home</title>
	<meta name="description" content="an ehr for the world" />
</svelte:head>

<!-- page content -->
<div
	class="dashboard-container absolute top-20 left-20 right-10 bottom-10 my-4 ml-5 mr-3 flex flex-row"
	class:resizing={isResizingLeft || isResizingRight || isResizingCalendar}
>
	<!-- LEFT PANEL: Schedule/Appointments -->
	<div class="panel bg-gray-50 dark:bg-gray-800 rounded-lg shadow-lg mt-5 overflow-auto" style="width: {leftPanelWidth}px; flex-shrink: 0;">
		<div class="ml-5 mt-3 mr-3">
			<p class="text-left font-bold text-3xl text-gray-800 dark:text-gray-100">Schedule</p>
			<div>
				<p class="text-gray-700 dark:text-gray-300">Dr. Madeline Chu, MD</p>
				<p class="mt-5 mb-1 font-bold text-lg text-gray-800 dark:text-gray-100">Wednesday 9/15</p>
			</div>

			{#each appointments as appointment}
			<div class="mb-3 flex-shrink-0">
				<Card data={appointment}></Card>
			</div>
			{/each}
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
		<div class="panel bg-gray-50 dark:bg-gray-800 rounded-lg shadow-xl overflow-auto" style="height: {calendarHeight}px; flex-shrink: 0;">
			<div class="calendar-container p-3">
				<div class="calendar-header">
					<h1 class="text-gray-800 dark:text-gray-100">
						<button onclick={() => year--} class="text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-gray-100">&Lt;</button>
						<button onclick={() => prev()} class="text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-gray-100">&lt;</button>
						{monthNames[month]}
						{year}
						<button onclick={() => next()} class="text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-gray-100">&gt;</button>
						<button onclick={() => year++} class="text-gray-600 dark:text-gray-300 hover:text-gray-800 dark:hover:text-gray-100">&Gt;</button>
					</h1>
					<span class="text-sm text-gray-500 dark:text-gray-400">{eventText}</span>
				</div>

				<Calendar
					{headers}
					{days}
					{items}
					ondayClick={dayClick}
					onitemClick={itemClick}
					onheaderClick={headerClick}
				/>
			</div>
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

		<!-- Stats -->
		<div class="panel bg-gray-50 dark:bg-gray-800 rounded-lg shadow-xl flex-1 overflow-auto min-h-0">
			<div class="ml-5 mt-3 mr-3">
				<h1 class="text-left font-bold text-xl text-gray-800 dark:text-gray-100">Work</h1>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					patients per day, time spent with each patient, time spent charting
				</p>

				<h1 class="text-left font-bold text-xl mt-3 text-gray-800 dark:text-gray-100">Patients</h1>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					total census, patients per day, time spent charting
				</p>

				<h1 class="text-left font-bold text-xl mt-3 text-gray-800 dark:text-gray-100">Wellness</h1>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					hours worked per day, time since last outside
				</p>

				<h1 class="text-left font-bold text-xl mt-3 text-gray-800 dark:text-gray-100">Finance</h1>
				<p class="text-sm text-gray-600 dark:text-gray-400">daily burn, revenue</p>
			</div>
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
	<div class="panel bg-gray-50 dark:bg-gray-800 rounded-lg shadow-xl mt-4 overflow-auto" style="width: {rightPanelWidth}px; flex-shrink: 0;">
		<div class="ml-5 mt-3 mr-3">
			<h1 class="text-left font-bold text-xl text-gray-800 dark:text-gray-100">My Tasks</h1>
			<p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
				Your pending tasks and action items will appear here.
			</p>
			<ul class="mt-3 space-y-2">
				<li class="p-2 bg-blue-50 dark:bg-blue-900/30 rounded text-sm text-gray-700 dark:text-gray-300">Review patient charts</li>
				<li class="p-2 bg-yellow-50 dark:bg-yellow-900/30 rounded text-sm text-gray-700 dark:text-gray-300">Follow up with lab results</li>
				<li class="p-2 bg-green-50 dark:bg-green-900/30 rounded text-sm text-gray-700 dark:text-gray-300">Complete documentation</li>
			</ul>
		</div>
	</div>
</div>

<Greet />

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

	.calendar-container {
		overflow: auto;
	}

	.calendar-header h1 {
		font-size: 1.1rem;
		text-align: center;
	}
</style>
