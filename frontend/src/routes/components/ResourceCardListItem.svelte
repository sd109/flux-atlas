<script lang="ts">
	import { Modal } from 'flowbite-svelte';
	import { stringify } from 'yaml';

	export let resource: ResourceView;

	function handleClick() {
		modalVisible = true;
		modalTitle = `Namespace: ${resource.namespace} Name: ${resource.name}`;
		modalContent = stringify(resource);
	}

	let modalVisible = false;
	let modalTitle = '';
	let modalContent = '';

	let label = `${resource.namespace}/${resource.name}`;
	let statusClasses = 'text-right';
	if (resource.status == 'Ready') {
		// TODO: Re-implement status condition true/false in API layer
		// statusClasses += ' ';
		// statusClasses += resource.status.toLowerCase() == 'true' ? 'text-green-400' : 'text-red-400';
		statusClasses += ' text-green-400';
	} else if (resource.status == 'Reconciling') {
		statusClasses += ' text-orange-500';
	}
</script>

<button
	class="grid grid-cols-5 justify-between hover:bg-slate-100 rounded-sm p-1 px-2"
	on:click={handleClick}
>
	<span class="text-left col-span-4">{label}</span>
	<span class={statusClasses}>{resource.status}</span>
</button>

<Modal title={modalTitle} size="lg" bind:open={modalVisible} outsideclose>
	<div class="multiline border-0 object-fill">{modalContent}</div>
</Modal>

<style lang="postcss">
	.multiline {
		white-space: pre;
	}
</style>
