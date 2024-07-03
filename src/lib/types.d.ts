interface KubernetesObjectMetadata {
	name: string;
	namespace: string;
	managedFields: any;
}

interface KubernetesObject {
	metadata: KubernetesObjectMetadata;
	spec: any;
	status: any;
}
