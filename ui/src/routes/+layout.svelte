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
	import LogsModal from './components/LogsModal.svelte';

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

	const textStyle = 'text-white text-md hover:text-slate-400';

	let modalVisible = false;
</script>

<svelte:document on:keydown={handleKeydown} />

<Navbar data-testid="navbar" class="bg-black" fluid>
	<NavBrand>
		{#if $navBarTitle}
			<Breadcrumb>
				<BreadcrumbItem href="/" classSpan={textStyle} classHome={textStyle} home>
					<svelte:fragment slot="icon">
						<img
							class="h-10 me-4"
							src="https://raw.githubusercontent.com/fluxcd/website/v2-3/assets/icons/logo.svg"
							alt="Flux CD Logo"
						/>
					</svelte:fragment>
					Flux CD
				</BreadcrumbItem>
				<BreadcrumbItem classSpan={textStyle}>
					{$navBarTitle}
				</BreadcrumbItem>
			</Breadcrumb>
		{/if}
	</NavBrand>

	<!-- Hamburger element is reactive menu shrinker / dropdown for small screens -->
	<NavHamburger />

	<NavUl>
		<button class={textStyle} on:click={() => (modalVisible = !modalVisible)}>Flux Logs</button>
		<!-- Only use white text above Hamburger break point -->
		<button class={textStyle} on:click={handleRefresh} disabled={!refreshEnabled}>
			{#if refreshEnabled}
				Refresh
			{:else}
				<Spinner class="mx-3" size="6" />
			{/if}
		</button>
	</NavUl>
</Navbar>

<LogsModal bind:open={modalVisible} />

<slot />
