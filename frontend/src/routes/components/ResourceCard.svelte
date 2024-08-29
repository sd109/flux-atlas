<script lang="ts">
	import { Card } from 'flowbite-svelte';
	import ResourceCardListItem from './ResourceCardListItem.svelte';

	export let key: string;
	export let items: any[];
</script>

<Card size="xl" class="" id={key}>
	<h1 class="text-3xl font-bold mb-4">
		<a href={`/${key.toLowerCase()}`}>{key}</a>
	</h1>
	{#if items.length == 0}
		<span class="px-2">No Resources</span>
	{:else}
		{#each items as r}
			<ResourceCardListItem resource={r} />
		{/each}
	{/if}
</Card>
