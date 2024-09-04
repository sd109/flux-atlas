<script lang="ts">
	import { A } from 'flowbite-svelte';
	import ResourceDetailsCard from '../components/ResourceDetailsCard.svelte';

	import { navBarTitle } from '$lib';
	import ResourceDetailsCardItem from '../components/ResourceDetailsCardItem.svelte';
	$navBarTitle = 'HelmRepos';

	export let repo: HelmRepoView;

	let status = '-';
	let statusColour = '';
	if (repo.conditions.length > 0) {
		status = 'Unknown';
		if (repo.conditions.some((c) => c.type == 'Ready' && c.status == 'True')) {
			status = 'Ready';
		} else if (repo.conditions.some((c) => c.type == 'Reconciling' && c.status == 'True')) {
			status = 'Reconciling';
		}

		if (status == 'Ready') {
			statusColour += 'green-400';
		} else if (status == 'Reconciling') {
			statusColour += 'orange-500';
		}
	}
</script>

<ResourceDetailsCard resource={repo}>
	<ResourceDetailsCardItem>
		Status: <span class={`text-${statusColour}`}>{status}</span>
	</ResourceDetailsCardItem>
	<ResourceDetailsCardItem>
		<!-- TODO: Is linking to index.yaml of site always going to work? -->
		Source: <A href={repo.url + '/index.yaml'} class="text-slate-400">{repo.url}</A>
	</ResourceDetailsCardItem>
</ResourceDetailsCard>
