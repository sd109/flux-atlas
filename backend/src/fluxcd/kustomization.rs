use kube_custom_resources_rs::kustomize_toolkit_fluxcd_io::v1::kustomizations::Kustomization;
use rocket::serde::Serialize;

use super::utils::latest_status;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct KustomizationView {
    name: String,
    namespace: String,
    source: String,
    status: String,
}

impl From<Kustomization> for KustomizationView {
    fn from(k: Kustomization) -> Self {
        // TODO: Can we do better than unwraps here?
        KustomizationView {
            name: k.metadata.name.unwrap_or_default(),
            namespace: k.metadata.namespace.unwrap_or_default(),
            source: k.spec.source_ref.name,
            status: match k.status {
                Some(status) => latest_status(status.conditions),
                None => String::new(),
            },
        }
    }
}
