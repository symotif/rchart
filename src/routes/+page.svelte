<script lang="ts">
	import { onMount } from 'svelte';
	import Greet from '../lib/greet.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Card from './card.svelte';
	import Calendar from './calendar.svelte';

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
	class="absolute top-20 left-20 right-10 bottom-5 my-4 ml-5 mr-3 flex flex-row justify-between gap-5"
>
	<!-- appointments -->
	<div class="bg-gray-50 rounded-lg shadow-l grow mt-5">
		<div class="ml-5 mt-3">
			<p class="text-left font-bold text-3xl">Schedule</p>
			<div>
				<p>Dr. Madeline Chu, MD</p>
				<p class="mt-5 mb-1 font-bold text-lg">Wednesday 9/15</p>
			</div>

			{#each appointments as appointment}
			<div class="mb-3">
				<Card data={appointment}></Card>
			</div>
				
			{/each}
		</div>
	</div>

	<div class="flex flex-col gap-5 w-1/3 mt-4">
		<!-- calendar-->
		<div class="bg-gray-50 rounded-lg shadow-xl h-2/4">
			<div class="calendar-container">
				<div class="calendar-header">
					<h1>
						<button onclick={() => year--}>&Lt;</button>
						<button onclick={() => prev()}>&lt;</button>
						{monthNames[month]}
						{year}
						<button onclick={() => next()}>&gt;</button>
						<button onclick={() => year++}>&Gt;</button>
					</h1>
					{eventText}
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
			<div class="ml-5 mt-3">
				<h1 class="text-left">{month}</h1>
				<p>
					Lorem ipsum dolor sit, amet consectetur adipisicing elit. Vero suscipit quae
					magni, nisi autem cupiditate necessitatibus molestias, delectus iste, mollitia
					debitis repellendus! Modi, eius sit. Rerum praesentium error vero natus!
				</p>
			</div>
		</div>

		<!-- stats -->
		<div class="bg-gray-50 rounded-lg shadow-xl grow">
			<div class="ml-5 mt-3">
				<h1 class="text-left">Work</h1>
				<p>
					patients per day, time spent with each patient, time spent charting, Large
					average + small graph over the past X days
				</p>

				<h1 class="text-left">Patients</h1>
				<p>
					total census, patients per day, time spent with each patient, time spent
					charting, Large average + small graph over the past X days
				</p>

				<h1 class="text-left">Wellness</h1>
				<p>
					hours worked per day, time since last outside, time since last vacation, Large
					average + small graph over the past X days
				</p>

				<h1 class="text-left">Finance</h1>
				<p>daily burn, revenue Large average + small graph over the past X days</p>
			</div>
		</div>
	</div>

	<!-- tasks -->
	<div class="bg-gray-50 rounded-lg shadow-xl w-1/5 mt-4">
		<div class="ml-5 mt-3">
			<h1 class="text-left">My Tasks</h1>
			<p>
				Lorem ipsum dolor sit, amet consectetur adipisicing elit. Vero suscipit quae magni,
				nisi autem cupiditate necessitatibus molestias, delectus iste, mollitia debitis
				repellendus! Modi, eius sit. Rerum praesentium error vero natus!
			</p>
		</div>
	</div>
</div>

<Greet />

<style>
</style>
