use kube::{api::ListParams, Api, Client};
use kube_custom_resources_rs::kustomize_toolkit_fluxcd_io::v1::kustomizations::Kustomization;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct KustomizationView {
    name: String,
    namespace: String,
    source: String,
}

impl From<Kustomization> for KustomizationView {
    fn from(k: Kustomization) -> Self {
        // TODO: Can we do better than unwraps here?
        KustomizationView {
            name: k.metadata.name.unwrap_or_default(),
            namespace: k.metadata.namespace.unwrap_or_default(),
            source: k.spec.source_ref.name,
        }
    }
}

impl KustomizationView {
    pub async fn fetch(client: &Client) -> Vec<KustomizationView> {
        Api::all(client.to_owned())
            .list(&ListParams::default())
            .await
            .unwrap()
            .items
            .into_iter()
            .map(|k: Kustomization| KustomizationView::from(k))
            .collect()
    }
}
