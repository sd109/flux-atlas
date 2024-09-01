<script lang="ts">
	import { A } from 'flowbite-svelte';
	import ResourceDetailsCard from '../components/ResourceDetailsCard.svelte';
	import ResourceDetailsCardItem from '../components/ResourceDetailsCardItem.svelte';

	export let repo: GitRepoView;

	let status = 'Unknown';
	if (repo.conditions.some((c) => c.type == 'Ready' && c.status == 'True')) {
		status = 'Ready';
	} else if (repo.conditions.some((c) => c.type == 'Reconciling' && c.status == 'True')) {
		status = 'Reconciling';
	}

	let statusColour = '';
	if (status == 'Ready') {
		statusColour += 'green-400';
	} else if (status == 'Reconciling') {
		statusColour += 'orange-500';
	}

	const updated = repo.conditions[0].lastTransitionTime;
</script>

<ResourceDetailsCard resource={repo}>
	<ResourceDetailsCardItem>
		Status: <span class={`text-${statusColour}`}>{status}</span>
	</ResourceDetailsCardItem>
	<ResourceDetailsCardItem>
		Source: <A href={repo.url} class="text-slate-400">{repo.url}</A>
	</ResourceDetailsCardItem>
</ResourceDetailsCard>
