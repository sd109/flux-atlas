use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube_custom_resources_rs::kustomize_toolkit_fluxcd_io::v1::kustomizations::Kustomization;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct KustomizationView {
    name: String,
    namespace: String,
    source: String,
    conditions: Vec<Condition>,
    suspended: bool,
}

impl From<Kustomization> for KustomizationView {
    fn from(k: Kustomization) -> Self {
        KustomizationView {
            name: k.metadata.name.unwrap_or_default(),
            namespace: k.metadata.namespace.unwrap_or_default(),
            source: k.spec.source_ref.name,
            conditions: match k.status {
                Some(status) => status.conditions.unwrap_or_default(),
                None => vec![],
            },
            suspended: k.spec.suspend.unwrap_or(false),
        }
    }
}
