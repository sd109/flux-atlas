<script lang="ts">
	import type { HelmRelease } from '@kubernetes-models/flux-cd/helm.toolkit.fluxcd.io/v2beta2';
	import type { GitRepository } from '@kubernetes-models/flux-cd/source.toolkit.fluxcd.io/v1';
	import type {
		HelmChart,
		HelmRepository,
		OCIRepository
	} from '@kubernetes-models/flux-cd/source.toolkit.fluxcd.io/v1beta2';

	export let resource:
		| KubernetesObject
		| GitRepository
		| OCIRepository
		| HelmRepository
		| HelmChart
		| HelmRelease;

	import { Modal } from 'flowbite-svelte';
	import { stringify } from 'yaml';

	function handleClick() {
		modalVisible = true;
		if (resource.metadata?.managedFields) {
			delete resource.metadata?.managedFields;
		}
		modalContent = stringify(resource);
	}

	let modalVisible = false;
	let modalContent = '';
</script>

<button
	class="absolute right-3 top-3 border border-slate-700 rounded-md px-6 py-2 hover:bg-slate-100"
	on:click={handleClick}
>
	Details
</button>

<Modal title="Resource Details" size="lg" bind:open={modalVisible} outsideclose>
	<div class="multiline border-0 object-fill">{modalContent}</div>
</Modal>

<style lang="postcss">
	.multiline {
		white-space: pre;
	}
</style>
