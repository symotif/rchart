<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { goto } from '$app/navigation';
	import { addTab } from '../../../stores/TabStore';
	import { setTab } from '../../../stores/SideBarStore';

	interface Props {
		todayAppointments: number;
		completedAppointments: number;
		telehealthCount: number;
	}

	let { todayAppointments, completedAppointments, telehealthCount }: Props = $props();

	let productivityGraphContainer: HTMLDivElement | null = $state(null);
	let populationGraphContainer: HTMLDivElement | null = $state(null);

	// Dummy data for patient panel statistics
	const sickestPatients = [
		{ id: 1, name: 'Robert Martinez', issue: 'A1c: 11.2%', severity: 'critical', detail: 'Uncontrolled T2DM, last A1c 11.2%, target <7%' },
		{ id: 2, name: 'Dorothy Williams', issue: 'BP: 178/102', severity: 'critical', detail: 'Resistant HTN, on 4 agents, non-adherent' },
		{ id: 3, name: 'James Thompson', issue: '3 ED visits (30d)', severity: 'high', detail: 'CHF exacerbations, frequent readmissions' },
		{ id: 4, name: 'Linda Brown', issue: 'A1c: 9.8%', severity: 'high', detail: 'T2DM, neuropathy progressing' },
		{ id: 5, name: 'William Davis', issue: 'BP: 165/95', severity: 'moderate', detail: 'Stage 2 HTN, newly diagnosed' }
	];

	const hospitalizedPatients = [
		{ id: 6, name: 'Margaret Wilson', room: 'ICU 302', reason: 'STEMI', admitDate: '2025-01-27', daysIn: 2 },
		{ id: 7, name: 'George Anderson', room: '4th Floor 412', reason: 'CHF Exacerbation', admitDate: '2025-01-25', daysIn: 4 }
	];

	const atRiskPatients = [
		{ id: 8, name: 'Patricia Garcia', risk: 'High', reason: 'Multiple comorbidities, recent fall, lives alone' },
		{ id: 9, name: 'Richard Lee', risk: 'High', reason: 'COPD exacerbation x2 in 6mo, low O2 sats' },
		{ id: 10, name: 'Susan Taylor', risk: 'Moderate', reason: 'New CHF diagnosis, learning medications' }
	];

	// Productivity data (last 6 months)
	const productivityData = [
		{ month: 'Aug', visits: 145, target: 160 },
		{ month: 'Sep', visits: 162, target: 160 },
		{ month: 'Oct', visits: 158, target: 160 },
		{ month: 'Nov', visits: 171, target: 165 },
		{ month: 'Dec', visits: 148, target: 165 },
		{ month: 'Jan', visits: 134, target: 165 }
	];

	// Patient population over time
	const populationData = [
		{ month: 'Aug', total: 1245, active: 1180, new: 28 },
		{ month: 'Sep', total: 1268, active: 1195, new: 35 },
		{ month: 'Oct', total: 1289, active: 1210, new: 31 },
		{ month: 'Nov', total: 1312, active: 1235, new: 42 },
		{ month: 'Dec', total: 1328, active: 1248, new: 24 },
		{ month: 'Jan', total: 1356, active: 1278, new: 38 }
	];

	function getSeverityColor(severity: string): string {
		switch (severity) {
			case 'critical': return 'bg-red-100 text-red-700 dark:bg-red-900/50 dark:text-red-300';
			case 'high': return 'bg-orange-100 text-orange-700 dark:bg-orange-900/50 dark:text-orange-300';
			case 'moderate': return 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/50 dark:text-yellow-300';
			default: return 'bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300';
		}
	}

	function navigateToPatientList(filter: string) {
		const tab = {
			id: `list-${filter}`,
			title: `Patient List - ${filter}`,
			path: `/list?filter=${filter}`
		};
		addTab(tab);
		setTab(1); // Patient List sidebar tab
		goto(tab.path);
	}

	function navigateToPatient(patientId: number, patientName: string) {
		const tab = {
			id: `patient-${patientId}`,
			title: patientName,
			path: `/patient/${patientId}`
		};
		addTab(tab);
		goto(tab.path);
	}

	async function drawProductivityGraph() {
		await tick();
		if (!productivityGraphContainer) return;

		productivityGraphContainer.innerHTML = '';

		const width = productivityGraphContainer.clientWidth;
		const height = productivityGraphContainer.clientHeight;
		const margin = { top: 20, right: 20, bottom: 30, left: 40 };
		const innerWidth = width - margin.left - margin.right;
		const innerHeight = height - margin.top - margin.bottom;

		const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
		svg.setAttribute('width', String(width));
		svg.setAttribute('height', String(height));

		const maxVisits = Math.max(...productivityData.map(d => Math.max(d.visits, d.target))) + 10;
		const barWidth = (innerWidth / productivityData.length) * 0.6;
		const gap = (innerWidth / productivityData.length) * 0.4;

		// Y-axis
		const yAxis = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		yAxis.setAttribute('x1', String(margin.left));
		yAxis.setAttribute('y1', String(margin.top));
		yAxis.setAttribute('x2', String(margin.left));
		yAxis.setAttribute('y2', String(margin.top + innerHeight));
		yAxis.setAttribute('stroke', '#9ca3af');
		svg.appendChild(yAxis);

		// X-axis
		const xAxis = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		xAxis.setAttribute('x1', String(margin.left));
		xAxis.setAttribute('y1', String(margin.top + innerHeight));
		xAxis.setAttribute('x2', String(margin.left + innerWidth));
		xAxis.setAttribute('y2', String(margin.top + innerHeight));
		xAxis.setAttribute('stroke', '#9ca3af');
		svg.appendChild(xAxis);

		// Y-axis labels
		const yTicks = [0, maxVisits / 2, maxVisits];
		for (const tick of yTicks) {
			const y = margin.top + innerHeight - (tick / maxVisits) * innerHeight;
			const label = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			label.setAttribute('x', String(margin.left - 8));
			label.setAttribute('y', String(y + 4));
			label.setAttribute('text-anchor', 'end');
			label.setAttribute('font-size', '10');
			label.setAttribute('fill', '#6b7280');
			label.textContent = String(Math.round(tick));
			svg.appendChild(label);
		}

		// Draw bars and labels
		productivityData.forEach((d, i) => {
			const x = margin.left + i * (barWidth + gap) + gap / 2;
			const barHeight = (d.visits / maxVisits) * innerHeight;
			const targetHeight = (d.target / maxVisits) * innerHeight;

			// Target line (dashed)
			const targetLine = document.createElementNS('http://www.w3.org/2000/svg', 'line');
			targetLine.setAttribute('x1', String(x - 5));
			targetLine.setAttribute('y1', String(margin.top + innerHeight - targetHeight));
			targetLine.setAttribute('x2', String(x + barWidth + 5));
			targetLine.setAttribute('y2', String(margin.top + innerHeight - targetHeight));
			targetLine.setAttribute('stroke', '#ef4444');
			targetLine.setAttribute('stroke-width', '2');
			targetLine.setAttribute('stroke-dasharray', '4,2');
			svg.appendChild(targetLine);

			// Bar
			const bar = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
			bar.setAttribute('x', String(x));
			bar.setAttribute('y', String(margin.top + innerHeight - barHeight));
			bar.setAttribute('width', String(barWidth));
			bar.setAttribute('height', String(barHeight));
			bar.setAttribute('fill', d.visits >= d.target ? '#10b981' : '#f59e0b');
			bar.setAttribute('rx', '4');
			svg.appendChild(bar);

			// Value label on bar
			const valueLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			valueLabel.setAttribute('x', String(x + barWidth / 2));
			valueLabel.setAttribute('y', String(margin.top + innerHeight - barHeight - 5));
			valueLabel.setAttribute('text-anchor', 'middle');
			valueLabel.setAttribute('font-size', '10');
			valueLabel.setAttribute('font-weight', 'bold');
			valueLabel.setAttribute('fill', d.visits >= d.target ? '#10b981' : '#f59e0b');
			valueLabel.textContent = String(d.visits);
			svg.appendChild(valueLabel);

			// Month label
			const monthLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			monthLabel.setAttribute('x', String(x + barWidth / 2));
			monthLabel.setAttribute('y', String(margin.top + innerHeight + 15));
			monthLabel.setAttribute('text-anchor', 'middle');
			monthLabel.setAttribute('font-size', '10');
			monthLabel.setAttribute('fill', '#6b7280');
			monthLabel.textContent = d.month;
			svg.appendChild(monthLabel);
		});

		// Legend
		const legendY = 8;
		const leg1 = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
		leg1.setAttribute('x', String(margin.left));
		leg1.setAttribute('y', String(legendY));
		leg1.setAttribute('width', '12');
		leg1.setAttribute('height', '12');
		leg1.setAttribute('fill', '#10b981');
		leg1.setAttribute('rx', '2');
		svg.appendChild(leg1);
		const leg1Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		leg1Text.setAttribute('x', String(margin.left + 16));
		leg1Text.setAttribute('y', String(legendY + 10));
		leg1Text.setAttribute('font-size', '9');
		leg1Text.setAttribute('fill', '#6b7280');
		leg1Text.textContent = 'Met Target';
		svg.appendChild(leg1Text);

		const leg2 = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
		leg2.setAttribute('x', String(margin.left + 70));
		leg2.setAttribute('y', String(legendY));
		leg2.setAttribute('width', '12');
		leg2.setAttribute('height', '12');
		leg2.setAttribute('fill', '#f59e0b');
		leg2.setAttribute('rx', '2');
		svg.appendChild(leg2);
		const leg2Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		leg2Text.setAttribute('x', String(margin.left + 86));
		leg2Text.setAttribute('y', String(legendY + 10));
		leg2Text.setAttribute('font-size', '9');
		leg2Text.setAttribute('fill', '#6b7280');
		leg2Text.textContent = 'Below Target';
		svg.appendChild(leg2Text);

		const leg3Line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		leg3Line.setAttribute('x1', String(margin.left + 150));
		leg3Line.setAttribute('y1', String(legendY + 6));
		leg3Line.setAttribute('x2', String(margin.left + 165));
		leg3Line.setAttribute('y2', String(legendY + 6));
		leg3Line.setAttribute('stroke', '#ef4444');
		leg3Line.setAttribute('stroke-width', '2');
		leg3Line.setAttribute('stroke-dasharray', '4,2');
		svg.appendChild(leg3Line);
		const leg3Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		leg3Text.setAttribute('x', String(margin.left + 170));
		leg3Text.setAttribute('y', String(legendY + 10));
		leg3Text.setAttribute('font-size', '9');
		leg3Text.setAttribute('fill', '#6b7280');
		leg3Text.textContent = 'Target';
		svg.appendChild(leg3Text);

		productivityGraphContainer.appendChild(svg);
	}

	async function drawPopulationGraph() {
		await tick();
		if (!populationGraphContainer) return;

		populationGraphContainer.innerHTML = '';

		const width = populationGraphContainer.clientWidth;
		const height = populationGraphContainer.clientHeight;
		const margin = { top: 20, right: 20, bottom: 30, left: 50 };
		const innerWidth = width - margin.left - margin.right;
		const innerHeight = height - margin.top - margin.bottom;

		const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
		svg.setAttribute('width', String(width));
		svg.setAttribute('height', String(height));

		const maxTotal = Math.max(...populationData.map(d => d.total)) + 50;
		const minTotal = Math.min(...populationData.map(d => d.active)) - 50;

		const xScale = (i: number) => margin.left + (i / (populationData.length - 1)) * innerWidth;
		const yScale = (value: number) => margin.top + innerHeight - ((value - minTotal) / (maxTotal - minTotal)) * innerHeight;

		// Y-axis
		const yAxis = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		yAxis.setAttribute('x1', String(margin.left));
		yAxis.setAttribute('y1', String(margin.top));
		yAxis.setAttribute('x2', String(margin.left));
		yAxis.setAttribute('y2', String(margin.top + innerHeight));
		yAxis.setAttribute('stroke', '#9ca3af');
		svg.appendChild(yAxis);

		// X-axis
		const xAxis = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		xAxis.setAttribute('x1', String(margin.left));
		xAxis.setAttribute('y1', String(margin.top + innerHeight));
		xAxis.setAttribute('x2', String(margin.left + innerWidth));
		xAxis.setAttribute('y2', String(margin.top + innerHeight));
		xAxis.setAttribute('stroke', '#9ca3af');
		svg.appendChild(xAxis);

		// Y-axis labels
		const yTicks = [minTotal, (minTotal + maxTotal) / 2, maxTotal];
		for (const tick of yTicks) {
			const y = yScale(tick);
			const label = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			label.setAttribute('x', String(margin.left - 8));
			label.setAttribute('y', String(y + 4));
			label.setAttribute('text-anchor', 'end');
			label.setAttribute('font-size', '10');
			label.setAttribute('fill', '#6b7280');
			label.textContent = String(Math.round(tick));
			svg.appendChild(label);
		}

		// Draw total patients line
		const totalPath = populationData.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(i)} ${yScale(d.total)}`).join(' ');
		const totalLine = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		totalLine.setAttribute('d', totalPath);
		totalLine.setAttribute('fill', 'none');
		totalLine.setAttribute('stroke', '#3b82f6');
		totalLine.setAttribute('stroke-width', '2');
		svg.appendChild(totalLine);

		// Draw active patients line
		const activePath = populationData.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(i)} ${yScale(d.active)}`).join(' ');
		const activeLine = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		activeLine.setAttribute('d', activePath);
		activeLine.setAttribute('fill', 'none');
		activeLine.setAttribute('stroke', '#10b981');
		activeLine.setAttribute('stroke-width', '2');
		svg.appendChild(activeLine);

		// Draw points and labels
		populationData.forEach((d, i) => {
			// Total point
			const totalCircle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			totalCircle.setAttribute('cx', String(xScale(i)));
			totalCircle.setAttribute('cy', String(yScale(d.total)));
			totalCircle.setAttribute('r', '4');
			totalCircle.setAttribute('fill', '#3b82f6');
			svg.appendChild(totalCircle);

			// Active point
			const activeCircle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			activeCircle.setAttribute('cx', String(xScale(i)));
			activeCircle.setAttribute('cy', String(yScale(d.active)));
			activeCircle.setAttribute('r', '4');
			activeCircle.setAttribute('fill', '#10b981');
			svg.appendChild(activeCircle);

			// Month label
			const monthLabel = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			monthLabel.setAttribute('x', String(xScale(i)));
			monthLabel.setAttribute('y', String(margin.top + innerHeight + 15));
			monthLabel.setAttribute('text-anchor', 'middle');
			monthLabel.setAttribute('font-size', '10');
			monthLabel.setAttribute('fill', '#6b7280');
			monthLabel.textContent = d.month;
			svg.appendChild(monthLabel);
		});

		// Legend
		const legendY = 8;
		const leg1 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
		leg1.setAttribute('cx', String(margin.left + 6));
		leg1.setAttribute('cy', String(legendY + 6));
		leg1.setAttribute('r', '4');
		leg1.setAttribute('fill', '#3b82f6');
		svg.appendChild(leg1);
		const leg1Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		leg1Text.setAttribute('x', String(margin.left + 14));
		leg1Text.setAttribute('y', String(legendY + 10));
		leg1Text.setAttribute('font-size', '9');
		leg1Text.setAttribute('fill', '#6b7280');
		leg1Text.textContent = 'Total Panel';
		svg.appendChild(leg1Text);

		const leg2 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
		leg2.setAttribute('cx', String(margin.left + 76));
		leg2.setAttribute('cy', String(legendY + 6));
		leg2.setAttribute('r', '4');
		leg2.setAttribute('fill', '#10b981');
		svg.appendChild(leg2);
		const leg2Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		leg2Text.setAttribute('x', String(margin.left + 84));
		leg2Text.setAttribute('y', String(legendY + 10));
		leg2Text.setAttribute('font-size', '9');
		leg2Text.setAttribute('fill', '#6b7280');
		leg2Text.textContent = 'Active (seen in 12mo)';
		svg.appendChild(leg2Text);

		populationGraphContainer.appendChild(svg);
	}

	onMount(() => {
		drawProductivityGraph();
		drawPopulationGraph();
	});
