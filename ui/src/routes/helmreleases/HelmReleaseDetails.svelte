<script lang="ts">
	import { A } from 'flowbite-svelte';
	import ResourceDetailsCard from '../components/ResourceDetailsCard.svelte';

	import { navBarTitle } from '$lib';
	import ResourceDetailsCardItem from '../components/ResourceDetailsCardItem.svelte';
	$navBarTitle = 'HelmReleases';

	export let hr: HelmReleaseView;

	const chart = `${hr.chart_ref.namespace}/${hr.chart_ref.name}`;
	const chartLink = `${hr.chart_ref.kind.toLowerCase()}s#${chart}`;

	let status = 'Unknown';
	if (hr.conditions.some((c) => c.type == 'Ready' && c.status == 'True')) {
		status = 'Ready';
	} else if (hr.conditions.some((c) => c.type == 'Reconciling' && c.status == 'True')) {
		status = 'Reconciling';
	}

	let statusColour = '';
	if (status == 'Ready') {
		statusColour += 'green-400';
	} else if (status == 'Reconciling') {
		statusColour += 'orange-500';
	}

	const updated = hr.conditions[0].lastTransitionTime;
</script>

<ResourceDetailsCard resource={hr}>
	<ResourceDetailsCardItem>
		Status: <span class={`text-${statusColour}`}>{status}</span>
	</ResourceDetailsCardItem>
	<ResourceDetailsCardItem>
		Chart: <A href={chartLink} class="text-slate-400">{chart}</A>
	</ResourceDetailsCardItem>
</ResourceDetailsCard>
