<script lang="ts">
	import ResourceCard from './components/ResourceCard.svelte';
	import { compactToggle, isCompact } from '$lib';
	import { page } from '$app/stores';

	export let data;

	$: layoutClasses = !isCompact($page.url, $compactToggle)
		? 'grid-cols-1'
		: 'grid-cols-1 md:grid-cols-2 2xl:grid-cols-3';
</script>

<div data-testid="resources" class={`grid ${layoutClasses} gap-4 m-4`}>
	{#each Object.entries(data) as [key, items]}
		<ResourceCard {key} {items} />
	{/each}
</div>
