<script lang="ts">
	import ResourceDetailsGrid from '../components/ResourceDetailsGrid.svelte';
	import KustomizationDetails from './KustomizationDetails.svelte';

	export let data;
	const key = 'Kustomizations';
</script>

<ResourceDetailsGrid {data} {key}>
	{#each data[key] as repo}
		<KustomizationDetails k={repo} />
	{/each}
</ResourceDetailsGrid>
