<script lang="ts">
	import { A } from 'flowbite-svelte';
	import YamlModal from './YamlModalButton.svelte';

	export let resource: KubernetesObject;

	let status = resource.status.conditions[0];
	let statusClasses = '';
	if (status.type == 'Ready') {
		statusClasses += ' ';
		statusClasses += status.status.toLowerCase() == 'true' ? 'text-green-400' : 'text-red-400';
	} else if (status.type == 'Reconciling') {
		statusClasses += ' text-orange-500';
	}

	// Drop timezone text from date string
	// const updated = new Date(resource.status.artifact.lastUpdateTime).toString().split(' (')[0];
</script>

<div
	class="relative grid grid-cols-1 justify-between rounded-md m-1 py-1 px-2 border border-slate-200"
>
	<span class="">Namespace: {resource.metadata.namespace}</span>
	<span class="">Name: {resource.metadata.name}</span>
	<span>Status: <span class={statusClasses}>{status.type}</span></span>
	<!-- <span>Url: <A class="text-slate-400" href={resource.spec.url}>{resource.spec.url}</A></span> -->
	<!-- <span>Revision: {resource.status.artifact.revision}</span> -->
	<!-- <span>Updated: {updated}</span> -->
	<YamlModal {resource} />
</div>
