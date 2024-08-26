use kube_custom_resources_rs::source_toolkit_fluxcd_io::v1::helmrepositories::HelmRepository;
use rocket::serde::Serialize;

use super::utils::latest_status;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HelmRepoView {
    name: String,
    namespace: String,
    url: String,
    status: String,
    suspended: bool,
}

impl From<HelmRepository> for HelmRepoView {
    fn from(hr: HelmRepository) -> Self {
        HelmRepoView {
            name: hr.metadata.name.unwrap_or_default(),
            namespace: hr.metadata.namespace.unwrap_or_default(),
            url: hr.spec.url,
            status: match hr.status {
                Some(status) => latest_status(status.conditions),
                None => String::new(),
            },
            suspended: hr.spec.suspend.unwrap_or(false),
        }
    }
}
