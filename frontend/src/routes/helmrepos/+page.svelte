<script lang="ts">
	import HelmRepoDetails from './HelmRepoDetails.svelte';

	export let data;
</script>

<div class="grid grid-cols-1 md:grid-cols-2 2xl:grid-cols-3 gap-4 m-4">
	<HelmRepoDetails repo={data.HelmRepos[0]} />
</div>
