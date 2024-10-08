import { env } from '$env/dynamic/private';
import { error } from '@sveltejs/kit';

function toTimeStamps<C extends ResourceCondition, T extends ResourceView<C>>(resource: T): T {
	resource.conditions = resource.conditions.map((c) => {
		c.lastTransitionTime = new Date(c.lastTransitionTime);
		return c;
	});
	return resource;
}

export async function load({ fetch, depends }) {
	depends('flux:resources');

	// Use Svelte's fetch function from load args
	async function fetch_view<C extends ResourceCondition, T extends ResourceView<C>>(
		resource: string
	): Promise<T[]> {
		const baseUrl = env.FLUX_ATLAS_API_ADDRESS || 'http://localhost:8000/api/';

		const response = await fetch(new URL(resource, baseUrl));
		if (!response.ok) throw Error(await response.text());
		const resources: T[] = await response.json();
		return resources.map((r) => toTimeStamps(r));
	}

	try {
		// TODO: Make these fetches concurrently and then await all
		const GitRepos: GitRepoView[] = await fetch_view('git-repos');
		const OCIRepos: OCIRepoView[] = await fetch_view('oci-repos');
		const HelmRepos: HelmRepoView[] = await fetch_view('helm-repos');
		const HelmCharts: HelmChartView[] = await fetch_view('helm-charts');
		const HelmReleases: HelmReleaseView[] = await fetch_view('helm-releases');
		const Kustomizations: KustomizationView[] = await fetch_view('kustomizations');

		return {
			HelmRepos,
			HelmCharts,
			HelmReleases,
			Kustomizations,
			GitRepos,
			OCIRepos
		};
	} catch (e: any) {
		error(503, e.toString());
	}
}
