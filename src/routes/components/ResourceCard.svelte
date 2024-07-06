<script lang="ts">
	import { Card } from 'flowbite-svelte';
	import ResourceCardListItem from './ResourceCardListItem.svelte';
	import { compactView } from '$lib';
	import GitRepoDetails from './GitRepoDetails.svelte';
	import OciRepoDetails from './OciRepoDetails.svelte';
	import HelmRepoDetails from './HelmRepoDetails.svelte';
	import HelmChartDetails from './HelmChartDetails.svelte';
	import HelmReleaseDetails from './HelmReleaseDetails.svelte';
	import KustomizationDetails from './KustomizationDetails.svelte';

	export let key: string;
	export let items: any[];

	// Fall back to key as heading if required
	let title = items.length > 0 ? items[0].kind : key[0] + key.slice(1);
</script>

<Card size={$compactView ? 'xl' : 'none'} class="" id={key}>
	<h1 class="text-3xl font-bold mb-4">
		{title}
	</h1>
	{#if items.length == 0}
		<span class="px-2">No Resources</span>
	{:else}
		{#each items as r}
			{#if $compactView}
				<ResourceCardListItem resource={r} />
			{:else if key.toLowerCase() == 'gitrepos'}
				<GitRepoDetails resource={r} />
			{:else if key.toLowerCase() == 'ocirepos'}
				<OciRepoDetails resource={r} />
			{:else if key.toLowerCase() == 'helmrepos'}
				<HelmRepoDetails resource={r} />
			{:else if key.toLowerCase() == 'helmcharts'}
				<HelmChartDetails resource={r} />
			{:else if key.toLowerCase() == 'helmreleases'}
				<HelmReleaseDetails resource={r} />
			{:else if key.toLowerCase() == 'kustomizations'}
				<KustomizationDetails resource={r} />
			{:else}
				<p>Detailed resource view not implemented for {key}</p>
			{/if}
		{/each}
	{/if}
</Card>
