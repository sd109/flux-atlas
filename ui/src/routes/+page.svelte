<script lang="ts">
	import CardGrid from './components/CardGrid.svelte';
	import ResourceCard from './components/ResourceListCard.svelte';
	export let data;

	import { navBarTitle } from '$lib';
	$navBarTitle = 'Overview';
</script>

<CardGrid>
	{#each Object.entries(data) as [title, items]}
		<ResourceCard {title} {items} />
	{/each}
</CardGrid>
