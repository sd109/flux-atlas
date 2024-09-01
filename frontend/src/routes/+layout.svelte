<script lang="ts">
	import '../app.css';
	import {
		Navbar,
		NavBrand,
		NavUl,
		NavHamburger,
		Spinner,
		Breadcrumb,
		BreadcrumbItem
	} from 'flowbite-svelte';

	import { invalidate } from '$app/navigation';
	import { onDestroy } from 'svelte';

	import { navBarTitle } from '$lib';

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

	function handleKeydown(event: KeyboardEvent) {
		if (event.metaKey && event.key == 'k') {
			// TODO: Make this work on non-MacOS keyboards
			// i.e. those which don't have a command key
			console.log(`Key pressed: cmd + ${event.key}`);
		}
	}
</script>

<svelte:document on:keydown={handleKeydown} />

<Navbar data-testid="navbar" class="bg-black" fluid>
	<NavBrand>
		<img
			style="height:40px"
			src="https://raw.githubusercontent.com/fluxcd/website/v2-3/assets/icons/logo.svg"
			alt="Flux CD Logo"
		/>
		<span class="text-xl text-white m-2"> <a href="/">Flux CD</a> </span>
		{#if $navBarTitle}
			<Breadcrumb>
				<BreadcrumbItem
					spanClass="ms-1 text-md text-white"
					linkClass="ms-1 text-md text-white ms-2"
				>
					{$navBarTitle}
				</BreadcrumbItem>
			</Breadcrumb>
		{/if}
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
	</NavUl>

	<!-- TODO: Add home icon / link -->
</Navbar>

<slot />
