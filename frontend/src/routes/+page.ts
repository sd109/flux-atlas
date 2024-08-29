export async function load({ fetch, depends }) {
	depends('flux:resources');
	const baseUrl = 'http://localhost:8000/api/';
	const fetch_view = async (resource: string) => (await fetch(new URL(resource, baseUrl))).json();

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
