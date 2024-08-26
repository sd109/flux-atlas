use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1::gitrepositories::GitRepository;
use rocket::serde::Serialize;

use super::utils::latest_status;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GitRepoView {
    name: String,
    namespace: String,
    source: String,
    status: String,
}

impl From<GitRepository> for GitRepoView {
    fn from(k: GitRepository) -> Self {
        GitRepoView {
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
