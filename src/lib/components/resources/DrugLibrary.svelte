<script lang="ts">
	import { ResourceStore } from '../../../stores/ResourceStore';
	import { DRUG_CATEGORY_INFO, type DrugCategory, type Drug } from '$lib/types/resource';

	let searchQuery = $state('');
	let selectedCategory = $state<DrugCategory>('all');
	let showDrugDetail = $state(false);

	const filteredDrugs = ResourceStore.filteredDrugs;
	const selectedDrug = ResourceStore.selectedDrug;

	function handleSearch(e: Event) {
		const target = e.target as HTMLInputElement;
		searchQuery = target.value;
		ResourceStore.setDrugSearch(searchQuery);
	}

	function handleCategoryChange(category: DrugCategory) {
		selectedCategory = category;
		ResourceStore.setDrugCategory(category);
	}

	function selectDrug(drugId: string) {
		ResourceStore.selectDrug(drugId);
		showDrugDetail = true;
	}

	function closeDrugDetail() {
		showDrugDetail = false;
		ResourceStore.selectDrug(null);
	}

	function getSeverityColor(severity: string): string {
		switch (severity) {
			case 'major':
			case 'absolute':
				return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
			case 'moderate':
			case 'relative':
				return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
			case 'minor':
				return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
			default:
				return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200';
		}
	}

	function getFrequencyColor(frequency: string): string {
		switch (frequency) {
			case 'common':
				return 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200';
			case 'uncommon':
				return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
			case 'rare':
				return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200';
			case 'very-rare':
				return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200';
			default:
				return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200';
		}
	}

	const categories = Object.entries(DRUG_CATEGORY_INFO) as [DrugCategory, { label: string; icon: string }][];
</script>

