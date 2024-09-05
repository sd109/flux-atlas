<script lang="ts">
	import ResourceDetailsGrid from '../components/ResourceDetailsGrid.svelte';
	import GitRepoDetails from './GitRepoDetails.svelte';

	export let data;
	const key = 'GitRepos';
</script>

<ResourceDetailsGrid {data} {key}>
	{#each data[key] as repo}
		<GitRepoDetails {repo} />
	{/each}
</ResourceDetailsGrid>
