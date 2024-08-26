<script lang="ts">
	import YamlModalButton from './YamlModalButton.svelte';
	import ResourceDetails from './ResourceDetails.svelte';
	import type { HelmRelease } from '@kubernetes-models/flux-cd/helm.toolkit.fluxcd.io/v2beta2';

	export let resource: HelmRelease;

	let status = resource.status?.conditions
		? resource.status?.conditions[0]
		: { type: 'Unknown', status: 'unknown', message: '' };
	let statusClasses = '';
	if (status.type == 'Ready') {
		statusClasses += ' ';
		statusClasses += status.status.toLowerCase() == 'true' ? 'text-green-400' : 'text-red-400';
	} else if (status.type == 'Reconciling') {
		statusClasses += ' text-orange-500';
	}
</script>

<ResourceDetails>
	<span class="">Namespace: {resource.metadata?.namespace}</span>
	<span class="">Name: {resource.metadata?.name}</span>
	<span>Status: <span class={statusClasses}>{status.type}</span></span>
	<span>Latest Message: {status.message}</span>
	<YamlModalButton {resource} />
</ResourceDetails>
