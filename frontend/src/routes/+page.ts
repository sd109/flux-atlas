export async function load({ fetch }) {
	const baseUrl = 'http://localhost:8000/api/';
	const fetch_view = async (resource: string) => (await fetch(new URL(resource, baseUrl))).json();
	return {
		HelmRepos: await fetch_view('helmrepositories'),
		HelmCharts: await fetch_view('helmcharts'),
		HelmReleases: await fetch_view('helmreleases'),
		Kustomizations: await fetch_view('kustomizations'),
		GitRepos: await fetch_view('gitrepositories'),
		OCIRepos: await fetch_view('ocirepositories')
	};
}
