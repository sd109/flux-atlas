<script lang="ts">
	import ResourceDetailsGrid from '../components/ResourceDetailsGrid.svelte';
	import OCIRepoDetails from './OCIRepoDetails.svelte';

	export let data;
	const key = 'OCIRepos';
</script>

<ResourceDetailsGrid {data} {key}>
	{#each data[key] as repo}
		<OCIRepoDetails {repo} />
	{/each}
</ResourceDetailsGrid>
