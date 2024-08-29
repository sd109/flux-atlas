// Data structures for FluxCD and backend API responses

// TODO: Test these data structures in frontend unit tests targetting backend API?

// Shared types

interface ResourceView {
	name: string;
	namespace: string;
	suspended: boolean;
	interval: string;
	conditions: SourceCondition[] | HelmReleaseCondition[] | KustomizationCondition[];
}

interface VersionRef {
	type: string;
	version: string;
}

interface ResourceCondition {
	status: 'True' | 'False';
	reason: string;
	timestamp: string;
	message: string;
}

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

interface GitRepoView extends ResourceView {
	url: string;
	target_ref: VersionRef;
	conditions: SourceCondition[];
}

interface OCIRepoView extends ResourceView {
	url: string;
	target_ref: VersionRef;
	conditions: SourceCondition[];
}

interface HelmRepoView extends ResourceView {
	url: string;
	conditions: SourceCondition[];
}

interface HelmChartView extends ResourceView {
	repo: string;
	chart: string;
	version: string;
	conditions: SourceCondition[];
}

// Helm

interface HelmReleaseView extends ResourceView {
	chart_ref: HelmChartRef;
	conditions: HelmReleaseCondition[];
	interval: string;
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

interface KustomizationView extends ResourceView {
	source_ref: KustomizationSourceRef;
	conditions: KustomizationCondition[];
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
