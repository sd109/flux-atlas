use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1beta2::ocirepositories::OCIRepository;
use rocket::serde::Serialize;

use super::utils::latest_status;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OCIRepoView {
    name: String,
    namespace: String,
    source: String,
    status: String,
}

impl From<OCIRepository> for OCIRepoView {
    fn from(k: OCIRepository) -> Self {
        OCIRepoView {
            name: k.metadata.name.unwrap_or_default(),
            namespace: k.metadata.namespace.unwrap_or_default(),
            source: k.spec.url,
            status: match k.status {
                Some(status) => latest_status(status.conditions),
                None => String::new(),
            },
        }
    }
}
