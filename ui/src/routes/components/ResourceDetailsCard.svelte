<script lang="ts">
	import { Card } from 'flowbite-svelte';
	import YamlModalButton from './YamlModalButton.svelte';
	import ResourceDetailsCardItem from './ResourceDetailsCardItem.svelte';

	export let resource: ResourceView<ConditionAny>;
	let title = `${resource.namespace}/${resource.name}`;

	let classes = 'relative border-2';

	const updated = resource.conditions.length == 0 ? '-' : resource.conditions[0].lastTransitionTime;
</script>

<Card size="xl" class={classes} id={title} data-testid="resource-details-card">
	<h1 class="text-xl font-bold mb-4 col-span-4">
		{title}
	</h1>
	<slot />
	<ResourceDetailsCardItem>
		Updated: {updated}
	</ResourceDetailsCardItem>
	<YamlModalButton {resource} />
</Card>
