<script lang="ts">
	import { ListgroupItem, Modal } from 'flowbite-svelte';
	import { stringify } from 'yaml';

	// export let resource: KubernetesObject & { status: string };
	export let resource: KubernetesObject;

	function handleClick() {
		modalVisible = true;
		modalTitle = `Namespace: ${resource.metadata?.namespace} Name: ${resource.metadata?.name}`;
		if (resource.metadata?.managedFields) {
			delete resource.metadata?.managedFields;
		}
		modalContent = stringify(resource);
	}

	let modalVisible = false;
	let modalTitle = '';
	let modalContent = '';

	let label = `${resource.metadata?.namespace}/${resource.metadata?.name}`;
	let status = resource.status.conditions[0];
	let statusClasses = 'text-right';
	if (status.type == 'Ready') {
		statusClasses += ' ';
		statusClasses += status.status.toLowerCase() == 'true' ? 'text-green-400' : 'text-red-400';
	} else if (status.type == 'Reconciling') {
		statusClasses += ' text-orange-500';
	}
</script>

<button
	class="grid grid-cols-5 justify-between hover:bg-slate-100 rounded-sm p-1 px-2"
	on:click={handleClick}
>
	<span class="text-left col-span-4">{label}</span>
	<span class={statusClasses}>{status.type}</span>
</button>

<Modal title={modalTitle} size="lg" bind:open={modalVisible} outsideclose>
	<div class="multiline border-0 object-fill">{modalContent}</div>
</Modal>

<style lang="postcss">
	.multiline {
		white-space: pre;
	}
</style>
