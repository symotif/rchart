<script lang="ts">
	import { onMount, tick } from 'svelte';

	let revenueGraphContainer: HTMLDivElement | null = $state(null);
	let productivityGraphContainer: HTMLDivElement | null = $state(null);
	let populationGraphContainer: HTMLDivElement | null = $state(null);

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

	// Financial data
	const financialStats = {
		noShowRate: 8.2,
		walkInRate: 3.5,
		avgReimbursement: 142.50,
		monthlyEstimate: 19075,
		ytdRevenue: 228900,
		collectionRate: 94.2
	};

	// Revenue data over time
	const revenueData = [
		{ month: 'Aug', revenue: 20850, collections: 19600 },
		{ month: 'Sep', revenue: 23120, collections: 21800 },
		{ month: 'Oct', revenue: 22540, collections: 21250 },
		{ month: 'Nov', revenue: 24380, collections: 22950 },
		{ month: 'Dec', revenue: 21100, collections: 19850 },
		{ month: 'Jan', revenue: 19075, collections: 17980 }
	];

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	async function drawRevenueGraph() {
		await tick();
		if (!revenueGraphContainer) return;

		revenueGraphContainer.innerHTML = '';

		const width = revenueGraphContainer.clientWidth;
		const height = revenueGraphContainer.clientHeight;
		const margin = { top: 20, right: 20, bottom: 30, left: 55 };
		const innerWidth = width - margin.left - margin.right;
		const innerHeight = height - margin.top - margin.bottom;

		const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
		svg.setAttribute('width', String(width));
		svg.setAttribute('height', String(height));

		const maxRevenue = Math.max(...revenueData.map(d => d.revenue)) + 2000;

		const xScale = (i: number) => margin.left + (i / (revenueData.length - 1)) * innerWidth;
		const yScale = (value: number) => margin.top + innerHeight - (value / maxRevenue) * innerHeight;

		// Fill area under revenue line
		const areaPath = `M ${xScale(0)} ${margin.top + innerHeight} ` +
			revenueData.map((d, i) => `L ${xScale(i)} ${yScale(d.revenue)}`).join(' ') +
			` L ${xScale(revenueData.length - 1)} ${margin.top + innerHeight} Z`;
		const area = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		area.setAttribute('d', areaPath);
		area.setAttribute('fill', 'rgba(59, 130, 246, 0.1)');
		svg.appendChild(area);

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
		const yTicks = [0, maxRevenue / 2, maxRevenue];
		for (const tick of yTicks) {
			const y = yScale(tick);
			const label = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			label.setAttribute('x', String(margin.left - 8));
			label.setAttribute('y', String(y + 4));
			label.setAttribute('text-anchor', 'end');
			label.setAttribute('font-size', '9');
			label.setAttribute('fill', '#6b7280');
			label.textContent = '$' + (tick / 1000).toFixed(0) + 'k';
			svg.appendChild(label);
		}

		// Draw revenue line
		const revenuePath = revenueData.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(i)} ${yScale(d.revenue)}`).join(' ');
		const revenueLine = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		revenueLine.setAttribute('d', revenuePath);
		revenueLine.setAttribute('fill', 'none');
		revenueLine.setAttribute('stroke', '#3b82f6');
		revenueLine.setAttribute('stroke-width', '2');
		svg.appendChild(revenueLine);

		// Draw collections line
		const collectionsPath = revenueData.map((d, i) => `${i === 0 ? 'M' : 'L'} ${xScale(i)} ${yScale(d.collections)}`).join(' ');
		const collectionsLine = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		collectionsLine.setAttribute('d', collectionsPath);
		collectionsLine.setAttribute('fill', 'none');
		collectionsLine.setAttribute('stroke', '#10b981');
		collectionsLine.setAttribute('stroke-width', '2');
		collectionsLine.setAttribute('stroke-dasharray', '5,3');
		svg.appendChild(collectionsLine);

		// Draw points and labels
		revenueData.forEach((d, i) => {
			// Revenue point
			const revenueCircle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			revenueCircle.setAttribute('cx', String(xScale(i)));
			revenueCircle.setAttribute('cy', String(yScale(d.revenue)));
			revenueCircle.setAttribute('r', '4');
			revenueCircle.setAttribute('fill', '#3b82f6');
			svg.appendChild(revenueCircle);

			// Collections point
			const collectionsCircle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
			collectionsCircle.setAttribute('cx', String(xScale(i)));
			collectionsCircle.setAttribute('cy', String(yScale(d.collections)));
			collectionsCircle.setAttribute('r', '4');
			collectionsCircle.setAttribute('fill', '#10b981');
			svg.appendChild(collectionsCircle);

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
		leg1Text.textContent = 'Revenue';
		svg.appendChild(leg1Text);

		const leg2 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
		leg2.setAttribute('cx', String(margin.left + 66));
		leg2.setAttribute('cy', String(legendY + 6));
		leg2.setAttribute('r', '4');
		leg2.setAttribute('fill', '#10b981');
		svg.appendChild(leg2);
		const leg2Text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		leg2Text.setAttribute('x', String(margin.left + 74));
		leg2Text.setAttribute('y', String(legendY + 10));
		leg2Text.setAttribute('font-size', '9');
		leg2Text.setAttribute('fill', '#6b7280');
		leg2Text.textContent = 'Collections';
		svg.appendChild(leg2Text);

		revenueGraphContainer.appendChild(svg);
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
		drawRevenueGraph();
		drawProductivityGraph();
		drawPopulationGraph();
	});
</script>

<div class="absolute left-20 top-20 right-0 bottom-5 px-8 py-8 overflow-y-auto">
	<div class="max-w-6xl mx-auto">
		<!-- Page Header -->
		<div class="mb-6">
			<h1 class="text-2xl font-bold text-gray-800 dark:text-gray-100">Statistics</h1>
			<p class="text-sm text-gray-500 dark:text-gray-400">Financial and productivity metrics</p>
		</div>

		<!-- Financial Stats Cards -->
		<div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
			<div class="bg-green-50 dark:bg-green-900/30 rounded-lg p-4">
				<p class="text-xs text-gray-500 dark:text-gray-400">Est. Monthly Revenue</p>
				<p class="text-2xl font-bold text-green-600 dark:text-green-400">{formatCurrency(financialStats.monthlyEstimate)}</p>
			</div>

			<div class="bg-blue-50 dark:bg-blue-900/30 rounded-lg p-4">
				<p class="text-xs text-gray-500 dark:text-gray-400">YTD Revenue</p>
				<p class="text-2xl font-bold text-blue-600 dark:text-blue-400">{formatCurrency(financialStats.ytdRevenue)}</p>
			</div>

			<div class="bg-purple-50 dark:bg-purple-900/30 rounded-lg p-4">
				<p class="text-xs text-gray-500 dark:text-gray-400">Collection Rate</p>
				<p class="text-2xl font-bold text-purple-600 dark:text-purple-400">{financialStats.collectionRate}%</p>
			</div>

			<div class="bg-teal-50 dark:bg-teal-900/30 rounded-lg p-4">
				<p class="text-xs text-gray-500 dark:text-gray-400">Avg Reimbursement</p>
				<p class="text-2xl font-bold text-teal-600 dark:text-teal-400">{formatCurrency(financialStats.avgReimbursement)}</p>
			</div>
		</div>

		<!-- Two Column Layout for Charts -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
			<!-- Revenue & Collections Graph -->
			<div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700 shadow-sm">
				<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-3 flex items-center gap-2">
					<i class="fa-solid fa-dollar-sign text-green-500"></i>
					Revenue & Collections
				</h3>
				<div bind:this={revenueGraphContainer} class="h-48 bg-gray-50 dark:bg-gray-900 rounded"></div>
			</div>

			<!-- Monthly Productivity Graph -->
			<div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700 shadow-sm">
				<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-3 flex items-center gap-2">
					<i class="fa-solid fa-chart-bar text-blue-500"></i>
					Monthly Productivity
				</h3>
				<div bind:this={productivityGraphContainer} class="h-48 bg-gray-50 dark:bg-gray-900 rounded"></div>
			</div>
		</div>

		<!-- Second Row -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
			<!-- No-Show & Walk-In Stats -->
			<div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700 shadow-sm">
				<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-4 flex items-center gap-2">
					<i class="fa-solid fa-user-clock text-amber-500"></i>
					Scheduling Metrics
				</h3>
				<div class="grid grid-cols-2 gap-6">
					<div>
						<div class="flex items-center justify-between mb-2">
							<span class="text-sm text-gray-500 dark:text-gray-400">No-Show Rate</span>
							<span class="text-lg font-semibold text-red-600 dark:text-red-400">{financialStats.noShowRate}%</span>
						</div>
						<div class="h-3 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
							<div class="h-full bg-red-500 rounded-full" style="width: {financialStats.noShowRate}%"></div>
						</div>
						<p class="text-xs text-gray-400 mt-2">Target: &lt;5%</p>
					</div>
					<div>
						<div class="flex items-center justify-between mb-2">
							<span class="text-sm text-gray-500 dark:text-gray-400">Walk-In Rate</span>
							<span class="text-lg font-semibold text-green-600 dark:text-green-400">{financialStats.walkInRate}%</span>
						</div>
						<div class="h-3 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
							<div class="h-full bg-green-500 rounded-full" style="width: {financialStats.walkInRate * 10}%"></div>
						</div>
						<p class="text-xs text-gray-400 mt-2">Extra revenue opportunity</p>
					</div>
				</div>
			</div>

			<!-- Visit Type Breakdown -->
			<div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700 shadow-sm">
				<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-4 flex items-center gap-2">
					<i class="fa-solid fa-pie-chart text-indigo-500"></i>
					Visit Type Mix (This Month)
				</h3>
				<div class="space-y-3">
					<div>
						<div class="flex items-center justify-between mb-1">
							<span class="text-sm text-gray-600 dark:text-gray-400">New Patient (99203-99205)</span>
							<span class="text-sm font-medium text-gray-800 dark:text-gray-200">18%</span>
						</div>
						<div class="h-3 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
							<div class="h-full bg-indigo-500 rounded-full" style="width: 18%"></div>
						</div>
					</div>

					<div>
						<div class="flex items-center justify-between mb-1">
							<span class="text-sm text-gray-600 dark:text-gray-400">Established (99213-99215)</span>
							<span class="text-sm font-medium text-gray-800 dark:text-gray-200">72%</span>
						</div>
						<div class="h-3 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
							<div class="h-full bg-blue-500 rounded-full" style="width: 72%"></div>
						</div>
					</div>

					<div>
						<div class="flex items-center justify-between mb-1">
							<span class="text-sm text-gray-600 dark:text-gray-400">Telehealth</span>
							<span class="text-sm font-medium text-gray-800 dark:text-gray-200">10%</span>
						</div>
						<div class="h-3 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
							<div class="h-full bg-cyan-500 rounded-full" style="width: 10%"></div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Third Row -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
			<!-- Patient Panel Growth Graph -->
			<div class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700 shadow-sm">
				<h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-3 flex items-center gap-2">
					<i class="fa-solid fa-chart-line text-green-500"></i>
					Patient Panel Growth
				</h3>
				<div bind:this={populationGraphContainer} class="h-48 bg-gray-50 dark:bg-gray-900 rounded"></div>
			</div>

			<!-- RVU Estimate -->
			<div class="bg-gradient-to-r from-blue-500 to-purple-500 rounded-lg p-6 text-white shadow-sm">
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm opacity-90">Estimated Monthly RVUs</p>
						<p class="text-4xl font-bold mt-1">487</p>
						<p class="text-sm opacity-75 mt-2">Target: 450 | +8.2% above target</p>
					</div>
					<div class="w-20 h-20 rounded-full bg-white/20 flex items-center justify-center">
						<i class="fa-solid fa-trophy text-3xl"></i>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
