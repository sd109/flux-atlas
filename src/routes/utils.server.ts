// NOTE: These utils can't live in $lib since k8s client is node/server only and
// leads to 'process variable not defined' errors in client side code.

import type { HelmRelease } from '@kubernetes-models/flux-cd/helm.toolkit.fluxcd.io/v2beta2';
import type { Kustomization } from '@kubernetes-models/flux-cd/kustomize.toolkit.fluxcd.io/v1';
import type { GitRepository } from '@kubernetes-models/flux-cd/source.toolkit.fluxcd.io/v1';
import type {
	HelmChart,
	HelmRepository,
	OCIRepository
} from '@kubernetes-models/flux-cd/source.toolkit.fluxcd.io/v1beta2';
import * as k8s from '@kubernetes/client-node';

type FluxAPIGroups =
	| 'source.toolkit.fluxcd.io'
	| 'helm.toolkit.fluxcd.io'
	| 'kustomize.toolkit.fluxcd.io';

type FluxAPIResources =
	| 'gitrepositories'
	| 'ocirepositories'
	| 'helmrepositories'
	| 'helmcharts'
	| 'helmreleases'
	| 'kustomizations';

type FluxAPIVersions = 'v1' | 'v1beta2' | 'v2';

interface FluxResourceType<T> {
	group: FluxAPIGroups;
	version: FluxAPIVersions;
	plural: FluxAPIResources;
}

export const GitRepoResourceGroup: FluxResourceType<GitRepository> = {
	group: 'source.toolkit.fluxcd.io',
	version: 'v1',
	plural: 'gitrepositories'
};

export const OciRepoResourceGroup: FluxResourceType<OCIRepository> = {
	group: 'source.toolkit.fluxcd.io',
	version: 'v1beta2',
	plural: 'ocirepositories'
};

export const HelmRepoResourceGroup: FluxResourceType<HelmRepository> = {
	group: 'source.toolkit.fluxcd.io',
	version: 'v1',
	plural: 'helmrepositories'
};

export const HelmChartResourceGroup: FluxResourceType<HelmChart> = {
	group: 'source.toolkit.fluxcd.io',
	version: 'v1',
	plural: 'helmcharts'
};

export const HelmReleaseResourceGroup: FluxResourceType<HelmRelease> = {
	group: 'helm.toolkit.fluxcd.io',
	version: 'v2',
	plural: 'helmreleases'
};

export const KustomizationResourceGroup: FluxResourceType<Kustomization> = {
	group: 'kustomize.toolkit.fluxcd.io',
	version: 'v1',
	plural: 'kustomizations'
};

export async function listFluxResource<T>(
	client: k8s.CustomObjectsApi,
	resource: FluxResourceType<T>
): Promise<T[]> {
	const response = await client.listClusterCustomObject({
		group: resource.group,
		version: resource.version,
		plural: resource.plural
	});
	return response.items;
}
