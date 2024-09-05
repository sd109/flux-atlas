<script lang="ts">
	import ResourceDetailsGrid from '../components/ResourceDetailsGrid.svelte';
	import HelmRepoDetails from './HelmRepoDetails.svelte';

	export let data;
	const key = 'HelmRepos';
</script>

<ResourceDetailsGrid {data} {key}>
	{#each data[key] as repo}
		<HelmRepoDetails {repo} />
	{/each}
</ResourceDetailsGrid>
