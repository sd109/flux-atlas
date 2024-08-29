<script lang="ts">
	import { A } from 'flowbite-svelte';
	import YamlModal from '../components/YamlModalButton.svelte';
	import ResourceDetails from '../components/ResourceDetails.svelte';

	export let repo: HelmRepoView;

	let statusText = 'Unknown';
	if (repo.conditions.some((c) => c.type == 'Ready' && c.status == 'True')) {
		statusText = 'Ready';
	} else if (repo.conditions.some((c) => c.type == 'Reconciling' && c.status == 'True')) {
		statusText = 'Reconciling';
	}

	let statusClasses = '';
	if (statusText == 'Ready') {
		statusClasses += ' text-green-400';
	} else if (statusText == 'Reconciling') {
		statusClasses += ' text-orange-500';
	}

	const updated = repo.conditions[0].lastTransitionTime;
</script>

<ResourceDetails>
	<span class="">Namespace: {repo.namespace}</span>
	<span class="">Name: {repo.name}</span>
	<span>Status: <span class={statusClasses}>{statusText}</span></span>
	<span>Source: <A href={repo.url} class="text-slate-400">{repo.url}</A></span>
	<span>Updated: {updated}</span>
	<YamlModal resource={repo} />
</ResourceDetails>
