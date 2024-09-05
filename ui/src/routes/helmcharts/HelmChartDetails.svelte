<script lang="ts">
	import { A } from 'flowbite-svelte';
	import ResourceDetailsCard from '../components/ResourceDetailsCard.svelte';

	import { navBarTitle } from '$lib';
	import ResourceDetailsCardItem from '../components/ResourceDetailsCardItem.svelte';
	$navBarTitle = 'HelmCharts';

	export let chart: HelmChartView;

	let status = 'Unknown';
	if (chart.conditions.some((c) => c.type == 'Ready' && c.status == 'True')) {
		status = 'Ready';
	} else if (chart.conditions.some((c) => c.type == 'Reconciling' && c.status == 'True')) {
		status = 'Reconciling';
	}

	let statusColour = '';
	if (status == 'Ready') {
		statusColour += 'green-400';
	} else if (status == 'Reconciling') {
		statusColour += 'orange-400';
	}

	const sourceRefName = `${chart.source_ref.namespace}/${chart.source_ref.name}`;
	const sourceRefLink = `/${chart.source_ref.kind.toLowerCase().replace('repository', 'repo')}s#${sourceRefName}`;
</script>

<ResourceDetailsCard resource={chart}>
	<ResourceDetailsCardItem>
		Status: <span class={`text-${statusColour}`}>{status}</span>
	</ResourceDetailsCardItem>
	<ResourceDetailsCardItem>
		Source: <A href={sourceRefLink} class="text-slate-400">{sourceRefName}</A>
	</ResourceDetailsCardItem>
	<ResourceDetailsCardItem>
		Version: {chart.version}
	</ResourceDetailsCardItem>
</ResourceDetailsCard>
