<script lang="ts">
	import CardGrid from './CardGrid.svelte';
	import { navBarTitle } from '$lib';

	export let data: any;
	export let key: string;
	const items = data[key];

	$navBarTitle = key;
</script>

{#if items.length > 0}
	<CardGrid>
		<slot />
	</CardGrid>
{:else}
	<div>
		<h1>No Resources</h1>
	</div>
{/if}

<style lang="postcss">
	h1 {
		@apply text-center my-64;
	}
</style>
