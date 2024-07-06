<script lang="ts">
	import '../app.css';
	import { Navbar, NavBrand, NavLi, NavUl, Toggle, NavHamburger, Spinner } from 'flowbite-svelte';

	import { invalidate } from '$app/navigation';
	import { compactView } from '$lib';
	import { onDestroy } from 'svelte';

	function handleToggle() {
		compactView.set(!$compactView);
	}

	let refreshEnabled = true;
	function handleRefresh() {
		invalidate('flux:resources');
		// Disable refresh button to avoid spamming API
		refreshEnabled = false;
		setTimeout(() => (refreshEnabled = true), 1000);
	}

	// Trigger auto refresh periodically
	const id = setInterval(handleRefresh, 30000);
	onDestroy(() => clearInterval(id));
</script>

<Navbar class="bg-black" fluid>
	<NavBrand>
		<img
			style="height:40px"
			src="https://raw.githubusercontent.com/fluxcd/website/v2-3/assets/icons/logo.svg"
			alt="Flux CD Logo"
		/>
		<span class="text-xl text-white ml-2"> Flux CD </span>
	</NavBrand>

	<!-- Hamburger element is reactive menu shrinker / dropdown for small screens -->
	<NavHamburger />

	<NavUl>
		<!-- Only use white text above Hamburger break point -->
		<button
			class="md:text-white hover:text-slate-300"
			on:click={handleRefresh}
			disabled={!refreshEnabled}
		>
			{#if refreshEnabled}
				Refresh
			{:else}
				<Spinner class="mx-3" size="6" />
			{/if}
		</button>

		<NavLi>
			<Toggle class="md:text-white" checked={$compactView} on:change={handleToggle}>
				Compact View
			</Toggle>
		</NavLi>
	</NavUl>
</Navbar>

<slot />
