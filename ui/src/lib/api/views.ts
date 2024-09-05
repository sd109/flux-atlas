// Data structures for FluxCD and backend API responses

// Shared types

interface ResourceView<T> {
	name: string;
	namespace: string;
	suspended: boolean;
	interval: string;
	conditions: T[];
}

interface VersionRef {
	type: string;
	version: string;
}

interface ResourceCondition {
	status: 'True' | 'False';
	reason: string;
	lastTransitionTime: Date;
	message: string;
}

type ConditionAny = SourceCondition | HelmReleaseCondition | KustomizationCondition;

// Sources

// Based on https://fluxcd.io/flux/components/source/gitrepositories/#conditions
type SourceConditionType =
	| 'Ready'
	| 'Failed'
	| 'Reconciling'
	| 'ArtifactInStorage'
	| 'FetchFailed'
	| 'IncludeUnavailable'
	| 'StorageOperationFailed';

interface SourceCondition extends ResourceCondition {
	type: SourceConditionType;
}

interface GitRepoView extends ResourceView<SourceCondition> {
	url: string;
	target_ref: VersionRef;
}

interface OCIRepoView extends ResourceView<SourceCondition> {
	url: string;
	target_ref: VersionRef;
}

interface HelmRepoView extends ResourceView<SourceCondition> {
	url: string;
}

interface HelmChartView extends ResourceView<SourceCondition> {
	source_ref: HelmChartSourceRef;
	chart: string;
	version: string;
}

interface HelmChartSourceRef {
	kind: 'HelmRepository' | 'GitRepository' | 'Bucket';
	name: string;
	namespace: string;
}

// Helm

interface HelmReleaseView extends ResourceView<HelmReleaseCondition> {
	chart_ref: HelmChartRef;
}

interface HelmChartRef {
	kind: 'OCIRepository' | 'HelmChart';
	name: string;
	namespace: string;
}

type HelmReleaseConditionType =
	| 'Ready'
	| 'Failed'
	| 'Reconciling'
	| 'TestSuccess'
	| 'Released'
	| 'Remediated';

interface HelmReleaseCondition extends ResourceCondition {
	type: HelmReleaseConditionType;
}

// Kustomization

interface KustomizationView extends ResourceView<KustomizationCondition> {
	source_ref: KustomizationSourceRef;
}

interface KustomizationSourceRef {
	kind: 'OCIRepository' | 'GitRepository' | 'Bucket';
	name: string;
	namespace: string;
}

type KustomizationConditionType = 'Ready' | 'Failed' | 'Reconciling' | 'Healthy';

interface KustomizationCondition extends ResourceCondition {
	type: KustomizationConditionType;
}
