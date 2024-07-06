import * as k8s from '@kubernetes/client-node';
import type {
	HelmChart,
	HelmRepository,
	GitRepository,
	OCIRepository
} from '@kubernetes-models/flux-cd/source.toolkit.fluxcd.io/v1beta2';
import { type HelmRelease } from '@kubernetes-models/flux-cd/helm.toolkit.fluxcd.io/v2beta2';
import { type Kustomization } from '@kubernetes-models/flux-cd/kustomize.toolkit.fluxcd.io/v1';
import { error } from '@sveltejs/kit';

export async function load({ depends }) {
	depends('flux:resources');

	const kc = new k8s.KubeConfig();
	kc.loadFromDefault();

	// Check context was loaded successfully
	const cluster = kc.getCurrentCluster();
	if (!cluster) error(404, 'Current cluster context not found');
	// console.debug('Using cluster:', cluster.name);

	// Check cluster is reachable
	try {
		const client = kc.makeApiClient(k8s.CoreV1Api);
		await client.listNode();
	} catch {
		error(500, 'Kubernetes cluster unreachable');
	}

	const client = kc.makeApiClient(k8s.CustomObjectsApi);

	const ocirepos: OCIRepository[] = (
		await client.listClusterCustomObject({
			group: 'source.toolkit.fluxcd.io',
			version: 'v1beta2',
			plural: 'ocirepositories'
		})
	).items;

	const gitrepos: GitRepository[] = (
		await client.listClusterCustomObject({
			group: 'source.toolkit.fluxcd.io',
			version: 'v1',
			plural: 'gitrepositories'
		})
	).items;

	const helmrepos: HelmRepository[] = (
		await client.listClusterCustomObject({
			group: 'source.toolkit.fluxcd.io',
			version: 'v1',
			plural: 'helmrepositories'
		})
	).items;

	const helmcharts: HelmChart[] = (
		await client.listClusterCustomObject({
			group: 'source.toolkit.fluxcd.io',
			version: 'v1',
			plural: 'helmcharts'
		})
	).items;

	const helmreleases: HelmRelease[] = (
		await client.listClusterCustomObject({
			group: 'helm.toolkit.fluxcd.io',
			version: 'v2',
			plural: 'helmreleases'
		})
	).items;

	const kustomizations: Kustomization[] = (
		await client.listClusterCustomObject({
			group: 'kustomize.toolkit.fluxcd.io',
			version: 'v1',
			plural: 'kustomizations'
		})
	).items;

	return {
		gitrepos,
		ocirepos,
		helmrepos,
		helmcharts,
		helmreleases,
		kustomizations
	};
}