</script>

<div class="h-full flex flex-col p-4">
	<h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-3">Dashboard</h2>

	<div class="flex-1 min-h-0 overflow-y-auto space-y-4 pr-1">
		<!-- Quick Stats Row -->
		<div class="grid grid-cols-2 gap-3">
			<div class="bg-blue-50 dark:bg-blue-900/30 rounded-lg p-3 border border-blue-200 dark:border-blue-800">
				<div class="flex items-center gap-2">
					<div class="w-8 h-8 rounded-full bg-blue-500 flex items-center justify-center text-white text-sm">
						<i class="fa-solid fa-calendar-check"></i>
					</div>
					<div>
						<p class="text-xl font-bold text-blue-700 dark:text-blue-300">{todayAppointments}</p>
						<p class="text-xs text-gray-500 dark:text-gray-400">Today's Appts</p>
					</div>
				</div>
			</div>

			<div class="bg-green-50 dark:bg-green-900/30 rounded-lg p-3 border border-green-200 dark:border-green-800">
				<div class="flex items-center gap-2">
					<div class="w-8 h-8 rounded-full bg-green-500 flex items-center justify-center text-white text-sm">
						<i class="fa-solid fa-check"></i>
					</div>
					<div>
						<p class="text-xl font-bold text-green-700 dark:text-green-300">{todayAppointments - completedAppointments}</p>
						<p class="text-xs text-gray-500 dark:text-gray-400">Remaining</p>
					</div>
				</div>
			</div>

			<div class="bg-cyan-50 dark:bg-cyan-900/30 rounded-lg p-3 border border-cyan-200 dark:border-cyan-800">
				<div class="flex items-center gap-2">
					<div class="w-8 h-8 rounded-full bg-cyan-500 flex items-center justify-center text-white text-sm">
						<i class="fa-solid fa-video"></i>
					</div>
					<div>
						<p class="text-xl font-bold text-cyan-700 dark:text-cyan-300">{telehealthCount}</p>
						<p class="text-xs text-gray-500 dark:text-gray-400">Telehealth</p>
					</div>
				</div>
			</div>

			<div class="bg-purple-50 dark:bg-purple-900/30 rounded-lg p-3 border border-purple-200 dark:border-purple-800">
				<div class="flex items-center gap-2">
					<div class="w-8 h-8 rounded-full bg-purple-500 flex items-center justify-center text-white text-sm">
						<i class="fa-solid fa-users"></i>
					</div>
					<div>
						<p class="text-xl font-bold text-purple-700 dark:text-purple-300">1,356</p>
						<p class="text-xs text-gray-500 dark:text-gray-400">Panel Size</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Hospitalized & At Risk Cards -->
		<div class="grid grid-cols-2 gap-3">
			<button
				onclick={() => navigateToPatientList('hospitalized')}
				class="bg-red-50 dark:bg-red-900/30 rounded-lg p-3 text-left hover:bg-red-100 dark:hover:bg-red-900/50 transition-colors border border-red-200 dark:border-red-800"
			>
				<div class="flex items-center justify-between mb-2">
					<div class="flex items-center gap-2">
						<i class="fa-solid fa-hospital text-red-500"></i>
						<span class="text-sm font-semibold text-red-700 dark:text-red-300">Hospitalized</span>
					</div>
					<span class="text-2xl font-bold text-red-600 dark:text-red-400">{hospitalizedPatients.length}</span>
				</div>
				<div class="space-y-1">
					{#each hospitalizedPatients.slice(0, 2) as patient}
						<p class="text-xs text-red-600 dark:text-red-400 truncate">
							{patient.name} - {patient.reason}
						</p>
					{/each}
				</div>
			</button>

			<button
				onclick={() => navigateToPatientList('at-risk')}
				class="bg-orange-50 dark:bg-orange-900/30 rounded-lg p-3 text-left hover:bg-orange-100 dark:hover:bg-orange-900/50 transition-colors border border-orange-200 dark:border-orange-800"
			>
				<div class="flex items-center justify-between mb-2">
					<div class="flex items-center gap-2">
						<i class="fa-solid fa-triangle-exclamation text-orange-500"></i>
						<span class="text-sm font-semibold text-orange-700 dark:text-orange-300">At Risk</span>
					</div>
					<span class="text-2xl font-bold text-orange-600 dark:text-orange-400">{atRiskPatients.length}</span>
				</div>
				<div class="space-y-1">
					{#each atRiskPatients.slice(0, 2) as patient}
						<p class="text-xs text-orange-600 dark:text-orange-400 truncate">
							{patient.name} - {patient.risk} risk
						</p>
					{/each}
				</div>
			</button>
		</div>

		<!-- Sickest Patients -->
		<div class="bg-white dark:bg-gray-700/50 rounded-lg p-3 border border-gray-200 dark:border-gray-600">
			<div class="flex items-center justify-between mb-2">
				<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 flex items-center gap-2">
					<i class="fa-solid fa-heart-pulse text-red-500"></i>
					Patients Needing Attention
				</h3>
				<button
					onclick={() => navigateToPatientList('sickest')}
					class="text-xs text-blue-600 dark:text-blue-400 hover:underline"
				>
					View All
				</button>
			</div>
			<div class="space-y-2">
				{#each sickestPatients.slice(0, 3) as patient}
					<button
						onclick={() => navigateToPatient(patient.id, patient.name)}
						class="w-full flex items-center justify-between p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors text-left"
					>
						<div class="flex-1 min-w-0">
							<p class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">{patient.name}</p>
							<p class="text-xs text-gray-500 dark:text-gray-400 truncate">{patient.detail}</p>
						</div>
						<span class="px-2 py-0.5 text-xs font-medium rounded-full ml-2 flex-shrink-0 {getSeverityColor(patient.severity)}">
							{patient.issue}
						</span>
					</button>
				{/each}
			</div>
		</div>

		<!-- Productivity Graph -->
		<div class="bg-white dark:bg-gray-700/50 rounded-lg p-3 border border-gray-200 dark:border-gray-600">
			<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-2 flex items-center gap-2">
				<i class="fa-solid fa-chart-bar text-blue-500"></i>
				Monthly Productivity
			</h3>
			<div bind:this={productivityGraphContainer} class="h-32 bg-gray-50 dark:bg-gray-800 rounded"></div>
		</div>

		<!-- Patient Population Graph -->
		<div class="bg-white dark:bg-gray-700/50 rounded-lg p-3 border border-gray-200 dark:border-gray-600">
			<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-2 flex items-center gap-2">
				<i class="fa-solid fa-chart-line text-green-500"></i>
				Patient Panel Growth
			</h3>
			<div bind:this={populationGraphContainer} class="h-32 bg-gray-50 dark:bg-gray-800 rounded"></div>
		</div>
	</div>
</div>
