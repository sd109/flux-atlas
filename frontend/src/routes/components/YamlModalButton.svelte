<script lang="ts">
	export let resource: ResourceView<ConditionAny>;

	import { Modal } from 'flowbite-svelte';
	import { stringify } from 'yaml';

	function handleClick() {
		modalVisible = true;
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
