<script lang="ts">
	import { ExtensionStore } from '../../stores/ExtensionStore';
	import { CATEGORY_INFO, type Extension, type ExtensionCategory } from '$lib/types/extension';
	import { ToastStore } from '../../stores/ToastStore';

	// Local state
	let searchQuery = $state('');
	let selectedExtension = $state<Extension | null>(null);
	let activeTab = $state<'marketplace' | 'installed'>('marketplace');

	// Derived from store
	const filter = ExtensionStore.filter;
	const filteredExtensions = ExtensionStore.filteredExtensions;
	const installedExtensions = ExtensionStore.installedExtensions;
	const featuredExtensions = ExtensionStore.featuredExtensions;

	// Categories for filter
	const categories: (ExtensionCategory | 'all')[] = [
		'all',
		'telehealth',
		'specialty',
		'workflow',
		'integration',
		'analytics',
		'documentation',
		'billing',
		'patient-engagement',
		'ai',
		'institutional'
	];

	function handleSearch() {
		ExtensionStore.setSearch(searchQuery);
	}

	function selectCategory(category: ExtensionCategory | 'all') {
		ExtensionStore.setCategory(category);
	}

	function openExtension(ext: Extension) {
		selectedExtension = ext;
	}

	function closeDetail() {
		selectedExtension = null;
	}

	function installExtension(ext: Extension) {
		ExtensionStore.install(ext.id);
		ToastStore.success(`${ext.displayName} installed successfully`);
		// Update selectedExtension if viewing detail
		if (selectedExtension?.id === ext.id) {
			selectedExtension = { ...ext, isInstalled: true, isEnabled: true };
		}
	}

	function uninstallExtension(ext: Extension) {
		ExtensionStore.uninstall(ext.id);
		ToastStore.info(`${ext.displayName} uninstalled`);
		if (selectedExtension?.id === ext.id) {
			selectedExtension = { ...ext, isInstalled: false, isEnabled: false };
		}
	}

	function toggleExtension(ext: Extension) {
		ExtensionStore.toggleEnabled(ext.id);
		const newEnabled = !ext.isEnabled;
		ToastStore.info(`${ext.displayName} ${newEnabled ? 'enabled' : 'disabled'}`);
		if (selectedExtension?.id === ext.id) {
			selectedExtension = { ...ext, isEnabled: newEnabled };
		}
	}

	function formatDownloads(count: number): string {
		if (count >= 1000000) return `${(count / 1000000).toFixed(1)}M`;
		if (count >= 1000) return `${(count / 1000).toFixed(1)}K`;
		return count.toString();
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function renderStars(rating: number): string[] {
		const stars: string[] = [];
		const fullStars = Math.floor(rating);
		const hasHalfStar = rating - fullStars >= 0.5;
		for (let i = 0; i < fullStars; i++) stars.push('fa-star');
		if (hasHalfStar) stars.push('fa-star-half-stroke');
		while (stars.length < 5) stars.push('fa-star text-gray-300 dark:text-gray-600');
		return stars;
	}
</script>

<div class="absolute top-20 left-20 right-0 bottom-6 flex flex-col overflow-hidden">
	<!-- Header -->
	<div class="flex-shrink-0 px-6 py-4 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="flex items-center justify-between mb-4">
			<div class="flex items-center gap-3">
				<i class="fa-solid fa-puzzle-piece text-2xl text-indigo-500"></i>
				<h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Extensions</h1>
			</div>

			<!-- Tab toggle -->
			<div class="flex items-center gap-2 bg-gray-100 dark:bg-gray-700 rounded-lg p-1">
				<button
					onclick={() => {
						activeTab = 'marketplace';
						ExtensionStore.setShowInstalled(false);
					}}
					class="px-4 py-2 rounded-md text-sm font-medium transition-colors {activeTab === 'marketplace'
						? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-gray-100 shadow-sm'
						: 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'}"
				>
					<i class="fa-solid fa-store mr-2"></i>
					Marketplace
				</button>
				<button
					onclick={() => {
						activeTab = 'installed';
						ExtensionStore.setShowInstalled(true);
					}}
					class="px-4 py-2 rounded-md text-sm font-medium transition-colors {activeTab === 'installed'
						? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-gray-100 shadow-sm'
						: 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'}"
				>
					<i class="fa-solid fa-check-circle mr-2"></i>
					Installed
					{#if $installedExtensions.length > 0}
						<span class="ml-1 px-1.5 py-0.5 text-xs bg-indigo-100 dark:bg-indigo-900 text-indigo-600 dark:text-indigo-400 rounded-full">
							{$installedExtensions.length}
						</span>
					{/if}
				</button>
			</div>
		</div>

		<!-- Search and filters -->
		<div class="flex items-center gap-4">
			<!-- Search -->
			<div class="flex-1 max-w-xl relative">
				<i class="fa-solid fa-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"></i>
				<input
					type="text"
					bind:value={searchQuery}
					oninput={handleSearch}
					placeholder="Search extensions by name, author, or organization..."
					class="w-full pl-10 pr-4 py-2.5 bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg
						text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400
						focus:outline-none focus:ring-2 focus:ring-indigo-500"
				/>
			</div>

			<!-- Sort -->
			<select
				value={$filter.sortBy}
				onchange={(e) => ExtensionStore.setSortBy(e.currentTarget.value as any)}
				class="px-3 py-2.5 bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg
					text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
			>
				<option value="popular">Most Popular</option>
				<option value="rating">Highest Rated</option>
				<option value="recent">Recently Updated</option>
				<option value="name">Name A-Z</option>
			</select>
		</div>
	</div>

	<!-- Main content area -->
	<div class="flex-1 flex overflow-hidden">
		<!-- Left sidebar - Categories -->
		<div class="w-56 flex-shrink-0 bg-gray-50 dark:bg-gray-800/50 border-r border-gray-200 dark:border-gray-700 overflow-y-auto">
			<div class="p-4">
				<h3 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-3">
					Categories
				</h3>
				<div class="space-y-1">
					{#each categories as category}
						{@const info = category === 'all' ? null : CATEGORY_INFO[category]}
						<button
							onclick={() => selectCategory(category)}
							class="w-full flex items-center gap-2 px-3 py-2 rounded-lg text-left transition-colors
								{$filter.category === category
								? 'bg-indigo-100 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300'
								: 'hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300'}"
						>
							{#if info}
								<i class="fa-solid {info.icon} w-4 {info.color}"></i>
								<span class="text-sm">{info.label}</span>
							{:else}
								<i class="fa-solid fa-border-all w-4 text-gray-500"></i>
								<span class="text-sm">All Extensions</span>
							{/if}
						</button>
					{/each}
				</div>
			</div>
		</div>

		<!-- Extension list / grid -->
		<div class="flex-1 overflow-y-auto p-6">
			{#if activeTab === 'marketplace' && $filter.search === '' && $filter.category === 'all'}
				<!-- Featured section (only on main marketplace view) -->
				<div class="mb-8">
					<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4 flex items-center gap-2">
						<i class="fa-solid fa-star text-yellow-500"></i>
						Featured Extensions
					</h2>
					<div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
						{#each $featuredExtensions.slice(0, 6) as ext}
							<button
								onclick={() => openExtension(ext)}
								class="text-left p-4 bg-gradient-to-br from-indigo-50 to-purple-50 dark:from-indigo-900/20 dark:to-purple-900/20
									border border-indigo-200 dark:border-indigo-800 rounded-xl hover:shadow-lg transition-all group"
							>
								<div class="flex items-start gap-3">
									<div class="w-12 h-12 rounded-lg {ext.iconColor} flex items-center justify-center text-white text-xl flex-shrink-0">
										<i class="fa-solid {ext.icon}"></i>
									</div>
									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2">
											<h3 class="font-semibold text-gray-900 dark:text-gray-100 truncate">{ext.displayName}</h3>
											{#if ext.isOfficial}
												<span class="px-1.5 py-0.5 text-xs bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded">Official</span>
											{/if}
										</div>
										<p class="text-sm text-gray-500 dark:text-gray-400 truncate">{ext.author.organization || ext.author.name}</p>
										<p class="text-sm text-gray-600 dark:text-gray-300 mt-1 line-clamp-2">{ext.description}</p>
										<div class="flex items-center gap-3 mt-2 text-xs text-gray-500 dark:text-gray-400">
											<span class="flex items-center gap-1">
												<i class="fa-solid fa-download"></i>
												{formatDownloads(ext.downloadCount)}
											</span>
											<span class="flex items-center gap-1">
												<i class="fa-solid fa-star text-yellow-500"></i>
												{ext.rating.toFixed(1)}
											</span>
										</div>
									</div>
								</div>
							</button>
						{/each}
					</div>
				</div>
			{/if}

			<!-- All extensions -->
			<div>
				<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
					{#if activeTab === 'installed'}
						Installed Extensions ({$filteredExtensions.length})
					{:else if $filter.category !== 'all'}
						{CATEGORY_INFO[$filter.category].label} Extensions ({$filteredExtensions.length})
					{:else if $filter.search}
						Search Results ({$filteredExtensions.length})
					{:else}
						All Extensions ({$filteredExtensions.length})
					{/if}
				</h2>

				{#if $filteredExtensions.length === 0}
					<div class="text-center py-12">
						<i class="fa-solid fa-puzzle-piece text-6xl text-gray-300 dark:text-gray-600 mb-4"></i>
						<p class="text-gray-500 dark:text-gray-400">
							{#if activeTab === 'installed'}
								No extensions installed yet
							{:else if $filter.search}
								No extensions match your search
							{:else}
								No extensions in this category
							{/if}
						</p>
					</div>
				{:else}
					<div class="space-y-3">
						{#each $filteredExtensions as ext}
							<button
								onclick={() => openExtension(ext)}
								class="w-full text-left p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700
									rounded-lg hover:border-indigo-300 dark:hover:border-indigo-700 hover:shadow-md transition-all group"
							>
								<div class="flex items-start gap-4">
									<div class="w-14 h-14 rounded-lg {ext.iconColor} flex items-center justify-center text-white text-2xl flex-shrink-0">
										<i class="fa-solid {ext.icon}"></i>
									</div>
									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2 flex-wrap">
											<h3 class="font-semibold text-gray-900 dark:text-gray-100">{ext.displayName}</h3>
											<span class="text-xs text-gray-500 dark:text-gray-400">v{ext.version}</span>
											{#if ext.isOfficial}
												<span class="px-1.5 py-0.5 text-xs bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded">Official</span>
											{/if}
											{#if ext.author.verified}
												<i class="fa-solid fa-circle-check text-blue-500 text-sm" title="Verified Author"></i>
											{/if}
											{#if ext.isInstalled}
												<span class="px-1.5 py-0.5 text-xs bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300 rounded flex items-center gap-1">
													<i class="fa-solid fa-check"></i>
													{ext.isEnabled ? 'Enabled' : 'Disabled'}
												</span>
											{/if}
										</div>
										<p class="text-sm text-gray-500 dark:text-gray-400">
											{ext.author.organization || ext.author.name}
										</p>
										<p class="text-sm text-gray-600 dark:text-gray-300 mt-1">{ext.description}</p>
										<div class="flex items-center gap-4 mt-2">
											<span class="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400">
												<i class="fa-solid fa-download"></i>
												{formatDownloads(ext.downloadCount)}
											</span>
											<span class="flex items-center gap-1 text-xs text-yellow-600 dark:text-yellow-400">
												{#each renderStars(ext.rating) as star, i}
													<i class="fa-solid {star}"></i>
												{/each}
												<span class="text-gray-500 dark:text-gray-400 ml-1">({ext.ratingCount})</span>
											</span>
											<span class="flex items-center gap-1 text-xs text-gray-500 dark:text-gray-400">
												<i class="fa-solid fa-clock"></i>
												Updated {formatDate(ext.lastUpdated)}
											</span>
										</div>
									</div>
									<div class="flex-shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
										{#if ext.isInstalled}
											<button
												onclick={(e) => { e.stopPropagation(); toggleExtension(ext); }}
												class="px-3 py-1.5 text-sm font-medium rounded-lg transition-colors
													{ext.isEnabled
													? 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'
													: 'bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300 hover:bg-green-200 dark:hover:bg-green-800'}"
											>
												{ext.isEnabled ? 'Disable' : 'Enable'}
											</button>
										{:else}
											<button
												onclick={(e) => { e.stopPropagation(); installExtension(ext); }}
												class="px-3 py-1.5 text-sm font-medium bg-indigo-500 text-white rounded-lg hover:bg-indigo-600 transition-colors"
											>
												Install
											</button>
										{/if}
									</div>
								</div>
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>

<!-- Extension Detail Modal -->
{#if selectedExtension}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" onclick={closeDetail}>
		<div
			class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-hidden flex flex-col"
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Modal header -->
			<div class="flex items-start gap-4 p-6 border-b border-gray-200 dark:border-gray-700">
				<div class="w-16 h-16 rounded-xl {selectedExtension.iconColor} flex items-center justify-center text-white text-3xl flex-shrink-0">
					<i class="fa-solid {selectedExtension.icon}"></i>
				</div>
				<div class="flex-1 min-w-0">
					<div class="flex items-center gap-2 flex-wrap">
						<h2 class="text-xl font-bold text-gray-900 dark:text-gray-100">{selectedExtension.displayName}</h2>
						{#if selectedExtension.isOfficial}
							<span class="px-2 py-0.5 text-xs bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded font-medium">Official</span>
						{/if}
					</div>
					<div class="flex items-center gap-2 mt-1">
						<span class="text-gray-600 dark:text-gray-400">
							{selectedExtension.author.organization || selectedExtension.author.name}
						</span>
						{#if selectedExtension.author.verified}
							<i class="fa-solid fa-circle-check text-blue-500" title="Verified Author"></i>
						{/if}
					</div>
					<div class="flex items-center gap-4 mt-2 text-sm">
						<span class="flex items-center gap-1 text-gray-500 dark:text-gray-400">
							<i class="fa-solid fa-download"></i>
							{formatDownloads(selectedExtension.downloadCount)} installs
						</span>
						<span class="flex items-center gap-1 text-yellow-600 dark:text-yellow-400">
							{#each renderStars(selectedExtension.rating) as star}
								<i class="fa-solid {star}"></i>
							{/each}
							<span class="text-gray-500 dark:text-gray-400 ml-1">{selectedExtension.rating.toFixed(1)} ({selectedExtension.ratingCount} reviews)</span>
						</span>
					</div>
				</div>
				<button
					onclick={closeDetail}
					class="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
				>
					<i class="fa-solid fa-times text-gray-500"></i>
				</button>
			</div>

			<!-- Modal body -->
			<div class="flex-1 overflow-y-auto p-6">
				<div class="prose prose-sm dark:prose-invert max-w-none">
					<p class="text-gray-700 dark:text-gray-300 text-base leading-relaxed">
						{selectedExtension.longDescription || selectedExtension.description}
					</p>
				</div>

				<!-- Tags -->
				<div class="mt-6">
					<h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Tags</h4>
					<div class="flex flex-wrap gap-2">
						{#each selectedExtension.tags as tag}
							<span class="px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded-full">
								{tag}
							</span>
						{/each}
					</div>
				</div>

				<!-- Permissions -->
				{#if selectedExtension.permissions && selectedExtension.permissions.length > 0}
					<div class="mt-6">
						<h4 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Permissions Required</h4>
						<div class="flex flex-wrap gap-2">
							{#each selectedExtension.permissions as perm}
								<span class="px-2 py-1 text-xs bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400 rounded-full flex items-center gap-1">
									<i class="fa-solid fa-shield-halved"></i>
									{perm}
								</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Info grid -->
				<div class="mt-6 grid grid-cols-2 gap-4">
					<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
						<p class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide">Version</p>
						<p class="font-medium text-gray-900 dark:text-gray-100">{selectedExtension.version}</p>
					</div>
					<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
						<p class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide">Last Updated</p>
						<p class="font-medium text-gray-900 dark:text-gray-100">{formatDate(selectedExtension.lastUpdated)}</p>
					</div>
					<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
						<p class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide">Category</p>
						<p class="font-medium text-gray-900 dark:text-gray-100 flex items-center gap-2">
							<i class="fa-solid {CATEGORY_INFO[selectedExtension.category].icon} {CATEGORY_INFO[selectedExtension.category].color}"></i>
							{CATEGORY_INFO[selectedExtension.category].label}
						</p>
					</div>
					<div class="p-3 bg-gray-50 dark:bg-gray-700/50 rounded-lg">
						<p class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide">Author</p>
						<p class="font-medium text-gray-900 dark:text-gray-100 flex items-center gap-1">
							{selectedExtension.author.name}
							{#if selectedExtension.author.verified}
								<i class="fa-solid fa-circle-check text-blue-500"></i>
							{/if}
						</p>
					</div>
				</div>
			</div>

			<!-- Modal footer -->
			<div class="flex items-center justify-between p-6 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
				<div>
					{#if selectedExtension.isInstalled}
						<button
							onclick={() => uninstallExtension(selectedExtension!)}
							class="px-4 py-2 text-sm font-medium text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
						>
							<i class="fa-solid fa-trash mr-2"></i>
							Uninstall
						</button>
					{/if}
				</div>
				<div class="flex items-center gap-3">
					<button
						onclick={closeDetail}
						class="px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
					>
						Close
					</button>
					{#if selectedExtension.isInstalled}
						<button
							onclick={() => toggleExtension(selectedExtension!)}
							class="px-4 py-2 text-sm font-medium rounded-lg transition-colors
								{selectedExtension.isEnabled
								? 'bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-500'
								: 'bg-green-500 text-white hover:bg-green-600'}"
						>
							{selectedExtension.isEnabled ? 'Disable' : 'Enable'}
						</button>
					{:else}
						<button
							onclick={() => installExtension(selectedExtension!)}
							class="px-4 py-2 text-sm font-medium bg-indigo-500 text-white rounded-lg hover:bg-indigo-600 transition-colors flex items-center gap-2"
						>
							<i class="fa-solid fa-download"></i>
							Install Extension
						</button>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
