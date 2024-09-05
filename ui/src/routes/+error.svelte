<script>
	import { page } from '$app/stores';
</script>

<div>
	<h1>Error {$page.status}: {$page.error?.message}</h1>
</div>

<style lang="postcss">
	h1 {
		@apply text-center my-64;
	}
</style>
