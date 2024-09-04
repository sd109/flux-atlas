<script lang="ts">
	import { Card } from 'flowbite-svelte';
	import ResourceCardListItem from './ResourceListCardItem.svelte';

	export let title: string;
	export let items: any[];
</script>

<Card size="xl" class="hover:bg-slate-100" id={title}>
	<a href={title.toLowerCase()}>
		<h1 class="text-3xl font-bold mb-4">
			{title}
		</h1>
		{#if items.length == 0}
			<span class="px-2">No Resources</span>
		{:else}
			{#each items as r}
				<ResourceCardListItem resource={r} />
			{/each}
		{/if}
	</a>
</Card>
