<script lang="ts">
	import ResourceDetailsGrid from '../components/ResourceDetailsGrid.svelte';
	import HelmChartDetails from './HelmChartDetails.svelte';

	export let data;
	const key = 'HelmCharts';
</script>

<ResourceDetailsGrid {data} {key}>
	{#each data[key] as chart}
		<HelmChartDetails {chart} />
	{/each}
</ResourceDetailsGrid>
