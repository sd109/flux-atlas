interface ResourceView {
	name: string;
	namespace: string;
	status: string;
}

interface VersionRef {
	type: string;
	version: string;
}

interface HelmReleaseView extends ResourceView {
	chart: string;
}

interface KustomizationView extends ResourceView {
	source: string;
}

interface HelmRepoView extends ResourceView {
	url: string;
}

interface GitRepoView extends ResourceView {
	url: string;
	version: VersionRef;
}

interface OCIRepoView extends ResourceView {
	url: string;
	version: VersionRef;
}
