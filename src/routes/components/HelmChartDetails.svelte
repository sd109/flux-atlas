<script lang="ts">
	import { A } from 'flowbite-svelte';
	import YamlModalButton from './YamlModalButton.svelte';
	import ResourceDetails from './ResourceDetails.svelte';
	import type { HelmChart } from '@kubernetes-models/flux-cd/source.toolkit.fluxcd.io/v1beta2';

	export let resource: HelmChart;

	let status = resource.status?.conditions
		? resource.status?.conditions[0]
		: { type: 'Unknown', status: 'unknown' };
	let statusClasses = '';
	if (status.type == 'Ready') {
		statusClasses += ' ';
		statusClasses += status.status.toLowerCase() == 'true' ? 'text-green-400' : 'text-red-400';
	} else if (status.type == 'Reconciling') {
		statusClasses += ' text-orange-500';
	}

	let version = resource.spec?.version;

	// Drop timezone text from date string
	const updated = resource.status?.artifact?.lastUpdateTime
		? new Date(resource.status!.artifact!.lastUpdateTime).toString().split(' (')[0]
		: 'Never';
</script>

<ResourceDetails>
	<span class="">Namespace: {resource.metadata?.namespace}</span>
	<span class="">Name: {resource.metadata?.name}</span>
	<span>Status: <span class={statusClasses}>{status.type}</span></span>
	<span>
		Chart:
		<span class="text-slate-400">{resource.spec?.sourceRef.name}/{resource.spec?.chart}</span>
	</span>
	<span>Version: {version}</span>
	<span>Updated: {updated}</span>
	<YamlModalButton {resource} />
</ResourceDetails>
