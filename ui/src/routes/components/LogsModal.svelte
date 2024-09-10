<script lang="ts">
	export const ssr = false;
	/*
	TODO: Add an explanation of how this component works and the relevant
	implementation details including SSE etc.
	*/

	import { Checkbox, Input, Listgroup, Modal, P } from 'flowbite-svelte';
	import { afterUpdate, beforeUpdate, onDestroy } from 'svelte';
	import type { Unsubscriber } from 'svelte/store';
	import { source, type Source } from 'sveltekit-sse';

	// Props
	export let open: boolean; // Is modal visible

	const controllers = ['source', 'helm', 'kustomize'];
	let streams: Map<string, Source> = new Map();
	let unsubscribers: Unsubscriber[] = [];
	let logLines: string[] = [];

	// Subscribe to server-sent-events
	// Only do this once modal is opened
	beforeUpdate(() => {
		if (open) {
			for (let controller of controllers) {
				// Avoid duplicating streams
				if (!streams.has(controller)) {
					// console.log(`Subscribing to event source ${controller}`);
					const eventStream = source(`/logs/controllers/${controller}`);
					const logStream = eventStream.select('message');
					// Each SSE overwrites the previous so store all events
					const unsubscribe = logStream.subscribe(function store(data) {
						// Strip out empty lines
						if (data) {
							// Add spaces to allow text wrapping via CSS
							data = data.replaceAll(',', ', ').replaceAll('":', '": ');
							logLines = [...logLines, data];
						}
					});
					// Store streams and unsubscribers for later use
					// streams.push(eventStream);
					streams.set(controller, eventStream);
					unsubscribers.push(unsubscribe);
				}
			}
		}
	});

	function cleanupStreams() {
		for (let s of streams.values()) {
			// console.log('Closing stream');
			s.close();
		}
		for (let unsub of unsubscribers) {
			// console.log('Unsubscribing from store');
			unsub();
		}
	}

	// Clean up when modal is closed or destroyed
	afterUpdate(() => {
		if (!open) cleanupStreams();
	});
	onDestroy(cleanupStreams);

	// Modal interactivity
	let followLogs = false;
	let searchText = '';
	$: filteredLogLines = logLines.filter((line) => line.includes(searchText));

	// Init scroll to bottom hook
	const id = 'logs-lines-ul-element';
	afterUpdate(() => {
		if (followLogs) {
			// id refers to <ul> but parent <div> is the node with
			// overflow-y-auto so this is the one that needs to scroll
			const element = document.getElementById(id)?.parentElement;
			if (element) {
				element.scroll({ top: element.scrollHeight, behavior: 'smooth' });
			}
		}
	});
</script>

<Modal title="Logs" size="xl" bind:open outsideclose>
	{#if filteredLogLines.length > 0}
		<Listgroup items={filteredLogLines} let:item {id}>
			{#if searchText}
				<!-- Highlight search term in log lines -->
				{@html item.replaceAll(searchText, `<b class="bg-orange-100">${searchText}</b>`)}
			{:else}
				{item}
			{/if}
		</Listgroup>
	{:else}
		<p>No logs</p>
	{/if}
	<svelte:fragment slot="footer">
		<Input bind:value={searchText} placeholder="Search" />
		<P class="w-52 text-center">{filteredLogLines.length} matched lines</P>
		<Checkbox bind:checked={followLogs}>Follow</Checkbox>
	</svelte:fragment>
</Modal>
