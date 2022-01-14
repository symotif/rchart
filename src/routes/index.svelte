<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
		import Sidebar from '$lib/Sidebar.svelte';

	import Modal from '$lib/modal.svelte';
	import Counter from '$lib/Counter.svelte';
	let data = 'hey';

	const handleClick = () => {
		data = 'hoy';
	};
</script>

<svelte:head>
	<title>Home</title>
</svelte:head>

<Sidebar />
<div>
	<div class="rounded-lg shadow bg-base-200 drawer h-52">
		<input id="my-drawer" type="checkbox" class="drawer-toggle" />
		<div class="flex flex-col items-center justify-center drawer-content">
			<label for="my-drawer" class="btn btn-primary drawer-button"
				>open menu</label
			>
		</div>
		<div class="drawer-side">
			<label for="my-drawer" class="drawer-overlay" />
			<ul class="menu p-4 overflow-y-auto w-80 bg-base-100 text-base-content">
				<li>
					<a>Menu Item</a>
				</li>
				<li>
					<a>Menu Item</a>
				</li>
			</ul>
		</div>
	</div>
</div>

<button class="btn"> Button </button>

<button
	class="inline-block px-4 py-3 text-sm font-semibold text-center text-white uppercase transition duration-200 ease-in-out bg-indigo-500 rounded-md cursor-pointer hover:bg-indigo-600"
	>Tailwind Button</button
>
<button class="btn btn-primary rounded-full">daisyUI Button</button>

<div class="avatar">
	<div
		class="mb-8 rounded-box w-24 h-24 ring ring-primary ring-offset-base-100 ring-offset-2"
	>
		<img src="http://daisyui.com/tailwind-css-component-profile-1@94w.png" />
	</div>
	<p>Jody Smith, MD</p>
</div>

<section>
	<Modal />

	<Counter />
</section>
<p>{data}</p>
<button class="btn" on:click={handleClick}> Button </button>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 1;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
