function toTimeStamps<C extends ResourceCondition, T extends ResourceView<C>>(resource: T) {
	resource.conditions = resource.conditions.map((c) => {
		c.lastTransitionTime = new Date(c.lastTransitionTime);
		return c;
	});
	return resource;
}

async function fetch_view<C extends ResourceCondition, T extends ResourceView<C>>(
	resource: string
) {
	const baseUrl = 'http://localhost:8000/api/';
	const resources: T[] = await (await fetch(new URL(resource, baseUrl))).json();
	return resources.map((r) => toTimeStamps(r));
}

export async function load({ fetch, depends }) {
	depends('flux:resources');

	const GitRepos: GitRepoView[] = await fetch_view('gitrepositories');
	const OCIRepos: OCIRepoView[] = await fetch_view('ocirepositories');
	const HelmRepos: HelmRepoView[] = await fetch_view('helmrepositories');
	const HelmCharts: HelmChartView[] = await fetch_view('helmcharts');
	const HelmReleases: HelmReleaseView[] = await fetch_view('helmreleases');
	const Kustomizations: KustomizationView[] = await fetch_view('kustomizations');

	return {
		HelmRepos,
		HelmCharts,
		HelmReleases,
		Kustomizations,
		GitRepos,
		OCIRepos
	};
}
