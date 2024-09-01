<script lang="ts">
	import ResourceDetailsGrid from '../components/ResourceDetailsGrid.svelte';
	import HelmRepoDetails from './HelmReleaseDetails.svelte';

	export let data;
	const key = 'HelmReleases';
</script>

<ResourceDetailsGrid {data} {key}>
	{#each data[key] as hr}
		<HelmRepoDetails {hr} />
	{/each}
</ResourceDetailsGrid>
