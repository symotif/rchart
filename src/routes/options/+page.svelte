<script lang="ts">
	import { ThemeStore, toggleTheme } from '../../stores/ThemeStore';
</script>

<div class="absolute left-20 top-20 right-10 bottom-5 mx-5 my-8">
	<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 max-w-2xl">
		<h1 class="text-3xl font-bold text-gray-800 dark:text-gray-100 mb-6">Options</h1>

		<div class="space-y-6">
			<!-- Appearance Section -->
			<section>
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">Appearance</h2>

				<div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
					<div>
						<p class="font-medium text-gray-800 dark:text-gray-100">Dark Mode</p>
						<p class="text-sm text-gray-500 dark:text-gray-400">
							Switch between light and dark themes
						</p>
					</div>

					<!-- Toggle Switch -->
					<button
						onclick={toggleTheme}
						class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
							{$ThemeStore === 'dark' ? 'bg-blue-600' : 'bg-gray-300'}"
						role="switch"
						aria-checked={$ThemeStore === 'dark'}
					>
						<span
							class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
								{$ThemeStore === 'dark' ? 'translate-x-6' : 'translate-x-1'}"
						></span>
					</button>
				</div>
			</section>

			<!-- Additional settings sections can be added here -->
			<section>
				<h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-4">General</h2>
				<div class="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
					<p class="text-sm text-gray-500 dark:text-gray-400">
						More settings coming soon...
					</p>
				</div>
			</section>
		</div>
	</div>
</div>
