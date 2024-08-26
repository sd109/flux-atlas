// import * as k8s from '@kubernetes/client-node';
// import { error } from '@sveltejs/kit';
// import {
// 	GitRepoResourceGroup,
// 	HelmChartResourceGroup,
// 	HelmReleaseResourceGroup,
// 	HelmRepoResourceGroup,
// 	KustomizationResourceGroup,
// 	listFluxResource,
// 	OciRepoResourceGroup
// } from '$lib/server';

// export async function load({ depends }) {
// 	depends('flux:resources');

// 	const kc = new k8s.KubeConfig();
// 	kc.loadFromDefault();

// 	// Check context was loaded successfully
// 	const cluster = kc.getCurrentCluster();
// 	if (!cluster) error(404, 'Current cluster context not found');
// 	// console.debug('Using cluster:', cluster.name);

// 	// Check cluster is reachable
// 	try {
// 		const client = kc.makeApiClient(k8s.CoreV1Api);
// 		await client.listNamespace();
// 	} catch (err) {
// 		// console.error(err);
// 		error(500, 'Kubernetes cluster unreachable');
// 	}

// 	const client = kc.makeApiClient(k8s.CustomObjectsApi);

// 	const OCIRepos = await listFluxResource(client, OciRepoResourceGroup);

// 	const GitRepos = await listFluxResource(client, GitRepoResourceGroup);

// 	const HelmRepos = await listFluxResource(client, HelmRepoResourceGroup);

// 	const HelmCharts = await listFluxResource(client, HelmChartResourceGroup);

// 	const HelmReleases = await listFluxResource(client, HelmReleaseResourceGroup);

// 	const Kustomizations = await listFluxResource(client, KustomizationResourceGroup);

// 	return {
// 		GitRepos,
// 		OCIRepos,
// 		HelmRepos,
// 		HelmCharts,
// 		HelmReleases,
// 		Kustomizations
// 	};
// }
