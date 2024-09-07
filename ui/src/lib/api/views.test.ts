import { describe, it, expect } from 'vitest';

describe('API responses match interface definition', async () => {
	const sourceConditionTypeList: SourceConditionType[] = [
		'Ready',
		'Failed',
		'Reconciling',
		'ArtifactInStorage',
		'FetchFailed',
		'IncludeUnavailable',
		'StorageOperationFailed'
	];

	it('Git repo view', async () => {
		const repos: GitRepoView[] = await (await fetch('http://localhost:8000/api/git-repos')).json();
		expect(repos.length).greaterThan(0);
		repos.map((repo) => {
			// Basic props
			expect(repo.name.length).greaterThan(0);
			expect(repo.namespace.length).greaterThan(0);
			expect(repo.url.length).greaterThan(0);
			expect(repo.yaml.length).greaterThan(0);
			expect(repo.suspended).toBeTypeOf('boolean');
			expect(repo.interval).toBeTypeOf('string');
			expect(repo.target_ref.type).toBeDefined();
			expect(repo.target_ref.version).toBeDefined();
			// Conditions
			expect(repo.conditions.length).greaterThan(0);
			repo.conditions.map((c) => {
				expect(['True', 'False']).toContain(c.status);
				expect(sourceConditionTypeList).toContain(c.type);
				expect(c.reason).toBeTypeOf('string');
				expect(c.message).toBeTypeOf('string');
			});
		});
	});

	it('OCI repo view', async () => {
		const repos: OCIRepoView[] = await (await fetch('http://localhost:8000/api/oci-repos')).json();
		expect(repos.length).greaterThan(0);
		repos.map((repo) => {
			// Basic props
			expect(repo.name.length).greaterThan(0);
			expect(repo.namespace.length).greaterThan(0);
			expect(repo.url.length).greaterThan(0);
			expect(repo.yaml.length).greaterThan(0);
			expect(repo.suspended).toBeTypeOf('boolean');
			expect(repo.interval).toBeTypeOf('string');
			expect(repo.target_ref.type).toBeDefined();
			expect(repo.target_ref.version).toBeDefined();
			// Conditions
			expect(repo.conditions.length).greaterThan(0);
			repo.conditions.map((c) => {
				expect(['True', 'False']).toContain(c.status);
				expect(sourceConditionTypeList).toContain(c.type);
				expect(c.reason).toBeTypeOf('string');
				expect(c.message).toBeTypeOf('string');
			});
		});
	});

	it('Helm repo view', async () => {
		const repos: HelmRepoView[] = await (
			await fetch('http://localhost:8000/api/helm-repos')
		).json();
		expect(repos.length).greaterThan(0);
		repos.map((repo) => {
			// Basic props
			expect(repo.name.length).greaterThan(0);
			expect(repo.namespace.length).greaterThan(0);
			expect(repo.url.length).greaterThan(0);
			expect(repo.yaml.length).greaterThan(0);
			expect(repo.suspended).toBeTypeOf('boolean');
			expect(repo.interval).toBeTypeOf('string');
			// Conditions
			// TODO: Do OCI repos ever have conditions...?
			if (!repo.url.startsWith('oci://')) {
				expect(repo.conditions.length).greaterThan(0);
				repo.conditions.map((c) => {
					expect(['True', 'False']).toContain(c.status);
					expect(sourceConditionTypeList).toContain(c.type);
					expect(c.reason).toBeTypeOf('string');
					expect(c.message).toBeTypeOf('string');
				});
			}
		});
	});

	it('Helm chart view', async () => {
		const charts: HelmChartView[] = await (
			await fetch('http://localhost:8000/api/helm-charts')
		).json();
		expect(charts.length).greaterThan(0);
		charts.map((chart) => {
			// Basic props
			expect(chart.name.length).greaterThan(0);
			expect(chart.namespace.length).greaterThan(0);
			expect(chart.yaml.length).greaterThan(0);
			expect(chart.source_ref.kind.length).greaterThan(0);
			expect(chart.source_ref.name.length).greaterThan(0);
			expect(chart.source_ref.namespace.length).greaterThan(0);
			expect(chart.suspended).toBeTypeOf('boolean');
			expect(chart.interval).toBeTypeOf('string');
			// Conditions
			expect(chart.conditions.length).greaterThan(0);
			chart.conditions.map((c) => {
				expect(['True', 'False']).toContain(c.status);
				expect(sourceConditionTypeList).toContain(c.type);
				expect(c.reason).toBeTypeOf('string');
				expect(c.message).toBeTypeOf('string');
			});
		});
	});

	const helmReleaseConditionList: HelmReleaseConditionType[] = [
		'Ready',
		'Failed',
		'Reconciling',
		'TestSuccess',
		'Released',
		'Remediated'
	];

	it('Helm release view', async () => {
		const helmreleases: HelmReleaseView[] = await (
			await fetch('http://localhost:8000/api/helm-releases')
		).json();
		expect(helmreleases.length).greaterThan(0);
		helmreleases.map((hr) => {
			// Basic props
			expect(hr.name.length).greaterThan(0);
			expect(hr.namespace.length).greaterThan(0);
			expect(hr.yaml.length).greaterThan(0);
			expect(['OCIRepository', 'HelmChart']).toContain(hr.chart_ref.kind);
			expect(hr.suspended).toBeTypeOf('boolean');
			expect(hr.interval).toBeTypeOf('string');
			// Conditions
			expect(hr.conditions.length).greaterThan(0);
			hr.conditions.map((c) => {
				expect(['True', 'False']).toContain(c.status);
				expect(helmReleaseConditionList).toContain(c.type);
				expect(c.reason).toBeTypeOf('string');
				expect(c.message).toBeTypeOf('string');
			});
		});
	});

	const kustomizationConditionTypes: KustomizationConditionType[] = [
		'Ready',
		'Failed',
		'Reconciling',
		'Healthy'
	];

	it('Kustomization view', async () => {
		const kustomizations: KustomizationView[] = await (
			await fetch('http://localhost:8000/api/kustomizations')
		).json();
		expect(kustomizations.length).greaterThan(0);
		kustomizations.map((k) => {
			// Basic props
			expect(k.name.length).greaterThan(0);
			expect(k.namespace.length).greaterThan(0);
			expect(k.yaml.length).greaterThan(0);
			expect(['OCIRepository', 'GitRepository', 'Bucket']).toContain(k.source_ref.kind);
			expect(k.suspended).toBeTypeOf('boolean');
			expect(k.interval).toBeTypeOf('string');
			// Conditions
			expect(k.conditions.length).greaterThan(0);
			k.conditions.map((c) => {
				expect(['True', 'False']).toContain(c.status);
				expect(kustomizationConditionTypes).toContain(c.type);
				expect(c.reason).toBeTypeOf('string');
				expect(c.message).toBeTypeOf('string');
			});
		});
	});
});
