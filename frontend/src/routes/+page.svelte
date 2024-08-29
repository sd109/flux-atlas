<script lang="ts">
	import ResourceCard from './components/ResourceCard.svelte';
	export let data;
</script>

<div data-testid="resources" class="grid grid-cols-1 md:grid-cols-2 2xl:grid-cols-3 gap-4 m-4">
	{#each Object.entries(data) as [key, items]}
		<ResourceCard {key} {items} />
	{/each}
</div>
