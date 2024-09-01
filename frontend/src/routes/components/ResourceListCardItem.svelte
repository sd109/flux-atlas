<script lang="ts">
	export let resource: ResourceView<ConditionAny>;

	let label = `${resource.namespace}/${resource.name}`;
	let statusText = 'Unknown';
	let statusClasses = '';
	if (resource.conditions.some((c) => c.type == 'Ready' && c.status == 'True')) {
		statusText = 'Ready';
		statusClasses += ' text-green-400';
	} else if (resource.conditions.some((c) => c.type == 'Reconciling' && c.status == 'True')) {
		statusText = 'Reconciling';
		statusClasses += ' text-orange-500';
	}
</script>

<div class="grid grid-cols-2 p-1 px-2">
	<span class="text-left">{label}</span>
	<span class={`text-right ${statusClasses}`}>{statusText}</span>
</div>
