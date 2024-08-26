<script lang="ts">
	import YamlModalButton from './YamlModalButton.svelte';
	import ResourceDetails from './ResourceDetails.svelte';
	import type { Kustomization } from '@kubernetes-models/flux-cd/kustomize.toolkit.fluxcd.io/v1';

	export let resource: Kustomization;

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
	<span>Namespace: {resource.metadata?.namespace}</span>
	<span>Name: {resource.metadata?.name}</span>
	<span>
		Source: {resource.spec?.sourceRef.kind} - {resource.spec?.sourceRef.name}
	</span>
	<span>Status: <span class={statusClasses}>{status.type}</span></span>
	<span>Latest Message: {status.message}</span>
	<YamlModalButton {resource} />
</ResourceDetails>