<div class="flex h-full">
	<!-- Sidebar with categories -->
	<aside class="w-56 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50 overflow-y-auto">
		<div class="p-4">
			<h3 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-3">Categories</h3>
			<nav class="space-y-1">
				{#each categories as [key, info]}
					<button
						onclick={() => handleCategoryChange(key)}
						class="w-full flex items-center gap-2 px-3 py-2 text-sm rounded-lg transition-colors
							{selectedCategory === key
								? 'bg-blue-100 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300'
								: 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
					>
						<i class="fa-solid {info.icon} w-4 text-center"></i>
						<span>{info.label}</span>
					</button>
				{/each}
			</nav>
		</div>
	</aside>

	<!-- Main content area -->
	<div class="flex-1 flex flex-col overflow-hidden">
		<!-- Search bar -->
		<div class="p-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
			<div class="relative">
				<i class="fa-solid fa-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"></i>
				<input
					type="text"
					placeholder="Search medications by name, brand, or class..."
					value={searchQuery}
					oninput={handleSearch}
					class="w-full pl-10 pr-4 py-2 bg-gray-100 dark:bg-gray-700 border-0 rounded-lg
						text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400
						focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400"
				/>
			</div>
		</div>

		<!-- Drug list -->
		<div class="flex-1 overflow-y-auto p-4">
			{#if $filteredDrugs.length === 0}
				<div class="flex flex-col items-center justify-center h-full text-gray-500 dark:text-gray-400">
					<i class="fa-solid fa-pills text-4xl mb-3"></i>
					<p>No medications found</p>
					<p class="text-sm">Try adjusting your search or category filter</p>
				</div>
			{:else}
				<div class="grid gap-3">
					{#each $filteredDrugs as drug}
						<button
							onclick={() => selectDrug(drug.id)}
							class="w-full text-left p-4 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700
								hover:border-blue-300 dark:hover:border-blue-600 hover:shadow-md transition-all"
						>
							<div class="flex items-start justify-between">
								<div class="flex-1">
									<div class="flex items-center gap-2">
										<h3 class="font-semibold text-gray-900 dark:text-gray-100">{drug.genericName}</h3>
										{#if drug.controlledSubstance}
											<span class="px-2 py-0.5 text-xs font-medium bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-300 rounded">
												C-{drug.controlledSubstance.schedule}
											</span>
										{/if}
									</div>
									<p class="text-sm text-gray-500 dark:text-gray-400">
										{drug.brandNames.join(', ')}
									</p>
									<div class="flex items-center gap-2 mt-2">
										<span class="px-2 py-0.5 text-xs font-medium bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded">
											{drug.drugClass}
										</span>
										{#if drug.subClass}
											<span class="px-2 py-0.5 text-xs font-medium bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300 rounded">
												{drug.subClass}
											</span>
										{/if}
									</div>
								</div>
								<i class="fa-solid fa-chevron-right text-gray-400"></i>
							</div>
						</button>
					{/each}
				</div>
			{/if}
		</div>
	</div>

	<!-- Drug detail modal/panel -->
	{#if showDrugDetail && $selectedDrug}
		<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={closeDrugDetail}>
			<div
				class="w-full max-w-4xl max-h-[90vh] bg-white dark:bg-gray-800 rounded-2xl shadow-2xl overflow-hidden flex flex-col"
				onclick={(e) => e.stopPropagation()}
			>
				<!-- Header -->
				<div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700 bg-gradient-to-r from-emerald-500 to-teal-500">
					<div>
						<div class="flex items-center gap-3">
							<h2 class="text-2xl font-bold text-white">{$selectedDrug.genericName}</h2>
							{#if $selectedDrug.controlledSubstance}
								<span class="px-2 py-0.5 text-sm font-semibold bg-white/20 text-white rounded">
									Schedule {$selectedDrug.controlledSubstance.schedule}
								</span>
							{/if}
						</div>
						<p class="text-emerald-100 mt-1">{$selectedDrug.brandNames.join(', ')}</p>
						<div class="flex items-center gap-2 mt-2">
							<span class="px-2 py-0.5 text-xs font-medium bg-white/20 text-white rounded">
								{$selectedDrug.drugClass}
							</span>
							{#if $selectedDrug.subClass}
								<span class="px-2 py-0.5 text-xs font-medium bg-white/10 text-white rounded">
									{$selectedDrug.subClass}
								</span>
							{/if}
						</div>
					</div>
					<button
						onclick={closeDrugDetail}
						class="p-2 text-white/80 hover:text-white hover:bg-white/10 rounded-lg transition-colors"
					>
						<i class="fa-solid fa-times text-xl"></i>
					</button>
				</div>

				<!-- Content -->
				<div class="flex-1 overflow-y-auto p-6 space-y-6">
					<!-- Black Box Warnings -->
					{#if $selectedDrug.blackBoxWarnings && $selectedDrug.blackBoxWarnings.length > 0}
						<section class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4">
							<h3 class="flex items-center gap-2 text-lg font-semibold text-red-700 dark:text-red-400 mb-3">
								<i class="fa-solid fa-triangle-exclamation"></i>
								Black Box Warning{$selectedDrug.blackBoxWarnings.length > 1 ? 's' : ''}
							</h3>
							{#each $selectedDrug.blackBoxWarnings as warning}
								<div class="mb-3 last:mb-0">
									<h4 class="font-semibold text-red-800 dark:text-red-300">{warning.title}</h4>
									<p class="text-sm text-red-700 dark:text-red-400">{warning.description}</p>
								</div>
							{/each}
						</section>
					{/if}

					<!-- FDA Indications -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-clipboard-check text-green-500"></i>
							FDA-Approved Indications
						</h3>
						<ul class="grid grid-cols-1 md:grid-cols-2 gap-2">
							{#each $selectedDrug.fdaIndications as indication}
								<li class="flex items-start gap-2 text-sm text-gray-700 dark:text-gray-300">
									<i class="fa-solid fa-check text-green-500 mt-0.5"></i>
									{indication}
								</li>
							{/each}
						</ul>
						{#if $selectedDrug.offLabelUses && $selectedDrug.offLabelUses.length > 0}
							<h4 class="text-sm font-medium text-gray-500 dark:text-gray-400 mt-4 mb-2">Off-Label Uses</h4>
							<ul class="flex flex-wrap gap-2">
								{#each $selectedDrug.offLabelUses as use}
									<li class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 rounded">
										{use}
									</li>
								{/each}
							</ul>
						{/if}
					</section>

					<!-- Formulations -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-pills text-purple-500"></i>
							Formulations
						</h3>
						<div class="grid gap-2">
							{#each $selectedDrug.formulations as form}
								<div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="font-medium text-gray-900 dark:text-gray-100">{form.form}</span>
									<span class="text-gray-500 dark:text-gray-400">({form.route})</span>
									<span class="text-sm text-gray-600 dark:text-gray-300">{form.strengths.join(', ')}</span>
								</div>
							{/each}
						</div>
					</section>

					<!-- Dosing -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-prescription text-blue-500"></i>
							Dosing
						</h3>
						<div class="space-y-3">
							{#each $selectedDrug.dosing as dose}
								<div class="p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<h4 class="font-semibold text-gray-900 dark:text-gray-100 mb-2">{dose.indication}</h4>
									<div class="grid grid-cols-2 gap-3 text-sm">
										<div>
											<span class="text-gray-500 dark:text-gray-400">Adult Dose:</span>
											<span class="ml-2 text-gray-900 dark:text-gray-100">{dose.adultDose}</span>
										</div>
										{#if dose.pediatricDose}
											<div>
												<span class="text-gray-500 dark:text-gray-400">Pediatric:</span>
												<span class="ml-2 text-gray-900 dark:text-gray-100">{dose.pediatricDose}</span>
											</div>
										{/if}
										<div>
											<span class="text-gray-500 dark:text-gray-400">Frequency:</span>
											<span class="ml-2 text-gray-900 dark:text-gray-100">{dose.frequency}</span>
										</div>
										{#if dose.maxDose}
											<div>
												<span class="text-gray-500 dark:text-gray-400">Max Dose:</span>
												<span class="ml-2 text-gray-900 dark:text-gray-100">{dose.maxDose}</span>
											</div>
										{/if}
									</div>
									{#if dose.notes}
										<p class="mt-2 text-sm text-gray-600 dark:text-gray-400 italic">{dose.notes}</p>
									{/if}
								</div>
							{/each}
						</div>
					</section>

					<!-- Tapering -->
					{#if $selectedDrug.tapering && $selectedDrug.tapering.length > 0}
						<section>
							<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
								<i class="fa-solid fa-arrow-trend-down text-orange-500"></i>
								Tapering Protocols
							</h3>
							<div class="space-y-3">
								{#each $selectedDrug.tapering as taper}
									<div class="p-4 bg-orange-50 dark:bg-orange-900/20 rounded-lg border border-orange-200 dark:border-orange-800">
										<h4 class="font-semibold text-gray-900 dark:text-gray-100">{taper.indication}</h4>
										<p class="text-sm text-gray-700 dark:text-gray-300 mt-1">{taper.protocol}</p>
										<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Duration: {taper.duration}</p>
										{#if taper.notes}
											<p class="text-sm text-orange-700 dark:text-orange-400 mt-2 italic">{taper.notes}</p>
										{/if}
									</div>
								{/each}
							</div>
						</section>
					{/if}

					<!-- Pharmacokinetics -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-flask text-cyan-500"></i>
							Pharmacokinetics
						</h3>
						<div class="grid grid-cols-2 md:grid-cols-3 gap-3">
							{#if $selectedDrug.pharmacokinetics.halfLife}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="text-xs text-gray-500 dark:text-gray-400 block">Half-Life</span>
									<span class="font-medium text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacokinetics.halfLife}</span>
								</div>
							{/if}
							{#if $selectedDrug.pharmacokinetics.bioavailability}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="text-xs text-gray-500 dark:text-gray-400 block">Bioavailability</span>
									<span class="font-medium text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacokinetics.bioavailability}</span>
								</div>
							{/if}
							{#if $selectedDrug.pharmacokinetics.proteinBinding}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="text-xs text-gray-500 dark:text-gray-400 block">Protein Binding</span>
									<span class="font-medium text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacokinetics.proteinBinding}</span>
								</div>
							{/if}
							{#if $selectedDrug.pharmacokinetics.onsetOfAction}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="text-xs text-gray-500 dark:text-gray-400 block">Onset of Action</span>
									<span class="font-medium text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacokinetics.onsetOfAction}</span>
								</div>
							{/if}
							{#if $selectedDrug.pharmacokinetics.peakPlasma}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="text-xs text-gray-500 dark:text-gray-400 block">Peak Plasma</span>
									<span class="font-medium text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacokinetics.peakPlasma}</span>
								</div>
							{/if}
							{#if $selectedDrug.pharmacokinetics.metabolism}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg col-span-2 md:col-span-3">
									<span class="text-xs text-gray-500 dark:text-gray-400 block">Metabolism</span>
									<span class="font-medium text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacokinetics.metabolism}</span>
								</div>
							{/if}
							{#if $selectedDrug.pharmacokinetics.elimination}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg col-span-2 md:col-span-3">
									<span class="text-xs text-gray-500 dark:text-gray-400 block">Elimination</span>
									<span class="font-medium text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacokinetics.elimination}</span>
								</div>
							{/if}
						</div>
					</section>

					<!-- Pharmacodynamics -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-atom text-indigo-500"></i>
							Pharmacodynamics
						</h3>
						<div class="p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
							<h4 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-2">Mechanism of Action</h4>
							<p class="text-gray-900 dark:text-gray-100">{$selectedDrug.pharmacodynamics.mechanismOfAction}</p>
							{#if $selectedDrug.pharmacodynamics.receptorBinding && $selectedDrug.pharmacodynamics.receptorBinding.length > 0}
								<h4 class="text-sm font-medium text-gray-500 dark:text-gray-400 mt-4 mb-2">Receptor Binding</h4>
								<ul class="flex flex-wrap gap-2">
									{#each $selectedDrug.pharmacodynamics.receptorBinding as receptor}
										<li class="px-2 py-1 text-xs bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 rounded">
											{receptor}
										</li>
									{/each}
								</ul>
							{/if}
						</div>
					</section>

					<!-- Contraindications -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-ban text-red-500"></i>
							Contraindications
						</h3>
						<div class="space-y-2">
							{#each $selectedDrug.contraindications as contra}
								<div class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="px-2 py-0.5 text-xs font-medium rounded {getSeverityColor(contra.severity)}">
										{contra.severity}
									</span>
									<div>
										<span class="font-medium text-gray-900 dark:text-gray-100">{contra.condition}</span>
										<p class="text-sm text-gray-600 dark:text-gray-400">{contra.description}</p>
									</div>
								</div>
							{/each}
						</div>
					</section>

					<!-- Drug Interactions -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-right-left text-amber-500"></i>
							Drug Interactions
						</h3>
						<div class="space-y-2">
							{#each $selectedDrug.interactions as interaction}
								<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<div class="flex items-center gap-2 mb-1">
										<span class="px-2 py-0.5 text-xs font-medium rounded {getSeverityColor(interaction.severity)}">
											{interaction.severity}
										</span>
										<span class="font-medium text-gray-900 dark:text-gray-100">{interaction.drug}</span>
									</div>
									<p class="text-sm text-gray-600 dark:text-gray-400">{interaction.description}</p>
									{#if interaction.management}
										<p class="text-sm text-blue-600 dark:text-blue-400 mt-1">
											<i class="fa-solid fa-lightbulb mr-1"></i>
											{interaction.management}
										</p>
									{/if}
								</div>
							{/each}
						</div>
					</section>

					<!-- Adverse Effects -->
					<section>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
							<i class="fa-solid fa-circle-exclamation text-yellow-500"></i>
							Adverse Effects
						</h3>
						<div class="grid grid-cols-1 md:grid-cols-2 gap-2">
							{#each $selectedDrug.adverseEffects as effect}
								<div class="flex items-center gap-2 p-2 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
									<span class="px-2 py-0.5 text-xs font-medium rounded {getFrequencyColor(effect.frequency)}">
										{effect.frequency}
									</span>
									<span class="text-sm text-gray-900 dark:text-gray-100">{effect.effect}</span>
									{#if effect.description}
										<span class="text-xs text-gray-500 dark:text-gray-400">({effect.description})</span>
									{/if}
								</div>
							{/each}
						</div>
					</section>

					<!-- Monitoring -->
					{#if $selectedDrug.monitoring && $selectedDrug.monitoring.length > 0}
						<section>
							<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
								<i class="fa-solid fa-eye text-teal-500"></i>
								Monitoring
							</h3>
							<ul class="grid gap-2">
								{#each $selectedDrug.monitoring as item}
									<li class="flex items-start gap-2 text-sm text-gray-700 dark:text-gray-300">
										<i class="fa-solid fa-circle-check text-teal-500 mt-0.5"></i>
										{item}
									</li>
								{/each}
							</ul>
						</section>
					{/if}

					<!-- Patient Counseling -->
					{#if $selectedDrug.patientCounseling && $selectedDrug.patientCounseling.length > 0}
						<section>
							<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3 flex items-center gap-2">
								<i class="fa-solid fa-comments text-pink-500"></i>
								Patient Counseling
							</h3>
							<ul class="grid gap-2">
								{#each $selectedDrug.patientCounseling as item}
									<li class="flex items-start gap-2 text-sm text-gray-700 dark:text-gray-300">
										<i class="fa-solid fa-message text-pink-500 mt-0.5"></i>
										{item}
									</li>
								{/each}
							</ul>
						</section>
					{/if}

					<!-- Pregnancy/Lactation -->
					<section class="grid grid-cols-2 gap-4">
						{#if $selectedDrug.pregnancyCategory}
							<div class="p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
								<h4 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-1">Pregnancy Category</h4>
								<span class="text-2xl font-bold text-gray-900 dark:text-gray-100">{$selectedDrug.pregnancyCategory}</span>
							</div>
						{/if}
						{#if $selectedDrug.lactationSafety}
							<div class="p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
								<h4 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-1">Lactation</h4>
								<p class="text-sm text-gray-900 dark:text-gray-100">{$selectedDrug.lactationSafety}</p>
							</div>
						{/if}
					</section>

					<!-- Footer info -->
					<div class="pt-4 border-t border-gray-200 dark:border-gray-700 text-xs text-gray-500 dark:text-gray-400">
						<p>Last updated: {$selectedDrug.lastUpdated}</p>
						{#if $selectedDrug.fdaApprovalDate}
							<p>FDA Approval Date: {$selectedDrug.fdaApprovalDate}</p>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
