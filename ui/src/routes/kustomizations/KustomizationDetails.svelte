<script lang="ts">
	import { A } from 'flowbite-svelte';
	import ResourceDetailsCard from '../components/ResourceDetailsCard.svelte';

	import { navBarTitle } from '$lib';
	import ResourceDetailsCardItem from '../components/ResourceDetailsCardItem.svelte';
	$navBarTitle = 'Kustomizations';

	export let k: KustomizationView;

	let status = 'Unknown';
	if (k.conditions.some((c) => c.type == 'Ready' && c.status == 'True')) {
		status = 'Ready';
	} else if (k.conditions.some((c) => c.type == 'Reconciling' && c.status == 'True')) {
		status = 'Reconciling';
	}

	let statusColour = '';
	if (status == 'Ready') {
		statusColour += 'green-400';
	} else if (status == 'Reconciling') {
		statusColour += 'orange-500';
	}

	const sourceTitle = `${k.source_ref.namespace}/${k.source_ref.name}`;
	const sourceLink = `${k.source_ref.kind.toLowerCase().replace('repository', 'repo')}s#${sourceTitle}`;
</script>

<ResourceDetailsCard resource={k}>
	<ResourceDetailsCardItem>
		Status: <span class={`text-${statusColour}`}>{status}</span>
	</ResourceDetailsCardItem>
	<ResourceDetailsCardItem>
		Source: <A href={sourceLink} class="text-slate-400">{sourceTitle}</A>
	</ResourceDetailsCardItem>
</ResourceDetailsCard>
